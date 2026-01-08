//! Q&A API 핸들러

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};

use crate::db::Question;
use crate::web::state::AppState;

/// 질문 목록 응답
#[derive(Serialize)]
pub struct QuestionListResponse {
    pub questions: Vec<QuestionInfo>,
    pub total: usize,
}

#[derive(Serialize)]
pub struct QuestionInfo {
    pub id: String,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub tags: Vec<String>,
    pub answer_count: usize,
    pub has_accepted: bool,
}

impl From<&Question> for QuestionInfo {
    fn from(q: &Question) -> Self {
        Self {
            id: q.id.clone(),
            author: q.author.clone(),
            title: q.title.clone(),
            content: q.content.clone(),
            created_at: q.created_at.format("%Y-%m-%d %H:%M").to_string(),
            tags: q.tags.clone(),
            answer_count: q.answers.len(),
            has_accepted: q.accepted_answer.is_some(),
        }
    }
}

/// GET /api/qna - 질문 목록
pub async fn list_questions(State(state): State<AppState>) -> Json<QuestionListResponse> {
    let store = state.qna.read().await;
    let questions: Vec<QuestionInfo> = store
        .list()
        .iter()
        .map(|q| QuestionInfo::from(*q))
        .collect();
    let total = questions.len();
    Json(QuestionListResponse { questions, total })
}

/// 질문 생성 요청
#[derive(Deserialize)]
pub struct CreateQuestionRequest {
    pub author: String,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// POST /api/qna - 질문 생성
pub async fn create_question(
    State(state): State<AppState>,
    Json(req): Json<CreateQuestionRequest>,
) -> Result<Json<QuestionInfo>, StatusCode> {
    let mut question = Question::new(req.author, req.title, req.content);
    question.tags = req.tags;

    let mut store = state.qna.write().await;
    store
        .save(question.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(QuestionInfo::from(&question)))
}

/// GET /api/qna/:id - 질문 상세
pub async fn get_question(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Question>, StatusCode> {
    let store = state.qna.read().await;
    store
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// DELETE /api/qna/:id - 질문 삭제
pub async fn delete_question(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut store = state.qna.write().await;
    store
        .delete(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"success": true})))
}

/// 답변 추가 요청
#[derive(Deserialize)]
pub struct AddAnswerRequest {
    pub author: String,
    pub content: String,
}

/// POST /api/qna/:id/answers - 답변 추가
pub async fn add_answer(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<AddAnswerRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut store = state.qna.write().await;
    match store.add_answer(&id, req.author, req.content) {
        Ok(Some(answer_id)) => Ok(Json(
            serde_json::json!({"success": true, "answer_id": answer_id}),
        )),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// POST /api/qna/:id/accept/:answer_id - 답변 채택
pub async fn accept_answer(
    State(state): State<AppState>,
    Path((id, answer_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut store = state.qna.write().await;
    match store.accept_answer(&id, &answer_id) {
        Ok(true) => Ok(Json(serde_json::json!({"success": true}))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// POST /api/qna/:id/vote/:answer_id - 투표
#[derive(Deserialize)]
pub struct VoteRequest {
    pub up: bool,
}

pub async fn vote_answer(
    State(state): State<AppState>,
    Path((id, answer_id)): Path<(String, String)>,
    Json(req): Json<VoteRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut store = state.qna.write().await;
    match store.vote_answer(&id, &answer_id, req.up) {
        Ok(true) => Ok(Json(serde_json::json!({"success": true}))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
