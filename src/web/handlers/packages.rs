//! Package API handlers
//!
//! Endpoints:
//! - GET    /api/packages           - List all packages
//! - POST   /api/packages           - Create/upload a package
//! - GET    /api/packages/:id       - Get package details
//! - DELETE /api/packages/:id       - Delete a package
//! - GET    /api/packages/:id/download - Download package file
//! - POST   /api/packages/preview   - Preview package before install
//! - POST   /api/packages/install   - Install a package

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::db::{PackageStore, PackageStoreError, PackageSummary};
use crate::laz::{
    read_package_info, AssetInfo, EncryptedInstallHandling, InstallOptions, InstallResult,
    Manifest, PackageBuilder, PackageCard, PackageInstaller, PackageNote, PackagePreviewInfo,
    PackageReader,
};

// ============================================================================
// Request/Response Types
// ============================================================================

/// Query parameters for listing packages
#[derive(Debug, Deserialize)]
pub struct ListPackagesQuery {
    /// Search query (optional)
    pub search: Option<String>,
    /// Filter by tag (optional)
    pub tag: Option<String>,
    /// Sort order: "date" (default), "name", "size"
    pub sort: Option<String>,
}

/// Response for package list
#[derive(Debug, Serialize)]
pub struct PackageListResponse {
    pub packages: Vec<PackageSummary>,
    pub total: usize,
    pub total_size: u64,
    pub tags: Vec<String>,
}

/// Request to create a package
#[derive(Debug, Deserialize)]
pub struct CreatePackageRequest {
    /// Package name
    pub name: String,
    /// Package description
    pub description: String,
    /// Note IDs to include
    pub note_ids: Vec<u64>,
    /// Whether to include SRS cards
    #[serde(default = "default_true")]
    pub include_cards: bool,
    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,
    /// Author name
    pub author: Option<String>,
    /// How to handle encrypted notes: "include" or "exclude"
    #[serde(default = "default_exclude")]
    pub encrypted_handling: String,
}

fn default_true() -> bool {
    true
}

fn default_exclude() -> String {
    "exclude".to_string()
}

/// Response for package details
#[derive(Debug, Serialize)]
pub struct PackageDetailResponse {
    pub summary: PackageSummary,
    pub manifest: Manifest,
    pub assets: Vec<AssetInfo>,
    pub total_size: u64,
    pub note_count: usize,
    pub has_cards: bool,
}

/// Request to install a package
#[derive(Debug, Deserialize)]
pub struct InstallPackageRequest {
    /// Package ID to install
    pub package_id: String,
    /// How to handle encrypted notes: "install" or "skip"
    #[serde(default = "default_skip")]
    pub encrypted_handling: String,
    /// PIN for encrypted notes (if encrypted_handling is "install")
    pub pin: Option<String>,
    /// Whether to install cards
    #[serde(default = "default_true")]
    pub install_cards: bool,
    /// Whether to overwrite existing notes with same title
    #[serde(default)]
    pub overwrite_duplicates: bool,
}

fn default_skip() -> String {
    "skip".to_string()
}

/// Response for install operation
#[derive(Debug, Serialize)]
pub struct InstallResponse {
    pub success: bool,
    pub message: String,
    pub notes_installed: usize,
    pub notes_skipped: usize,
    pub cards_installed: usize,
    pub warnings: Vec<String>,
    pub skipped_notes: Vec<SkippedNoteInfo>,
}

#[derive(Debug, Serialize)]
pub struct SkippedNoteInfo {
    pub title: String,
    pub reason: String,
}

/// Generic API response
#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

// ============================================================================
// Shared State (example - adapt to your AppState)
// ============================================================================

/// Application state containing package store
/// In practice, this would be part of your main AppState
#[derive(Clone)]
pub struct PackageState {
    pub store: Arc<RwLock<PackageStore>>,
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/packages - List all packages
pub async fn list_packages(
    State(state): State<PackageState>,
    Query(query): Query<ListPackagesQuery>,
) -> Result<Json<PackageListResponse>, (StatusCode, Json<ApiResponse>)> {
    let store = state.store.read().await;

    // Get packages based on query
    let mut packages = if let Some(search) = &query.search {
        store.search(search)
    } else if let Some(tag) = &query.tag {
        store.by_tag(tag)
    } else {
        store.list()
    };

    // Sort
    match query.sort.as_deref() {
        Some("name") => packages.sort_by(|a, b| a.name.cmp(&b.name)),
        Some("size") => packages.sort_by(|a, b| b.file_size.cmp(&a.file_size)),
        _ => packages.sort_by(|a, b| b.added_at.cmp(&a.added_at)), // date (default)
    }

    let total = packages.len();
    let total_size = store.total_size();
    let tags = store.all_tags();

    Ok(Json(PackageListResponse {
        packages,
        total,
        total_size,
        tags,
    }))
}

/// POST /api/packages - Upload a package file
pub async fn upload_package(
    State(state): State<PackageState>,
    mut multipart: Multipart,
) -> Result<Json<PackageSummary>, (StatusCode, Json<ApiResponse>)> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;

