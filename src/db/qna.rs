//! Q&A (질문/답변)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 질문
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub answers: Vec<Answer>,
    #[serde(default)]
    pub accepted_answer: Option<String>,
}

/// 답변
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub id: String,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub votes: i32,
}

impl Question {
    /// 새 질문 생성
    pub fn new(author: String, title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            author,
            title,
            content,
            created_at: Utc::now(),
            tags: Vec::new(),
            answers: Vec::new(),
            accepted_answer: None,
        }
    }

    /// 답변 추가
    pub fn add_answer(&mut self, author: String, content: String) -> String {
        let answer = Answer::new(author, content);
        let id = answer.id.clone();
        self.answers.push(answer);
        id
    }

    /// 답변 채택
    pub fn accept_answer(&mut self, answer_id: &str) -> bool {
        if self.answers.iter().any(|a| a.id == answer_id) {
            self.accepted_answer = Some(answer_id.to_string());
            true
        } else {
            false
        }
    }
}

impl Answer {
    pub fn new(author: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            author,
            content,
            created_at: Utc::now(),
            votes: 0,
        }
    }

    pub fn upvote(&mut self) {
        self.votes += 1;
    }

    pub fn downvote(&mut self) {
        self.votes -= 1;
    }
}
