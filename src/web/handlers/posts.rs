//! Posts (게시판) API 핸들러

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};

use crate::db::Post;
use crate::web::state::AppState;

/// 게시글 목록 응답
#[derive(Serialize)]
pub struct PostListResponse {
    pub posts: Vec<PostInfo>,
    pub total: usize,
}

#[derive(Serialize)]
pub struct PostInfo {
    pub id: String,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub tags: Vec<String>,
    pub reply_count: usize,
}

impl From<&Post> for PostInfo {
    fn from(p: &Post) -> Self {
        Self {
            id: p.id.clone(),
            author: p.author.clone(),
            title: p.title.clone(),
            content: p.content.clone(),
            created_at: p.created_at.format("%Y-%m-%d %H:%M").to_string(),
            tags: p.tags.clone(),
            reply_count: p.replies.len(),
        }
    }
}

/// GET /api/posts - 게시글 목록
pub async fn list_posts(State(state): State<AppState>) -> Json<PostListResponse> {
    let store = state.posts.read().await;
    let posts: Vec<PostInfo> = store.list().iter().map(|p| PostInfo::from(*p)).collect();
    let total = posts.len();
    Json(PostListResponse { posts, total })
}

/// 게시글 생성 요청
#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub author: String,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// POST /api/posts - 게시글 생성
pub async fn create_post(
    State(state): State<AppState>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<PostInfo>, StatusCode> {
    let mut post = Post::new(req.author, req.title, req.content);
    post.tags = req.tags;

    let mut store = state.posts.write().await;
    store
        .save(post.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PostInfo::from(&post)))
}

/// GET /api/posts/:id - 게시글 상세
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Post>, StatusCode> {
    let store = state.posts.read().await;
    store
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// DELETE /api/posts/:id - 게시글 삭제
pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut store = state.posts.write().await;
    store
        .delete(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"success": true})))
}

/// 댓글 추가 요청
#[derive(Deserialize)]
pub struct AddReplyRequest {
    pub author: String,
    pub content: String,
}

/// POST /api/posts/:id/replies - 댓글 추가
pub async fn add_reply(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<AddReplyRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut store = state.posts.write().await;
    match store.add_reply(&id, req.author, req.content) {
        Ok(Some(reply_id)) => Ok(Json(
            serde_json::json!({"success": true, "reply_id": reply_id}),
        )),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
