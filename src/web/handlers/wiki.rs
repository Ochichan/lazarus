//! 위키 핸들러 (다중 ZIM 지원)
use std::collections::HashMap;
use std::path::PathBuf;

use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;

use crate::error::{LazarusError, Result};
use crate::i18n::all_translations;
use crate::web::state::AppState;

#[derive(Deserialize)]
pub struct WikiSearchParams {
    pub q: Option<String>,
    pub zim: Option<String>,
}

/// 템플릿용 검색 결과 항목
pub struct WikiSearchEntry {
    pub title: String,
    pub url: String,
    pub url_encoded: String,
}

/// 위키 검색 템플릿
#[derive(Template)]
#[template(path = "wiki_search.html")]
struct WikiSearchTemplate {
    t: HashMap<String, String>,
    version: &'static str,
    lang: String,
    query: String,
    results: Vec<WikiSearchEntry>,
    zim_options: Vec<(String, bool)>,
    selected_zim: String,
}

/// 위키 문서 템플릿
#[derive(Template)]
#[template(path = "wiki_article.html")]
struct WikiArticleTemplate {
    title: String,
    content: String,
    selected_zim: String,
    zim_options: Vec<(String, bool)>,
}

/// ZIM 없음 템플릿
#[derive(Template)]
#[template(path = "wiki_no_zim.html")]
struct WikiNoZimTemplate {
    t: HashMap<String, String>,
    version: &'static str,
    lang: String,
}

/// ZIM 관리 템플릿
#[derive(Template)]
#[template(path = "wiki_manage.html")]
struct WikiManageTemplate {
    t: HashMap<String, String>,
    version: &'static str,
    lang: String,
    zim_count: usize,
    zim_dir: String,
    zim_list: Vec<(String, String)>,
}

/// GET /wiki/search
pub async fn search_wiki(
    State(state): State<AppState>,
    Query(params): Query<WikiSearchParams>,
) -> Result<Response> {
    let zim_names = state.zim_names().await;

    if zim_names.is_empty() {
        let lang = state.get_lang().await;
        let t = all_translations(lang);
        let template = WikiNoZimTemplate {
            t,
            version: env!("CARGO_PKG_VERSION"),
            lang: lang.code().to_string(),
        };
        return Ok(Html(template.render().unwrap_or_default()).into_response());
    }

    let selected_name = params
        .zim
        .clone()
        .unwrap_or_else(|| zim_names.first().cloned().unwrap_or_default());

    let selected_zim = state
        .get_zim_by_name(&selected_name)
        .await
        .or(state.get_zim().await)
        .ok_or_else(|| LazarusError::ZimNotFound {
            title: "ZIM 파일이 로드되지 않음".to_string(),
        })?;

    let zim = selected_zim.read().await;
    let query = params.q.clone().unwrap_or_default();

    let results = if query.is_empty() {
        zim.list_articles(50)?
    } else {
        zim.search_fuzzy(&query, 50)?
    };

    let search_results: Vec<WikiSearchEntry> = results
        .iter()
        .map(|entry| WikiSearchEntry {
            title: entry.title.clone(),
            url: entry.url.clone(),
            url_encoded: urlencoding::encode(&entry.url).to_string(),
        })
        .collect();

    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let zim_options: Vec<(String, bool)> = zim_names
        .iter()
        .map(|name| (name.clone(), name == &selected_name))
        .collect();

    let template = WikiSearchTemplate {
        t,
        version: env!("CARGO_PKG_VERSION"),
        lang: lang.code().to_string(),
        query,
        results: search_results,
        zim_options,
        selected_zim: selected_name,
    };

    Ok(Html(template.render().unwrap_or_default()).into_response())
}

/// GET /wiki/*path
pub async fn get_article(
    State(state): State<AppState>,
    path: Option<Path<String>>,
    Query(params): Query<WikiSearchParams>,
) -> Result<Response> {
    let path_str = path.map(|p| p.0).unwrap_or_default();
    let zim_names = state.zim_names().await;

    if zim_names.is_empty() {
        let lang = state.get_lang().await;
        let t = all_translations(lang);
        let template = WikiNoZimTemplate {
            t,
            version: env!("CARGO_PKG_VERSION"),
            lang: lang.code().to_string(),
        };
        return Ok(Html(template.render().unwrap_or_default()).into_response());
    }

    let selected_name = params
        .zim
        .clone()
        .unwrap_or_else(|| zim_names.first().cloned().unwrap_or_default());

    let selected_zim = state
        .get_zim_by_name(&selected_name)
        .await
        .or(state.get_zim().await)
        .ok_or_else(|| LazarusError::ZimNotFound {
            title: "ZIM 파일이 로드되지 않음".to_string(),
        })?;

    let zim = selected_zim.read().await;
    let url = path_str.trim_start_matches('/');

    if url.is_empty() {
        return Ok(
            axum::response::Redirect::to(&format!("/wiki/search?zim={}", selected_name))
                .into_response(),
        );
    }

    let decoded_url = urlencoding::decode(url)
        .map(|s| s.to_string())
        .unwrap_or_else(|_| url.to_string());

    // 빠른 검색
    if let Some(content) = zim.get_content_fast(&decoded_url)? {
        let content_type = guess_mime(&decoded_url);
        if content_type.contains("html") {
            let html = String::from_utf8_lossy(&content);
            let title = decoded_url.split('/').next_back().unwrap_or(&decoded_url);
            let wrapped = wrap_wiki_html(&html, title, &zim_names, &selected_name);
            return Ok(Html(wrapped).into_response());
        }
        return Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, content_type)],
            content,
        )
            .into_response());
    }

    // 느린 검색
    if let Some(entry) = zim.find_by_url('A', &decoded_url)? {
        if let Some(content) = zim.get_content('A', &decoded_url)? {
            let content_type = guess_mime(&decoded_url);
            if content_type.contains("html") {
                let html = String::from_utf8_lossy(&content);
                let wrapped = wrap_wiki_html(&html, &entry.title, &zim_names, &selected_name);
                return Ok(Html(wrapped).into_response());
            }
            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type)],
                content,
            )
                .into_response());
        }
    }

    // 404
    Ok((
        StatusCode::NOT_FOUND,
        Html(format!(
            r#"<!DOCTYPE html><html><head><title>404</title><link rel="stylesheet" href="/static/style.css"></head>
            <body><main class="container"><h1>Not Found</h1><p>{}</p>
            <a href="/wiki/search?zim={}" class="btn btn-primary">Back to Search</a></main></body></html>"#,
            decoded_url, selected_name
        )),
    ).into_response())
}

