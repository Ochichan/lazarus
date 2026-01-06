//! 노트 CRUD API
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::db::CompactResult;
use crate::db::Note;
use crate::error::{LazarusError, Result};
use crate::web::state::AppState;

/// 노트 응답
#[derive(Serialize)]
pub struct NoteResponse {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub note_type: String,
}

impl From<Note> for NoteResponse {
    fn from(note: Note) -> Self {
        Self {
            id: note.id,
            title: note.title,
            content: note.content,
            created_at: note.created_at.to_rfc3339(),
            updated_at: note.updated_at.to_rfc3339(),
            tags: note.tags,
            note_type: format!("{:?}", note.note_type).to_lowercase(),
        }
    }
}

/// 노트 목록 응답
#[derive(Serialize)]
pub struct NotesListResponse {
    pub notes: Vec<NoteResponse>,
    pub total: usize,
}

/// GET /api/notes
pub async fn list(State(state): State<AppState>) -> Result<Json<NotesListResponse>> {
    let db = state.db.read().await;
    let ids = db.list_ids();

    let mut notes = Vec::new();
    for id in ids {
        if let Some(note) = db.get(id)? {
            notes.push(NoteResponse::from(note));
        }
    }

    // 최신순 정렬
    notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    let total = notes.len();
    Ok(Json(NotesListResponse { notes, total }))
}

/// POST /api/notes
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<NoteResponse>)> {
    let mut note = Note::new(0, req.title, req.content);
    note.tags = req.tags;

    let mut db = state.db.write().await;
    let id = db.save(&note, None)?;
    note.id = id;

    tracing::info!("노트 생성: id={}, title={}", id, note.title);

    Ok((StatusCode::CREATED, Json(NoteResponse::from(note))))
}

/// POST /api/notes (Form)
pub async fn create_form(
    State(state): State<AppState>,
    Form(req): Form<CreateNoteFormRequest>,
) -> Result<axum::response::Redirect> {
    tracing::info!(
        "폼 데이터: title={}, note_type={:?}",
        req.title,
        req.note_type
    );
    let mut note = Note::new(0, req.title, req.content);

    // 쉼표로 구분된 태그 파싱
    note.tags = req
        .tags
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // 암호화 여부
    note.encrypted = req.encrypted == "true";

    // 노트 타입
    if let Some(ref t) = req.note_type {
        note.note_type = match t.as_str() {
            "journal" => crate::db::note::NoteType::Journal,
            "review" => crate::db::note::NoteType::Review,
            "idea" => crate::db::note::NoteType::Idea,
            _ => crate::db::note::NoteType::Note,
        };
    }
    let id = {
        let mut db = state.db.write().await;
        if note.encrypted {
            // 암호화된 노트 저장
            let crypto = state.crypto.read().await;
            db.save_encrypted(&note, None, crypto.as_ref())?
        } else {
            db.save(&note, None)?
        }
    };

    // 검색 인덱스에 추가
    {
        let mut search = state.search.write().await;
        search.index_note(id, &note.title, &note.content, &note.tags)?;
    }
    //링크 인덱스 업데이트
    {
        let mut index = state.link_index.write().await;
        index.register_note(id, &note.title);
        index.update_links(id, &note.content);
    }

    tracing::info!(
        "노트 생성 (Form): id={}, encrypted={}, type={:?}",
        id,
        note.encrypted,
        note.note_type
    );
    Ok(axum::response::Redirect::to(&format!("/notes/{}", id)))
}

/// GET /api/notes/:id
pub async fn get(State(state): State<AppState>, Path(id): Path<u64>) -> Result<Json<NoteResponse>> {
    let db = state.db.read().await;

    match db.get(id)? {
        Some(note) => Ok(Json(NoteResponse::from(note))),
        None => Err(LazarusError::NotFound(format!("노트 ID: {}", id))),
    }
}

/// POST /api/notes/:id
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(req): Json<CreateNoteRequest>,
) -> Result<Json<NoteResponse>> {
    let mut db = state.db.write().await;

    // 기존 노트 확인
    let existing = db
        .get(id)?
        .ok_or_else(|| LazarusError::NotFound(format!("노트 ID: {}", id)))?;

    let mut note = Note::new(id, req.title, req.content);
    note.tags = req.tags;
    note.created_at = existing.created_at;

    db.save(&note, None)?;

    tracing::info!("노트 수정: id={}", id);

    Ok(Json(NoteResponse::from(note)))
}

