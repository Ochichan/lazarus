//! Package writer for creating .laz files
//!
//! The PackageBuilder collects notes, cards, and assets, then compresses
//! them into a .laz package (ZIP format) with a manifest.

use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

use super::manifest::{
    AssetInfo, AssetType, Author, EncryptionInfo, Manifest, PackageStats, PackageWarning,
    PACKAGE_MAX_SIZE, VIDEO_MAX_SIZE, VIDEO_WARNING_THRESHOLD,
};

/// How to handle encrypted notes during package creation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptedNoteHandling {
    /// Include encrypted notes as-is (recipients need same PIN)
    Include,
    /// Exclude all encrypted notes from the package
    Exclude,
}

/// A note to be included in the package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageNote {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub encrypted: bool,
    pub note_type: String,
}

/// An SRS card to be included in the package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageCard {
    pub id: String,
    pub front: String,
    pub back: String,
    pub card_type: String,
    pub source_note_id: Option<u64>,
}

/// An asset file to be included in the package
#[derive(Debug, Clone)]
pub struct PackageAsset {
    /// Source file path on disk
    pub source_path: PathBuf,
    /// Target path within the package (e.g., "assets/image.png")
    pub package_path: String,
    /// File size in bytes
    pub size_bytes: u64,
    /// Asset type
    pub asset_type: AssetType,
}

impl PackageAsset {
    /// Create from a file path
    pub fn from_path(source: &Path, _package_name: &str) -> std::io::Result<Self> {
        let metadata = std::fs::metadata(source)?;
        let filename = source
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        Ok(Self {
            source_path: source.to_path_buf(),
            package_path: format!("assets/{}", filename),
            size_bytes: metadata.len(),
            asset_type: AssetType::from_path(source),
        })
    }
}

/// Result of package build operation
#[derive(Debug)]
pub struct BuildResult {
    /// The built package data (ZIP bytes)
    pub data: Vec<u8>,
    /// Final manifest
    pub manifest: Manifest,
    /// Warnings generated during build
    pub warnings: Vec<PackageWarning>,
    /// Notes that were excluded (with reasons)
    pub excluded_notes: Vec<(u64, String)>,
    /// Assets that were excluded (with reasons)
    pub excluded_assets: Vec<(String, String)>,
}

/// Builder for creating .laz packages
pub struct PackageBuilder {
    name: String,
    description: String,
    author: Author,
    tags: Vec<String>,
    language: String,
    license: String,
    notes: Vec<PackageNote>,
    cards: Vec<PackageCard>,
    assets: Vec<PackageAsset>,
    encrypted_handling: EncryptedNoteHandling,
    warnings: Vec<PackageWarning>,
    excluded_notes: Vec<(u64, String)>,
    excluded_assets: Vec<(String, String)>,
}