    // Process multipart form
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: format!("Failed to read form data: {}", e),
            }),
        )
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            filename = field.file_name().map(String::from);
            file_data = Some(field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse {
                        success: false,
                        message: format!("Failed to read file: {}", e),
                    }),
                )
            })?.to_vec());
        }
    }

    let data = file_data.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "No file provided".to_string(),
            }),
        )
    })?;

    // Add to store
    let mut store = state.store.write().await;
    let summary = store
        .add_from_bytes(&data, filename.as_deref())
        .map_err(|e| {
            let (status, msg) = match &e {
                PackageStoreError::AlreadyExists(id) => (
                    StatusCode::CONFLICT,
                    format!("Package already exists: {}", id),
                ),
                PackageStoreError::Reader(re) => {
                    (StatusCode::BAD_REQUEST, format!("Invalid package: {}", re))
                }
                _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            };
            (status, Json(ApiResponse { success: false, message: msg }))
        })?;

    Ok(Json(summary))
}

/// GET /api/packages/:id - Get package details
pub async fn get_package(
    State(state): State<PackageState>,
    Path(id): Path<String>,
) -> Result<Json<PackageDetailResponse>, (StatusCode, Json<ApiResponse>)> {
    let store = state.store.read().await;

    // Get summary
    let summary = store.get(&id).cloned().ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!("Package not found: {}", id),
            }),
        )
    })?;

    // Get full info
    let info = store.get_info(&id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Failed to read package: {}", e),
            }),
        )
    })?;

    Ok(Json(PackageDetailResponse {
        summary,
        manifest: info.manifest,
        assets: info.assets,
        total_size: info.total_size,
        note_count: info.note_count,
        has_cards: info.has_cards,
    }))
}

/// DELETE /api/packages/:id - Delete a package
pub async fn delete_package(
    State(state): State<PackageState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse>, (StatusCode, Json<ApiResponse>)> {
    let mut store = state.store.write().await;

    let removed = store.remove(&id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Failed to delete package: {}", e),
            }),
        )
    })?;

    if removed {
        Ok(Json(ApiResponse {
            success: true,
            message: "Package deleted".to_string(),
        }))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!("Package not found: {}", id),
            }),
        ))
    }
}

/// GET /api/packages/:id/download - Download package file
pub async fn download_package(
    State(state): State<PackageState>,
    Path(id): Path<String>,
) -> Result<Response, (StatusCode, Json<ApiResponse>)> {
    let store = state.store.read().await;

    // Get summary for filename
    let summary = store.get(&id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                message: format!("Package not found: {}", id),
            }),
        )
    })?;

    let filename = summary.filename.clone();

    // Get data
    let data = store.get_data(&id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: format!("Failed to read package: {}", e),
            }),
        )
    })?;

    // Build response with download headers
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/x-lazarus-package")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .header(header::CONTENT_LENGTH, data.len())
        .body(Body::from(data))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: format!("Failed to build response: {}", e),
                }),
            )
        })?;

    Ok(response)
}

/// POST /api/packages/preview - Preview package contents before install
pub async fn preview_package(
    State(state): State<PackageState>,
    Path(id): Path<String>,
) -> Result<Json<PackagePreviewInfo>, (StatusCode, Json<ApiResponse>)> {
    let store = state.store.read().await;

    let info = store.get_info(&id).map_err(|e| {
        let (status, msg) = match &e {
            PackageStoreError::NotFound(_) => (StatusCode::NOT_FOUND, "Package not found".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        (status, Json(ApiResponse { success: false, message: msg }))
    })?;

    Ok(Json(info))
}

// ============================================================================
// Router Helper
// ============================================================================

/// Create package routes
/// Usage: router.nest("/api/packages", package_routes(state))
pub fn package_routes(state: PackageState) -> axum::Router {
    use axum::routing::{delete, get, post};

    axum::Router::new()
        .route("/", get(list_packages).post(upload_package))
        .route("/:id", get(get_package).delete(delete_package))
        .route("/:id/download", get(download_package))
        .route("/:id/preview", post(preview_package))
        .with_state(state)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_query_deserialize() {
        let query: ListPackagesQuery = serde_json::from_str(r#"{"search":"physics"}"#).unwrap();
        assert_eq!(query.search, Some("physics".to_string()));
        assert_eq!(query.tag, None);
    }

    #[test]
    fn test_install_request_defaults() {
        let json = r#"{"package_id":"abc123"}"#;
        let req: InstallPackageRequest = serde_json::from_str(json).unwrap();

        assert_eq!(req.package_id, "abc123");
        assert_eq!(req.encrypted_handling, "skip");
        assert!(req.install_cards);
        assert!(!req.overwrite_duplicates);
    }

    #[test]
    fn test_create_request() {
        let json = r#"{
            "name": "Physics 101",
            "description": "Basic physics",
            "note_ids": [1, 2, 3],
            "tags": ["physics", "science"]
        }"#;

        let req: CreatePackageRequest = serde_json::from_str(json).unwrap();

        assert_eq!(req.name, "Physics 101");
        assert_eq!(req.note_ids, vec![1, 2, 3]);
        assert!(req.include_cards);
        assert_eq!(req.encrypted_handling, "exclude");
    }
}
