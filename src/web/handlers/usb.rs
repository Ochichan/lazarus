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

/// Export 요청
#[derive(Deserialize)]
pub struct ExportRequest {
    pub usb_path: String,
    pub note_ids: Option<Vec<u64>>, // None이면 전체
}

/// Export 응답
#[derive(Serialize)]
pub struct ExportResponse {
    pub success: bool,
    pub count: usize,
    pub message: String,
}

/// POST /api/usb/export - 노트를 USB로 내보내기
pub async fn export_to_usb(
    State(state): State<AppState>,
    Json(req): Json<ExportRequest>,
) -> Result<Json<ExportResponse>, (StatusCode, Json<ApiResponse>)> {
    use crate::sync::export_notes;

    let usb_path = std::path::Path::new(&req.usb_path);

    // USB 경로 검증
    if !SyncManager::is_lazarus_usb(usb_path) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "유효한 Lazarus USB가 아닙니다".to_string(),
            }),
        ));
    }

    // 노트 가져오기
    let db = state.db.read().await;
    let all_notes: Vec<_> = db
        .list_ids()
        .into_iter()
        .filter_map(|id| db.get(id).ok().flatten())
        .collect();

    let notes_to_export: Vec<_> = if let Some(ids) = req.note_ids {
        all_notes
            .into_iter()
            .filter(|n| ids.contains(&n.id))
            .collect()
    } else {
        all_notes
    };

    match export_notes(usb_path, &notes_to_export) {
        Ok(count) => Ok(Json(ExportResponse {
            success: true,
            count,
            message: format!("{} 노트 내보내기 완료", count),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("내보내기 실패: {}", e),
            }),
        )),
    }
}

/// Import 요청
#[derive(Deserialize)]
pub struct ImportRequest {
    pub usb_path: String,
}

/// Import 응답
#[derive(Serialize)]
pub struct ImportResponse {
    pub success: bool,
    pub count: usize,
    pub message: String,
}

/// POST /api/usb/import - USB에서 노트 가져오기
pub async fn import_from_usb(
    State(state): State<AppState>,
    Json(req): Json<ImportRequest>,
) -> Result<Json<ImportResponse>, (StatusCode, Json<ApiResponse>)> {
    use crate::sync::import_notes;

    let usb_path = std::path::Path::new(&req.usb_path);

    // USB 경로 검증
    if !SyncManager::is_lazarus_usb(usb_path) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "유효한 Lazarus USB가 아닙니다".to_string(),
            }),
        ));
    }

    match import_notes(usb_path) {
        Ok(notes) => {
            let count = notes.len();
            // DB에 저장
            let mut db = state.db.write().await;
            for note in notes {
                let _ = db.save(&note, None);
            }

            Ok(Json(ImportResponse {
                success: true,
                count,
                message: format!("{} 노트 가져오기 완료", count),
            }))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("가져오기 실패: {}", e),
            }),
        )),
    }
}

/// USB 삭제 요청
#[derive(Deserialize)]
pub struct RemoveUsbRequest {
    pub path: String,
}

/// DELETE /api/usb/remove - 목록에서 USB 제거
pub async fn remove_usb(Json(req): Json<RemoveUsbRequest>) -> Json<ApiResponse> {
    // lazarus.sync 파일 삭제하면 더 이상 Lazarus USB로 인식 안 됨
    let sync_file = std::path::Path::new(&req.path).join("lazarus.sync");

    if sync_file.exists() {
        match std::fs::remove_file(&sync_file) {
            Ok(_) => Json(ApiResponse {
                success: true,
                message: format!("USB 제거됨: {}", req.path),
            }),
            Err(e) => Json(ApiResponse {
                success: false,
                message: format!("제거 실패: {}", e),
            }),
        }
    } else {
        Json(ApiResponse {
            success: true,
            message: "이미 제거됨".to_string(),
        })
    }
}

/// Sync 응답
#[derive(Serialize)]
pub struct SyncResponse {
    pub success: bool,
    pub uploaded: usize,
    pub downloaded: usize,
    pub conflicts: usize,
    pub unchanged: usize,
    pub message: String,
}

/// POST /api/usb/sync - 양방향 동기화
pub async fn sync_with_usb(
    State(state): State<AppState>,
    Json(req): Json<ImportRequest>,  // usb_path만 필요하니 재사용
) -> Result<Json<SyncResponse>, (StatusCode, Json<ApiResponse>)> {
    use crate::sync::sync_notes;

    let usb_path = std::path::Path::new(&req.usb_path);

    if !SyncManager::is_lazarus_usb(usb_path) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "유효한 Lazarus USB가 아닙니다".to_string(),
            }),
        ));
    }

    // Local 노트 가져오기
    let db = state.db.read().await;
    let local_notes: Vec<_> = db.list_ids()
        .into_iter()
        .filter_map(|id| db.get(id).ok().flatten())
        .collect();
    drop(db);  // 락 해제

    // 동기화 실행
    let state_clone = state.clone();
    let save_fn = |note: &crate::db::Note| -> Result<(), crate::sync::SyncError> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let mut db = state_clone.db.write().await;
            db.save(note, None).map_err(|e| crate::sync::SyncError::Io(
                std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
            ))?;
            Ok(())
        })
    };

    match sync_notes(usb_path, &local_notes, save_fn) {
        Ok(result) => Ok(Json(SyncResponse {
            success: true,
            uploaded: result.uploaded,
            downloaded: result.downloaded,
            conflicts: result.conflicts,
            unchanged: result.unchanged,
            message: format!("동기화 완료: ↑{} ↓{}", result.uploaded, result.downloaded),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("동기화 실패: {}", e),
            }),
        )),
    }
}
