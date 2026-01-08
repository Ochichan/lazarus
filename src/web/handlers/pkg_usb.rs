//! USB 패키지 동기화 API 핸들러 (pkg 시스템용)
//!
//! 기존 handlers/usb.rs와 별개로, src/laz/pkg 시스템용 USB 핸들러
//!
//! Endpoints:
//! - GET    /api/pkg/usb/drives           - USB 드라이브 목록
//! - GET    /api/pkg/usb/drives/:idx/scan - 드라이브 스캔
//! - POST   /api/pkg/usb/drives/:idx/init - Lazarus 폴더 초기화
//! - POST   /api/pkg/usb/export           - 로컬 → USB 내보내기
//! - POST   /api/pkg/usb/import           - USB → 로컬 가져오기

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::db::PackageStore;
use crate::sync::usb_packages::{
    SyncDirection, SyncResult, SyncStatus, UsbDrive, UsbPackageInfo, UsbPackageManager,
};

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Serialize)]
pub struct DriveListResponse {
    pub drives: Vec<UsbDriveInfo>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct UsbDriveInfo {
    pub index: usize,
    pub label: Option<String>,
    pub path: String,
    pub available: String,
    pub package_count: usize,
    pub has_lazarus_dir: bool,
}

impl UsbDriveInfo {
    fn from_drive(index: usize, drive: &UsbDrive) -> Self {
        Self {
            index,
            label: drive.label.clone(),
            path: drive.path.to_string_lossy().to_string(),
            available: drive.formatted_available(),
            package_count: drive.package_count,
            has_lazarus_dir: drive.has_lazarus_dir,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ScanResponse {
    pub drive: UsbDriveInfo,
    pub packages: Vec<UsbPackageInfo>,
    pub summary: ScanSummary,
    pub sync_status: Option<SyncStatus>,
}

#[derive(Debug, Serialize)]
pub struct ScanSummary {
    pub total_on_usb: usize,
    pub usb_only: usize,
    pub local_only: usize,
    pub usb_newer: usize,
    pub local_newer: usize,
    pub in_sync: usize,
}

#[derive(Debug, Deserialize)]
pub struct ExportRequest {
    pub drive_index: usize,
    #[serde(default)]
    pub package_ids: Vec<String>,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Deserialize)]
pub struct ImportRequest {
    pub drive_index: usize,
    #[serde(default)]
    pub package_ids: Vec<String>,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Serialize)]
pub struct SyncResponse {
    pub success: bool,
    pub direction: String,
    pub transferred: usize,
    pub skipped: usize,
    pub transferred_packages: Vec<String>,
    pub skipped_packages: Vec<SkippedInfo>,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SkippedInfo {
    pub id: String,
    pub reason: String,
}

impl From<SyncResult> for SyncResponse {
    fn from(result: SyncResult) -> Self {
        Self {
            success: result.is_success(),
            direction: match result.direction {
                SyncDirection::ToUsb => "export".to_string(),
                SyncDirection::FromUsb => "import".to_string(),
            },
            transferred: result.transferred,
            skipped: result.skipped,
            transferred_packages: result.transferred_packages,
            skipped_packages: result
                .skipped_packages
                .into_iter()
                .map(|(id, reason)| SkippedInfo { id, reason })
                .collect(),
            errors: result.errors,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

// ============================================================================
// State
// ============================================================================

#[derive(Clone)]
pub struct PkgUsbState {
    pub package_store: Arc<RwLock<PackageStore>>,
    pub usb_manager: Arc<UsbPackageManager>,
    pub drives_cache: Arc<RwLock<Vec<UsbDrive>>>,
}

impl PkgUsbState {
    pub fn new(package_store: Arc<RwLock<PackageStore>>) -> Self {
        Self {
            package_store,
            usb_manager: Arc::new(UsbPackageManager::new()),
            drives_cache: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/pkg/usb/drives
pub async fn list_drives(
    State(state): State<PkgUsbState>,
) -> Result<Json<DriveListResponse>, (StatusCode, Json<ApiResponse>)> {
    let drives = state.usb_manager.detect_drives();

    {
        let mut cache = state.drives_cache.write().await;
        *cache = drives.clone();
    }

    let drive_infos: Vec<UsbDriveInfo> = drives
        .iter()
        .enumerate()
        .map(|(i, d)| UsbDriveInfo::from_drive(i, d))
        .collect();

    Ok(Json(DriveListResponse {
        count: drive_infos.len(),
        drives: drive_infos,
    }))
}

/// GET /api/pkg/usb/drives/:idx/scan
pub async fn scan_drive(
    State(state): State<PkgUsbState>,
    Path(idx): Path<usize>,
) -> Result<Json<ScanResponse>, (StatusCode, Json<ApiResponse>)> {
    let drives = state.drives_cache.read().await;
    let drive = drives.get(idx).cloned().ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!("드라이브 인덱스 {}를 찾을 수 없습니다. 목록을 새로고침하세요.", idx),
            }),
        )
    })?;
    drop(drives);

    let store = state.package_store.read().await;
    let scan_result = state
        .usb_manager
        .scan_drive(&drive, &store)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: format!("드라이브 스캔 실패: {}", e),
                }),
            )
        })?;

    let sync_status = state.usb_manager.read_sync_status(&drive);

    Ok(Json(ScanResponse {
        drive: UsbDriveInfo::from_drive(idx, &drive),
        packages: scan_result.packages,
        summary: ScanSummary {
            total_on_usb: scan_result.packages.len(),
            usb_only: scan_result.usb_only.len(),
            local_only: scan_result.local_only.len(),
            usb_newer: scan_result.usb_newer.len(),
            local_newer: scan_result.local_newer.len(),
            in_sync: scan_result.in_sync.len(),
        },
        sync_status,
    }))
}

