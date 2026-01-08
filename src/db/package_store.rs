//! Local package storage and management

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::laz::LazPackage;

/// Package extension
pub const PACKAGE_EXTENSION: &str = "laz";

/// Summary of a stored package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSummary {
    pub id: String,
    pub title: String,
    pub author: String,
    pub version: u32,
    pub description: String,
    pub language: String,
    pub note_count: usize,
    pub card_count: usize,
    pub file_size: u64,
    pub added_at: DateTime<Utc>,
    pub filename: String,
}

/// Package store
pub struct PackageStore {
    packages_dir: PathBuf,
    index_path: PathBuf,
    index: PackageIndex,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PackageIndex {
    packages: Vec<PackageSummary>,
    updated_at: DateTime<Utc>,
}

impl PackageStore {
    /// Open or create a package store
    pub fn open(data_dir: &Path) -> Result<Self, PackageStoreError> {
        let packages_dir = data_dir.join("packages");
        let index_path = data_dir.join("packages_index.json");

        fs::create_dir_all(&packages_dir)?;

        let index = if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            PackageIndex::default()
        };

        let mut store = Self {
            packages_dir,
            index_path,
            index,
        };

        store.sync_index()?;
        Ok(store)
    }

    /// Sync index with actual files
    fn sync_index(&mut self) -> Result<(), PackageStoreError> {
        let mut found_ids: Vec<String> = Vec::new();
        let mut updated = false;

        for entry in fs::read_dir(&self.packages_dir)? {
            let entry = entry?;
            let path = entry.path();

            if !is_package_file(&path) {
                continue;
            }

            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            match LazPackage::import(&path) {
                Ok(pkg) => {
                    let id = pkg.meta.uuid.clone();
                    found_ids.push(id.clone());

                    if !self.index.packages.iter().any(|p| p.id == id) {
                        let file_size = entry.metadata()?.len();
                        let summary = PackageSummary {
                            id,
                            title: pkg.meta.title,
                            author: pkg.meta.author,
                            version: pkg.meta.version,
                            description: pkg.meta.description,
                            language: pkg.meta.language,
                            note_count: pkg.content.len(),
                            card_count: pkg.srs.len(),
                            file_size,
                            added_at: Utc::now(),
                            filename,
                        };
                        self.index.packages.push(summary);
                        updated = true;
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to read package {}: {}", path.display(), e);
                }
            }
        }

        let before_len = self.index.packages.len();
        self.index.packages.retain(|p| found_ids.contains(&p.id));
        if self.index.packages.len() != before_len {
            updated = true;
        }

        if updated {
            self.save_index()?;
        }

        Ok(())
    }

    fn save_index(&mut self) -> Result<(), PackageStoreError> {
        self.index.updated_at = Utc::now();
        let json = serde_json::to_string_pretty(&self.index)?;
        fs::write(&self.index_path, json)?;
        Ok(())
    }

    pub fn list(&self) -> Vec<PackageSummary> {
        self.index.packages.clone()
    }

    pub fn get(&self, id: &str) -> Option<&PackageSummary> {
        self.index.packages.iter().find(|p| p.id == id)
    }

    pub fn get_path(&self, id: &str) -> Option<PathBuf> {
        self.get(id).map(|p| self.packages_dir.join(&p.filename))
    }

    pub fn exists(&self, id: &str) -> bool {
        self.get(id).is_some()
    }

    pub fn add_from_file(&mut self, source: &Path) -> Result<PackageSummary, PackageStoreError> {
        let pkg = LazPackage::import(source)?;
        let id = pkg.meta.uuid.clone();

        if self.exists(&id) {
            return Err(PackageStoreError::AlreadyExists(id));
        }

        let filename = source
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("package.laz")
            .to_string();

        let dest = self.packages_dir.join(&filename);
        fs::copy(source, &dest)?;

        let file_size = fs::metadata(&dest)?.len();
        let summary = PackageSummary {
            id,
            title: pkg.meta.title,
            author: pkg.meta.author,
            version: pkg.meta.version,
            description: pkg.meta.description,
            language: pkg.meta.language,
            note_count: pkg.content.len(),
            card_count: pkg.srs.len(),
            file_size,
            added_at: Utc::now(),
            filename,
        };

        self.index.packages.push(summary.clone());
        self.save_index()?;

        Ok(summary)
    }

    pub fn remove(&mut self, id: &str) -> Result<bool, PackageStoreError> {
        let path = match self.get_path(id) {
            Some(p) => p,
            None => return Ok(false),
        };

        if path.exists() {
            fs::remove_file(&path)?;
        }

        self.index.packages.retain(|p| p.id != id);
        self.save_index()?;

        Ok(true)
    }

    pub fn get_data(&self, id: &str) -> Result<Vec<u8>, PackageStoreError> {
        let path = self
            .get_path(id)
            .ok_or_else(|| PackageStoreError::NotFound(id.to_string()))?;
        Ok(fs::read(&path)?)
    }

    pub fn count(&self) -> usize {
        self.index.packages.len()
    }
}

/// Check if file is a .laz package
pub fn is_package_file(path: &Path) -> bool {
    path.extension()
        .map(|e| e == PACKAGE_EXTENSION)
        .unwrap_or(false)
}

/// Errors for package store
#[derive(Debug, thiserror::Error)]
pub enum PackageStoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Package error: {0}")]
    Package(#[from] crate::error::LazarusError),

    #[error("Package not found: {0}")]
    NotFound(String),

    #[error("Package already exists: {0}")]
    AlreadyExists(String),
}