impl PackageBuilder {
    /// Create a new package builder
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            author: Author::default(),
            tags: Vec::new(),
            language: "en".to_string(),
            license: "CC-BY-4.0".to_string(),
            notes: Vec::new(),
            cards: Vec::new(),
            assets: Vec::new(),
            encrypted_handling: EncryptedNoteHandling::Include,
            warnings: Vec::new(),
            excluded_notes: Vec::new(),
            excluded_assets: Vec::new(),
        }
    }

    /// Set the package author
    pub fn author(mut self, name: impl Into<String>) -> Self {
        self.author = Author::new(name.into());
        self
    }

    /// Set author with contact
    pub fn author_with_contact(mut self, name: impl Into<String>, contact: impl Into<String>) -> Self {
        self.author = Author::new(name.into()).with_contact(contact.into());
        self
    }

    /// Add tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Set language
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = lang.into();
        self
    }

    /// Set license
    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license = license.into();
        self
    }

    /// Set how to handle encrypted notes
    pub fn encrypted_handling(mut self, handling: EncryptedNoteHandling) -> Self {
        self.encrypted_handling = handling;
        self
    }

    /// Add a note to the package
    pub fn add_note(&mut self, note: PackageNote) {
        if note.encrypted {
            match self.encrypted_handling {
                EncryptedNoteHandling::Include => {
                    self.notes.push(note);
                }
                EncryptedNoteHandling::Exclude => {
                    self.excluded_notes.push((
                        note.id,
                        "Encrypted notes excluded by user choice".to_string(),
                    ));
                }
            }
        } else {
            self.notes.push(note);
        }
    }

    /// Add multiple notes
    pub fn add_notes(&mut self, notes: impl IntoIterator<Item = PackageNote>) {
        for note in notes {
            self.add_note(note);
        }
    }

    /// Add a card to the package
    pub fn add_card(&mut self, card: PackageCard) {
        self.cards.push(card);
    }

    /// Add multiple cards
    pub fn add_cards(&mut self, cards: impl IntoIterator<Item = PackageCard>) {
        self.cards.extend(cards);
    }

    /// Add an asset file
    pub fn add_asset(&mut self, asset: PackageAsset) -> Result<(), PackageWarning> {
        if asset.asset_type == AssetType::Video {
            if asset.size_bytes > VIDEO_MAX_SIZE {
                let warning = PackageWarning::OversizedVideo {
                    filename: asset.package_path.clone(),
                    size_bytes: asset.size_bytes,
                    max_bytes: VIDEO_MAX_SIZE,
                };
                self.excluded_assets.push((
                    asset.package_path,
                    format!("Exceeds maximum size of {} MB", VIDEO_MAX_SIZE / 1024 / 1024),
                ));
                self.warnings.push(warning.clone());
                return Err(warning);
            }

            if asset.size_bytes > VIDEO_WARNING_THRESHOLD {
                self.warnings.push(PackageWarning::LargeVideo {
                    filename: asset.package_path.clone(),
                    size_bytes: asset.size_bytes,
                });
            }
        }

        self.assets.push(asset);
        Ok(())
    }

    /// Add an asset from a file path
    pub fn add_asset_from_path(&mut self, path: &Path) -> std::io::Result<()> {
        let asset = PackageAsset::from_path(path, &self.name)?;
        let _ = self.add_asset(asset);
        Ok(())
    }

    /// Get current warnings
    pub fn warnings(&self) -> &[PackageWarning] {
        &self.warnings
    }

    /// Get blocking warnings (that prevent build)
    pub fn blocking_warnings(&self) -> Vec<&PackageWarning> {
        self.warnings.iter().filter(|w| w.is_blocking()).collect()
    }

    /// Check if build can proceed
    pub fn can_build(&self) -> bool {
        !self.warnings.iter().any(|w| w.is_blocking())
    }

    /// Preview what will be in the package
    pub fn preview(&self) -> PackagePreview {
        let encrypted_count = self.notes.iter().filter(|n| n.encrypted).count();

        PackagePreview {
            name: self.name.clone(),
            description: self.description.clone(),
            note_count: self.notes.len(),
            encrypted_note_count: encrypted_count,
            card_count: self.cards.len(),
            assets: self
                .assets
                .iter()
                .map(|a| AssetInfo::new(a.package_path.clone(), a.size_bytes))
                .collect(),
            warnings: self.warnings.clone(),
            excluded_notes: self.excluded_notes.clone(),
            excluded_assets: self.excluded_assets.clone(),
        }
    }

    /// Build the package
    pub fn build(mut self) -> Result<BuildResult, BuildError> {
        if let Some(blocking) = self.warnings.iter().find(|w| w.is_blocking()) {
            return Err(BuildError::BlockingWarning(blocking.message()));
        }

        let total_size: u64 = self
            .notes
            .iter()
            .map(|n| n.content.len() as u64)
            .sum::<u64>()
            + self.assets.iter().map(|a| a.size_bytes).sum::<u64>();

        if total_size > PACKAGE_MAX_SIZE {
            self.warnings.push(PackageWarning::PackageTooLarge {
                size_bytes: total_size,
                max_bytes: PACKAGE_MAX_SIZE,
            });
            return Err(BuildError::PackageTooLarge {
                size: total_size,
                max: PACKAGE_MAX_SIZE,
            });
        }

        let encrypted_count = self.notes.iter().filter(|n| n.encrypted).count();
        let video_count = self
            .assets
            .iter()
            .filter(|a| a.asset_type == AssetType::Video)
            .count();

        if encrypted_count > 0 {
            self.warnings.push(PackageWarning::EncryptedNotesIncluded {
                count: encrypted_count,
            });
        }

        let mut manifest = Manifest::new(self.name.clone(), self.description.clone());
        manifest.author = self.author.clone();
        manifest.tags = self.tags.clone();
        manifest.language = self.language.clone();
        manifest.license = self.license.clone();
        manifest.stats = PackageStats {
            note_count: self.notes.len(),
            encrypted_note_count: encrypted_count,
            card_count: self.cards.len(),
            asset_count: self.assets.len(),
            video_count,
            total_size_bytes: total_size,
        };

        if encrypted_count > 0 {
            manifest.encryption = Some(EncryptionInfo::new(encrypted_count));
        }

        let zip_data = self.build_zip(&manifest)?;
        manifest.update_checksum(&zip_data);

        Ok(BuildResult {
            data: zip_data,
            manifest,
            warnings: self.warnings,
            excluded_notes: self.excluded_notes,
            excluded_assets: self.excluded_assets,
        })
    }

    /// Build the ZIP file contents
    fn build_zip(&self, manifest: &Manifest) -> Result<Vec<u8>, BuildError> {
        let buffer = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(buffer);

        let options: FileOptions<()> = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o644);

        // Write manifest.json
        let manifest_json = manifest
            .to_json()
            .map_err(|e| BuildError::Serialization(e.to_string()))?;
        zip.start_file("manifest.json", options.clone())?;
        zip.write_all(manifest_json.as_bytes())?;

        // Write notes/
        for note in &self.notes {
            let note_json = serde_json::json!({
                "id": note.id,
                "title": note.title,
                "content": note.content,
                "tags": note.tags,
                "created_at": note.created_at,
                "updated_at": note.updated_at,
                "encrypted": note.encrypted,
                "note_type": note.note_type,
            });

            let filename = format!("notes/{}.json", note.id);
            zip.start_file(&filename, options.clone())?;
            zip.write_all(
                serde_json::to_string_pretty(&note_json)
                    .map_err(|e| BuildError::Serialization(e.to_string()))?
                    .as_bytes(),
            )?;
        }

        // Write cards/cards.jsonl
        if !self.cards.is_empty() {
            zip.start_file("cards/cards.jsonl", options.clone())?;
            for card in &self.cards {
                let card_json = serde_json::json!({
                    "id": card.id,
                    "front": card.front,
                    "back": card.back,
                    "card_type": card.card_type,
                    "source_note_id": card.source_note_id,
                });
                let line = serde_json::to_string(&card_json)
                    .map_err(|e| BuildError::Serialization(e.to_string()))?;
                zip.write_all(line.as_bytes())?;
                zip.write_all(b"\n")?;
            }
        }

        // Write assets/
        for asset in &self.assets {
            let asset_options: FileOptions<()> = if asset.asset_type == AssetType::Video {
                FileOptions::default()
                    .compression_method(CompressionMethod::Stored)
                    .unix_permissions(0o644)
            } else {
                options.clone()
            };

            zip.start_file(&asset.package_path, asset_options)?;

            let mut file = File::open(&asset.source_path)?;
            let mut buf = [0u8; 8192];
            loop {
                let n = file.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                zip.write_all(&buf[..n])?;
            }
        }

        // Finish and get buffer back
        let result = zip.finish()?;
        Ok(result.into_inner())
    }

    /// Save package to a file
    pub fn save_to_file(self, path: &Path) -> Result<BuildResult, BuildError> {
        let result = self.build()?;
        std::fs::write(path, &result.data)?;
        Ok(result)
    }
}

