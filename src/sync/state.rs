//! 동기화 상태 관리

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// 동기화 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub last_sync: Option<DateTime<Utc>>,
    pub device_id: String,
    #[serde(default)]
    pub synced_notes: HashMap<u64, DateTime<Utc>>,
}

impl SyncState {
    pub fn new(device_id: String) -> Self {
        Self {
            last_sync: None,
            device_id,
            synced_notes: HashMap::new(),
        }
    }

    pub fn load(usb_path: &Path) -> Result<Self, std::io::Error> {
        let path = usb_path.join("sync_state.json");
        if !path.exists() {
            return Ok(Self::new(Self::generate_device_id()));
        }
        let content = std::fs::read_to_string(&path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn save(&self, usb_path: &Path) -> Result<(), std::io::Error> {
        let path = usb_path.join("sync_state.json");
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)
    }

    pub fn mark_synced(&mut self, note_id: u64, updated_at: DateTime<Utc>) {
        self.synced_notes.insert(note_id, updated_at);
        self.last_sync = Some(Utc::now());
    }

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
    pub uploaded: usize,
    pub downloaded: usize,
    pub conflicts: usize,
    pub unchanged: usize,
}

impl SyncResult {
    pub fn total(&self) -> usize {
        self.uploaded + self.downloaded
    }
}

/// 동기화 에러
#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// 노트 내보내기
pub fn export_notes(usb_path: &Path, notes: &[crate::db::Note]) -> Result<usize, SyncError> {
    let notes_dir = usb_path.join("notes");
    std::fs::create_dir_all(&notes_dir)?;

    let mut count = 0;
    for note in notes {
        let filename = format!("{}.json", note.id);
        let path = notes_dir.join(&filename);
        let json = serde_json::to_string_pretty(note)?;
        std::fs::write(&path, json)?;
        count += 1;
    }

    Ok(count)
}

/// 노트 가져오기
pub fn import_notes(usb_path: &Path) -> Result<Vec<crate::db::Note>, SyncError> {
    let notes_dir = usb_path.join("notes");
    let mut notes = Vec::new();

    if !notes_dir.exists() {
        return Ok(notes);
    }

    for entry in std::fs::read_dir(&notes_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let content = std::fs::read_to_string(&path)?;
            if let Ok(note) = serde_json::from_str::<crate::db::Note>(&content) {
                notes.push(note);
            }
        }
    }

    Ok(notes)
}

/// 노트 동기화 (양방향)
pub fn sync_notes<F>(
    usb_path: &Path,
    local_notes: &[crate::db::Note],
    mut save_fn: F,
) -> Result<SyncResult, SyncError>
where
    F: FnMut(&crate::db::Note) -> Result<(), SyncError>,
{
    let notes_dir = usb_path.join("notes");
    std::fs::create_dir_all(&notes_dir)?;

    let mut result = SyncResult::default();
    let mut usb_note_ids = HashSet::new();

    if notes_dir.exists() {
        for entry in std::fs::read_dir(&notes_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "json").unwrap_or(false) {
                let content = std::fs::read_to_string(&path)?;
                if let Ok(usb_note) = serde_json::from_str::<crate::db::Note>(&content) {
                    usb_note_ids.insert(usb_note.id);

                    let local = local_notes.iter().find(|n| n.id == usb_note.id);

                    match local {
                        Some(local_note) => {
                            if usb_note.updated_at > local_note.updated_at {
                                save_fn(&usb_note)?;
                                result.downloaded += 1;
                            } else if local_note.updated_at > usb_note.updated_at {
                                let json = serde_json::to_string_pretty(local_note)?;
                                std::fs::write(&path, json)?;
                                result.uploaded += 1;
                            } else {
                                result.unchanged += 1;
                            }
                        }
                        None => {
                            save_fn(&usb_note)?;
                            result.downloaded += 1;
                        }
                    }
                }
            }
        }
    }

    for note in local_notes {
        if !usb_note_ids.contains(&note.id) {
            let filename = format!("{}.json", note.id);
            let path = notes_dir.join(&filename);
            let json = serde_json::to_string_pretty(note)?;
            std::fs::write(&path, json)?;
            result.uploaded += 1;
        }
    }

    Ok(result)
}

/// Posts 동기화
pub fn sync_posts(
    usb_path: &Path,
    local_posts: &[crate::db::Post],
) -> Result<(Vec<crate::db::Post>, usize), SyncError> {
    use super::jsonl::{read_jsonl, write_jsonl};

    let bulletin_dir = usb_path.join("bulletin");
    std::fs::create_dir_all(&bulletin_dir)?;
    let posts_file = bulletin_dir.join("posts.jsonl");

    let usb_posts: Vec<crate::db::Post> = if posts_file.exists() {
        read_jsonl(&posts_file)?
    } else {
        Vec::new()
    };

    let mut merged: HashMap<String, crate::db::Post> = HashMap::new();

    for post in usb_posts {
        merged.insert(post.id.clone(), post);
    }

    let mut uploaded = 0;
    for post in local_posts {
        if !merged.contains_key(&post.id) {
            merged.insert(post.id.clone(), post.clone());
            uploaded += 1;
        }
    }

    let all_posts: Vec<_> = merged.values().cloned().collect();
    write_jsonl(&posts_file, &all_posts)?;

    let local_ids: HashSet<_> = local_posts.iter().map(|p| &p.id).collect();
    let downloaded: Vec<_> = all_posts
        .into_iter()
        .filter(|p| !local_ids.contains(&p.id))
        .collect();

    Ok((downloaded, uploaded))
}

/// Q&A 동기화
pub fn sync_qna(
    usb_path: &Path,
    local_questions: &[crate::db::Question],
) -> Result<(Vec<crate::db::Question>, usize), SyncError> {
    use super::jsonl::{read_jsonl, write_jsonl};

    let qna_dir = usb_path.join("qna");
    std::fs::create_dir_all(&qna_dir)?;
    let qna_file = qna_dir.join("questions.jsonl");

    let usb_questions: Vec<crate::db::Question> = if qna_file.exists() {
        read_jsonl(&qna_file)?
    } else {
        Vec::new()
    };

    let mut merged: HashMap<String, crate::db::Question> = HashMap::new();

    for q in usb_questions {
        merged.insert(q.id.clone(), q);
    }

    let mut uploaded = 0;
    for q in local_questions {
        if !merged.contains_key(&q.id) {
            merged.insert(q.id.clone(), q.clone());
            uploaded += 1;
        }
    }

    let all_questions: Vec<_> = merged.values().cloned().collect();
    write_jsonl(&qna_file, &all_questions)?;

    let local_ids: HashSet<_> = local_questions.iter().map(|q| &q.id).collect();
    let downloaded: Vec<_> = all_questions
        .into_iter()
        .filter(|q| !local_ids.contains(&q.id))
        .collect();

    Ok((downloaded, uploaded))
}
