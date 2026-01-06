//! 그래프 뷰 API

use axum::{extract::State, Json};
use crate::error::Result;
use crate::links::GraphData;
use crate::web::state::AppState;

/// GET /api/graph - 그래프 데이터 반환
pub async fn get_graph_data(
    State(state): State<AppState>,
) -> Result<Json<GraphData>> {
    let index = state.link_index.read().await;
    let data = index.get_graph_data();
    Ok(Json(data))
}