/// POST /api/notes/:id (Form)
pub async fn update_form(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Form(req): Form<CreateNoteFormRequest>,
) -> Result<axum::response::Redirect> {
    // 태그 파싱
    let tags: Vec<String> = req
        .tags
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // 암호화 여부
    let encrypted = req.encrypted == "true";

    let note = {
        let mut db = state.db.write().await;
        // 기존 노트 확인
        let existing = db
            .get(id)?
            .ok_or_else(|| LazarusError::NotFound(format!("노트 ID: {}", id)))?;

        let mut note = Note::new(id, req.title, req.content);
        note.tags = tags;
        note.created_at = existing.created_at;
        note.encrypted = encrypted;

        // 노트 타입
        if let Some(ref t) = req.note_type {
            note.note_type = match t.as_str() {
                "journal" => crate::db::note::NoteType::Journal,
                "review" => crate::db::note::NoteType::Review,
                "idea" => crate::db::note::NoteType::Idea,
                _ => crate::db::note::NoteType::Note,
            };
        } else {
            // 기존 타입 유지
            note.note_type = existing.note_type;
        }
        if encrypted {
            let crypto = state.crypto.read().await;
            db.save_encrypted(&note, None, crypto.as_ref())?;
        } else {
            db.save(&note, None)?;
        }
        note
    };

    // 검색 인덱스 업데이트
    {
        let mut search = state.search.write().await;
        search.index_note(id, &note.title, &note.content, &note.tags)?;
    }

    tracing::info!("노트 수정 (Form): id={}, encrypted={}", id, note.encrypted);
    Ok(axum::response::Redirect::to(&format!("/notes/{}", id)))
}

/// DELETE /api/notes/:id
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl axum::response::IntoResponse> {
    // 검색 인덱스에서 삭제
    {
        let mut search = state.search.write().await;
        let _ = search.delete_note(id);
    }

    // DB에서 삭제
    let mut db = state.db.write().await;

    if db.delete(id)? {
        tracing::info!("노트 삭제: id={}", id);

        // HTMX 리다이렉트 헤더
        Ok((StatusCode::OK, [("HX-Redirect", "/notes")], ""))
    } else {
        Err(LazarusError::NotFound(format!("노트 ID: {}", id)))
    }
}
/// 노트 생성 요청 (JSON)
#[derive(Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub encrypted: bool,
    #[serde(default)]
    pub note_type: Option<String>,
}

/// 노트 생성 요청 (Form)
#[derive(Deserialize)]
pub struct CreateNoteFormRequest {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: String, // 쉼표로 구분된 문자열
    #[serde(default)]
    pub encrypted: String, // "true" 또는 "false"
    #[serde(default)]
    pub note_type: Option<String>,
}

/// GET /api/notes/duplicates - 중복 노트 찾기
pub async fn find_duplicates(State(state): State<AppState>) -> Result<Json<Vec<DuplicateGroup>>> {
    let db = state.db.read().await;
    let ids = db.list_ids();

    // 제목+내용 해시로 그룹화
    let mut hash_map: std::collections::HashMap<String, Vec<NoteInfo>> =
        std::collections::HashMap::new();

    for id in ids {
        if let Some(note) = db.get(id)? {
            // 제목과 내용 첫 500자로 해시 생성
            let content_preview = note.content.chars().take(500).collect::<String>();
            // 이스케이프 문자 정규화
            let clean_title = note
                .title
                .trim()
                .to_lowercase()
                .replace("\\\"", "")
                .replace("\"", "")
                .replace("\\", "");
            let clean_content = content_preview
                .trim()
                .to_lowercase()
                .replace("\\\"", "")
                .replace("\"", "")
                .replace("\\", "");
            let hash_input = format!("{}:{}", clean_title, clean_content);

            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(hash_input.as_bytes());
            let hash = format!("{:x}", hasher.finalize())[..16].to_string();

            hash_map.entry(hash).or_default().push(NoteInfo {
                id: note.id,
                title: note.title,
                preview: note.content.chars().take(100).collect(),
                created_at: note.created_at.format("%Y-%m-%d %H:%M").to_string(),
                updated_at: note.updated_at.format("%Y-%m-%d %H:%M").to_string(),
            });
        }
    }

    // 2개 이상인 그룹만 반환
    let duplicates: Vec<DuplicateGroup> = hash_map
        .into_iter()
        .filter(|(_, notes)| notes.len() > 1)
        .map(|(hash, mut notes)| {
            // 최신 순 정렬
            notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
            DuplicateGroup {
                hash,
                count: notes.len(),
                notes,
            }
        })
        .collect();

    Ok(Json(duplicates))
}

/// POST /api/notes/duplicates/remove - 중복 노트 삭제 (최신 1개 유지)
pub async fn remove_duplicates(
    State(state): State<AppState>,
    Json(params): Json<RemoveDuplicatesParams>,
) -> Result<Json<RemoveResult>> {
    let mut db = state.db.write().await;
    let mut search = state.search.write().await;
    let mut removed_count = 0;

    for group in params.groups {
        // 첫 번째(최신)는 유지, 나머지 삭제
        for note_id in group.note_ids.iter().skip(1) {
            if db.delete(*note_id)? {
                search.delete_note(*note_id)?;
                removed_count += 1;
            }
        }
    }

    Ok(Json(RemoveResult {
        success: true,
        removed_count,
        message: format!("{}개의 중복 노트가 삭제되었습니다.", removed_count),
    }))
}

