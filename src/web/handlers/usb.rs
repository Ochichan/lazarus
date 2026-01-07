//! USB API 핸들러
//!
//! /api/usb/* 엔드포인트

use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::sync::{LazarusUsb, SyncManager, UsbManifest};
use crate::web::state::AppState;

/// USB 목록 응답
#[derive(Serialize)]
pub struct UsbListResponse {
    pub usbs: Vec<UsbInfo>,
    pub count: usize,
}

/// USB 정보 (API용)
#[derive(Serialize)]
pub struct UsbInfo {
    pub name: String,
    pub path: String,
    pub note_count: usize,
    pub post_count: usize,
    pub qna_count: usize,
    pub package_count: usize,
    pub total_content: usize,
    pub has_manifest: bool,
}

impl From<LazarusUsb> for UsbInfo {
    fn from(usb: LazarusUsb) -> Self {
        Self {
            name: usb.name.clone(),
            path: usb.path.display().to_string(),
            note_count: usb.note_count,
            post_count: usb.post_count,
            qna_count: usb.qna_count,
            package_count: usb.package_count,
            total_content: usb.total_content(),
            has_manifest: usb.has_manifest,
        }
    }
}

/// USB 초기화 요청
#[derive(Deserialize)]
pub struct InitUsbRequest {
    pub path: String,
}

/// 일반 응답
#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

/// USB 목록 조회
/// GET /api/usb
pub async fn list_usbs() -> Json<UsbListResponse> {
    let manager = SyncManager::new();
    let usbs = manager.scan().await;

    let usb_infos: Vec<UsbInfo> = usbs.into_iter().map(|u| u.into()).collect();
    let count = usb_infos.len();

    Json(UsbListResponse {
        usbs: usb_infos,
        count,
    })
}

/// USB 스캔 (수동)
/// POST /api/usb/scan
pub async fn scan_usbs() -> Json<UsbListResponse> {
    let manager = SyncManager::new();
    let usbs = manager.scan().await;

    let usb_infos: Vec<UsbInfo> = usbs.into_iter().map(|u| u.into()).collect();
    let count = usb_infos.len();

    Json(UsbListResponse {
        usbs: usb_infos,
        count,
    })
}

/// USB 초기화
/// POST /api/usb/init
pub async fn init_usb(
    Json(req): Json<InitUsbRequest>,
) -> Result<Json<ApiResponse>, (StatusCode, Json<ApiResponse>)> {
    let path = std::path::Path::new(&req.path);

    match SyncManager::init_usb(path) {
        Ok(()) => Ok(Json(ApiResponse {
            success: true,
            message: format!("USB 초기화 완료: {}", req.path),
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: format!("USB 초기화 실패: {}", e),
            }),
        )),
    }
}

/// USB 매니페스트 조회
/// GET /api/usb/manifest?path=/path/to/usb
pub async fn get_manifest(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<UsbManifest>, (StatusCode, Json<ApiResponse>)> {
    let path = params.get("path").ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "path 파라미터 필요".to_string(),
            }),
        )
    })?;

    let usb_path = std::path::Path::new(path);

    UsbManifest::load(usb_path).map(Json).map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!("매니페스트 로드 실패: {}", e),
            }),
        )
    })
}
