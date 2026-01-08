//! Package installer for importing .laz packages into local database
//!
//! Handles:
//! - ID conflict resolution (remapping)
//! - Encrypted note handling (with PIN or skip)
//! - Card source_note_id remapping
//! - Installation tracking

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::manifest::Manifest;
use super::reader::{PackageReader, ReaderError};
use super::writer::{PackageCard, PackageNote};

/// How to handle encrypted notes during installation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptedInstallHandling {
    /// Install encrypted notes (requires PIN verification)
    Install,
    /// Skip all encrypted notes
    Skip,
}

/// Installation options
#[derive(Debug, Clone)]
pub struct InstallOptions {
    /// How to handle encrypted notes
    pub encrypted_handling: EncryptedInstallHandling,

    /// Whether to install SRS cards
    pub install_cards: bool,

    /// Whether to overwrite existing notes with same title
    pub overwrite_duplicates: bool,

    /// Author name override (for imported notes)
    pub author_override: Option<String>,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            encrypted_handling: EncryptedInstallHandling::Skip,
            install_cards: true,
            overwrite_duplicates: false,
            author_override: None,
        }
    }
}

impl InstallOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_encrypted_handling(mut self, handling: EncryptedInstallHandling) -> Self {
        self.encrypted_handling = handling;
        self
    }

    pub fn with_cards(mut self, install: bool) -> Self {
        self.install_cards = install;
        self
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite_duplicates = overwrite;
        self
    }
}

/// Result of package installation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    /// Package ID
    pub package_id: String,

    /// Package name
    pub package_name: String,

    /// Package version
    pub package_version: String,

    /// Installation timestamp
    pub installed_at: DateTime<Utc>,

    /// Number of notes installed
    pub notes_installed: usize,

    /// Number of notes skipped (encrypted or duplicate)
    pub notes_skipped: usize,

    /// Number of cards installed
    pub cards_installed: usize,

    /// ID remapping (old_id -> new_id)
    pub id_remap: HashMap<u64, u64>,

    /// List of installed note IDs (new IDs)
    pub installed_note_ids: Vec<u64>,

    /// List of installed card IDs
    pub installed_card_ids: Vec<String>,

    /// Skipped notes with reasons
    pub skipped_notes: Vec<SkippedNote>,

    /// Warnings during installation
    pub warnings: Vec<String>,
}

impl InstallResult {
    fn new(manifest: &Manifest) -> Self {
        Self {
            package_id: manifest.id.clone(),
            package_name: manifest.name.clone(),
            package_version: manifest.version.clone(),
            installed_at: Utc::now(),
            notes_installed: 0,
            notes_skipped: 0,
            cards_installed: 0,
            id_remap: HashMap::new(),
            installed_note_ids: Vec::new(),
            installed_card_ids: Vec::new(),
            skipped_notes: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Check if installation was successful (at least one note installed)
    pub fn is_success(&self) -> bool {
        self.notes_installed > 0
    }

    /// Get total items processed
    pub fn total_processed(&self) -> usize {
        self.notes_installed + self.notes_skipped
    }
}

/// Information about a skipped note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkippedNote {
    pub original_id: u64,
    pub title: String,
    pub reason: SkipReason,
}

/// Reason why a note was skipped
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkipReason {
    /// Note is encrypted and user chose to skip
    EncryptedSkipped,
    /// Note is encrypted but PIN verification failed
    EncryptedPinFailed,
    /// Duplicate note exists and overwrite is disabled
    DuplicateExists,
    /// Note validation failed
    ValidationFailed(String),
}

impl SkipReason {
    pub fn message(&self) -> String {
        match self {
            SkipReason::EncryptedSkipped => "Encrypted note skipped by user choice".to_string(),
            SkipReason::EncryptedPinFailed => "Encrypted note skipped: PIN verification failed".to_string(),
            SkipReason::DuplicateExists => "Duplicate note exists".to_string(),
            SkipReason::ValidationFailed(msg) => format!("Validation failed: {}", msg),
        }
    }
}

/// Trait for database operations during installation
/// Implement this trait for your actual database
pub trait InstallTarget {
    /// Get the next available note ID
    fn next_note_id(&self) -> u64;

