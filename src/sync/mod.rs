//! USB 동기화 모듈
//!
//! 인터넷 없이 USB를 통한 데이터 동기화
//!
//! # 기능
//! - USB 자동 감지
//! - 노트/게시판/Q&A 동기화
//! - 충돌 해결 (CRDT)
//! - 매니페스트 관리
use crate::db::Note;
use crate::db::{Post, Question};

pub mod detect;
pub mod jsonl;
pub mod manifest;
pub mod state;
pub mod watcher;

pub use detect::{LazarusUsb, UsbDetector};
pub use jsonl::{append_jsonl, count_jsonl, read_jsonl, write_jsonl};
pub use manifest::{ContentSummary, SyncDirection, SyncRecord, UsbManifest};
pub use state::{SyncResult, SyncState};
pub use watcher::{UsbEvent, UsbWatcher};

use std::fs;
use std::path::Path;
use thiserror::Error;
use tracing::info;

use chrono::Utc;

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

/// 노트를 USB로 내보내기
pub fn export_notes(usb_path: &Path, notes: &[Note]) -> Result<usize, SyncError> {
    let notes_dir = usb_path.join("notes");
    fs::create_dir_all(&notes_dir)?;

    let mut count = 0;
    for note in notes {
        let filename = format!("{}.json", note.id);
        let filepath = notes_dir.join(&filename);
        let json = serde_json::to_string_pretty(note)?;
        fs::write(&filepath, json)?;
        count += 1;
    }

    // 매니페스트 업데이트
    let mut manifest =
        UsbManifest::load(usb_path).unwrap_or_else(|_| UsbManifest::new("Lazarus USB".to_string()));
    manifest.content_summary.total_notes = count;
    manifest.last_sync = Some(chrono::Utc::now());
    manifest.save(usb_path)?;

    info!("📤 {} 노트 내보내기 완료: {}", count, usb_path.display());
    Ok(count)
}

/// USB에서 노트 가져오기
pub fn import_notes(usb_path: &Path) -> Result<Vec<Note>, SyncError> {
    let notes_dir = usb_path.join("notes");
    if !notes_dir.exists() {
        return Ok(Vec::new());
    }

    let mut notes = Vec::new();
    for entry in fs::read_dir(&notes_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let content = fs::read_to_string(&path)?;
            if let Ok(note) = serde_json::from_str::<Note>(&content) {
                notes.push(note);
            }
        }
    }

    info!(
        "📥 {} 노트 가져오기 완료: {}",
        notes.len(),
        usb_path.display()
    );
    Ok(notes)
}

