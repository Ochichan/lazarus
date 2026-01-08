//! USB package synchronization
//!
//! Handles:
//! - Detecting USB drives with Lazarus packages
//! - Exporting packages from local store to USB
//! - Importing packages from USB to local store
//! - Tracking sync status

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::{PackageStore, PackageStoreError, PackageSummary};
use crate::laz::{is_package_file, read_package_info, PACKAGE_EXTENSION};

/// Directory name for Lazarus packages on USB
pub const USB_PACKAGE_DIR: &str = "lazarus-packages";

/// Sync status file name
pub const SYNC_STATUS_FILE: &str = ".lazarus-sync.json";

/// Represents a detected USB drive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDrive {
    /// Mount path
    pub path: PathBuf,

    /// Drive label (if available)
    pub label: Option<String>,

    /// Total capacity in bytes
    pub total_bytes: Option<u64>,

    /// Available space in bytes
    pub available_bytes: Option<u64>,

    /// Whether it has a Lazarus package directory
    pub has_lazarus_dir: bool,

    /// Number of packages on this drive
    pub package_count: usize,
}

impl UsbDrive {
    /// Get the Lazarus packages directory path
    pub fn packages_dir(&self) -> PathBuf {
        self.path.join(USB_PACKAGE_DIR)
    }

    /// Format available space for display
    pub fn formatted_available(&self) -> String {
        match self.available_bytes {
            Some(bytes) if bytes < 1024 * 1024 => format!("{:.1} KB", bytes as f64 / 1024.0),
            Some(bytes) if bytes < 1024 * 1024 * 1024 => {
                format!("{:.1} MB", bytes as f64 / 1024.0 / 1024.0)
            }
            Some(bytes) => format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0),
            None => "Unknown".to_string(),
        }
    }
}

/// Package info as found on USB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbPackageInfo {
    /// Package ID
    pub id: String,

    /// Package name
    pub name: String,

    /// Version (timestamp)
    pub version: String,

    /// Author
    pub author: String,

    /// Note count
    pub note_count: usize,

    /// Card count
    pub card_count: usize,

    /// File size in bytes
    pub file_size: u64,

    /// Filename on USB
    pub filename: String,

    /// Full path on USB
    pub path: PathBuf,

    /// Whether this package exists in local store
    pub is_local: bool,

    /// Whether local version is newer
    pub local_is_newer: bool,

    /// Whether USB version is newer
    pub usb_is_newer: bool,
}

/// Result of scanning USB for packages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbScanResult {
    /// Drive info
    pub drive: UsbDrive,

    /// Packages found
    pub packages: Vec<UsbPackageInfo>,

    /// Packages only on USB (not in local store)
    pub usb_only: Vec<String>,

    /// Packages only in local store (not on USB)
    pub local_only: Vec<String>,

    /// Packages on both with USB newer
    pub usb_newer: Vec<String>,

    /// Packages on both with local newer
    pub local_newer: Vec<String>,

    /// Packages in sync
    pub in_sync: Vec<String>,

    /// Scan timestamp
    pub scanned_at: DateTime<Utc>,
}

/// Sync operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncDirection {
    /// Export from local to USB
    ToUsb,
    /// Import from USB to local
    FromUsb,
}

/// Result of a sync operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// Direction of sync
    pub direction: SyncDirection,

    /// Number of packages transferred
    pub transferred: usize,

    /// Number of packages skipped
    pub skipped: usize,

    /// Packages that were transferred
    pub transferred_packages: Vec<String>,

    /// Packages that were skipped with reasons
    pub skipped_packages: Vec<(String, String)>,

    /// Errors encountered
    pub errors: Vec<String>,

    /// Sync timestamp
    pub completed_at: DateTime<Utc>,
}

impl SyncResult {
    fn new(direction: SyncDirection) -> Self {
        Self {
            direction,
            transferred: 0,
            skipped: 0,
            transferred_packages: Vec::new(),
            skipped_packages: Vec::new(),
            errors: Vec::new(),
            completed_at: Utc::now(),
        }
    }

    pub fn is_success(&self) -> bool {
        self.errors.is_empty() && self.transferred > 0
    }
}

/// Sync status tracking (stored on USB)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncStatus {
    /// Last sync timestamp per package ID
    pub last_sync: HashMap<String, DateTime<Utc>>,

    /// Device ID that last synced (for multi-device tracking)
    pub last_device: Option<String>,

    /// Last full sync timestamp
    pub last_full_sync: Option<DateTime<Utc>>,
}

/// USB package manager
pub struct UsbPackageManager {
    /// Known USB mount points to scan
    mount_points: Vec<PathBuf>,
}

