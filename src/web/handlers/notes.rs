//! 노트 CRUD API
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
    let mut note = Note::new(0, req.title, req.content);
    
    // 쉼표로 구분된 태그 파싱
    note.tags = req.tags
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    // 암호화 여부
    note.encrypted = req.encrypted == "true";
    
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
    
	tracing::info!("노트 생성 (Form): id={}, encrypted={}", id, note.encrypted);
    Ok(axum::response::Redirect::to(&format!("/notes/{}", id)))
}
/// GET /api/notes/:id
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<NoteResponse>> {
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
    let existing = db.get(id)?
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
    let tags: Vec<String> = req.tags
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    // 암호화 여부
    let encrypted = req.encrypted == "true";
    
    let note = {
        let mut db = state.db.write().await;
        // 기존 노트 확인
        let existing = db.get(id)?
            .ok_or_else(|| LazarusError::NotFound(format!("노트 ID: {}", id)))?;
        
        let mut note = Note::new(id, req.title, req.content);
        note.tags = tags;
        note.created_at = existing.created_at;
        note.encrypted = encrypted;
        
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
        Ok((
            StatusCode::OK,
            [("HX-Redirect", "/notes")],
            ""
        ))
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
}

/// 노트 생성 요청 (Form)
#[derive(Deserialize)]
pub struct CreateNoteFormRequest {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: String,  // 쉼표로 구분된 문자열
    #[serde(default)]
    pub encrypted: String,  // "true" 또는 "false"
}

/// GET /api/notes/duplicates - 중복 노트 찾기
pub async fn find_duplicates(
    State(state): State<AppState>,
) -> Result<Json<Vec<DuplicateGroup>>> {
    let db = state.db.read().await;
    let ids = db.list_ids();
    
    // 제목+내용 해시로 그룹화
    let mut hash_map: std::collections::HashMap<String, Vec<NoteInfo>> = std::collections::HashMap::new();
    
    for id in ids {
        if let Some(note) = db.get(id)? {
            // 제목과 내용 첫 500자로 해시 생성
            let content_preview = note.content.chars().take(500).collect::<String>();
            // 이스케이프 문자 정규화
            let clean_title = note.title
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
            
            use sha2::{Sha256, Digest};
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
pub async fn backup_info(
    State(state): State<AppState>,
) -> Result<Json<BackupInfoResponse>> {
    let info = state.backup.info()?;
    
    Ok(Json(BackupInfoResponse {
        count: info.count,
        total_size_kb: info.total_size / 1024,
        latest: info.latest.map(|p| p.display().to_string()),
        backups: info.backups.iter().map(|p| p.display().to_string()).collect(),
    }))
}

/// POST /api/backup/now - 수동 백업
pub async fn backup_now(
    State(state): State<AppState>,
) -> Result<Json<BackupResult>> {
    match state.backup.backup()? {
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
pub async fn compact_db(
    State(state): State<AppState>,
) -> Result<Json<CompactResult>> {
    // 먼저 백업
    if let Err(e) = state.backup.backup() {
        tracing::warn!("압축 전 백업 실패: {}", e);
    }
    
    let mut db = state.db.write().await;
    let result = db.compact()?;
    
    Ok(Json(result))
}
