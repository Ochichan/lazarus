//! HTTP 라우터
//!
//! 모든 엔드포인트 정의

use axum::{
    middleware,
    routing::{get, post, delete},
    Router,
};
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
    compression::CompressionLayer,
};

use super::handlers;
use super::middleware::require_unlock;
use super::state::AppState;

/// 라우터 생성
pub fn create_router(state: AppState) -> Router {
    Router::new()
		//Language
		.route("/api/lang", post(handlers::lang::set_lang))
		//Compaction
		.route("/api/db/compact", post(handlers::notes::compact_db))
		//백업
		.route("/api/backup/info", get(handlers::notes::backup_info))
		.route("/api/backup/now", post(handlers::notes::backup_now))
        // SRS 페이지
        .route("/srs", get(handlers::srs::review_page))
        .route("/srs/cards", get(handlers::srs::cards_page))
        // SRS API
        .route("/api/srs/stats", get(handlers::srs::get_stats))
        .route("/api/srs/due", get(handlers::srs::get_due_cards))
        .route("/api/srs/cards", get(handlers::srs::get_all_cards))
        .route("/api/srs/cards", post(handlers::srs::add_card))
        .route("/api/srs/cards/:id/review", post(handlers::srs::review_card))
        .route("/api/srs/cards/:id", delete(handlers::srs::delete_card))
        .route("/api/srs/extract/:note_id", post(handlers::srs::extract_from_note))
        .route("/api/srs/extract/:note_id", post(handlers::srs::extract_from_note))
        .route("/api/srs/optimize", post(handlers::srs::optimize_params))
        .route("/api/srs/params", get(handlers::srs::get_params))
        // 보안 API
        .route("/security", get(handlers::security::security_page))
        .route("/api/security/status", get(handlers::security::get_status))
        .route("/api/security/unlock", post(handlers::security::unlock))
        .route("/api/security/lock", post(handlers::security::lock))
        .route("/api/security/set-pin", post(handlers::security::set_pin))
        .route("/api/security/remove-pin", post(handlers::security::remove_pin))
        //노트 중복 확인 및 제거, laz
        .route("/api/notes/duplicates", get(handlers::notes::find_duplicates))
        .route("/api/notes/duplicates/remove", post(handlers::notes::remove_duplicates))
        .route("/api/laz/export", post(handlers::laz::export_package))
        .route("/api/laz/import", post(handlers::laz::import_package))
        .route("/api/laz/verify", post(handlers::laz::verify_package))
        // === 페이지 라우트 ===
        .route("/", get(handlers::pages::index))
        .route("/notes", get(handlers::pages::notes_list))
        .route("/notes/new", get(handlers::pages::notes_new))
        .route("/notes/:id", get(handlers::pages::notes_view))
        .route("/notes/:id/edit", get(handlers::pages::notes_edit))
        .route("/search", get(handlers::pages::search))
        .route("/notes/split", get(handlers::pages::notes_split))
        .route("/notes/split/:id", get(handlers::pages::notes_split_with_id))
        
        // === API 라우트 ===
        .route("/api/notes", get(handlers::notes::list))
        .route("/api/notes", post(handlers::notes::create_form))
        .route("/api/notes/:id", get(handlers::notes::get))
        .route("/api/notes/:id", post(handlers::notes::update_form))
        .route("/api/notes/:id", delete(handlers::notes::delete))
        .route("/api/notes/:id/lock", post(handlers::notes::acquire_lock))
        .route("/api/notes/:id/lock", get(handlers::notes::check_lock))
        .route("/api/notes/:id/unlock", post(handlers::notes::release_lock))
        .route("/api/search", get(handlers::search::search))
        
        // === 위키 라우트 ===
        .route("/wiki/manage", get(handlers::wiki::manage_zims))
        .route("/wiki/search", get(handlers::wiki::search_wiki))
        .route("/wiki", get(handlers::wiki::get_article))
        .route("/wiki/", get(handlers::wiki::get_article))
        .route("/wiki/*path", get(handlers::wiki::get_article))
        // ZIM API
        .route("/api/zim/reload", post(handlers::wiki::reload_zims))
        .route("/api/zim/add", post(handlers::wiki::add_zim))
        .route("/api/zim/list", get(handlers::wiki::list_zims))
        .route("/api/zim/dir", get(handlers::wiki::get_zim_dir))
        .route("/api/zim/:name", delete(handlers::wiki::remove_zim))
        // === 동기화 라우트 ===
        .route("/sync/export", post(handlers::sync::export))
        .route("/sync/import", post(handlers::sync::import))
        
        // === 헬스체크 ===
        .route("/health", get(handlers::health::check))
        
        // === 정적 파일 ===
        .nest_service("/static", ServeDir::new("static"))
        
        // === 미들웨어 ===
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(state.clone(), require_unlock))
        
        // === 상태 주입 ===
        .with_state(state)
}
