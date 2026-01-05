//! .laz 패키지 작성기

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

use super::{LazPackage, Manifest, MIMETYPE};
use crate::error::{LazarusError, Result};

impl LazPackage {
    /// .laz 파일로 내보내기
    pub fn export<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let file = File::create(path.as_ref()).map_err(|e| LazarusError::Io(e))?;

        let mut zip = ZipWriter::new(file);
        let mut manifest_files = std::collections::HashMap::new();

        // 1. mimetype (첫 번째, 압축 없음)
        let options_store = FileOptions::default().compression_method(CompressionMethod::Stored);

        zip.start_file("mimetype", options_store).map_err(zip_err)?;
        zip.write_all(MIMETYPE.as_bytes())
            .map_err(|e| LazarusError::Io(e))?;
        manifest_files.insert("mimetype".to_string(), hash_bytes(MIMETYPE.as_bytes()));

        // 2. meta.json
        let meta_json = serde_json::to_string_pretty(&self.meta)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;
        write_json(&mut zip, "meta.json", &meta_json, &mut manifest_files)?;

        // 3. curriculum.json
        let curriculum_json = serde_json::to_string_pretty(&self.curriculum)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;
        write_json(
            &mut zip,
            "curriculum.json",
            &curriculum_json,
            &mut manifest_files,
        )?;

        // 4. srs.json
        let srs_json = serde_json::to_string_pretty(&self.srs)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;
        write_json(&mut zip, "srs.json", &srs_json, &mut manifest_files)?;

        // 5. content/*.json
        for (id, note) in &self.content {
            let note_json = serde_json::to_string_pretty(note)
                .map_err(|e| LazarusError::Serialize(e.to_string()))?;
            let path = format!("content/{}.json", id);
            write_json(&mut zip, &path, &note_json, &mut manifest_files)?;
        }

        // 6. manifest.json (마지막에 생성)
        self.manifest = Manifest {
            files: manifest_files.clone(),
            generated_at: chrono::Utc::now().timestamp(),
        };
        let manifest_json = serde_json::to_string_pretty(&self.manifest)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;

        let options_deflate =
            FileOptions::default().compression_method(CompressionMethod::Deflated);
        zip.start_file("manifest.json", options_deflate)
            .map_err(zip_err)?;
        zip.write_all(manifest_json.as_bytes())
            .map_err(|e| LazarusError::Io(e))?;

        zip.finish().map_err(zip_err)?;

        tracing::info!("패키지 내보내기 완료: {}", path.as_ref().display());
        Ok(())
    }

    /// 에셋 파일 추가 (이미지, 오디오 등)
    pub fn add_asset_to_zip<P: AsRef<Path>>(
        zip: &mut ZipWriter<File>,
        asset_path: P,
        manifest: &mut std::collections::HashMap<String, String>,
    ) -> Result<()> {
        let path = asset_path.as_ref();
        let filename = path.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
            LazarusError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "잘못된 파일명",
            ))
        })?;

        let data = std::fs::read(path).map_err(LazarusError::Io)?;

        // 미디어 파일은 압축 없이 저장 (mmap 가능하도록)
        let options = if is_media_file(filename) {
            FileOptions::default().compression_method(CompressionMethod::Stored)
        } else {
            FileOptions::default().compression_method(CompressionMethod::Deflated)
        };

        let zip_path = format!("assets/{}", filename);
        zip.start_file(&zip_path, options).map_err(zip_err)?;
        zip.write_all(&data).map_err(|e| LazarusError::Io(e))?;

        manifest.insert(zip_path, hash_bytes(&data));
        Ok(())
    }
}

/// JSON 파일 쓰기 (Deflate 압축)
fn write_json(
    zip: &mut ZipWriter<File>,
    path: &str,
    content: &str,
    manifest: &mut std::collections::HashMap<String, String>,
) -> Result<()> {
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    zip.start_file(path, options).map_err(zip_err)?;
    zip.write_all(content.as_bytes())
        .map_err(|e| LazarusError::Io(e))?;

    manifest.insert(path.to_string(), hash_bytes(content.as_bytes()));
    Ok(())
}

/// SHA-256 해시 계산
fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// 미디어 파일 여부 확인
fn is_media_file(filename: &str) -> bool {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    matches!(
        ext.as_str(),
        "jpg"
            | "jpeg"
            | "png"
            | "gif"
            | "webp"
            | "svg"
            | "mp3"
            | "ogg"
            | "wav"
            | "m4a"
            | "mp4"
            | "webm"
            | "mkv"
    )
}

/// ZIP 에러 변환
fn zip_err(e: zip::result::ZipError) -> LazarusError {
    LazarusError::Io(std::io::Error::new(
        std::io::ErrorKind::Other,
        e.to_string(),
    ))
}
