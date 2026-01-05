//! .laz 패키지 읽기

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

use super::{Curriculum, LazPackage, Manifest, NoteContent, PackageMeta, SrsCard, MIMETYPE};
use crate::error::{LazarusError, Result};

impl LazPackage {
    /// .laz 파일에서 가져오기
    pub fn import<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path.as_ref()).map_err(LazarusError::Io)?;
        let mut archive = ZipArchive::new(file).map_err(|e| {
            LazarusError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            ))
        })?;

        // 1. mimetype 확인
        let mimetype = read_file_string(&mut archive, "mimetype")?;
        if mimetype.trim() != MIMETYPE {
            return Err(LazarusError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("잘못된 MIME 타입: {}", mimetype),
            )));
        }

        // 2. meta.json
        let meta_str = read_file_string(&mut archive, "meta.json")?;
        let meta: PackageMeta =
            serde_json::from_str(&meta_str).map_err(|e| LazarusError::Serialize(e.to_string()))?;

        // 3. manifest.json
        let manifest_str = read_file_string(&mut archive, "manifest.json")?;
        let manifest: Manifest = serde_json::from_str(&manifest_str)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;

        // 4. curriculum.json
        let curriculum_str = read_file_string(&mut archive, "curriculum.json")?;
        let curriculum: Curriculum = serde_json::from_str(&curriculum_str)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;

        // 5. srs.json
        let srs_str = read_file_string(&mut archive, "srs.json")?;
        let srs: Vec<SrsCard> =
            serde_json::from_str(&srs_str).map_err(|e| LazarusError::Serialize(e.to_string()))?;

        // 6. content/*.json
        let mut content = HashMap::new();
        let file_names: Vec<String> = archive.file_names().map(|s| s.to_string()).collect();

        for name in file_names {
            if name.starts_with("content/") && name.ends_with(".json") {
                let note_str = read_file_string(&mut archive, &name)?;
                let note: NoteContent = serde_json::from_str(&note_str)
                    .map_err(|e| LazarusError::Serialize(e.to_string()))?;
                content.insert(note.id.clone(), note);
            }
        }

        // 7. assets 목록
        let mut assets = Vec::new();
        let file_names: Vec<String> = archive.file_names().map(|s| s.to_string()).collect();
        for name in file_names {
            if name.starts_with("assets/") {
                assets.push(name);
            }
        }

        tracing::info!(
            "패키지 가져오기 완료: {} (노트 {}개, 카드 {}개)",
            meta.title,
            content.len(),
            srs.len()
        );

        Ok(Self {
            meta,
            manifest,
            curriculum,
            srs,
            content,
            assets,
        })
    }

    /// 무결성 검사
    pub fn verify_integrity<P: AsRef<Path>>(&self, path: P) -> Result<VerifyResult> {
        let file = File::open(path.as_ref()).map_err(LazarusError::Io)?;
        let mut archive = ZipArchive::new(file).map_err(|e| {
            LazarusError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            ))
        })?;

        let mut errors = Vec::new();
        let mut verified = 0;

        for (file_path, expected_hash) in &self.manifest.files {
            match read_file_bytes(&mut archive, file_path) {
                Ok(data) => {
                    let actual_hash = hash_bytes(&data);
                    if actual_hash != *expected_hash {
                        errors.push(format!(
                            "{}: 해시 불일치 (예상: {}..., 실제: {}...)",
                            file_path,
                            &expected_hash[..8],
                            &actual_hash[..8]
                        ));
                    } else {
                        verified += 1;
                    }
                }
                Err(_) => {
                    errors.push(format!("{}: 파일 없음", file_path));
                }
            }
        }

        Ok(VerifyResult {
            is_valid: errors.is_empty(),
            verified_count: verified,
            total_count: self.manifest.files.len(),
            errors,
        })
    }
}

/// 무결성 검사 결과
#[derive(Debug)]
pub struct VerifyResult {
    pub is_valid: bool,
    pub verified_count: usize,
    pub total_count: usize,
    pub errors: Vec<String>,
}

/// ZIP 파일에서 문자열 읽기
fn read_file_string(archive: &mut ZipArchive<File>, name: &str) -> Result<String> {
    let mut file = archive.by_name(name).map_err(|e| {
        LazarusError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            e.to_string(),
        ))
    })?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(LazarusError::Io)?;
    Ok(content)
}

/// ZIP 파일에서 바이트 읽기
fn read_file_bytes(archive: &mut ZipArchive<File>, name: &str) -> Result<Vec<u8>> {
    let mut file = archive.by_name(name).map_err(|e| {
        LazarusError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            e.to_string(),
        ))
    })?;

    let mut content = Vec::new();
    file.read_to_end(&mut content).map_err(LazarusError::Io)?;
    Ok(content)
}

/// SHA-256 해시 계산
fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