    /// Check if a note with the given title exists
    fn note_exists_by_title(&self, title: &str) -> Option<u64>;

    /// Save a note and return its new ID
    fn save_note(&mut self, note: InstalledNote) -> Result<u64, InstallError>;

    /// Save a card
    fn save_card(&mut self, card: InstalledCard) -> Result<(), InstallError>;

    /// Verify PIN for encrypted content (returns true if valid)
    fn verify_pin(&self, pin: &str) -> bool;
}

/// Note ready for installation (with remapped ID)
#[derive(Debug, Clone)]
pub struct InstalledNote {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub encrypted: bool,
    pub note_type: String,
    /// Original package ID for tracking
    pub source_package: Option<String>,
}

impl InstalledNote {
    fn from_package_note(note: &PackageNote, new_id: u64, package_id: Option<String>) -> Self {
        Self {
            id: new_id,
            title: note.title.clone(),
            content: note.content.clone(),
            tags: note.tags.clone(),
            created_at: note.created_at,
            updated_at: note.updated_at,
            encrypted: note.encrypted,
            note_type: note.note_type.clone(),
            source_package: package_id,
        }
    }
}

/// Card ready for installation (with remapped source_note_id)
#[derive(Debug, Clone)]
pub struct InstalledCard {
    pub id: String,
    pub front: String,
    pub back: String,
    pub card_type: String,
    pub source_note_id: Option<u64>,
    /// Original package ID for tracking
    pub source_package: Option<String>,
}

impl InstalledCard {
    fn from_package_card(
        card: &PackageCard,
        id_remap: &HashMap<u64, u64>,
        package_id: Option<String>,
    ) -> Self {
        // Remap source_note_id if it exists
        let remapped_note_id = card
            .source_note_id
            .and_then(|old_id| id_remap.get(&old_id).copied());

        Self {
            id: card.id.clone(),
            front: card.front.clone(),
            back: card.back.clone(),
            card_type: card.card_type.clone(),
            source_note_id: remapped_note_id,
            source_package: package_id,
        }
    }
}

/// Package installer
pub struct PackageInstaller {
    options: InstallOptions,
    /// PIN for encrypted notes (if provided)
    pin: Option<String>,
}

impl PackageInstaller {
    /// Create a new installer with default options
    pub fn new() -> Self {
        Self {
            options: InstallOptions::default(),
            pin: None,
        }
    }

    /// Create with custom options
    pub fn with_options(options: InstallOptions) -> Self {
        Self { options, pin: None }
    }

    /// Set PIN for encrypted notes
    pub fn with_pin(mut self, pin: String) -> Self {
        self.pin = Some(pin);
        self.options.encrypted_handling = EncryptedInstallHandling::Install;
        self
    }

    /// Install a package from a file path
    pub fn install_from_file<T: InstallTarget>(
        &self,
        path: &Path,
        target: &mut T,
    ) -> Result<InstallResult, InstallError> {
        let mut reader = PackageReader::open(path)?;
        self.install_from_reader(&mut reader, target)
    }

