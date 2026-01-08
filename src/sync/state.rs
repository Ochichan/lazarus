//! 동기화 상태 관리

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// 동기화 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    /// 마지막 동기화 시간
    pub last_sync: Option<DateTime<Utc>>,
    /// 기기 ID
    pub device_id: String,
    /// 동기화된 노트들 (id → updated_at)
    #[serde(default)]
    pub synced_notes: HashMap<u64, DateTime<Utc>>,
}

impl SyncState {
    /// 새 상태 생성
    pub fn new(device_id: String) -> Self {
        Self {
            last_sync: None,
            device_id,
            synced_notes: HashMap::new(),
        }
    }

    /// 파일에서 로드
    pub fn load(usb_path: &Path) -> Result<Self, std::io::Error> {
        let path = usb_path.join("sync_state.json");
        if !path.exists() {
            return Ok(Self::new(Self::generate_device_id()));
        }
        let content = std::fs::read_to_string(&path)?;
        serde_json::from_str(&content).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })
    }

    /// 파일에 저장
    pub fn save(&self, usb_path: &Path) -> Result<(), std::io::Error> {
        let path = usb_path.join("sync_state.json");
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)
    }

    /// 동기화 완료 기록
    pub fn mark_synced(&mut self, note_id: u64, updated_at: DateTime<Utc>) {
        self.synced_notes.insert(note_id, updated_at);
        self.last_sync = Some(Utc::now());
    }

    /// 기기 ID 생성
    fn generate_device_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        format!("device_{:x}", timestamp)
    }
}

/// 동기화 결과
#[derive(Debug, Clone, Serialize, Default)]
pub struct SyncResult {
    /// Local → USB
    pub uploaded: usize,
    /// USB → Local
    pub downloaded: usize,
    /// 충돌 해결됨
    pub conflicts: usize,
    /// 변경 없음
    pub unchanged: usize,
}

impl SyncResult {
    pub fn total(&self) -> usize {
        self.uploaded + self.downloaded
    }
}
