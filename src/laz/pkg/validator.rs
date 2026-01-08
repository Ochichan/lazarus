//! Package validation and security checks
//!
//! This module provides security validation for .laz packages to prevent:
//! - Path traversal attacks (e.g., "../../../etc/passwd")
//! - Zip bomb attacks (excessive compression ratios)
//! - Oversized files
//! - Malicious filenames

use std::path::{Path, PathBuf};

use super::manifest::{FORMAT_VERSION, PACKAGE_MAX_SIZE, VIDEO_MAX_SIZE};

/// Maximum compression ratio allowed (prevents zip bombs)
/// A ratio of 100:1 is very generous for legitimate content
pub const MAX_COMPRESSION_RATIO: u64 = 100;

/// Maximum number of entries in a package
pub const MAX_ENTRY_COUNT: usize = 10_000;

/// Maximum filename length
pub const MAX_FILENAME_LENGTH: usize = 255;

/// Maximum path depth
pub const MAX_PATH_DEPTH: usize = 10;

/// Validation result
#[derive(Debug)]
pub struct ValidationResult {
    /// Whether the package passed validation
    pub valid: bool,
    /// List of errors found
    pub errors: Vec<ValidationError>,
    /// List of warnings (non-blocking)
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn with_error(error: ValidationError) -> Self {
        Self {
            valid: false,
            errors: vec![error],
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.valid = false;
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    pub fn merge(&mut self, other: ValidationResult) {
        if !other.valid {
            self.valid = false;
        }
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

/// Validation errors (blocking)
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Path contains traversal sequences
    PathTraversal { path: String },

    /// Filename contains dangerous characters
    DangerousFilename { filename: String, reason: String },

    /// File exceeds maximum size
    FileTooLarge { filename: String, size: u64, max: u64 },

    /// Package exceeds maximum size
    PackageTooLarge { size: u64, max: u64 },

    /// Compression ratio is suspiciously high (zip bomb)
    SuspiciousCompression { filename: String, ratio: u64 },

    /// Too many entries in package
    TooManyEntries { count: usize, max: usize },

    /// Invalid manifest
    InvalidManifest { reason: String },

    /// Unsupported format version
    UnsupportedVersion { version: String },

    /// Checksum mismatch
    ChecksumMismatch { expected: String, actual: String },

    /// Missing required file
    MissingFile { filename: String },

    /// Invalid ZIP structure
    InvalidZipStructure { reason: String },
}

impl ValidationError {
    pub fn message(&self) -> String {
        match self {
            ValidationError::PathTraversal { path } => {
                format!("Path traversal detected: '{}'", path)
            }
            ValidationError::DangerousFilename { filename, reason } => {
                format!("Dangerous filename '{}': {}", filename, reason)
            }
            ValidationError::FileTooLarge { filename, size, max } => {
                format!(
                    "File '{}' is too large: {:.1} MB (max: {:.1} MB)",
                    filename,
                    *size as f64 / 1024.0 / 1024.0,
                    *max as f64 / 1024.0 / 1024.0
                )
            }
            ValidationError::PackageTooLarge { size, max } => {
                format!(
                    "Package is too large: {:.1} MB (max: {:.1} MB)",
                    *size as f64 / 1024.0 / 1024.0,
                    *max as f64 / 1024.0 / 1024.0
                )
            }
            ValidationError::SuspiciousCompression { filename, ratio } => {
                format!(
                    "Suspicious compression ratio for '{}': {}:1 (possible zip bomb)",
                    filename, ratio
                )
            }
            ValidationError::TooManyEntries { count, max } => {
                format!("Too many entries: {} (max: {})", count, max)
            }
            ValidationError::InvalidManifest { reason } => {
                format!("Invalid manifest: {}", reason)
            }
            ValidationError::UnsupportedVersion { version } => {
                format!("Unsupported format version: {} (supported: {})", version, FORMAT_VERSION)
            }
            ValidationError::ChecksumMismatch { expected, actual } => {
                format!("Checksum mismatch: expected {}, got {}", expected, actual)
            }
            ValidationError::MissingFile { filename } => {
                format!("Missing required file: {}", filename)
            }
            ValidationError::InvalidZipStructure { reason } => {
                format!("Invalid ZIP structure: {}", reason)
            }
        }
    }
}

/// Validation warnings (non-blocking)
#[derive(Debug, Clone)]
pub enum ValidationWarning {
    /// Unknown file in package (will be ignored)
    UnknownFile { filename: String },

    /// File has unusual extension
    UnusualExtension { filename: String },

    /// Large file that may slow down operations
    LargeFile { filename: String, size: u64 },
}

impl ValidationWarning {
    pub fn message(&self) -> String {
        match self {
            ValidationWarning::UnknownFile { filename } => {
                format!("Unknown file will be ignored: '{}'", filename)
            }
            ValidationWarning::UnusualExtension { filename } => {
                format!("File has unusual extension: '{}'", filename)
            }
            ValidationWarning::LargeFile { filename, size } => {
                format!(
                    "Large file '{}': {:.1} MB",
                    filename,
                    *size as f64 / 1024.0 / 1024.0
                )
            }
        }
    }
}

/// Package validator
pub struct PackageValidator {
    /// Maximum allowed uncompressed size
    max_size: u64,
    /// Maximum compression ratio
    max_compression_ratio: u64,
    /// Maximum entry count
    max_entries: usize,
}

impl Default for PackageValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageValidator {
    pub fn new() -> Self {
        Self {
            max_size: PACKAGE_MAX_SIZE,
            max_compression_ratio: MAX_COMPRESSION_RATIO,
            max_entries: MAX_ENTRY_COUNT,
        }
    }

    /// Set custom max size
    pub fn with_max_size(mut self, size: u64) -> Self {
        self.max_size = size;
        self
    }

    /// Validate a path for safety
    pub fn validate_path(&self, path: &str) -> ValidationResult {
        let mut result = ValidationResult::ok();

        // Check for null bytes
        if path.contains('\0') {
            result.add_error(ValidationError::DangerousFilename {
                filename: path.to_string(),
                reason: "Contains null byte".to_string(),
            });
            return result;
        }

        // Check for path traversal
        if path.contains("..") {
            result.add_error(ValidationError::PathTraversal {
                path: path.to_string(),
            });
            return result;
        }

        // Check for absolute paths
        if path.starts_with('/') || path.starts_with('\\') {
            result.add_error(ValidationError::PathTraversal {
                path: path.to_string(),
            });
            return result;
        }

        // Check for Windows drive letters
        if path.len() >= 2 && path.chars().nth(1) == Some(':') {
            result.add_error(ValidationError::PathTraversal {
                path: path.to_string(),
            });
            return result;
        }

        // Check for double slashes
        if path.contains("//") || path.contains("\\\\") {
            result.add_error(ValidationError::DangerousFilename {
                filename: path.to_string(),
                reason: "Contains double slashes".to_string(),
            });
            return result;
        }

        // Check filename length
        if path.len() > MAX_FILENAME_LENGTH {
            result.add_error(ValidationError::DangerousFilename {
                filename: path.to_string(),
                reason: format!("Path too long: {} chars (max: {})", path.len(), MAX_FILENAME_LENGTH),
            });
            return result;
        }

        // Check path depth
        let depth = path.split('/').count();
        if depth > MAX_PATH_DEPTH {
            result.add_error(ValidationError::DangerousFilename {
                filename: path.to_string(),
                reason: format!("Path too deep: {} levels (max: {})", depth, MAX_PATH_DEPTH),
            });
            return result;
        }

        // Check for allowed directories
        let allowed_prefixes = ["manifest.json", "notes/", "cards/", "assets/", "signature.json"];
        if !allowed_prefixes.iter().any(|p| path == *p || path.starts_with(p)) {
            result.add_warning(ValidationWarning::UnknownFile {
                filename: path.to_string(),
            });
        }

        result
    }

    /// Validate file size
    pub fn validate_file_size(&self, filename: &str, size: u64) -> ValidationResult {
        let mut result = ValidationResult::ok();

        // Check against max package size
        if size > self.max_size {
            result.add_error(ValidationError::FileTooLarge {
                filename: filename.to_string(),
                size,
                max: self.max_size,
            });
            return result;
        }

        // Check video size limit
        let ext = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if ["mp4", "webm", "mkv"].contains(&ext.as_str()) && size > VIDEO_MAX_SIZE {
            result.add_error(ValidationError::FileTooLarge {
                filename: filename.to_string(),
                size,
                max: VIDEO_MAX_SIZE,
            });
        }

        result
    }

    /// Validate compression ratio (detect zip bombs)
    pub fn validate_compression_ratio(
        &self,
        filename: &str,
        compressed_size: u64,
        uncompressed_size: u64,
    ) -> ValidationResult {
        let mut result = ValidationResult::ok();

        // Avoid division by zero
        if compressed_size == 0 {
            if uncompressed_size > 0 {
                result.add_error(ValidationError::SuspiciousCompression {
                    filename: filename.to_string(),
                    ratio: u64::MAX,
                });
            }
            return result;
        }

        let ratio = uncompressed_size / compressed_size;
        if ratio > self.max_compression_ratio {
            result.add_error(ValidationError::SuspiciousCompression {
                filename: filename.to_string(),
                ratio,
            });
        }

        result
    }

    /// Validate entry count
    pub fn validate_entry_count(&self, count: usize) -> ValidationResult {
        let mut result = ValidationResult::ok();

        if count > self.max_entries {
            result.add_error(ValidationError::TooManyEntries {
                count,
                max: self.max_entries,
            });
        }

        result
    }

    /// Validate manifest format version
    pub fn validate_format_version(&self, version: &str) -> ValidationResult {
        let mut result = ValidationResult::ok();

        // Parse version (we support "1.0" and potentially "1.x" in the future)
        let supported: Vec<&str> = vec!["1.0"];

        if !supported.contains(&version) {
            result.add_error(ValidationError::UnsupportedVersion {
                version: version.to_string(),
            });
        }

        result
    }

    /// Validate checksum
    pub fn validate_checksum(&self, expected: &str, actual: &str) -> ValidationResult {
        let mut result = ValidationResult::ok();

        if expected != actual {
            result.add_error(ValidationError::ChecksumMismatch {
                expected: expected.to_string(),
                actual: actual.to_string(),
            });
        }

        result
    }

    /// Sanitize a path for safe extraction
    /// Returns None if the path is unsafe
    pub fn sanitize_path(&self, path: &str) -> Option<PathBuf> {
        // First validate
        let validation = self.validate_path(path);
        if !validation.valid {
            return None;
        }

        // Normalize the path
        let normalized: PathBuf = path
            .split('/')
            .filter(|s| !s.is_empty() && *s != ".")
            .collect();

        // Double-check for any remaining traversal
        for component in normalized.components() {
            if let std::path::Component::ParentDir = component {
                return None;
            }
        }

        Some(normalized)
    }
}

/// Quick validation functions
pub fn is_safe_path(path: &str) -> bool {
    PackageValidator::new().validate_path(path).valid
}

pub fn sanitize_path(path: &str) -> Option<PathBuf> {
    PackageValidator::new().sanitize_path(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_paths() {
        let validator = PackageValidator::new();

        assert!(validator.validate_path("manifest.json").valid);
        assert!(validator.validate_path("notes/1.json").valid);
        assert!(validator.validate_path("assets/image.png").valid);
        assert!(validator.validate_path("cards/cards.jsonl").valid);
    }

    #[test]
    fn test_path_traversal() {
        let validator = PackageValidator::new();

        assert!(!validator.validate_path("../etc/passwd").valid);
        assert!(!validator.validate_path("notes/../../../etc/passwd").valid);
        assert!(!validator.validate_path("..").valid);
        assert!(!validator.validate_path("notes/..").valid);
    }

    #[test]
    fn test_absolute_paths() {
        let validator = PackageValidator::new();

        assert!(!validator.validate_path("/etc/passwd").valid);
        assert!(!validator.validate_path("\\Windows\\System32").valid);
        assert!(!validator.validate_path("C:\\Windows").valid);
    }

    #[test]
    fn test_null_bytes() {
        let validator = PackageValidator::new();

        assert!(!validator.validate_path("notes/\0malicious.json").valid);
    }

    #[test]
    fn test_compression_ratio() {
        let validator = PackageValidator::new();

        // Normal ratio
        assert!(validator.validate_compression_ratio("test.json", 1000, 5000).valid);

        // Suspicious ratio (zip bomb)
        assert!(!validator.validate_compression_ratio("bomb.json", 100, 100_000_000).valid);
    }

    #[test]
    fn test_file_size() {
        let validator = PackageValidator::new();

        // Normal file
        assert!(validator.validate_file_size("notes/1.json", 1024).valid);

        // Too large
        assert!(!validator.validate_file_size("huge.bin", PACKAGE_MAX_SIZE + 1).valid);
    }

    #[test]
    fn test_video_size_limit() {
        let validator = PackageValidator::new();

        // Video within limit
        assert!(validator.validate_file_size("assets/video.mp4", 100 * 1024 * 1024).valid);

        // Video exceeds limit
        assert!(!validator.validate_file_size("assets/huge.mp4", VIDEO_MAX_SIZE + 1).valid);
    }

    #[test]
    fn test_entry_count() {
        let validator = PackageValidator::new();

        assert!(validator.validate_entry_count(100).valid);
        assert!(validator.validate_entry_count(MAX_ENTRY_COUNT).valid);
        assert!(!validator.validate_entry_count(MAX_ENTRY_COUNT + 1).valid);
    }

    #[test]
    fn test_sanitize_path() {
        let validator = PackageValidator::new();

        assert_eq!(
            validator.sanitize_path("notes/1.json"),
            Some(PathBuf::from("notes/1.json"))
        );

        assert_eq!(validator.sanitize_path("../etc/passwd"), None);
        assert_eq!(validator.sanitize_path("/etc/passwd"), None);
    }

    #[test]
    fn test_unknown_file_warning() {
        let validator = PackageValidator::new();

        let result = validator.validate_path("random/file.txt");
        assert!(result.valid); // Valid but with warning
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_format_version() {
        let validator = PackageValidator::new();

        assert!(validator.validate_format_version("1.0").valid);
        assert!(!validator.validate_format_version("2.0").valid);
        assert!(!validator.validate_format_version("invalid").valid);
    }
}
