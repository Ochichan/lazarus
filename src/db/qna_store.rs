//! Q&A 저장소

use crate::db::{Answer, Question};
use crate::sync::jsonl::{read_jsonl, write_jsonl};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct QnaStore {
    path: PathBuf,
    questions: HashMap<String, Question>,
}

impl QnaStore {
    /// 저장소 열기
    pub fn open<P: AsRef<Path>>(data_dir: P) -> Result<Self, std::io::Error> {
        let path = data_dir.as_ref().join("questions.jsonl");
        let questions_vec: Vec<Question> = read_jsonl(&path)?;
        let questions = questions_vec
            .into_iter()
            .map(|q| (q.id.clone(), q))
            .collect();

        Ok(Self { path, questions })
    }

    /// 모든 질문 (최신순)
    pub fn list(&self) -> Vec<&Question> {
        let mut questions: Vec<_> = self.questions.values().collect();
        questions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        questions
    }

    /// 질문 조회
    pub fn get(&self, id: &str) -> Option<&Question> {
        self.questions.get(id)
    }

    /// 질문 저장
    pub fn save(&mut self, question: Question) -> Result<(), std::io::Error> {
        self.questions.insert(question.id.clone(), question);
        self.flush()
    }

    /// 질문 삭제
    pub fn delete(&mut self, id: &str) -> Result<bool, std::io::Error> {
        if self.questions.remove(id).is_some() {
            self.flush()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 답변 추가
    pub fn add_answer(
        &mut self,
        question_id: &str,
        author: String,
        content: String,
    ) -> Result<Option<String>, std::io::Error> {
        if let Some(question) = self.questions.get_mut(question_id) {
            let answer = Answer::new(author, content);
            let answer_id = answer.id.clone();
            question.answers.push(answer);
            self.flush()?;
            Ok(Some(answer_id))
        } else {
            Ok(None)
        }
    }

    /// 답변 채택
    pub fn accept_answer(
        &mut self,
        question_id: &str,
        answer_id: &str,
    ) -> Result<bool, std::io::Error> {
        if let Some(question) = self.questions.get_mut(question_id) {
            if question.accept_answer(answer_id) {
                self.flush()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// 투표
    pub fn vote_answer(
        &mut self,
        question_id: &str,
        answer_id: &str,
        up: bool,
    ) -> Result<bool, std::io::Error> {
        if let Some(question) = self.questions.get_mut(question_id) {
            if let Some(answer) = question.answers.iter_mut().find(|a| a.id == answer_id) {
                if up {
                    answer.upvote();
                } else {
                    answer.downvote();
                }
                self.flush()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// 파일에 저장
    fn flush(&self) -> Result<(), std::io::Error> {
        let questions: Vec<_> = self.questions.values().cloned().collect();
        write_jsonl(&self.path, &questions)
    }

    /// 개수
    pub fn count(&self) -> usize {
        self.questions.len()
    }

    /// 전체 가져오기 (동기화용)
    pub fn all(&self) -> Vec<Question> {
        self.questions.values().cloned().collect()
    }

    /// 병합 (동기화용)
    pub fn merge(&mut self, questions: Vec<Question>) -> Result<usize, std::io::Error> {
        let mut added = 0;
        for question in questions {
            if !self.questions.contains_key(&question.id) {
                self.questions.insert(question.id.clone(), question);
                added += 1;
            }
        }
        if added > 0 {
            self.flush()?;
        }
        Ok(added)
    }
}
