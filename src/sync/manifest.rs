//! USB 매니페스트 관리
//!
//! manifest.json 읽기/쓰기

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("IO 오류: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON 파싱 오류: {0}")]
    Json(#[from] serde_json::Error),
    #[error("매니페스트 없음")]
    NotFound,
}

/// USB 매니페스트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbManifest {
    /// 버전
    pub version: String,
    /// 생성 시간
    pub created_at: DateTime<Utc>,
    /// 마지막 동기화 시간
    #[serde(default)]
    pub last_sync: Option<DateTime<Utc>>,
    /// 기기 이름
    pub device_name: String,
    /// 동기화 히스토리
    #[serde(default)]
    pub sync_history: Vec<SyncRecord>,
    /// 콘텐츠 요약
    #[serde(default)]
    pub content_summary: ContentSummary,
}

/// 동기화 기록
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRecord {
    /// 동기화 시간
    pub timestamp: DateTime<Utc>,
    /// 기기 이름
    pub device_name: String,
    /// 방향 (import/export)
    pub direction: SyncDirection,
    /// 노트 수
    pub notes: usize,
    /// 게시글 수
    pub posts: usize,
    /// Q&A 수
    pub qna: usize,
}

/// 동기화 방향
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SyncDirection {
    Import,
    Export,
}

/// 콘텐츠 요약
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentSummary {
    /// 총 노트 수
    pub total_notes: usize,
    /// 총 게시글 수
    pub total_posts: usize,
    /// 총 Q&A 수
    pub total_qna: usize,
    /// 총 패키지 수
    pub total_packages: usize,
    /// 마지막 업데이트
    pub last_updated: Option<DateTime<Utc>>,
}

impl UsbManifest {
    /// 경로에서 매니페스트 로드
    pub fn load(usb_path: &Path) -> Result<Self, ManifestError> {
        let manifest_path = usb_path.join("manifest.json");

        if !manifest_path.exists() {
            return Err(ManifestError::NotFound);
        }

        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: Self = serde_json::from_str(&content)?;

        Ok(manifest)
    }

    /// 매니페스트 저장
    pub fn save(&self, usb_path: &Path) -> Result<(), ManifestError> {
        let manifest_path = usb_path.join("manifest.json");
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&manifest_path, content)?;
        Ok(())
    }

    /// 새 매니페스트 생성
    pub fn new(device_name: String) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: Utc::now(),
            last_sync: None,
            device_name,
            sync_history: Vec::new(),
            content_summary: ContentSummary::default(),
        }
    }

    /// 동기화 기록 추가
    pub fn add_sync_record(&mut self, record: SyncRecord) {
        self.last_sync = Some(record.timestamp);
        self.sync_history.push(record);

        // 최근 100개만 유지
        if self.sync_history.len() > 100 {
            self.sync_history.remove(0);
        }
    }

    /// 콘텐츠 요약 업데이트
    pub fn update_summary(&mut self, usb_path: &Path) {
        use crate::sync::detect::LazarusUsb;

        if let Some(usb) = LazarusUsb::from_path(usb_path) {
            self.content_summary = ContentSummary {
                total_notes: usb.note_count,
                total_posts: usb.post_count,
                total_qna: usb.qna_count,
                total_packages: usb.package_count,
                last_updated: Some(Utc::now()),
            };
        }
    }
}

impl SyncRecord {
    /// 새 동기화 기록 생성
    pub fn new(
        device_name: String,
        direction: SyncDirection,
        notes: usize,
        posts: usize,
        qna: usize,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            device_name,
            direction,
            notes,
            posts,
            qna,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_manifest_create_save_load() {
        let dir = tempdir().unwrap();

        let mut manifest = UsbManifest::new("Test Device".to_string());
        manifest.add_sync_record(SyncRecord::new(
            "Other Device".to_string(),
            SyncDirection::Import,
            10,
            5,
            3,
        ));

        // 저장
        manifest.save(dir.path()).unwrap();

        // 로드
        let loaded = UsbManifest::load(dir.path()).unwrap();

        assert_eq!(loaded.device_name, "Test Device");
        assert_eq!(loaded.sync_history.len(), 1);
        assert_eq!(loaded.sync_history[0].notes, 10);
    }
}
