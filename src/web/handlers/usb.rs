//! USB API 핸들러
//!
//! /api/usb/* 엔드포인트

use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::sync::{LazarusUsb, UsbDetector, UsbManifest};
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
    let manager = UsbDetector::new();
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
    let manager = UsbDetector::new();
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

    match UsbDetector::init_usb(path) {
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
    if !UsbDetector::is_lazarus_usb(usb_path) {
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

    let note_count = match export_notes(usb_path, &notes_to_export) {
        Ok(count) => count,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: format!("노트 내보내기 실패: {}", e),
                }),
            ))
        }
    };

    // 패키지 내보내기
    let packages_store = state.packages.read().await;
    let local_packages = packages_store.list();
    let packages_dir = state.data_dir.join("packages");
    drop(packages_store);

    let pkg_count = if let Ok((_, uploaded, _)) =
        crate::sync::sync_packages(usb_path, &local_packages, &packages_dir)
    {
        uploaded
    } else {
        0
    };

    Ok(Json(ExportResponse {
        success: true,
        count: note_count + pkg_count,
        message: format!(
            "{}개 노트, {}개 패키지 내보내기 완료",
            note_count, pkg_count
        ),
    }))
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
    if !UsbDetector::is_lazarus_usb(usb_path) {
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
    Json(req): Json<ImportRequest>,
) -> Result<Json<SyncResponse>, (StatusCode, Json<ApiResponse>)> {
    use crate::sync::{sync_notes, sync_posts, sync_qna};

    let usb_path = std::path::Path::new(&req.usb_path);

    if !UsbDetector::is_lazarus_usb(usb_path) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Not a Valid USB".to_string(),
            }),
        ));
    }

    let mut total_uploaded = 0;
    let mut total_downloaded = 0;
    let mut total_conflicts = 0;
    let mut total_unchanged = 0;

    // === Notes 동기화 ===
    let db = state.db.read().await;
    let local_notes: Vec<_> = db
        .list_ids()
        .into_iter()
        .filter_map(|id| db.get(id).ok().flatten())
        .collect();
    drop(db);

    let state_clone = state.clone();
    let save_fn = |note: &crate::db::Note| -> Result<(), crate::sync::SyncError> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let mut db = state_clone.db.write().await;
            db.save(note, None)
                .map_err(|e| crate::sync::SyncError::Io(std::io::Error::other(e.to_string())))?;
            Ok(())
        })
    };

    if let Ok(result) = sync_notes(usb_path, &local_notes, save_fn) {
        total_uploaded += result.uploaded;
        total_downloaded += result.downloaded;
        total_conflicts += result.conflicts;
        total_unchanged += result.unchanged;
    }

    // === Posts 동기화 ===
    let posts_store = state.posts.read().await;
    let local_posts = posts_store.all();
    drop(posts_store);

    if let Ok((downloaded_posts, uploaded)) = sync_posts(usb_path, &local_posts) {
        total_uploaded += uploaded;
        total_downloaded += downloaded_posts.len();

        // 다운로드한 posts 저장
        if !downloaded_posts.is_empty() {
            let mut posts_store = state.posts.write().await;
            let _ = posts_store.merge(downloaded_posts);
        }
    }

    // === Q&A 동기화 ===
    let qna_store = state.qna.read().await;
    let local_questions = qna_store.all();
    drop(qna_store);

    if let Ok((downloaded_questions, uploaded)) = sync_qna(usb_path, &local_questions) {
        total_uploaded += uploaded;
        total_downloaded += downloaded_questions.len();

        // 다운로드한 questions 저장
        if !downloaded_questions.is_empty() {
            let mut qna_store = state.qna.write().await;
            let _ = qna_store.merge(downloaded_questions);
        }
        // === Packages 동기화 ===
        let packages_store = state.packages.read().await;
        let local_packages: Vec<_> = packages_store.list();
        let packages_dir = state.data_dir.join("packages");
        drop(packages_store);

        if let Ok((downloaded_files, uploaded, _)) =
            crate::sync::sync_packages(usb_path, &local_packages, &packages_dir)
        {
            total_uploaded += uploaded;
            total_downloaded += downloaded_files.len();

            // 새 패키지 인덱스 갱신
            if !downloaded_files.is_empty() {
                let packages_store = state.packages.write().await;
                // PackageStore가 자동으로 sync_index 하므로 drop 후 reopen이 필요
                drop(packages_store);
            }
        }
    }

    Ok(Json(SyncResponse {
        success: true,
        uploaded: total_uploaded,
        downloaded: total_downloaded,
        conflicts: total_conflicts,
        unchanged: total_unchanged,
        message: format!("Sync Complete: ↑{} ↓{}", total_uploaded, total_downloaded),
    }))
}
