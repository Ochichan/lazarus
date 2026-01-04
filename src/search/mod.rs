//! 검색 레이어
//!
//! Tantivy 기반 풀텍스트 검색
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur};
use std::path::Path;
use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    doc,
    query::QueryParser,
    schema::{Schema, STORED, TEXT, Field, NumericOptions, Value},
    Index, IndexReader, IndexWriter, ReloadPolicy, TantivyDocument,
};

use crate::error::{LazarusError, Result};

/// 검색 엔진
pub struct SearchEngine {
    index: Index,
    reader: IndexReader,
    writer: IndexWriter,
    
    // 필드들
    field_id: Field,
    field_title: Field,
    field_content: Field,
    field_tags: Field,
}

impl SearchEngine {
    /// 검색 엔진 생성
    pub fn open<P: AsRef<Path>>(index_path: P) -> Result<Self> {
        let index_path = index_path.as_ref();
        
        // 디렉토리 생성
        std::fs::create_dir_all(index_path)?;
        
        // 스키마 정의
        let mut schema_builder = Schema::builder();
        
        let field_id = schema_builder.add_u64_field("id", NumericOptions::default().set_stored());
        let field_title = schema_builder.add_text_field("title", TEXT | STORED);
        let field_content = schema_builder.add_text_field("content", TEXT | STORED);
        let field_tags = schema_builder.add_text_field("tags", TEXT | STORED);
        
        let schema = schema_builder.build();
        
        // 인덱스 열기 또는 생성
        let dir = MmapDirectory::open(index_path)
            .map_err(|e| LazarusError::IndexCreate(e.to_string()))?;
            
        let index = Index::open_or_create(dir, schema.clone())
            .map_err(|e| LazarusError::IndexCreate(e.to_string()))?;
        
        // 리더 생성
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .map_err(|e| LazarusError::IndexCreate(e.to_string()))?;
        
        // 라이터 생성 (15MB 힙 메모리)
        let writer = index
            .writer(15_000_000)
            .map_err(|e| LazarusError::IndexCreate(e.to_string()))?;
        
        tracing::info!("SearchEngine 초기화 완료: {}", index_path.display());
        
        Ok(Self {
            index,
            reader,
            writer,
            field_id,
            field_title,
            field_content,
            field_tags,
        })
    }
    
    /// 노트 인덱싱
    pub fn index_note(&mut self, id: u64, title: &str, content: &str, tags: &[String]) -> Result<()> {
        // 기존 문서 삭제 (업데이트를 위해)
        self.delete_note(id)?;
        
        let tags_str = tags.join(" ");
        
        self.writer.add_document(doc!(
            self.field_id => id,
            self.field_title => title,
            self.field_content => content,
            self.field_tags => tags_str,
        )).map_err(|e| LazarusError::IndexCreate(e.to_string()))?;
        
        self.writer.commit()
            .map_err(|e| LazarusError::IndexCreate(e.to_string()))?;
        
        tracing::debug!("노트 인덱싱: id={}", id);
        
        Ok(())
    }
    
    /// 노트 삭제
    pub fn delete_note(&mut self, id: u64) -> Result<()> {
        let term = tantivy::Term::from_field_u64(self.field_id, id);
        self.writer.delete_term(term);
        Ok(())
    }
 
    /// 검색 실행
    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
        if query_str.trim().is_empty() {
            return Ok(Vec::new());
        }
        
        let searcher = self.reader.searcher();
        
        // 제목과 내용 필드에서 검색
        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.field_title, self.field_content, self.field_tags],
        );
        
        let query = query_parser
            .parse_query(query_str)
            .map_err(|e| LazarusError::SearchFailed(e.to_string()))?;
        
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit))
            .map_err(|e| LazarusError::SearchFailed(e.to_string()))?;
        
        let mut results = Vec::new();
        
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)
                .map_err(|e| LazarusError::SearchFailed(e.to_string()))?;
            
            let id = doc
                .get_first(self.field_id)
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
                
            let title = doc
                .get_first(self.field_title)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
                
            let content = doc
                .get_first(self.field_content)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            // 미리보기 생성 (첫 150자)
            let preview = content.chars().take(150).collect::<String>() + "...";
            
            results.push(SearchResult {
                id,
                title,
                preview,
                score,
            });
        }
        
        Ok(results)
    }
    /// Fuzzy 검색 (오타 허용)
        pub fn search_fuzzy(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
            if query_str.trim().is_empty() {
                return Ok(Vec::new());
            }
    
            let searcher = self.reader.searcher();
            
            // 먼저 일반 검색 시도
            let normal_results = self.search(query_str, limit)?;
            if !normal_results.is_empty() {
                return Ok(normal_results);
            }
    
            // 결과 없으면 Fuzzy 검색
            let terms: Vec<&str> = query_str.split_whitespace().collect();
            if terms.is_empty() {
                return Ok(Vec::new());
            }
    
            let mut subqueries: Vec<(Occur, Box<dyn tantivy::query::Query>)> = Vec::new();
    
            for term in &terms {
                // 제목 필드 Fuzzy (edit distance 1)
                let title_term = tantivy::Term::from_field_text(self.field_title, term);
                let title_fuzzy = FuzzyTermQuery::new(title_term, 1, true);
                subqueries.push((Occur::Should, Box::new(title_fuzzy)));
    
                // 내용 필드 Fuzzy (edit distance 1)
                let content_term = tantivy::Term::from_field_text(self.field_content, term);
                let content_fuzzy = FuzzyTermQuery::new(content_term, 1, true);
                subqueries.push((Occur::Should, Box::new(content_fuzzy)));
    
                // 태그 필드 Fuzzy
                let tags_term = tantivy::Term::from_field_text(self.field_tags, term);
                let tags_fuzzy = FuzzyTermQuery::new(tags_term, 1, true);
                subqueries.push((Occur::Should, Box::new(tags_fuzzy)));
            }
    
            let query = BooleanQuery::new(subqueries);
    
            let top_docs = searcher
                .search(&query, &TopDocs::with_limit(limit))
                .map_err(|e| LazarusError::SearchFailed(e.to_string()))?;
    
            let mut results = Vec::new();
            for (score, doc_address) in top_docs {
                let doc: TantivyDocument = searcher.doc(doc_address)
                    .map_err(|e| LazarusError::SearchFailed(e.to_string()))?;
    
                let id = doc
                    .get_first(self.field_id)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
    
                let title = doc
                    .get_first(self.field_title)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
    
                let content = doc
                    .get_first(self.field_content)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
    
                let preview = content.chars().take(150).collect::<String>() + "...";
    
                results.push(SearchResult {
                    id,
                    title,
                    preview,
                    score,
                });
            }
    
            Ok(results)
        }
}

/// 검색 결과
#[derive(Debug)]
pub struct SearchResult {
    pub id: u64,
    pub title: String,
    pub preview: String,
    pub score: f32,
}