impl Default for UsbPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UsbPackageManager {
    /// Create a new USB package manager
    pub fn new() -> Self {
        Self {
            mount_points: Self::default_mount_points(),
        }
    }

    /// Create with custom mount points
    pub fn with_mount_points(mount_points: Vec<PathBuf>) -> Self {
        Self { mount_points }
    }

    /// Get default mount points based on OS
    fn default_mount_points() -> Vec<PathBuf> {
        let mut points = Vec::new();

        // Linux
        #[cfg(target_os = "linux")]
        {
            points.push(PathBuf::from("/media"));
            points.push(PathBuf::from("/mnt"));
            points.push(PathBuf::from("/run/media"));

            if let Ok(user) = std::env::var("USER") {
                points.push(PathBuf::from(format!("/media/{}", user)));
                points.push(PathBuf::from(format!("/run/media/{}", user)));
            }
        }

        // macOS
        #[cfg(target_os = "macos")]
        {
            points.push(PathBuf::from("/Volumes"));
        }

        // Windows
        #[cfg(target_os = "windows")]
        {
            for letter in b'D'..=b'Z' {
                let drive = format!("{}:", letter as char);
                points.push(PathBuf::from(&drive));
            }
        }

        // For testing - also check current directory
        if let Ok(cwd) = std::env::current_dir() {
            points.push(cwd.join("test-usb"));
        }

        points
    }

