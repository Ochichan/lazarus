//! Post 저장소

use crate::db::{Post, Reply};
use crate::sync::jsonl::{read_jsonl, write_jsonl};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct PostStore {
    path: PathBuf,
    posts: HashMap<String, Post>,
}

impl PostStore {
    /// 저장소 열기
    pub fn open<P: AsRef<Path>>(data_dir: P) -> Result<Self, std::io::Error> {
        let path = data_dir.as_ref().join("posts.jsonl");
        let posts_vec: Vec<Post> = read_jsonl(&path)?;
        let posts = posts_vec.into_iter().map(|p| (p.id.clone(), p)).collect();

        Ok(Self { path, posts })
    }

    /// 모든 게시글 (최신순)
    pub fn list(&self) -> Vec<&Post> {
        let mut posts: Vec<_> = self.posts.values().collect();
        posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        posts
    }

    /// 게시글 조회
    pub fn get(&self, id: &str) -> Option<&Post> {
        self.posts.get(id)
    }

    /// 게시글 저장
    pub fn save(&mut self, post: Post) -> Result<(), std::io::Error> {
        self.posts.insert(post.id.clone(), post);
        self.flush()
    }

    /// 게시글 삭제
    pub fn delete(&mut self, id: &str) -> Result<bool, std::io::Error> {
        if self.posts.remove(id).is_some() {
            self.flush()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 댓글 추가
    pub fn add_reply(
        &mut self,
        post_id: &str,
        author: String,
        content: String,
    ) -> Result<Option<String>, std::io::Error> {
        if let Some(post) = self.posts.get_mut(post_id) {
            let reply = Reply::new(author, content);
            let reply_id = reply.id.clone();
            post.replies.push(reply);
            self.flush()?;
            Ok(Some(reply_id))
        } else {
            Ok(None)
        }
    }

    /// 파일에 저장
    fn flush(&self) -> Result<(), std::io::Error> {
        let posts: Vec<_> = self.posts.values().cloned().collect();
        write_jsonl(&self.path, &posts)
    }

    /// 개수
    pub fn count(&self) -> usize {
        self.posts.len()
    }

    /// 전체 가져오기 (동기화용)
    pub fn all(&self) -> Vec<Post> {
        self.posts.values().cloned().collect()
    }

    /// 병합 (동기화용)
    pub fn merge(&mut self, posts: Vec<Post>) -> Result<usize, std::io::Error> {
        let mut added = 0;
        for post in posts {
            if !self.posts.contains_key(&post.id) {
                self.posts.insert(post.id.clone(), post);
                added += 1;
            }
        }
        if added > 0 {
            self.flush()?;
        }
        Ok(added)
    }
}
