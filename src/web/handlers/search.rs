//! 검색 API 핸들러

use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::web::state::AppState;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    20
}

#[derive(Serialize)]
pub struct SearchResult {
    pub id: u64,
    pub title: String,
    pub preview: String,
    pub source: String,
    pub score: f32,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub total: usize,
}

/// GET /api/search
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<SearchResponse>> {
    let mut results = Vec::new();

    if !params.q.is_empty() {
        let db = state.db.read().await;
        let query_lower = params.q.to_lowercase();

        // 간단한 텍스트 검색 (나중에 Tantivy로 대체)
        for id in db.list_ids() {
            if let Some(note) = db.get(id)? {
                let title_match = note.title.to_lowercase().contains(&query_lower);
                let content_match = note.content.to_lowercase().contains(&query_lower);

                if title_match || content_match {
                    // 간단한 점수 계산
                    let score = if title_match { 2.0 } else { 1.0 };

                    results.push(SearchResult {
                        id: note.id,
                        title: note.title,
                        preview: note.content.chars().take(150).collect::<String>() + "...",
                        source: "note".to_string(),
                        score,
                    });
                }
            }
        }

        // 점수순 정렬
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(params.limit);
    }

    let total = results.len();

    Ok(Json(SearchResponse {
        query: params.q,
        results,
        total,
    }))
}
