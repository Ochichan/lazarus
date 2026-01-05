//! SRS 플래시카드 핸들러

#[derive(Serialize)]
pub struct StatsResponse {
    pub total: usize,
    pub due: usize,
    pub new: usize,
    pub learning: usize,
    pub mature: usize,
    pub streak: u32,
    pub streak_emoji: String,
    pub total_reviews: u64,
    pub total_days: u32,
}

use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::{LazarusError, Result};
use crate::i18n::all_translations;
use crate::srs::{extractor, Card, CardType, ReviewResult, SrsStats};
use crate::web::state::AppState;
use std::collections::HashMap;

/// GET /api/srs/stats - 통계
pub async fn get_stats(State(state): State<AppState>) -> Result<Json<StatsResponse>> {
    let srs = state.srs.read().await;
    let stats = srs.stats();
    let user_stats = &srs.user_stats;

    Ok(Json(StatsResponse {
        total: stats.total,
        due: stats.due,
        new: stats.new,
        learning: stats.learning,
        mature: stats.mature,
        streak: user_stats.streak,
        streak_emoji: user_stats.streak_emoji().to_string(),
        total_reviews: user_stats.total_reviews,
        total_days: user_stats.total_days,
    }))
}

/// GET /api/srs/due - 복습할 카드 목록
pub async fn get_due_cards(State(state): State<AppState>) -> Result<Json<Vec<CardResponse>>> {
    let srs = state.srs.read().await;
    let cards: Vec<CardResponse> = srs
        .due_cards()
        .into_iter()
        .map(|c| CardResponse::from(c.clone()))
        .collect();
    Ok(Json(cards))
}

/// GET /api/srs/cards - 모든 카드
pub async fn get_all_cards(State(state): State<AppState>) -> Result<Json<Vec<CardResponse>>> {
    let srs = state.srs.read().await;
    let cards: Vec<CardResponse> = srs
        .all_cards()
        .into_iter()
        .map(|c| CardResponse::from(c.clone()))
        .collect();
    Ok(Json(cards))
}

/// POST /api/srs/cards - 카드 추가
pub async fn add_card(
    State(state): State<AppState>,
    Json(params): Json<AddCardParams>,
) -> Result<Json<CardResponse>> {
    let mut srs = state.srs.write().await;

    let card = Card {
        id: 0,
        card_type: params.card_type.unwrap_or(CardType::Basic),
        question: params.question,
        answer: params.answer,
        source_note_id: params.source_note_id,
        source_wiki_url: params.source_wiki_url,
        hints: params.hints.unwrap_or_default(),
        tags: params.tags.unwrap_or_default(),
        srs: Default::default(),
        created_at: chrono::Utc::now(),
    };

    let id = srs.add_card(card)?;
    let card = srs.get_card(id).unwrap();

    Ok(Json(CardResponse::from(card.clone())))
}

/// POST /api/srs/cards/:id/review - 복습 결과
pub async fn review_card(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(params): Json<ReviewParams>,
) -> Result<Json<ReviewResponse>> {
    let mut srs = state.srs.write().await;

    let result = ReviewResult::from_score(params.score);
    srs.review(id, result)?;

    let card = srs.get_card(id).unwrap();

    Ok(Json(ReviewResponse {
        success: true,
        next_review: card.srs.next_review.map(|d| d.to_rfc3339()),
        interval: card.srs.interval,
    }))
}