/// 양방향 동기화
pub fn sync_notes(
    usb_path: &Path,
    local_notes: &[Note],
    db_save: impl Fn(&Note) -> Result<(), SyncError>,
) -> Result<SyncResult, SyncError> {
    let notes_dir = usb_path.join("notes");
    fs::create_dir_all(&notes_dir)?;

    // 동기화 상태 로드
    let mut sync_state =
        SyncState::load(usb_path).unwrap_or_else(|_| SyncState::new("local".to_string()));
    let mut result = SyncResult::default();

    // Local 노트를 HashMap으로
    let local_map: std::collections::HashMap<u64, &Note> =
        local_notes.iter().map(|n| (n.id, n)).collect();

    // USB 노트 로드
    let usb_notes = import_notes(usb_path)?;
    let usb_map: std::collections::HashMap<u64, Note> =
        usb_notes.into_iter().map(|n| (n.id, n)).collect();

    // 모든 ID 수집
    let mut all_ids: std::collections::HashSet<u64> = local_map.keys().copied().collect();
    all_ids.extend(usb_map.keys());

    for id in all_ids {
        let local_note = local_map.get(&id);
        let usb_note = usb_map.get(&id);

        match (local_note, usb_note) {
            // Local에만 있음 → USB로 업로드
            (Some(local), None) => {
                let filepath = notes_dir.join(format!("{}.json", id));
                let json = serde_json::to_string_pretty(local)?;
                fs::write(&filepath, json)?;
                sync_state.mark_synced(id, local.updated_at);
                result.uploaded += 1;
            }
            // USB에만 있음 → Local로 다운로드
            (None, Some(usb)) => {
                db_save(usb)?;
                sync_state.mark_synced(id, usb.updated_at);
                result.downloaded += 1;
            }
            // 양쪽 다 있음 → 최신 것 선택
            (Some(local), Some(usb)) => {
                if local.updated_at > usb.updated_at {
                    // Local이 더 최신 → USB로
                    let filepath = notes_dir.join(format!("{}.json", id));
                    let json = serde_json::to_string_pretty(local)?;
                    fs::write(&filepath, json)?;
                    sync_state.mark_synced(id, local.updated_at);
                    result.uploaded += 1;
                    if sync_state.synced_notes.contains_key(&id) {
                        result.conflicts += 1;
                    }
                } else if usb.updated_at > local.updated_at {
                    // USB가 더 최신 → Local로
                    db_save(usb)?;
                    sync_state.mark_synced(id, usb.updated_at);
                    result.downloaded += 1;
                    if sync_state.synced_notes.contains_key(&id) {
                        result.conflicts += 1;
                    }
                } else {
                    // 동일 → skip
                    result.unchanged += 1;
                }
            }
            (None, None) => unreachable!(),
        }
    }

    // 매니페스트 업데이트
    let mut manifest =
        UsbManifest::load(usb_path).unwrap_or_else(|_| UsbManifest::new("Lazarus USB".to_string()));
    manifest.content_summary.total_notes = local_map.len() + result.downloaded;
    manifest.last_sync = Some(Utc::now());
    manifest.save(usb_path)?;

    // 동기화 상태 저장
    sync_state.save(usb_path)?;

    info!(
        "🔄 동기화 완료: ↑{} ↓{} conflicts:{} unchanged:{}",
        result.uploaded, result.downloaded, result.conflicts, result.unchanged
    );

    Ok(result)
}
/// Posts 동기화
pub fn sync_posts(usb_path: &Path, local_posts: &[Post]) -> Result<(Vec<Post>, usize), SyncError> {
    let posts_path = usb_path.join("bulletin/posts.jsonl");

    // USB에서 읽기
    let usb_posts: Vec<Post> = jsonl::read_jsonl(&posts_path)?;
    let usb_ids: std::collections::HashSet<_> = usb_posts.iter().map(|p| p.id.clone()).collect();
    let local_ids: std::collections::HashSet<_> =
        local_posts.iter().map(|p| p.id.clone()).collect();

    // Local → USB (USB에 없는 것)
    let to_upload: Vec<&Post> = local_posts
        .iter()
        .filter(|p| !usb_ids.contains(&p.id))
        .collect();

    // USB → Local (Local에 없는 것)
    let to_download: Vec<Post> = usb_posts
        .into_iter()
        .filter(|p| !local_ids.contains(&p.id))
        .collect();

    // USB에 추가
    let uploaded = to_upload.len();
    for post in &to_upload {
        jsonl::append_jsonl(&posts_path, post)?;
    }

    info!("📋 Posts 동기화: ↑{} ↓{}", uploaded, to_download.len());
    Ok((to_download, uploaded))
}

/// Q&A 동기화
pub fn sync_qna(
    usb_path: &Path,
    local_questions: &[Question],
) -> Result<(Vec<Question>, usize), SyncError> {
    let qna_path = usb_path.join("qna/questions.jsonl");

    // USB에서 읽기
    let usb_questions: Vec<Question> = jsonl::read_jsonl(&qna_path)?;
    let usb_ids: std::collections::HashSet<_> =
        usb_questions.iter().map(|q| q.id.clone()).collect();
    let local_ids: std::collections::HashSet<_> =
        local_questions.iter().map(|q| q.id.clone()).collect();

    // Local → USB
    let to_upload: Vec<&Question> = local_questions
        .iter()
        .filter(|q| !usb_ids.contains(&q.id))
        .collect();

    // USB → Local
    let to_download: Vec<Question> = usb_questions
        .into_iter()
        .filter(|q| !local_ids.contains(&q.id))
        .collect();

    // USB에 추가
    let uploaded = to_upload.len();
    for question in &to_upload {
        jsonl::append_jsonl(&qna_path, question)?;
    }

    info!("❓ Q&A 동기화: ↑{} ↓{}", uploaded, to_download.len());
    Ok((to_download, uploaded))
}

// TODO: 향후 구현

// - sync_bulletin(): 게시판 동기화
// - sync_qna(): Q&A 동기화
// - resolve_conflicts(): 충돌 해결
