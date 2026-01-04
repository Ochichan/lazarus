//! USB 동기화 핸들러
//!
//! 스니커넷 방식의 데이터 동기화

use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::web::state::AppState;

#[derive(Deserialize)]
pub struct ExportRequest {
    /// 내보낼 경로 (USB 마운트 포인트)
    pub path: String,
    /// 마지막 동기화 이후만 내보내기
    #[serde(default)]
    pub incremental: bool,
}

#[derive(Serialize)]
pub struct ExportResponse {
    pub success: bool,
    pub notes_exported: usize,
    pub path: String,
}

/// POST /sync/export
pub async fn export(
    State(_state): State<AppState>,
    Json(req): Json<ExportRequest>,
) -> Result<Json<ExportResponse>> {
    // TODO: 실제 내보내기 구현
    // 1. 지정된 경로에 lazarus_sync 폴더 생성
    // 2. 노트들을 YAML/Markdown 형식으로 내보내기
    // 3. 메타데이터(마지막 동기화 시간 등) 저장

    tracing::info!("내보내기 요청: path={}", req.path);

    Ok(Json(ExportResponse {
        success: true,
        notes_exported: 0, // TODO
        path: req.path,
    }))
}

#[derive(Deserialize)]
pub struct ImportRequest {
    /// 가져올 경로 (USB 마운트 포인트)
    pub path: String,
}

#[derive(Serialize)]
pub struct ImportResponse {
    pub success: bool,
    pub notes_imported: usize,
    pub notes_updated: usize,
    pub conflicts: usize,
}

/// POST /sync/import
pub async fn import(
    State(_state): State<AppState>,
    Json(req): Json<ImportRequest>,
) -> Result<Json<ImportResponse>> {
    // TODO: 실제 가져오기 구현
    // 1. lazarus_sync 폴더에서 데이터 읽기
    // 2. 충돌 감지 (동일 ID, 다른 수정 시간)
    // 3. 병합 또는 사용자에게 선택 요청

    tracing::info!("가져오기 요청: path={}", req.path);

    Ok(Json(ImportResponse {
        success: true,
        notes_imported: 0, // TODO
        notes_updated: 0,
        conflicts: 0,
    }))
}