#[derive(serde::Serialize)]
pub struct NoteInfo {
    pub id: u64,
    pub title: String,
    pub preview: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(serde::Serialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub count: usize,
    pub notes: Vec<NoteInfo>,
}

#[derive(serde::Deserialize)]
pub struct RemoveDuplicatesParams {
    pub groups: Vec<RemoveGroup>,
}

#[derive(serde::Deserialize)]
pub struct RemoveGroup {
    pub note_ids: Vec<u64>,
}

#[derive(serde::Serialize)]
pub struct RemoveResult {
    pub success: bool,
    pub removed_count: usize,
    pub message: String,
}

/// GET /api/backup/info - 백업 정보
pub async fn backup_info(State(state): State<AppState>) -> Result<Json<BackupInfoResponse>> {
    let backup = state.backup.read().await;
    let info = backup.info()?;
    let total_size: u64 = info
        .backups
        .iter()
        .filter_map(|p| std::fs::metadata(p).ok())
        .map(|m| m.len())
        .sum();

    Ok(Json(BackupInfoResponse {
        count: info.backups.len(),
        total_size_kb: total_size / 1024,
        encrypted: info.encrypted,
        latest: info.latest.map(|p| p.display().to_string()),
        backups: info
            .backups
            .iter()
            .map(|p| p.display().to_string())
            .collect(),
    }))
}

/// POST /api/backup/now - 수동 백업
pub async fn backup_now(State(state): State<AppState>) -> Result<Json<BackupResult>> {
    let backup = state.backup.read().await;
    match backup.backup()? {
        Some(path) => Ok(Json(BackupResult {
            success: true,
            message: format!("백업 완료: {}", path.display()),
            path: Some(path.display().to_string()),
        })),
        None => Ok(Json(BackupResult {
            success: true,
            message: "변경사항 없음 (백업 스킵)".to_string(),
            path: None,
        })),
    }
}

#[derive(serde::Serialize)]
pub struct BackupInfoResponse {
    pub count: usize,
    pub total_size_kb: u64,
    pub encrypted: bool,
    pub latest: Option<String>,
    pub backups: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct BackupResult {
    pub success: bool,
    pub message: String,
    pub path: Option<String>,
}

/// POST /api/db/compact - DB 압축
pub async fn compact_db(State(state): State<AppState>) -> Result<Json<CompactResult>> {
    // 먼저 백업
    let backup = state.backup.read().await;
    if let Err(e) = backup.backup() {
        tracing::warn!("압축 전 백업 실패: {}", e);
    }

    let mut db = state.db.write().await;
    let result = db.compact()?;

    Ok(Json(result))
}

/// POST /api/notes/:id/lock - 편집 락 획득
pub async fn acquire_lock(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<LockResponse>> {
    let now = chrono::Utc::now();
    let mut locks = state.edit_locks.write().await;

    // 기존 락 확인 (5분 이내면 충돌)
    if let Some(locked_at) = locks.get(&id) {
        let elapsed = now - *locked_at;
        if elapsed.num_minutes() < 5 {
            return Ok(Json(LockResponse {
                success: false,
                locked: true,
                message: "This note is being edited in another tab/window".to_string(),
            }));
        }
    }

    // 락 획득
    locks.insert(id, now);

    Ok(Json(LockResponse {
        success: true,
        locked: false,
        message: "Lock acquired".to_string(),
    }))
}

/// POST /api/notes/:id/unlock - 편집 락 해제
pub async fn release_lock(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<LockResponse>> {
    let mut locks = state.edit_locks.write().await;
    locks.remove(&id);

    Ok(Json(LockResponse {
        success: true,
        locked: false,
        message: "Lock released".to_string(),
    }))
}

/// GET /api/notes/:id/lock - 락 상태 확인
pub async fn check_lock(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<LockResponse>> {
    let locks = state.edit_locks.read().await;
    let now = chrono::Utc::now();

    if let Some(locked_at) = locks.get(&id) {
        let elapsed = now - *locked_at;
        if elapsed.num_minutes() < 5 {
            return Ok(Json(LockResponse {
                success: true,
                locked: true,
                message: "Note is locked".to_string(),
            }));
        }
    }

    Ok(Json(LockResponse {
        success: true,
        locked: false,
        message: "Note is not locked".to_string(),
    }))
}

#[derive(Serialize)]
pub struct LockResponse {
    pub success: bool,
    pub locked: bool,
    pub message: String,
}

/// 제목으로 노트 조회 (링크 클릭 시)
pub async fn get_by_title(
    State(state): State<AppState>,
    Path(title): Path<String>,
) -> impl IntoResponse {
    // URL 디코딩 (실패하면 원본 사용)
    let decoded_title = urlencoding::decode(&title)
        .map(|s| s.to_string())
        .unwrap_or(title);

    // 링크 인덱스에서 ID 찾기
    let note_id = {
        let index = state.link_index.read().await;
        index.get_id_by_title(&decoded_title)
    };

    match note_id {
        Some(id) => Redirect::to(&format!("/notes/{}", id)),
        None => {
            let encoded = urlencoding::encode(&decoded_title);
            Redirect::to(&format!("/notes/new?title={}", encoded))
        }
    }
}
