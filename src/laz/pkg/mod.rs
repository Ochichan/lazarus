//! USB 배포용 패키지 시스템 (.laz v2)
//!
//! 기존 LazPackage와 별개로, USB를 통한 대량 배포를 위한 새 포맷
//!
//! ```text
//! package.laz (ZIP)
//! ├── manifest.json      # 패키지 메타데이터
//! ├── notes/             # JSON 노트 파일
//! │   ├── 1.json
//! │   └── ...
//! ├── cards/             # SRS 카드
//! │   └── cards.jsonl
//! └── assets/            # 미디어 파일
//!     ├── image.png
//!     └── video.mp4
//! ```

pub mod manifest;
pub mod reader;
pub mod validator;
pub mod writer;
pub mod installer;

// Re-exports
pub use manifest::{
    Manifest as PkgManifest,
    Author,
    PackageStats,
    EncryptionInfo,
    AssetType,
    AssetInfo,
    PackageWarning,
    FORMAT_VERSION,
    VIDEO_WARNING_THRESHOLD,
    VIDEO_MAX_SIZE,
    PACKAGE_MAX_SIZE,
    all_supported_extensions,
    asset_types,
};

pub use writer::{
    PackageBuilder,
    PackagePreview,
    BuildResult,
    BuildError,
    PackageNote,
    PackageCard,
    PackageAsset,
    EncryptedNoteHandling,
};

pub use reader::{
    PackageReader,
    PackagePreviewInfo,
    EntryInfo,
    ExtractionResult,
    ReaderError,
    read_package_info,
    is_valid_package,
};

pub use validator::{
    PackageValidator,
    ValidationResult,
    ValidationError,
    ValidationWarning,
    MAX_COMPRESSION_RATIO,
    MAX_ENTRY_COUNT,
    MAX_FILENAME_LENGTH,
    MAX_PATH_DEPTH,
    is_safe_path,
    sanitize_path,
};

pub use installer::{
    PackageInstaller,
    InstallOptions,
    InstallResult,
    InstallError,
    EncryptedInstallHandling,
    InstallTarget,
    InstalledNote,
    InstalledCard,
    InstalledPackagesStore,
    InstalledPackageRecord,
    SkippedNote,
    SkipReason,
};

/// File extension for Lazarus packages
pub const PACKAGE_EXTENSION: &str = "laz";

/// MIME type for Lazarus packages
pub const PACKAGE_MIME_TYPE: &str = "application/x-lazarus-package";

/// Check if a file is a Lazarus package by extension
pub fn is_package_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case(PACKAGE_EXTENSION))
        .unwrap_or(false)
}

/// Generate a package filename from name and version
pub fn generate_package_filename(name: &str, version: &str) -> String {
    let safe_name: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect();

    let date_part = if version.len() >= 10 {
        &version[..10]
    } else {
        version
    };

    format!("{}-{}.{}", safe_name, date_part, PACKAGE_EXTENSION)
}
