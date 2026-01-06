//! HTML 페이지 핸들러
//!
//! Askama 템플릿을 사용한 서버사이드 렌더링

use crate::error::{LazarusError, Result};
use crate::i18n::all_translations;
use crate::web::state::AppState;
use crate::links;

use ammonia::clean;
use askama::Template;
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use serde::Deserialize;
use std::collections::HashMap;

/// 인덱스 페이지
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
    version: &'static str,
    notes_count: usize,
    has_zim: bool,
    streak: u32,
    lang: &'static str,
    t: HashMap<String, String>,
}

/// GET /
pub async fn index(State(state): State<AppState>) -> Result<Html<String>> {
    let db = state.db.read().await;
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    // 스트릭 가져오기
    let srs = state.srs.read().await;
    let stats = srs.user_stats.clone();

    let template = IndexTemplate {
        title: "Lazarus",
        version: state.version,
        notes_count: db.count(),
        has_zim: !state.zims.read().await.is_empty(),
        streak: stats.streak,
        lang: lang.code(),
        t,
    };
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

/// 노트 목록 템플릿
#[derive(Template)]
#[template(path = "notes_list.html")]
struct NotesListTemplate {
    version: &'static str,
    notes: Vec<NoteListItem>,
    lang: &'static str,
    t: HashMap<String, String>,
}

struct NoteListItem {
    id: u64,
    title: String,
    preview: String,
    updated_at: String,
    note_type: String,
    note_type_emoji: String,
}

// 템플릿 구조체 추가
#[derive(Template)]
#[template(path = "graph.html")]
struct GraphTemplate {
    version: &'static str,
    lang: &'static str,
    t: HashMap<String, String>,
}

/// GET /graph
pub async fn graph_view(
    State(state): State<AppState>,
) -> Result<Html<String>> {
    let lang = state.get_lang().await;
    let t = all_translations(lang);
    
    let template = GraphTemplate {
        version: state.version,
        lang: lang.code(),
        t,
    };
    
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

/// GET /notes
pub async fn notes_list(State(state): State<AppState>) -> Result<Html<String>> {
    let db = state.db.read().await;
    let lang = state.get_lang().await;
    let t = all_translations(lang);
    let ids = db.list_ids();

    let mut notes: Vec<NoteListItem> = Vec::new();
    for id in ids {
        if let Some(note) = db.get(id)? {
            notes.push(NoteListItem {
                id: note.id,
                title: if note.title.is_empty() {
                    t.get("notes.no_title")
                        .cloned()
                        .unwrap_or_else(|| "Untitled".to_string())
                } else {
                    note.title
                },
                preview: note.content.chars().take(100).collect::<String>() + "...",
                updated_at: note.updated_at.format("%Y-%m-%d %H:%M").to_string(),
                note_type: format!("{:?}", note.note_type).to_lowercase(),
                note_type_emoji: note.note_type.emoji().to_string(),
            });
        }
    }

    // 최신순 정렬
    notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    let template = NotesListTemplate {
        version: state.version,
        notes,
        lang: lang.code(),
        t,
    };

    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

/// 노트 작성 템플릿
#[derive(Template)]
#[template(path = "notes_edit.html")]
struct NotesEditTemplate {
    version: &'static str,
    note_id: u64,
    is_new: bool,
    note_title: String,
    content: String,
    tags: String,
    all_tags: Vec<String>,
    lang: &'static str,
    t: HashMap<String, String>,
    note_type: String,
}

/// GET /notes/new
pub async fn notes_new(State(state): State<AppState>) -> Result<Html<String>> {
    let all_tags = get_all_tags(&state).await?;
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let template = NotesEditTemplate {
        version: state.version,
        note_id: 0,
        is_new: true,
        note_title: String::new(),
        content: String::new(),
        tags: String::new(),
        all_tags,
        lang: lang.code(),
        t,
        note_type: "note".to_string(),
    };
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

/// 노트 보기 템플릿
#[derive(Template)]
#[template(path = "notes_view.html")]
struct NotesViewTemplate {
    version: &'static str,
    note: NoteViewData,
    lang: &'static str,
    t: HashMap<String, String>,
    backlinks: Vec<BacklinkInfo>, 
}

struct NoteViewData {
    id: u64,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
    tags: Vec<String>,
}

struct BacklinkInfo {
    id: u64,
    title: String,
}

/// GET /notes/:id
pub async fn notes_view(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Html<String>> {
    let db = state.db.read().await;
    let note = db
        .get(id)?
        .ok_or_else(|| LazarusError::NotFound(format!("Note ID: {}", id)))?;

    // 백링크 조회
    let backlinks: Vec<BacklinkInfo> = {
        let index = state.link_index.read().await;
        let backlink_ids = index.get_backlinks(&note.title);
        
        backlink_ids.iter()
            .filter_map(|bid| {
                index.get_title_by_id(*bid).map(|title| BacklinkInfo {
                    id: *bid,
                    title: title.to_string(),
                })
            })
            .collect()
    };

    // [[링크]] → HTML로 렌더링
    let existing_titles = state.link_index.read().await.existing_titles();
    let rendered_content = links::render_links(&note.content, Some(&existing_titles));

    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let template = NotesViewTemplate {
        version: state.version,
        note: NoteViewData {
            id: note.id,
            title: note.title,
            content: clean(&rendered_content),
            created_at: note.created_at.format("%Y-%m-%d %H:%M").to_string(),
            updated_at: note.updated_at.format("%Y-%m-%d %H:%M").to_string(),
            tags: note.tags,
        },
        lang: lang.code(),
        t,
        backlinks,
    };
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

/// GET /notes/:id/edit
pub async fn notes_edit(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Html<String>> {
    let db = state.db.read().await;
    let note = db
        .get(id)?
        .ok_or_else(|| LazarusError::NotFound(format!("Note ID: {}", id)))?;
    drop(db);

    let all_tags = get_all_tags(&state).await?;
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let template = NotesEditTemplate {
        version: state.version,
        note_id: note.id,
        is_new: false,
        note_title: note.title.clone(),
        content: note.content,
        tags: note.tags.join(", "),
        all_tags,
        lang: lang.code(),
        t,
        note_type: format!("{:?}", note.note_type).to_lowercase(),
    };
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

/// 검색 쿼리
#[derive(Deserialize, Default)]
pub struct SearchQuery {
    #[serde(default)]
    pub q: String,
}

/// 검색 결과 템플릿
#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate {
    version: &'static str,
    query: String,
    results: Vec<SearchResultItem>,
    lang: &'static str,
    t: HashMap<String, String>,
}

struct SearchResultItem {
    id: u64,
    title: String,
    preview: String,
    source: String, // "note" or "wiki"
}

/// GET /search
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Html<String>> {
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let mut results = Vec::new();
    if !params.q.is_empty() {
        let search_engine = state.search.read().await;
        let search_results = search_engine.search_fuzzy(&params.q, 20)?;
        for r in search_results {
            results.push(SearchResultItem {
                id: r.id,
                title: r.title,
                preview: r.preview,
                source: "note".to_string(),
            });
        }
    }
    let template = SearchTemplate {
        version: state.version,
        query: params.q,
        results,
        lang: lang.code(),
        t,
    };
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

/// 모든 태그 목록 가져오기
async fn get_all_tags(state: &AppState) -> Result<Vec<String>> {
    let db = state.db.read().await;
    let mut tags_set = std::collections::HashSet::new();

    for id in db.list_ids() {
        if let Ok(Some(note)) = db.get(id) {
            for tag in note.tags {
                tags_set.insert(tag);
            }
        }
    }

    let mut tags: Vec<String> = tags_set.into_iter().collect();
    tags.sort();
    Ok(tags)
}

/// 스플릿뷰 템플릿
#[derive(Template)]
#[template(path = "notes_split.html")]
struct NotesSplitTemplate {
    version: &'static str,
    notes: Vec<NoteListItem>,
    selected_id: Option<u64>,
    note_title: String,
    note_content: String,
    note_tags: String,
    lang: &'static str,
    t: HashMap<String, String>,
}

/// GET /notes/split
/// GET /notes/split
pub async fn notes_split(State(state): State<AppState>) -> Result<Html<String>> {
    let db = state.db.read().await;
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let ids = db.list_ids();
    let mut notes: Vec<NoteListItem> = Vec::new();
    for id in ids {
        if let Some(note) = db.get(id)? {
            notes.push(NoteListItem {
                id: note.id,
                title: if note.title.is_empty() {
                    t.get("notes.no_title")
                        .cloned()
                        .unwrap_or_else(|| "Untitled".to_string())
                } else {
                    note.title
                },
                preview: note.content.chars().take(100).collect::<String>() + "...",
                updated_at: note.updated_at.format("%Y-%m-%d %H:%M").to_string(),
                note_type: format!("{:?}", note.note_type).to_lowercase(),
                note_type_emoji: note.note_type.emoji().to_string(),
            });
        }
    }
    notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    let template = NotesSplitTemplate {
        version: state.version,
        notes,
        selected_id: None,
        note_title: String::new(),
        note_content: String::new(),
        note_tags: String::new(),
        lang: lang.code(),
        t,
    };
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

/// GET /notes/split/:id
pub async fn notes_split_with_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Html<String>> {
    let db = state.db.read().await;
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let ids = db.list_ids();
    let mut notes: Vec<NoteListItem> = Vec::new();
    for nid in ids {
        if let Some(note) = db.get(nid)? {
            notes.push(NoteListItem {
                id: note.id,
                title: if note.title.is_empty() {
                    t.get("notes.no_title")
                        .cloned()
                        .unwrap_or_else(|| "Untitled".to_string())
                } else {
                    note.title
                },
                preview: note.content.chars().take(100).collect::<String>() + "...",
                updated_at: note.updated_at.format("%Y-%m-%d %H:%M").to_string(),
                note_type: format!("{:?}", note.note_type).to_lowercase(),
                note_type_emoji: note.note_type.emoji().to_string(),
            });
        }
    }
    notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    let selected = db
        .get(id)?
        .ok_or_else(|| LazarusError::NotFound(format!("Note ID: {}", id)))?;
    let template = NotesSplitTemplate {
        version: state.version,
        notes,
        selected_id: Some(id),
        note_title: selected.title,
        note_content: selected.content,
        note_tags: selected.tags.join(", "),
        lang: lang.code(),
        t,
    };
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}
