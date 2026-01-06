//! 노트 링크 시스템
//!
//! [[노트제목]] 형식의 위키 스타일 링크를 파싱하고 렌더링합니다.

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

/// [[링크]] 파싱용 정규식
static LINK_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[\[([^\[\]]+)\]\]").expect("Invalid regex"));

/// 노트 내용에서 [[링크]]들을 추출
///
/// # Example
/// ```
/// let content = "오늘 [[Rust]] 공부하고 [[Lazarus]] 개발했다.";
/// let links = extract_links(content);
/// assert_eq!(links, vec!["Rust", "Lazarus"]);
/// ```
pub fn extract_links(content: &str) -> Vec<String> {
    LINK_REGEX
        .captures_iter(content)
        .map(|cap| cap[1].to_string())
        .collect()
}

/// [[링크]]를 클릭 가능한 HTML 링크로 변환
///
/// # Example
/// ```
/// let content = "[[Rust]]는 좋은 언어다.";
/// let html = render_links(content, None);
/// // <a href="/notes/by-title/Rust" class="note-link">Rust</a>는 좋은 언어다.
/// ```
pub fn render_links(content: &str, existing_titles: Option<&HashSet<String>>) -> String {
    LINK_REGEX
        .replace_all(content, |caps: &regex::Captures| {
            let title = &caps[1];
            let encoded = urlencoding::encode(title);

            // 존재하는 노트인지 확인
            let exists = existing_titles
                .map(|titles| titles.contains(title))
                .unwrap_or(true); // 체크 안 하면 존재한다고 가정

            let class = if exists {
                "note-link"
            } else {
                "note-link note-link-missing" // 없는 노트는 다른 스타일
            };

            format!(
                r#"<a href="/notes/by-title/{}" class="{}">{}</a>"#,
                encoded, class, title
            )
        })
        .to_string()
}

/// 백링크 인덱스
///
/// 노트 간의 링크 관계를 양방향으로 추적합니다.
#[derive(Debug, Default)]
pub struct LinkIndex {
    /// 노트 ID → 이 노트가 링크하는 노트 제목들
    outgoing: HashMap<u64, HashSet<String>>,
    /// 노트 제목 → 이 노트를 링크하는 노트 ID들 (백링크)
    incoming: HashMap<String, HashSet<u64>>,
    /// 노트 ID → 제목 매핑 (역방향 조회용)
    id_to_title: HashMap<u64, String>,
    /// 노트 제목 → ID 매핑
    title_to_id: HashMap<String, u64>,
}

