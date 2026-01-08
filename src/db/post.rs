//! 게시판 포스트

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 게시글
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub replies: Vec<Reply>,
}

/// 댓글
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply {
    pub id: String,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Post {
    /// 새 게시글 생성
    pub fn new(author: String, title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            author,
            title,
            content,
            created_at: Utc::now(),
            tags: Vec::new(),
            replies: Vec::new(),
        }
    }

    /// 댓글 추가
    pub fn add_reply(&mut self, author: String, content: String) {
        self.replies.push(Reply {
            id: Uuid::new_v4().to_string(),
            author,
            content,
            created_at: Utc::now(),
        });
    }
}

impl Reply {
    pub fn new(author: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            author,
            content,
            created_at: Utc::now(),
        }
    }
}