    /// Install a package from a reader
    pub fn install_from_reader<R: std::io::Read + std::io::Seek, T: InstallTarget>(
        &self,
        reader: &mut PackageReader<R>,
        target: &mut T,
    ) -> Result<InstallResult, InstallError> {
        let manifest = reader.manifest().clone();
        let mut result = InstallResult::new(&manifest);

        // Check for encrypted notes
        let has_encrypted = reader.has_encrypted_notes();
        if has_encrypted {
            match self.options.encrypted_handling {
                EncryptedInstallHandling::Install => {
                    // Verify PIN if required
                    if let Some(ref pin) = self.pin {
                        if !target.verify_pin(pin) {
                            return Err(InstallError::PinVerificationFailed);
                        }
                    } else {
                        return Err(InstallError::PinRequired);
                    }
                }
                EncryptedInstallHandling::Skip => {
                    result.warnings.push(
                        "Package contains encrypted notes which will be skipped".to_string(),
                    );
                }
            }
        }

        // Read all notes
        let notes = reader.read_notes()?;

        // Install notes
        for note in &notes {
            match self.install_note(note, &manifest, target, &mut result) {
                Ok(new_id) => {
                    result.id_remap.insert(note.id, new_id);
                    result.installed_note_ids.push(new_id);
                    result.notes_installed += 1;
                }
                Err(InstallNoteError::Skipped(reason)) => {
                    result.skipped_notes.push(SkippedNote {
                        original_id: note.id,
                        title: note.title.clone(),
                        reason,
                    });
                    result.notes_skipped += 1;
                }
                Err(InstallNoteError::Error(e)) => {
                    return Err(e);
                }
            }
        }

        // Install cards if enabled
        if self.options.install_cards {
            let cards = reader.read_cards()?;

            for card in &cards {
                match self.install_card(card, &manifest, &result.id_remap, target) {
                    Ok(()) => {
                        result.installed_card_ids.push(card.id.clone());
                        result.cards_installed += 1;
                    }
                    Err(e) => {
                        // Card installation failure is a warning, not fatal
                        result.warnings.push(format!("Failed to install card {}: {}", card.id, e));
                    }
                }
            }
        }

        Ok(result)
    }

    /// Install a single note
    fn install_note<T: InstallTarget>(
        &self,
        note: &PackageNote,
        manifest: &Manifest,
        target: &mut T,
        result: &mut InstallResult,
    ) -> Result<u64, InstallNoteError> {
        // Handle encrypted notes
        if note.encrypted {
            match self.options.encrypted_handling {
                EncryptedInstallHandling::Skip => {
                    return Err(InstallNoteError::Skipped(SkipReason::EncryptedSkipped));
                }
                EncryptedInstallHandling::Install => {
                    // PIN should already be verified at this point
                    if self.pin.is_none() {
                        return Err(InstallNoteError::Skipped(SkipReason::EncryptedPinFailed));
                    }
                }
            }
        }

        // Check for duplicates
        if !self.options.overwrite_duplicates {
            if let Some(existing_id) = target.note_exists_by_title(&note.title) {
                return Err(InstallNoteError::Skipped(SkipReason::DuplicateExists));
            }
        }

        // Get new ID
        let new_id = target.next_note_id();

        // Create installed note
        let installed = InstalledNote::from_package_note(note, new_id, Some(manifest.id.clone()));

        // Save to database
        target
            .save_note(installed)
            .map_err(InstallNoteError::Error)?;

        Ok(new_id)
    }

    /// Install a single card
    fn install_card<T: InstallTarget>(
        &self,
        card: &PackageCard,
        manifest: &Manifest,
        id_remap: &HashMap<u64, u64>,
        target: &mut T,
    ) -> Result<(), InstallError> {
        // Skip cards whose source note was skipped
        if let Some(source_id) = card.source_note_id {
            if !id_remap.contains_key(&source_id) {
                // Source note was not installed (probably skipped)
                return Ok(()); // Silent skip
            }
        }

        let installed = InstalledCard::from_package_card(card, id_remap, Some(manifest.id.clone()));
        target.save_card(installed)?;

        Ok(())
    }
}

impl Default for PackageInstaller {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal error for note installation
enum InstallNoteError {
    Skipped(SkipReason),
    Error(InstallError),
}

/// Errors during installation
#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Reader error: {0}")]
    Reader(#[from] ReaderError),

    #[error("PIN required for encrypted notes")]
    PinRequired,

    #[error("PIN verification failed")]
    PinVerificationFailed,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Package already installed: {0}")]
    AlreadyInstalled(String),
}

// ============================================================================
// Installation Record Management
// ============================================================================

/// Record of an installed package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackageRecord {
    /// Package ID
    pub package_id: String,