/// HTML 문서에 네비게이션 추가
fn wrap_wiki_html(content: &str, title: &str, zim_names: &[String], selected_zim: &str) -> String {
    let zim_options: Vec<(String, bool)> = zim_names
        .iter()
        .map(|name| (name.clone(), name == selected_zim))
        .collect();

    let template = WikiArticleTemplate {
        title: title.to_string(),
        content: content.to_string(),
        selected_zim: selected_zim.to_string(),
        zim_options,
    };

    template.render().unwrap_or_else(|_| content.to_string())
}

/// MIME 타입 추측
fn guess_mime(url: &str) -> &'static str {
    let lower = url.to_lowercase();
    if lower.ends_with(".html") || lower.ends_with(".htm") || !lower.contains('.') {
        "text/html; charset=utf-8"
    } else if lower.ends_with(".css") {
        "text/css"
    } else if lower.ends_with(".js") {
        "application/javascript"
    } else if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".json") {
        "application/json"
    } else {
        "application/octet-stream"
    }
}

/// GET /api/wiki/list
pub async fn list_zims(State(state): State<AppState>) -> Result<axum::Json<Vec<String>>> {
    Ok(axum::Json(state.zim_names().await))
}

/// POST /api/zim/reload
pub async fn reload_zims(State(state): State<AppState>) -> Result<axum::Json<ReloadResult>> {
    let added = state.reload_zims().await?;
    Ok(axum::Json(ReloadResult {
        success: true,
        added,
        total: state.zim_names().await.len(),
    }))
}

/// POST /api/zim/add
pub async fn add_zim(
    State(state): State<AppState>,
    axum::Json(req): axum::Json<AddZimRequest>,
) -> Result<axum::Json<AddZimResult>> {
    let path = PathBuf::from(&req.path);
    let canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            return Ok(axum::Json(AddZimResult {
                success: false,
                name: None,
                error: Some("Invalid path".to_string()),
            }))
        }
    };

    if !canonical.to_string_lossy().to_lowercase().ends_with(".zim") {
        return Ok(axum::Json(AddZimResult {
            success: false,
            name: None,
            error: Some("Only .zim files allowed".to_string()),
        }));
    }

    let path_str = canonical.to_string_lossy();
    let blocked = ["/etc", "/proc", "/sys", "/dev", "/root", "/boot"];
    for prefix in blocked {
        if path_str.starts_with(prefix) {
            return Ok(axum::Json(AddZimResult {
                success: false,
                name: None,
                error: Some("Access denied".to_string()),
            }));
        }
    }

    if !canonical.exists() {
        return Ok(axum::Json(AddZimResult {
            success: false,
            name: None,
            error: Some("File not found".to_string()),
        }));
    }

    match state.add_zim(canonical).await {
        Ok(name) => Ok(axum::Json(AddZimResult {
            success: true,
            name: Some(name),
            error: None,
        })),
        Err(e) => Ok(axum::Json(AddZimResult {
            success: false,
            name: None,
            error: Some(e.to_string()),
        })),
    }
}

/// DELETE /api/zim/:name
pub async fn remove_zim(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<axum::Json<RemoveResult>> {
    let removed = state.remove_zim(&name).await;
    Ok(axum::Json(RemoveResult {
        success: removed,
        name,
    }))
}

/// GET /api/zim/dir
pub async fn get_zim_dir(State(state): State<AppState>) -> Result<axum::Json<ZimDirResult>> {
    Ok(axum::Json(ZimDirResult {
        path: state.zim_dir.display().to_string(),
    }))
}

#[derive(serde::Serialize)]
pub struct ReloadResult {
    pub success: bool,
    pub added: Vec<String>,
    pub total: usize,
}

#[derive(serde::Deserialize)]
pub struct AddZimRequest {
    pub path: String,
}

#[derive(serde::Serialize)]
pub struct AddZimResult {
    pub success: bool,
    pub name: Option<String>,
    pub error: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RemoveResult {
    pub success: bool,
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct ZimDirResult {
    pub path: String,
}

/// GET /wiki/manage
pub async fn manage_zims(State(state): State<AppState>) -> Result<Html<String>> {
    let zim_list = state.zim_list().await;
    let zim_dir = state.zim_dir.display().to_string();
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let template = WikiManageTemplate {
        t,
        version: env!("CARGO_PKG_VERSION"),
        lang: lang.code().to_string(),
        zim_count: zim_list.len(),
        zim_dir,
        zim_list,
    };

    Ok(Html(template.render().unwrap_or_default()))
}
