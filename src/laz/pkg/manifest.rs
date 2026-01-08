//! Package manifest structures for .laz format

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path;

/// Current manifest format version
pub const FORMAT_VERSION: &str = "1.0";

/// Video size threshold for warning (100MB)
pub const VIDEO_WARNING_THRESHOLD: u64 = 100 * 1024 * 1024;

/// Maximum video size allowed (500MB)
pub const VIDEO_MAX_SIZE: u64 = 500 * 1024 * 1024;

/// Maximum total package size (1GB)
pub const PACKAGE_MAX_SIZE: u64 = 1024 * 1024 * 1024;

/// Supported asset extensions by category
pub mod asset_types {
    pub const IMAGES: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "svg"];
    pub const DOCUMENTS: &[&str] = &["pdf"];
    pub const AUDIO: &[&str] = &["mp3", "ogg", "wav", "flac"];
    pub const VIDEO: &[&str] = &["mp4", "webm", "mkv"];
}

/// Package manifest - the core metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub format_version: String,
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Author,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub language: String,
    pub license: String,
    pub stats: PackageStats,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionInfo>,
    pub checksum: String,
}

impl Manifest {
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        let version = now.to_rfc3339();

        Self {
            format_version: FORMAT_VERSION.to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            version,
            author: Author::default(),
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            language: "en".to_string(),
            license: "CC-BY-4.0".to_string(),
            stats: PackageStats::default(),
            encryption: None,
            checksum: String::new(),
        }
    }

    pub fn update_checksum(&mut self, data: &[u8]) {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        self.checksum = format!("sha256:{:x}", result);
    }

    pub fn verify_checksum(&self, data: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let computed = format!("sha256:{:x}", result);
        self.checksum == computed
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
}

impl Author {
    pub fn new(name: String) -> Self {
        Self { name, contact: None }
    }

    pub fn with_contact(mut self, contact: String) -> Self {
        self.contact = Some(contact);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackageStats {
    pub note_count: usize,
    pub encrypted_note_count: usize,
    pub card_count: usize,
    pub asset_count: usize,
    pub video_count: usize,
    pub total_size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionInfo {
    pub has_encrypted_notes: bool,
    pub encrypted_note_count: usize,
    pub encryption_method: String,
    pub kdf: String,
}

impl Default for EncryptionInfo {
    fn default() -> Self {
        Self {
            has_encrypted_notes: false,
            encrypted_note_count: 0,
            encryption_method: "XChaCha20-Poly1305".to_string(),
            kdf: "Argon2id".to_string(),
        }
    }
}

impl EncryptionInfo {
    pub fn new(encrypted_count: usize) -> Self {
        Self {
            has_encrypted_notes: encrypted_count > 0,
            encrypted_note_count: encrypted_count,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PackageWarning {
    LargeVideo { filename: String, size_bytes: u64 },
    OversizedVideo { filename: String, size_bytes: u64, max_bytes: u64 },
    EncryptedNotesIncluded { count: usize },
    TotalSizeLarge { size_bytes: u64 },
    PackageTooLarge { size_bytes: u64, max_bytes: u64 },
}

impl PackageWarning {
    pub fn message(&self) -> String {
        match self {
            PackageWarning::LargeVideo { filename, size_bytes } => {
                format!("Large video: '{}' ({:.1} MB)", filename, *size_bytes as f64 / 1024.0 / 1024.0)
            }
            PackageWarning::OversizedVideo { filename, size_bytes, max_bytes } => {
                format!("Video '{}' ({:.1} MB) exceeds max ({:.1} MB)", filename,
                    *size_bytes as f64 / 1024.0 / 1024.0, *max_bytes as f64 / 1024.0 / 1024.0)
            }
            PackageWarning::EncryptedNotesIncluded { count } => {
                format!("{} encrypted note(s) included", count)
            }
            PackageWarning::TotalSizeLarge { size_bytes } => {
                format!("Package size: {:.1} MB", *size_bytes as f64 / 1024.0 / 1024.0)
            }
            PackageWarning::PackageTooLarge { size_bytes, max_bytes } => {
                format!("Package ({:.1} MB) exceeds max ({:.1} MB)",
                    *size_bytes as f64 / 1024.0 / 1024.0, *max_bytes as f64 / 1024.0 / 1024.0)
            }
        }
    }

    pub fn is_blocking(&self) -> bool {
        matches!(self, PackageWarning::OversizedVideo { .. } | PackageWarning::PackageTooLarge { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Image,
    Document,
    Audio,
    Video,
    Unknown,
}

impl AssetType {
    pub fn from_extension(ext: &str) -> Self {
        let ext_lower = ext.to_lowercase();
        let ext_str = ext_lower.as_str();

        if asset_types::IMAGES.contains(&ext_str) {
            AssetType::Image
        } else if asset_types::DOCUMENTS.contains(&ext_str) {
            AssetType::Document
        } else if asset_types::AUDIO.contains(&ext_str) {
            AssetType::Audio
        } else if asset_types::VIDEO.contains(&ext_str) {
            AssetType::Video
        } else {
            AssetType::Unknown
        }
    }

    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|e| e.to_str())
            .map(Self::from_extension)
            .unwrap_or(AssetType::Unknown)
    }

    pub fn requires_streaming(&self) -> bool {
        matches!(self, AssetType::Video)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetInfo {
    pub filename: String,
    pub asset_type: String,
    pub size_bytes: u64,
    pub mime_type: String,
}

impl AssetInfo {
    pub fn new(filename: String, size_bytes: u64) -> Self {
        let path = Path::new(&filename);
        let asset_type = AssetType::from_path(path);
        let mime_type = Self::guess_mime_type(&filename);

        Self {
            filename,
            asset_type: format!("{:?}", asset_type),
            size_bytes,
            mime_type,
        }
    }

    fn guess_mime_type(filename: &str) -> String {
        let ext = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            "pdf" => "application/pdf",
            "mp3" => "audio/mpeg",
            "ogg" => "audio/ogg",
            "wav" => "audio/wav",
            "flac" => "audio/flac",
            "mp4" => "video/mp4",
            "webm" => "video/webm",
            "mkv" => "video/x-matroska",
            _ => "application/octet-stream",
        }
        .to_string()
    }
}

pub fn all_supported_extensions() -> HashSet<&'static str> {
    let mut exts = HashSet::new();
    exts.extend(asset_types::IMAGES.iter());
    exts.extend(asset_types::DOCUMENTS.iter());
    exts.extend(asset_types::AUDIO.iter());
    exts.extend(asset_types::VIDEO.iter());
    exts
}
