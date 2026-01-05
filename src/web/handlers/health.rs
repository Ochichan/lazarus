//! 헬스체크 엔드포인트

use axum::{extract::State, Json};
use serde::Serialize;

use crate::web::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub notes_count: usize,
}

/// GET /health
pub async fn check(State(state): State<AppState>) -> Json<HealthResponse> {
    let db = state.db.read().await;

    Json(HealthResponse {
        status: "ok",
        version: state.version,
        notes_count: db.count(),
    })
}