/// POST /api/pkg/usb/drives/:idx/init
pub async fn initialize_drive(
    State(state): State<PkgUsbState>,
    Path(idx): Path<usize>,
) -> Result<Json<ApiResponse>, (StatusCode, Json<ApiResponse>)> {
    let drives = state.drives_cache.read().await;
    let drive = drives.get(idx).cloned().ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!("드라이브 인덱스 {}를 찾을 수 없습니다.", idx),
            }),
        )
    })?;
    drop(drives);

    state.usb_manager.initialize_drive(&drive).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("드라이브 초기화 실패: {}", e),
            }),
        )
    })?;

    {
        let mut cache = state.drives_cache.write().await;
        if let Some(d) = cache.get_mut(idx) {
            d.has_lazarus_dir = true;
        }
    }

    Ok(Json(ApiResponse {
        success: true,
        message: "USB 드라이브에 Lazarus 패키지 폴더가 생성되었습니다.".to_string(),
    }))
}

/// POST /api/pkg/usb/export
pub async fn export_packages(
    State(state): State<PkgUsbState>,
    Json(request): Json<ExportRequest>,
) -> Result<Json<SyncResponse>, (StatusCode, Json<ApiResponse>)> {
    let drives = state.drives_cache.read().await;
    let drive = drives.get(request.drive_index).cloned().ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!(
                    "드라이브 인덱스 {}를 찾을 수 없습니다.",
                    request.drive_index
                ),
            }),
        )
    })?;
    drop(drives);

    let store = state.package_store.read().await;

    let package_ids = if request.package_ids.is_empty() {
        store.list().iter().map(|p| p.id.clone()).collect()
    } else {
        request.package_ids
    };

    let result = state
        .usb_manager
        .export_to_usb(&drive, &store, &package_ids, request.overwrite)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: format!("내보내기 실패: {}", e),
                }),
            )
        })?;

    Ok(Json(result.into()))
}

/// POST /api/pkg/usb/import
pub async fn import_packages(
    State(state): State<PkgUsbState>,
    Json(request): Json<ImportRequest>,
) -> Result<Json<SyncResponse>, (StatusCode, Json<ApiResponse>)> {
    let drives = state.drives_cache.read().await;
    let drive = drives.get(request.drive_index).cloned().ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!(
                    "드라이브 인덱스 {}를 찾을 수 없습니다.",
                    request.drive_index
                ),
            }),
        )
    })?;
    drop(drives);

    let mut store = state.package_store.write().await;

    let package_ids = if request.package_ids.is_empty() {
        let scan = state.usb_manager.scan_drive(&drive, &store).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: format!("드라이브 스캔 실패: {}", e),
                }),
            )
        })?;
        scan.packages.iter().map(|p| p.id.clone()).collect()
    } else {
        request.package_ids
    };

    let result = state
        .usb_manager
        .import_from_usb(&drive, &mut store, &package_ids, request.overwrite)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: format!("가져오기 실패: {}", e),
                }),
            )
        })?;

    Ok(Json(result.into()))
}

// ============================================================================
// Router
// ============================================================================

/// 라우터 생성
/// 사용: router.nest("/api/pkg/usb", pkg_usb_routes(state))
pub fn pkg_usb_routes(state: PkgUsbState) -> axum::Router {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/drives", get(list_drives))
        .route("/drives/:idx/scan", get(scan_drive))
        .route("/drives/:idx/init", post(initialize_drive))
        .route("/export", post(export_packages))
        .route("/import", post(import_packages))
        .with_state(state)
}