/// Preview of package contents before building
#[derive(Debug, Clone)]
pub struct PackagePreview {
    pub name: String,
    pub description: String,
    pub note_count: usize,
    pub encrypted_note_count: usize,
    pub card_count: usize,
    pub assets: Vec<AssetInfo>,
    pub warnings: Vec<PackageWarning>,
    pub excluded_notes: Vec<(u64, String)>,
    pub excluded_assets: Vec<(String, String)>,
}

impl PackagePreview {
    pub fn estimated_size(&self) -> u64 {
        self.assets.iter().map(|a| a.size_bytes).sum()
    }

    pub fn has_blocking_issues(&self) -> bool {
        self.warnings.iter().any(|w| w.is_blocking())
    }
}

/// Errors that can occur during package building
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Package too large: {size} bytes (max: {max} bytes)")]
    PackageTooLarge { size: u64, max: u64 },

    #[error("Blocking warning: {0}")]
    BlockingWarning(String),

    #[error("No notes to package")]
    NoNotes,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_note(id: u64, encrypted: bool) -> PackageNote {
        PackageNote {
            id,
            title: format!("Note {}", id),
            content: "Test content".to_string(),
            tags: vec!["test".to_string()],
            created_at: 1704672000,
            updated_at: 1704672000,
            encrypted,
            note_type: "Note".to_string(),
        }
    }

    #[test]
    fn test_builder_creation() {
        let builder = PackageBuilder::new("Test Package", "A test")
            .author("Test Author")
            .tags(vec!["physics".to_string()])
            .language("ko");

        let preview = builder.preview();
        assert_eq!(preview.name, "Test Package");
        assert_eq!(preview.note_count, 0);
    }

    #[test]
    fn test_add_notes() {
        let mut builder = PackageBuilder::new("Test", "Test");
        builder.add_note(sample_note(1, false));
        builder.add_note(sample_note(2, false));

        let preview = builder.preview();
        assert_eq!(preview.note_count, 2);
    }

    #[test]
    fn test_build_simple_package() {
        let mut builder = PackageBuilder::new("Physics 101", "Basic physics concepts")
            .author("Teacher")
            .language("en");

        builder.add_note(sample_note(1, false));
        builder.add_note(sample_note(2, false));

        let result = builder.build().unwrap();

        assert!(!result.data.is_empty());
        assert_eq!(result.manifest.name, "Physics 101");
        assert_eq!(result.manifest.stats.note_count, 2);
    }
}