    /// Detect connected USB drives with Lazarus packages
    pub fn detect_drives(&self) -> Vec<UsbDrive> {
        let mut drives = Vec::new();

        for mount_point in &self.mount_points {
            if !mount_point.exists() {
                continue;
            }

            if self.is_container_mount(mount_point) {
                if let Ok(entries) = fs::read_dir(mount_point) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            if let Some(drive) = self.check_drive(&path) {
                                drives.push(drive);
                            }
                        }
                    }
                }
            } else {
                if let Some(drive) = self.check_drive(mount_point) {
                    drives.push(drive);
                }
            }
        }

        drives
    }

    /// Check if path is a container for USB mounts
    fn is_container_mount(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path_str.starts_with("/media")
            || path_str.starts_with("/mnt")
            || path_str.starts_with("/run/media")
            || path_str == "/Volumes"
    }

    /// Check if a path is a valid USB drive with packages
    fn check_drive(&self, path: &Path) -> Option<UsbDrive> {
        if !path.is_dir() {
            return None;
        }

        let packages_dir = path.join(USB_PACKAGE_DIR);
        let has_lazarus_dir = packages_dir.exists() && packages_dir.is_dir();

        let package_count = if has_lazarus_dir {
            self.count_packages(&packages_dir)
        } else {
            0
        };

        // Only include drives with lazarus dir or writable
        if !has_lazarus_dir && package_count == 0 {
            let test_file = path.join(".lazarus-test");
            let is_writable = fs::write(&test_file, "test").is_ok();
            if is_writable {
                let _ = fs::remove_file(&test_file);
            } else {
                return None;
            }
        }

        let label = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(String::from);

        let (total_bytes, available_bytes) = self.get_disk_space(path);

        Some(UsbDrive {
            path: path.to_path_buf(),
            label,
            total_bytes,
            available_bytes,
            has_lazarus_dir,
            package_count,
        })
    }

    /// Count packages in a directory
    fn count_packages(&self, dir: &Path) -> usize {
        fs::read_dir(dir)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|e| is_package_file(&e.path()))
                    .count()
            })
            .unwrap_or(0)
    }

    /// Get disk space (stub - implement per platform)
    fn get_disk_space(&self, _path: &Path) -> (Option<u64>, Option<u64>) {
        // Platform-specific implementation would go here
        // For now, return None
        (None, None)
    }

    /// Scan a USB drive for packages
    pub fn scan_drive(
        &self,
        drive: &UsbDrive,
        local_store: &PackageStore,
    ) -> Result<UsbScanResult, UsbError> {
        let packages_dir = drive.packages_dir();

        if !packages_dir.exists() {
            return Ok(UsbScanResult {
                drive: drive.clone(),
                packages: Vec::new(),
                usb_only: Vec::new(),
                local_only: local_store.list().iter().map(|p| p.id.clone()).collect(),
                usb_newer: Vec::new(),
                local_newer: Vec::new(),
                in_sync: Vec::new(),
                scanned_at: Utc::now(),
            });
        }

        let mut packages = Vec::new();
        let mut usb_ids: HashMap<String, UsbPackageInfo> = HashMap::new();

        // Scan USB packages
        for entry in fs::read_dir(&packages_dir)? {
            let entry = entry?;
            let path = entry.path();

            if !is_package_file(&path) {
                continue;
            }

            match read_package_info(&path) {
                Ok(info) => {
                    let file_size = entry.metadata()?.len();
                    let filename = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();

                    let local_pkg = local_store.get(&info.manifest.id);
                    let is_local = local_pkg.is_some();
                    let (local_is_newer, usb_is_newer) = if let Some(local) = local_pkg {
                        (
                            local.version > info.manifest.version,
                            info.manifest.version > local.version,
                        )
                    } else {
                        (false, false)
                    };

                    let pkg_info = UsbPackageInfo {
                        id: info.manifest.id.clone(),
                        name: info.manifest.name,
                        version: info.manifest.version,
                        author: info.manifest.author.name,
                        note_count: info.note_count,
                        card_count: info.manifest.stats.card_count,
                        file_size,
                        filename,
                        path: path.clone(),
                        is_local,
                        local_is_newer,
                        usb_is_newer,
                    };

                    usb_ids.insert(pkg_info.id.clone(), pkg_info.clone());
                    packages.push(pkg_info);
                }
                Err(e) => {
                    tracing::warn!("Failed to read USB package {}: {}", path.display(), e);
                }
            }
        }

        // Categorize packages
        let local_packages = local_store.list();
        let local_ids: HashMap<String, &PackageSummary> =
            local_packages.iter().map(|p| (p.id.clone(), p)).collect();

        let mut usb_only = Vec::new();
        let mut local_only = Vec::new();
        let mut usb_newer = Vec::new();
        let mut local_newer = Vec::new();
        let mut in_sync = Vec::new();

        for (id, usb_pkg) in &usb_ids {
            if !local_ids.contains_key(id) {
                usb_only.push(id.clone());
            } else if usb_pkg.usb_is_newer {
                usb_newer.push(id.clone());
            } else if usb_pkg.local_is_newer {
                local_newer.push(id.clone());
            } else {
                in_sync.push(id.clone());
            }
        }

        for id in local_ids.keys() {
            if !usb_ids.contains_key(id) {
                local_only.push(id.clone());
            }
        }

        Ok(UsbScanResult {
            drive: drive.clone(),
            packages,
            usb_only,
            local_only,
            usb_newer,
            local_newer,
            in_sync,
            scanned_at: Utc::now(),
        })
    }

    /// Export packages from local store to USB
    pub fn export_to_usb(
        &self,
        drive: &UsbDrive,
        local_store: &PackageStore,
        package_ids: &[String],
        overwrite: bool,
    ) -> Result<SyncResult, UsbError> {
        let mut result = SyncResult::new(SyncDirection::ToUsb);
        let packages_dir = drive.packages_dir();

        fs::create_dir_all(&packages_dir)?;

        for id in package_ids {
            match self.export_single(local_store, &packages_dir, id, overwrite) {
                Ok(true) => {
                    result.transferred += 1;
                    result.transferred_packages.push(id.clone());
                }
                Ok(false) => {
                    result.skipped += 1;
                    result.skipped_packages
                        .push((id.clone(), "Already exists".to_string()));
                }
                Err(e) => {
                    result.errors.push(format!("Failed to export {}: {}", id, e));
                }
            }
        }

        result.completed_at = Utc::now();
        self.update_sync_status(&packages_dir, &result.transferred_packages)?;

        Ok(result)
    }

    /// Export a single package
    fn export_single(
        &self,
        local_store: &PackageStore,
        usb_dir: &Path,
        package_id: &str,
        overwrite: bool,
    ) -> Result<bool, UsbError> {
        let summary = local_store
            .get(package_id)
            .ok_or_else(|| UsbError::PackageNotFound(package_id.to_string()))?;

        let dest_path = usb_dir.join(&summary.filename);

        if dest_path.exists() && !overwrite {
            return Ok(false);
        }

        let data = local_store.get_data(package_id)?;
        fs::write(&dest_path, data)?;

        Ok(true)
    }

    /// Import packages from USB to local store
    pub fn import_from_usb(
        &self,
        drive: &UsbDrive,
        local_store: &mut PackageStore,
        package_ids: &[String],
        overwrite: bool,
    ) -> Result<SyncResult, UsbError> {
        let mut result = SyncResult::new(SyncDirection::FromUsb);
        let packages_dir = drive.packages_dir();

        if !packages_dir.exists() {
            return Err(UsbError::NoDriveFound);
        }

        // Build map of USB packages
        let usb_packages: HashMap<String, PathBuf> = fs::read_dir(&packages_dir)?
            .flatten()
            .filter(|e| is_package_file(&e.path()))
            .filter_map(|e| {
                let path = e.path();
                read_package_info(&path)
                    .ok()
                    .map(|info| (info.manifest.id, path))
            })
            .collect();

        for id in package_ids {
            match self.import_single(local_store, &usb_packages, id, overwrite) {
                Ok(true) => {
                    result.transferred += 1;
                    result.transferred_packages.push(id.clone());
                }
                Ok(false) => {
                    result.skipped += 1;
                    result.skipped_packages
                        .push((id.clone(), "Already exists locally".to_string()));
                }
                Err(e) => {
                    result.errors.push(format!("Failed to import {}: {}", id, e));
                }
            }
        }

        result.completed_at = Utc::now();

        Ok(result)
    }

    /// Import a single package
    fn import_single(
        &self,
        local_store: &mut PackageStore,
        usb_packages: &HashMap<String, PathBuf>,
        package_id: &str,
        overwrite: bool,
    ) -> Result<bool, UsbError> {
        let usb_path = usb_packages
            .get(package_id)
            .ok_or_else(|| UsbError::PackageNotFound(package_id.to_string()))?;

        if local_store.exists(package_id) && !overwrite {
            return Ok(false);
        }

        if local_store.exists(package_id) && overwrite {
            local_store.remove(package_id)?;
        }

        local_store.add_from_file(usb_path)?;

        Ok(true)
    }

    /// Export all local packages to USB
    pub fn export_all(
        &self,
        drive: &UsbDrive,
        local_store: &PackageStore,
        overwrite: bool,
    ) -> Result<SyncResult, UsbError> {
        let ids: Vec<String> = local_store.list().iter().map(|p| p.id.clone()).collect();
        self.export_to_usb(drive, local_store, &ids, overwrite)
    }

    /// Import all USB packages to local
    pub fn import_all(
        &self,
        drive: &UsbDrive,
        local_store: &mut PackageStore,
        overwrite: bool,
    ) -> Result<SyncResult, UsbError> {
        let scan = self.scan_drive(drive, local_store)?;
        let ids: Vec<String> = scan.packages.iter().map(|p| p.id.clone()).collect();
        self.import_from_usb(drive, local_store, &ids, overwrite)
    }

    /// Update sync status file on USB
    fn update_sync_status(&self, usb_dir: &Path, synced_ids: &[String]) -> Result<(), UsbError> {
        let status_path = usb_dir.join(SYNC_STATUS_FILE);

        let mut status: SyncStatus = if status_path.exists() {
            let content = fs::read_to_string(&status_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            SyncStatus::default()
        };

        let now = Utc::now();
        for id in synced_ids {
            status.last_sync.insert(id.clone(), now);
        }
        status.last_full_sync = Some(now);

        let json = serde_json::to_string_pretty(&status)?;
        fs::write(&status_path, json)?;

        Ok(())
    }

    /// Read sync status from USB
    pub fn read_sync_status(&self, drive: &UsbDrive) -> Option<SyncStatus> {
        let status_path = drive.packages_dir().join(SYNC_STATUS_FILE);

        if !status_path.exists() {
            return None;
        }

        fs::read_to_string(&status_path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
    }

    /// Initialize Lazarus directory on USB
    pub fn initialize_drive(&self, drive: &UsbDrive) -> Result<(), UsbError> {
        let packages_dir = drive.packages_dir();
        fs::create_dir_all(&packages_dir)?;

        let status = SyncStatus::default();
        let status_path = packages_dir.join(SYNC_STATUS_FILE);
        let json = serde_json::to_string_pretty(&status)?;
        fs::write(&status_path, json)?;

        Ok(())
    }
}

/// Errors for USB operations
#[derive(Debug, thiserror::Error)]
pub enum UsbError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Package store error: {0}")]
    PackageStore(#[from] PackageStoreError),

    #[error("No USB drive found")]
    NoDriveFound,

    #[error("Package not found: {0}")]
    PackageNotFound(String),

    #[error("Not enough space on USB drive")]
    InsufficientSpace,

    #[error("USB drive is read-only")]
    ReadOnly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usb_drive_packages_dir() {
        let drive = UsbDrive {
            path: PathBuf::from("/media/usb"),
            label: Some("USB".to_string()),
            total_bytes: None,
            available_bytes: None,
            has_lazarus_dir: false,
            package_count: 0,
        };

        assert_eq!(
            drive.packages_dir(),
            PathBuf::from("/media/usb/lazarus-packages")
        );
    }

    #[test]
    fn test_sync_result_success() {
        let mut result = SyncResult::new(SyncDirection::ToUsb);
        result.transferred = 3;

        assert!(result.is_success());

        result.errors.push("Error".to_string());
        assert!(!result.is_success());
    }

    #[test]
    fn test_formatted_available() {
        let drive = UsbDrive {
            path: PathBuf::from("/media/usb"),
            label: None,
            total_bytes: None,
            available_bytes: Some(1024 * 1024 * 500),
            has_lazarus_dir: false,
            package_count: 0,
        };

        assert_eq!(drive.formatted_available(), "500.0 MB");
    }
}