impl LinkIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// 노트 등록 (제목-ID 매핑)
    pub fn register_note(&mut self, id: u64, title: &str) {
        // 기존 제목 제거 (제목 변경된 경우)
        if let Some(old_title) = self.id_to_title.get(&id) {
            self.title_to_id.remove(old_title);
        }

        self.id_to_title.insert(id, title.to_string());
        self.title_to_id.insert(title.to_string(), id);
    }

    /// 노트 삭제
    pub fn remove_note(&mut self, id: u64) {
        // outgoing 링크 제거
        if let Some(targets) = self.outgoing.remove(&id) {
            for target_title in targets {
                if let Some(backlinks) = self.incoming.get_mut(&target_title) {
                    backlinks.remove(&id);
                }
            }
        }

        // 제목 매핑 제거
        if let Some(title) = self.id_to_title.remove(&id) {
            self.title_to_id.remove(&title);
            // 이 노트를 향하던 백링크들 제거
            self.incoming.remove(&title);
        }
    }

    /// 노트 내용 업데이트 시 링크 인덱스 갱신
    pub fn update_links(&mut self, note_id: u64, content: &str) {
        // 기존 outgoing 링크 제거
        if let Some(old_targets) = self.outgoing.remove(&note_id) {
            for target_title in old_targets {
                if let Some(backlinks) = self.incoming.get_mut(&target_title) {
                    backlinks.remove(&note_id);
                }
            }
        }

        // 새 링크 추출 및 등록
        let new_links: HashSet<String> = extract_links(content).into_iter().collect();

        for target_title in &new_links {
            self.incoming
                .entry(target_title.clone())
                .or_default()
                .insert(note_id);
        }

        self.outgoing.insert(note_id, new_links);
    }

    /// 특정 노트를 링크하는 노트들 (백링크) 조회
    pub fn get_backlinks(&self, title: &str) -> Vec<u64> {
        self.incoming
            .get(title)
            .map(|ids| ids.iter().copied().collect())
            .unwrap_or_default()
    }

    /// 특정 노트가 링크하는 노트들 조회
    pub fn get_outgoing_links(&self, note_id: u64) -> Vec<String> {
        self.outgoing
            .get(&note_id)
            .map(|titles| titles.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// 제목으로 노트 ID 조회
    pub fn get_id_by_title(&self, title: &str) -> Option<u64> {
        self.title_to_id.get(title).copied()
    }

    /// ID로 노트 제목 조회
    pub fn get_title_by_id(&self, id: u64) -> Option<&str> {
        self.id_to_title.get(&id).map(|s| s.as_str())
    }

    /// 모든 노트 제목 목록 (자동완성용)
    pub fn all_titles(&self) -> Vec<&str> {
        self.title_to_id.keys().map(|s| s.as_str()).collect()
    }

    /// 존재하는 노트 제목 집합 반환
    pub fn existing_titles(&self) -> HashSet<String> {
        self.title_to_id.keys().cloned().collect()
    }

    /// 그래프 뷰용 데이터 - 모든 링크 관계 반환
    pub fn get_graph_data(&self) -> GraphData {
        let mut nodes: Vec<GraphNode> = Vec::new();
        let mut edges: Vec<GraphEdge> = Vec::new();

        // 노드 생성
        for (id, title) in &self.id_to_title {
            let backlink_count = self.incoming.get(title).map(|s| s.len()).unwrap_or(0);

            nodes.push(GraphNode {
                id: *id,
                title: title.clone(),
                backlink_count,
            });
        }

        // 엣지 생성
        for (source_id, targets) in &self.outgoing {
            for target_title in targets {
                if let Some(target_id) = self.title_to_id.get(target_title) {
                    edges.push(GraphEdge {
                        source: *source_id,
                        target: *target_id,
                    });
                }
            }
        }

        GraphData { nodes, edges }
    }
}

/// 그래프 뷰용 노드 데이터
#[derive(Debug, Clone, serde::Serialize)]
pub struct GraphNode {
    pub id: u64,
    pub title: String,
    pub backlink_count: usize,
}

/// 그래프 뷰용 엣지 데이터
#[derive(Debug, Clone, serde::Serialize)]
pub struct GraphEdge {
    pub source: u64,
    pub target: u64,
}

/// 그래프 뷰용 전체 데이터
#[derive(Debug, Clone, serde::Serialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links() {
        let content = "오늘 [[Rust]] 공부하고 [[Lazarus]] 개발했다.";
        let links = extract_links(content);
        assert_eq!(links, vec!["Rust", "Lazarus"]);
    }

    #[test]
    fn test_extract_links_empty() {
        let content = "링크가 없는 노트";
        let links = extract_links(content);
        assert!(links.is_empty());
    }

    #[test]
    fn test_extract_links_nested_brackets() {
        let content = "[[노트 [특수]]]는 안 됨"; // 중첩 괄호는 무시
        let links = extract_links(content);
        assert!(links.is_empty());
    }

    #[test]
    fn test_render_links() {
        let content = "[[Rust]]는 좋아";
        let html = render_links(content, None);
        assert!(html.contains(r#"href="/notes/by-title/Rust""#));
        assert!(html.contains("class=\"note-link\""));
    }

    #[test]
    fn test_render_links_missing() {
        let content = "[[없는노트]]";
        let existing: HashSet<String> = HashSet::new();
        let html = render_links(content, Some(&existing));
        assert!(html.contains("note-link-missing"));
    }

    #[test]
    fn test_link_index_backlinks() {
        let mut index = LinkIndex::new();

        // 노트 등록
        index.register_note(1, "Rust");
        index.register_note(2, "프로그래밍");
        index.register_note(3, "일기");

        // 링크 업데이트
        index.update_links(2, "[[Rust]]는 좋은 언어");
        index.update_links(3, "오늘 [[Rust]] 공부함. [[프로그래밍]] 재밌다.");

        // 백링크 확인
        let rust_backlinks = index.get_backlinks("Rust");
        assert_eq!(rust_backlinks.len(), 2);
        assert!(rust_backlinks.contains(&2));
        assert!(rust_backlinks.contains(&3));

        let prog_backlinks = index.get_backlinks("프로그래밍");
        assert_eq!(prog_backlinks.len(), 1);
        assert!(prog_backlinks.contains(&3));
    }

    #[test]
    fn test_graph_data() {
        let mut index = LinkIndex::new();

        index.register_note(1, "A");
        index.register_note(2, "B");
        index.update_links(1, "[[B]] 링크");

        let graph = index.get_graph_data();
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0].source, 1);
        assert_eq!(graph.edges[0].target, 2);
    }
}
