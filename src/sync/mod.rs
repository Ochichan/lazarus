//! USB 동기화 모듈
//!
//! 인터넷 없이 USB를 통한 데이터 동기화
//!
//! # 기능
//! - USB 자동 감지
//! - 노트/게시판/Q&A 동기화
//! - 충돌 해결 (CRDT)
//! - 매니페스트 관리

pub mod detect;
pub mod manifest;
pub mod watcher;

pub use detect::{LazarusUsb, UsbDetector};
pub use manifest::{ContentSummary, SyncDirection, SyncRecord, UsbManifest};
pub use watcher::{UsbEvent, UsbWatcher};

use std::path::Path;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("IO 오류: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON 오류: {0}")]
    Json(#[from] serde_json::Error),
    #[error("매니페스트 오류: {0}")]
    Manifest(#[from] manifest::ManifestError),
    #[error("USB가 Lazarus USB가 아님")]
    NotLazarusUsb,
    #[error("동기화 충돌: {0}")]
    Conflict(String),
}

/// USB 동기화 매니저
pub struct SyncManager {
    watcher: UsbWatcher,
}

impl SyncManager {
    /// 새 동기화 매니저 생성
    pub fn new() -> Self {
        Self {
            watcher: UsbWatcher::default(),
        }
    }

    /// USB 감시 시작
    pub async fn start_watching(&self) {
        self.watcher.start().await;
    }

    /// USB 감시 중지
    pub async fn stop_watching(&self) {
        self.watcher.stop().await;
    }

    /// 이벤트 구독
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<UsbEvent> {
        self.watcher.subscribe()
    }

    /// 현재 감지된 USB 목록
    pub async fn get_detected_usbs(&self) -> Vec<LazarusUsb> {
        self.watcher.get_detected().await
    }

    /// 수동 스캔
    pub async fn scan(&self) -> Vec<LazarusUsb> {
        self.watcher.scan_now().await
    }

    /// USB 초기화 (Lazarus USB로 만들기)
    pub fn init_usb(path: &Path) -> Result<(), SyncError> {
        if !path.exists() || !path.is_dir() {
            return Err(SyncError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "경로가 존재하지 않거나 디렉토리가 아님",
            )));
        }

        UsbDetector::init_usb(path)?;
        info!("✅ USB 초기화 완료: {}", path.display());
        Ok(())
    }

    /// USB가 Lazarus USB인지 확인
    pub fn is_lazarus_usb(path: &Path) -> bool {
        UsbDetector::is_lazarus_usb(path)
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: 향후 구현
// - export_notes(): 노트 내보내기
// - import_notes(): 노트 가져오기
// - sync_bulletin(): 게시판 동기화
// - sync_qna(): Q&A 동기화
// - resolve_conflicts(): 충돌 해결
