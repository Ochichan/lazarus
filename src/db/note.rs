//! 노트 데이터 구조
//!
//! rkyv를 사용한 제로카피 직렬화

use chrono::{DateTime, Utc};
use rkyv::{Archive, Deserialize, Serialize};
use bytecheck::CheckBytes;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

/// 저장되는 노트의 원자 단위
#[derive(Archive, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct NoteAtom {
    /// 고유 ID
    pub id: u64,
    
    /// 생성 타임스탬프 (Unix timestamp)
    pub created_at: i64,
    
    /// 수정 타임스탬프 (Unix timestamp)
    pub updated_at: i64,
    
    /// Zstd 압축된 콘텐츠
    #[with(rkyv::with::Raw)]
    pub content: Vec<u8>,
    
    /// 벡터 임베딩 (선택, 검색용)
    pub vector: Option<Vec<i8>>,
    
    /// 암호화 여부
    pub encrypted: bool,
    
    /// 삭제 표시 (soft delete)
    pub deleted: bool,
}

/// API 응답용 노트 구조체
#[derive(Debug, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub encrypted: bool,
}

impl Note {
    /// 새 노트 생성
    pub fn new(id: u64, title: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            content,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            encrypted: false,
        }
    }

    /// 마크다운 형식으로 직렬화
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        
        // YAML 프론트매터
        md.push_str("---\n");
        md.push_str(&format!("id: {}\n", self.id));
        md.push_str(&format!("title: \"{}\"\n", self.title.replace('"', "\\\"")));
        md.push_str(&format!("created: {}\n", self.created_at.to_rfc3339()));
        md.push_str(&format!("updated: {}\n", self.updated_at.to_rfc3339()));
        if !self.tags.is_empty() {
            md.push_str(&format!("tags: [{}]\n", self.tags.join(", ")));
        }
        md.push_str("---\n\n");
        
        // 본문
        md.push_str(&self.content);
        
        md
    }

    /// 마크다운에서 파싱
    pub fn from_markdown(id: u64, markdown: &str) -> Option<Self> {
        // 간단한 프론트매터 파싱
        if !markdown.starts_with("---\n") {
            return Some(Self::new(id, String::new(), markdown.to_string()));
        }

        let parts: Vec<&str> = markdown.splitn(3, "---\n").collect();
        if parts.len() < 3 {
            return Some(Self::new(id, String::new(), markdown.to_string()));
        }

        let frontmatter = parts[1];
        let content = parts[2].trim().to_string();

        let mut title = String::new();
        let mut tags = Vec::new();

        for line in frontmatter.lines() {
            if let Some(t) = line.strip_prefix("title: ") {
                title = t.trim_matches('"').to_string();
            } else if let Some(t) = line.strip_prefix("tags: ") {
                // [tag1, tag2] 형식 파싱
                let t = t.trim_matches(|c| c == '[' || c == ']');
                tags = t.split(',').map(|s| s.trim().to_string()).collect();
            }
        }

        Some(Self {
            id,
            title,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags,
            encrypted: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_to_markdown() {
        let note = Note::new(1, "테스트 제목".to_string(), "본문 내용".to_string());
        let md = note.to_markdown();
        
        assert!(md.contains("title: \"테스트 제목\""));
        assert!(md.contains("본문 내용"));
    }

    #[test]
    fn test_note_from_markdown() {
        let md = r#"---
id: 1
title: "테스트"
tags: [rust, pkm]
---

본문입니다."#;
        
        let note = Note::from_markdown(1, md).unwrap();
        assert_eq!(note.title, "테스트");
        assert_eq!(note.tags, vec!["rust", "pkm"]);
        assert_eq!(note.content, "본문입니다.");
    }
}