/// DELETE /api/srs/cards/:id - 카드 삭제
pub async fn delete_card(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<DeleteResponse>> {
    let mut srs = state.srs.write().await;
    let deleted = srs.delete_card(id)?;

    Ok(Json(DeleteResponse {
        success: deleted,
        message: if deleted {
            "삭제됨".to_string()
        } else {
            "카드를 찾을 수 없음".to_string()
        },
    }))
}

/// POST /api/srs/extract/:note_id - 노트에서 카드 추출
pub async fn extract_from_note(
    State(state): State<AppState>,
    Path(note_id): Path<u64>,
) -> Result<Json<ExtractResponse>> {
    let db = state.db.read().await;
    let note = db
        .get(note_id)?
        .ok_or_else(|| LazarusError::NotFound(format!("노트 ID: {}", note_id)))?;
    drop(db);

    let cards = extractor::extract_cards_from_note(note_id, &note.title, &note.content);

    let mut srs = state.srs.write().await;
    let mut added_ids = Vec::new();

    for card in cards {
        let id = srs.add_card(card)?;
        added_ids.push(id);
    }

    Ok(Json(ExtractResponse {
        success: true,
        cards_created: added_ids.len(),
        card_ids: added_ids.clone(),
        message: format!("{}개의 카드가 생성되었습니다.", added_ids.len()),
    }))
}

// === 페이지 핸들러 ===

/// 복습 페이지 템플릿
#[derive(Template)]
#[template(path = "srs_review.html")]
struct SrsReviewTemplate {
    version: &'static str,
    stats: SrsStats,
    streak: u32,
    lang: &'static str,
    t: HashMap<String, String>,
}

/// GET /srs - 복습 페이지
/// GET /srs - 복습 페이지
pub async fn review_page(State(state): State<AppState>) -> Result<Html<String>> {
    let srs = state.srs.read().await;
    let stats = srs.stats();
    let streak = srs.user_stats.streak;
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let template = SrsReviewTemplate {
        version: state.version,
        stats,
        streak,
        lang: lang.code(),
        t,
    };
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

/// 카드 목록 페이지 템플릿
#[derive(Template)]
#[template(path = "srs_cards.html")]
struct SrsCardsTemplate {
    version: &'static str,
    cards: Vec<CardResponse>,
    lang: &'static str,
    t: HashMap<String, String>,
}

/// GET /srs/cards - 카드 목록 페이지
pub async fn cards_page(State(state): State<AppState>) -> Result<Html<String>> {
    let srs = state.srs.read().await;
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let cards: Vec<CardResponse> = srs
        .all_cards()
        .into_iter()
        .map(|c| CardResponse::from(c.clone()))
        .collect();
    let template = SrsCardsTemplate {
        version: state.version,
        cards,
        lang: lang.code(),
        t,
    };
    Ok(Html(
        template
            .render()
            .map_err(|e| LazarusError::ServerStart(e.to_string()))?,
    ))
}

// === DTO ===

#[derive(Serialize)]
pub struct CardResponse {
    pub id: u64,
    pub card_type: String,
    pub question: String,
    pub answer: String,
    pub source_note_id: Option<u64>,
    pub source_wiki_url: Option<String>,
    pub tags: Vec<String>,
    pub interval: u32,
    pub repetitions: u32,
    pub streak: u32,
    pub next_review: Option<String>,
}

impl From<Card> for CardResponse {
    fn from(c: Card) -> Self {
        Self {
            id: c.id,
            card_type: format!("{:?}", c.card_type).to_lowercase(),
            question: c.question,
            answer: c.answer,
            source_note_id: c.source_note_id,
            source_wiki_url: c.source_wiki_url,
            tags: c.tags,
            interval: c.srs.interval,
            repetitions: c.srs.repetitions,
            streak: c.srs.streak,
            next_review: c.srs.next_review.map(|d| d.to_rfc3339()),
        }
    }
}

#[derive(Deserialize)]
pub struct AddCardParams {
    pub question: String,
    pub answer: String,
    pub card_type: Option<CardType>,
    pub source_note_id: Option<u64>,
    pub source_wiki_url: Option<String>,
    pub hints: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct ReviewParams {
    pub score: u8, // 0: Again, 1: Hard, 2: Good, 3: Easy
}

#[derive(Serialize)]
pub struct ReviewResponse {
    pub success: bool,
    pub next_review: Option<String>,
    pub interval: u32,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct ExtractResponse {
    pub success: bool,
    pub cards_created: usize,
    pub card_ids: Vec<u64>,
    pub message: String,
}

/// POST /api/srs/optimize - FSRS 파라미터 최적화
pub async fn optimize_params(State(state): State<AppState>) -> Result<Json<OptimizeResponse>> {
    let mut srs = state.srs.write().await;

    let result = srs.optimize_params()?;

    Ok(Json(OptimizeResponse {
        success: true,
        log_count: result.log_count,
        rmse: result.rmse,
        predicted_retention: result.predicted_retention,
        message: format!(
            "{}개의 복습 기록으로 최적화 완료! 예상 기억률: {:.1}%",
            result.log_count,
            result.predicted_retention * 100.0
        ),
    }))
}

/// GET /api/srs/params - 현재 FSRS 파라미터
pub async fn get_params(State(state): State<AppState>) -> Result<Json<ParamsResponse>> {
    let srs = state.srs.read().await;
    let params = srs.current_params();
    let log_count = srs.log_count();
    let is_custom = srs.custom_params.is_some();

    Ok(Json(ParamsResponse {
        params: params.w.to_vec(),
        is_custom,
        log_count,
        min_logs_required: 100,
    }))
}

#[derive(Serialize)]
pub struct OptimizeResponse {
    pub success: bool,
    pub log_count: usize,
    pub rmse: f32,
    pub predicted_retention: f32,
    pub message: String,
}

#[derive(Serialize)]
pub struct ParamsResponse {
    pub params: Vec<f32>,
    pub is_custom: bool,
    pub log_count: usize,
    pub min_logs_required: usize,
}