    /// Package name
    pub package_name: String,

    /// Package version (timestamp)
    pub version: String,

    /// Installation timestamp
    pub installed_at: DateTime<Utc>,

    /// IDs of notes installed from this package
    pub note_ids: Vec<u64>,

    /// IDs of cards installed from this package
    pub card_ids: Vec<String>,
}

impl InstalledPackageRecord {
    pub fn from_result(result: &InstallResult) -> Self {
        Self {
            package_id: result.package_id.clone(),
            package_name: result.package_name.clone(),
            version: result.package_version.clone(),
            installed_at: result.installed_at,
            note_ids: result.installed_note_ids.clone(),
            card_ids: result.installed_card_ids.clone(),
        }
    }
}

/// Manager for tracking installed packages
pub struct InstalledPackagesStore {
    path: PathBuf,
    records: Vec<InstalledPackageRecord>,
}

impl InstalledPackagesStore {
    /// Open or create the installed packages store
    pub fn open(data_dir: &Path) -> Result<Self, std::io::Error> {
        let path = data_dir.join("installed_packages.jsonl");
        let records = if path.exists() {
            Self::load_records(&path)?
        } else {
            Vec::new()
        };

        Ok(Self { path, records })
    }

    /// Load records from file
    fn load_records(path: &Path) -> Result<Vec<InstalledPackageRecord>, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut records = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(record) = serde_json::from_str(&line) {
                records.push(record);
            }
        }

        Ok(records)
    }

    /// Save records to file
    fn save_records(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(&self.path)?;
        for record in &self.records {
            let json = serde_json::to_string(record)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            writeln!(file, "{}", json)?;
        }
        Ok(())
    }

    /// Record a new installation
    pub fn record_installation(&mut self, result: &InstallResult) -> Result<(), std::io::Error> {
        let record = InstalledPackageRecord::from_result(result);
        self.records.push(record);
        self.save_records()
    }

    /// Check if a package is already installed
    pub fn is_installed(&self, package_id: &str) -> bool {
        self.records.iter().any(|r| r.package_id == package_id)
    }

    /// Get installed version of a package
    pub fn get_installed_version(&self, package_id: &str) -> Option<&str> {
        self.records
            .iter()
            .find(|r| r.package_id == package_id)
            .map(|r| r.version.as_str())
    }

    /// Check if an update is available (new version is newer)
    pub fn has_update(&self, package_id: &str, new_version: &str) -> bool {
        match self.get_installed_version(package_id) {
            Some(installed) => new_version > installed,
            None => true, // Not installed = update available
        }
    }

    /// Get all installed packages
    pub fn all(&self) -> &[InstalledPackageRecord] {
        &self.records
    }

    /// Get record by package ID
    pub fn get(&self, package_id: &str) -> Option<&InstalledPackageRecord> {
        self.records.iter().find(|r| r.package_id == package_id)
    }

    /// Remove installation record (for uninstall)
    pub fn remove(&mut self, package_id: &str) -> Result<Option<InstalledPackageRecord>, std::io::Error> {
        let index = self.records.iter().position(|r| r.package_id == package_id);
        let removed = index.map(|i| self.records.remove(i));
        if removed.is_some() {
            self.save_records()?;
        }
        Ok(removed)
    }
}

