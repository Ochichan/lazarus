//! 자동 백업 시스템
//!
//! Rolling Backup: 최근 3개만 유지

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

use crate::error::{LazarusError, Result};

/// 백업 관리자
pub struct BackupManager {
    /// 원본 파일 경로
    source_path: PathBuf,
    /// 백업 디렉토리
    backup_dir: PathBuf,
    /// 최대 백업 수
    max_backups: usize,
}

impl BackupManager {
    /// 새 백업 관리자 생성
    pub fn new<P: AsRef<Path>>(source_path: P, backup_dir: P) -> Self {
        Self {
            source_path: source_path.as_ref().to_path_buf(),
            backup_dir: backup_dir.as_ref().to_path_buf(),
            max_backups: 3,
        }
    }

    /// 백업 디렉토리 생성
    fn ensure_backup_dir(&self) -> Result<()> {
        if !self.backup_dir.exists() {
            fs::create_dir_all(&self.backup_dir).map_err(LazarusError::Io)?;
        }
        Ok(())
    }

    /// 백업 실행
    pub fn backup(&self) -> Result<Option<PathBuf>> {
        // 원본 파일이 없으면 스킵
        if !self.source_path.exists() {
            tracing::debug!("백업 스킵: 원본 파일 없음");
            return Ok(None);
        }

        self.ensure_backup_dir()?;

        // 오늘 날짜로 백업 파일명 생성
        let today = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = self.source_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("backup");
        let backup_name = format!("{}_{}.gz", filename, today);
        let backup_path = self.backup_dir.join(&backup_name);

        // 이미 최근 백업이 있고 내용이 같으면 스킵
        if let Some(latest) = self.get_latest_backup()? {
            if self.is_same_content(&latest)? {
                tracing::debug!("백업 스킵: 변경사항 없음");
                return Ok(None);
            }
        }

        // gzip 압축하여 백업
        let source_data = fs::read(&self.source_path).map_err(LazarusError::Io)?;
        
        let backup_file = File::create(&backup_path).map_err(LazarusError::Io)?;
        let mut encoder = GzEncoder::new(backup_file, Compression::fast());
        encoder.write_all(&source_data).map_err(LazarusError::Io)?;
        encoder.finish().map_err(LazarusError::Io)?;

        tracing::info!("백업 완료: {}", backup_path.display());

        // 오래된 백업 정리
        self.cleanup_old_backups()?;

        Ok(Some(backup_path))
    }

    /// 최신 백업 파일 가져오기
    fn get_latest_backup(&self) -> Result<Option<PathBuf>> {
        let backups = self.list_backups()?;
        Ok(backups.into_iter().next())
    }

    /// 백업 목록 (최신순)
    pub fn list_backups(&self) -> Result<Vec<PathBuf>> {
        if !self.backup_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups: Vec<PathBuf> = fs::read_dir(&self.backup_dir)
            .map_err(LazarusError::Io)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|e| e == "gz").unwrap_or(false))
            .collect();

        // 최신순 정렬 (파일명에 날짜가 포함되어 있으므로)
        backups.sort_by(|a, b| b.cmp(a));

        Ok(backups)
    }

    /// 오래된 백업 정리
    fn cleanup_old_backups(&self) -> Result<()> {
        let backups = self.list_backups()?;
        
        for old_backup in backups.into_iter().skip(self.max_backups) {
            fs::remove_file(&old_backup).map_err(LazarusError::Io)?;
            tracing::debug!("오래된 백업 삭제: {}", old_backup.display());
        }

        Ok(())
    }

    /// 내용 비교 (해시 기반)
    fn is_same_content(&self, backup_path: &Path) -> Result<bool> {
        use sha2::{Sha256, Digest};

        // 원본 해시
        let source_data = fs::read(&self.source_path).map_err(LazarusError::Io)?;
        let mut source_hasher = Sha256::new();
        source_hasher.update(&source_data);
        let source_hash = source_hasher.finalize();

        // 백업 해시 (압축 해제 후)
        let backup_file = File::open(backup_path).map_err(LazarusError::Io)?;
        let mut decoder = GzDecoder::new(backup_file);
        let mut backup_data = Vec::new();
        decoder.read_to_end(&mut backup_data).map_err(LazarusError::Io)?;
        
        let mut backup_hasher = Sha256::new();
        backup_hasher.update(&backup_data);
        let backup_hash = backup_hasher.finalize();

        Ok(source_hash == backup_hash)
    }

    /// 백업에서 복원
    pub fn restore(&self, backup_path: &Path) -> Result<()> {
        let backup_file = File::open(backup_path).map_err(LazarusError::Io)?;
        let mut decoder = GzDecoder::new(backup_file);
        let mut data = Vec::new();
        decoder.read_to_end(&mut data).map_err(LazarusError::Io)?;

        fs::write(&self.source_path, data).map_err(LazarusError::Io)?;
        tracing::info!("복원 완료: {} -> {}", backup_path.display(), self.source_path.display());

        Ok(())
    }

    /// 백업 정보
    pub fn info(&self) -> Result<BackupInfo> {
        let backups = self.list_backups()?;
        let total_size: u64 = backups.iter()
            .filter_map(|p| fs::metadata(p).ok())
            .map(|m| m.len())
            .sum();

        Ok(BackupInfo {
            count: backups.len(),
            total_size,
            latest: backups.first().cloned(),
            backups,
        })
    }
}

/// 백업 정보
#[derive(Debug)]
pub struct BackupInfo {
    pub count: usize,
    pub total_size: u64,
    pub latest: Option<PathBuf>,
    pub backups: Vec<PathBuf>,
}