// ============================================================================
// Mock Implementation for Testing
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Mock database for testing
    struct MockDatabase {
        next_id: u64,
        notes: HashMap<u64, InstalledNote>,
        cards: Vec<InstalledCard>,
        titles: HashMap<String, u64>,
        valid_pin: Option<String>,
    }

    impl MockDatabase {
        fn new() -> Self {
            Self {
                next_id: 1,
                notes: HashMap::new(),
                cards: Vec::new(),
                titles: HashMap::new(),
                valid_pin: Some("123456".to_string()),
            }
        }

        fn with_existing_note(mut self, id: u64, title: &str) -> Self {
            self.titles.insert(title.to_string(), id);
            self.next_id = self.next_id.max(id + 1);
            self
        }
    }

    impl InstallTarget for MockDatabase {
        fn next_note_id(&self) -> u64 {
            self.next_id
        }

        fn note_exists_by_title(&self, title: &str) -> Option<u64> {
            self.titles.get(title).copied()
        }

        fn save_note(&mut self, note: InstalledNote) -> Result<u64, InstallError> {
            let id = note.id;
            self.titles.insert(note.title.clone(), id);
            self.notes.insert(id, note);
            self.next_id = self.next_id.max(id + 1);
            Ok(id)
        }

        fn save_card(&mut self, card: InstalledCard) -> Result<(), InstallError> {
            self.cards.push(card);
            Ok(())
        }

        fn verify_pin(&self, pin: &str) -> bool {
            self.valid_pin.as_ref().map(|p| p == pin).unwrap_or(false)
        }
    }

    fn create_test_package(encrypted_notes: bool) -> Vec<u8> {
        use crate::laz::writer::{EncryptedNoteHandling, PackageBuilder};

        let mut builder = PackageBuilder::new("Test Package", "A test")
            .author("Test")
            .encrypted_handling(if encrypted_notes {
                EncryptedNoteHandling::Include
            } else {
                EncryptedNoteHandling::Exclude
            });

        builder.add_note(PackageNote {
            id: 100,
            title: "Note 1".to_string(),
            content: "Content 1".to_string(),
            tags: vec!["test".to_string()],
            created_at: 1704672000,
            updated_at: 1704672000,
            encrypted: false,
            note_type: "Note".to_string(),
        });

        builder.add_note(PackageNote {
            id: 200,
            title: "Note 2".to_string(),
            content: "Content 2".to_string(),
            tags: vec![],
            created_at: 1704672000,
            updated_at: 1704672000,
            encrypted: encrypted_notes,
            note_type: "Note".to_string(),
        });

        builder.add_card(PackageCard {
            id: "card-1".to_string(),
            front: "Q1".to_string(),
            back: "A1".to_string(),
            card_type: "Basic".to_string(),
            source_note_id: Some(100),
        });

        builder.build().unwrap().data
    }

    #[test]
    fn test_basic_installation() {
        let data = create_test_package(false);
        let cursor = Cursor::new(data);
        let mut reader = PackageReader::from_reader(cursor).unwrap();

        let mut db = MockDatabase::new();
        let installer = PackageInstaller::new();

        let result = installer.install_from_reader(&mut reader, &mut db).unwrap();

        assert!(result.is_success());
        assert_eq!(result.notes_installed, 2);
        assert_eq!(result.cards_installed, 1);
        assert_eq!(db.notes.len(), 2);
    }

    #[test]
    fn test_id_remapping() {
        let data = create_test_package(false);
        let cursor = Cursor::new(data);
        let mut reader = PackageReader::from_reader(cursor).unwrap();

        let mut db = MockDatabase::new();
        let installer = PackageInstaller::new();

        let result = installer.install_from_reader(&mut reader, &mut db).unwrap();

        // Original IDs were 100 and 200
        // Should be remapped to 1 and 2
        assert!(result.id_remap.contains_key(&100));
        assert!(result.id_remap.contains_key(&200));
        assert_eq!(result.id_remap[&100], 1);
        assert_eq!(result.id_remap[&200], 2);
    }

    #[test]
    fn test_card_source_remapping() {
        let data = create_test_package(false);
        let cursor = Cursor::new(data);
        let mut reader = PackageReader::from_reader(cursor).unwrap();

        let mut db = MockDatabase::new();
        let installer = PackageInstaller::new();

        let result = installer.install_from_reader(&mut reader, &mut db).unwrap();

        // Card's source_note_id should be remapped from 100 to 1
        let card = &db.cards[0];
        assert_eq!(card.source_note_id, Some(1));
    }

    #[test]
    fn test_skip_encrypted_notes() {
        let data = create_test_package(true);
        let cursor = Cursor::new(data);
        let mut reader = PackageReader::from_reader(cursor).unwrap();

        let mut db = MockDatabase::new();
        let installer = PackageInstaller::with_options(
            InstallOptions::new().with_encrypted_handling(EncryptedInstallHandling::Skip),
        );

        let result = installer.install_from_reader(&mut reader, &mut db).unwrap();

        assert_eq!(result.notes_installed, 1); // Only non-encrypted
        assert_eq!(result.notes_skipped, 1);   // Encrypted was skipped
        assert!(result.skipped_notes.iter().any(|s| matches!(s.reason, SkipReason::EncryptedSkipped)));
    }

    #[test]
    fn test_install_encrypted_with_pin() {
        let data = create_test_package(true);
        let cursor = Cursor::new(data);
        let mut reader = PackageReader::from_reader(cursor).unwrap();

        let mut db = MockDatabase::new();
        let installer = PackageInstaller::new().with_pin("123456".to_string());

        let result = installer.install_from_reader(&mut reader, &mut db).unwrap();

        assert_eq!(result.notes_installed, 2); // Both installed
        assert_eq!(result.notes_skipped, 0);
    }

    #[test]
    fn test_wrong_pin() {
        let data = create_test_package(true);
        let cursor = Cursor::new(data);
        let mut reader = PackageReader::from_reader(cursor).unwrap();

        let mut db = MockDatabase::new();
        let installer = PackageInstaller::new().with_pin("wrong".to_string());

        let result = installer.install_from_reader(&mut reader, &mut db);

        assert!(matches!(result, Err(InstallError::PinVerificationFailed)));
    }

    #[test]
    fn test_duplicate_handling() {
        let data = create_test_package(false);
        let cursor = Cursor::new(data);
        let mut reader = PackageReader::from_reader(cursor).unwrap();

        // Database already has "Note 1"
        let mut db = MockDatabase::new().with_existing_note(999, "Note 1");
        let installer = PackageInstaller::with_options(
            InstallOptions::new().with_overwrite(false),
        );

        let result = installer.install_from_reader(&mut reader, &mut db).unwrap();

        assert_eq!(result.notes_installed, 1); // Only "Note 2"
        assert_eq!(result.notes_skipped, 1);   // "Note 1" was duplicate
        assert!(result.skipped_notes.iter().any(|s| matches!(s.reason, SkipReason::DuplicateExists)));
    }

    #[test]
    fn test_installed_packages_store() {
        let temp_dir = std::env::temp_dir().join("lazarus_test_install");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let mut store = InstalledPackagesStore::open(&temp_dir).unwrap();

        assert!(!store.is_installed("pkg-1"));

        // Create a mock result
        let result = InstallResult {
            package_id: "pkg-1".to_string(),
            package_name: "Test Package".to_string(),
            package_version: "2026-01-08T10:00:00Z".to_string(),
            installed_at: Utc::now(),
            notes_installed: 5,
            notes_skipped: 0,
            cards_installed: 10,
            id_remap: HashMap::new(),
            installed_note_ids: vec![1, 2, 3, 4, 5],
            installed_card_ids: vec!["c1".to_string(), "c2".to_string()],
            skipped_notes: vec![],
            warnings: vec![],
        };

        store.record_installation(&result).unwrap();

        assert!(store.is_installed("pkg-1"));
        assert_eq!(store.get_installed_version("pkg-1"), Some("2026-01-08T10:00:00Z"));
        assert!(store.has_update("pkg-1", "2026-01-09T10:00:00Z"));
        assert!(!store.has_update("pkg-1", "2026-01-07T10:00:00Z"));

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
