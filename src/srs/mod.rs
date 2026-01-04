//! SRS (Spaced Repetition System) 엔진
//!
//! SM-2 알고리즘 기반 간격 반복 학습
pub mod extractor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use chrono::{DateTime, Utc, Duration};

use crate::error::{LazarusError, Result};

/// 플래시카드
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// 카드 ID
    pub id: u64,
    /// 카드 타입
    pub card_type: CardType,
    /// 질문 (빈칸은 {{answer}} 형태)
    pub question: String,
    /// 정답
    pub answer: String,
    /// 원본 노트 ID (있으면)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_note_id: Option<u64>,
    /// 원본 위키 URL (있으면)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_wiki_url: Option<String>,
    /// 힌트
    #[serde(default)]
    pub hints: Vec<String>,
    /// 태그
    #[serde(default)]
    pub tags: Vec<String>,
    /// SRS 데이터
    #[serde(default)]
    pub srs: SrsData,
    /// 생성 시간
    pub created_at: DateTime<Utc>,
}

/// 카드 타입
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CardType {
    /// 기본 Q&A
    Basic,
    /// 빈칸 채우기
    Cloze,
    /// 정의
    Definition,
}

impl Default for CardType {
    fn default() -> Self {
        Self::Basic
    }
}

/// SRS 학습 데이터
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SrsData {
    /// 다음 복습 시간
    pub next_review: Option<DateTime<Utc>>,
    /// 간격 (일)
    pub interval: u32,
    /// 난이도 계수 (2.5 기본)
    pub ease_factor: f32,
    /// 복습 횟수
    pub repetitions: u32,
    /// 연속 정답 횟수
    pub streak: u32,
}

impl SrsData {
    pub fn new() -> Self {
        Self {
            next_review: Some(Utc::now()),
            interval: 0,
            ease_factor: 2.5,
            repetitions: 0,
            streak: 0,
        }
    }
}

/// 복습 결과
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReviewResult {
    /// 완전히 틀림
    Again,
    /// 어려웠음
    Hard,
    /// 맞음
    Good,
    /// 쉬웠음
    Easy,
}

impl ReviewResult {
    pub fn from_score(score: u8) -> Self {
        match score {
            0 => Self::Again,
            1 => Self::Hard,
            2 => Self::Good,
            _ => Self::Easy,
        }
    }
}

/// SRS 엔진
pub struct SrsEngine {
    cards: HashMap<u64, Card>,
    next_id: u64,
    file_path: String,
    pub user_stats: UserStats,
    stats_path: String,
}

impl SrsEngine {
    /// 새 엔진 생성 또는 파일에서 로드
        pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
            let file_path = path.as_ref().to_string_lossy().to_string();
            let stats_path = file_path.replace(".jsonl", "_stats.json");
            
            let mut engine = Self {
                cards: HashMap::new(),
                next_id: 1,
                file_path,
                user_stats: UserStats::default(),
                stats_path,
            };
    
            if path.as_ref().exists() {
                engine.load()?;
            }
            
            engine.load_stats();
    
            Ok(engine)
        }

    /// 파일에서 로드
    fn load(&mut self) -> Result<()> {
        let file = File::open(&self.file_path).map_err(LazarusError::Io)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(LazarusError::Io)?;
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(card) = serde_json::from_str::<Card>(&line) {
                if card.id >= self.next_id {
                    self.next_id = card.id + 1;
                }
                self.cards.insert(card.id, card);
            }
        }

        tracing::info!("SRS 엔진: {}개의 카드 로드됨", self.cards.len());
        Ok(())
    }

    /// 카드 추가
    pub fn add_card(&mut self, mut card: Card) -> Result<u64> {
        card.id = self.next_id;
        self.next_id += 1;
        card.srs = SrsData::new();
        card.created_at = Utc::now();

        self.append_to_file(&card)?;
        self.cards.insert(card.id, card);

        Ok(self.next_id - 1)
    }

    /// 파일에 추가
    fn append_to_file(&self, card: &Card) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(LazarusError::Io)?;

        let json = serde_json::to_string(card)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;
        writeln!(file, "{}", json).map_err(LazarusError::Io)?;

        Ok(())
    }

    /// 전체 저장 (덮어쓰기)
    pub fn save_all(&self) -> Result<()> {
        let mut file = File::create(&self.file_path).map_err(LazarusError::Io)?;

        for card in self.cards.values() {
            let json = serde_json::to_string(card)
                .map_err(|e| LazarusError::Serialize(e.to_string()))?;
            writeln!(file, "{}", json).map_err(LazarusError::Io)?;
        }

        Ok(())
    }

    /// 카드 가져오기
    pub fn get_card(&self, id: u64) -> Option<&Card> {
        self.cards.get(&id)
    }

    /// 모든 카드
    pub fn all_cards(&self) -> Vec<&Card> {
        self.cards.values().collect()
    }

    /// 오늘 복습할 카드들
    pub fn due_cards(&self) -> Vec<&Card> {
        let now = Utc::now();
        self.cards
            .values()
            .filter(|c| {
                c.srs.next_review
                    .map(|r| r <= now)
                    .unwrap_or(true)
            })
            .collect()
    }

    /// 복습 결과 처리 (SM-2 알고리즘)
    pub fn review(&mut self, card_id: u64, result: ReviewResult) -> Result<()> {
        let card = self.cards.get_mut(&card_id)
            .ok_or_else(|| LazarusError::NotFound(format!("카드 ID: {}", card_id)))?;

        let srs = &mut card.srs;

        match result {
            ReviewResult::Again => {
                // 틀림: 간격 초기화
                srs.interval = 1;
                srs.streak = 0;
                srs.ease_factor = (srs.ease_factor - 0.2).max(1.3);
            }
            ReviewResult::Hard => {
                // 어려움: 간격 약간 증가
                srs.interval = ((srs.interval as f32) * 1.2).ceil() as u32;
                srs.interval = srs.interval.max(1);
                srs.streak += 1;
                srs.ease_factor = (srs.ease_factor - 0.15).max(1.3);
            }
            ReviewResult::Good => {
                // 맞음: 정상 간격 증가
                if srs.repetitions == 0 {
                    srs.interval = 1;
                } else if srs.repetitions == 1 {
                    srs.interval = 3;
                } else {
                    srs.interval = ((srs.interval as f32) * srs.ease_factor).ceil() as u32;
                }
                srs.streak += 1;
            }
            ReviewResult::Easy => {
                // 쉬움: 간격 크게 증가
                if srs.repetitions == 0 {
                    srs.interval = 4;
                } else {
                    srs.interval = ((srs.interval as f32) * srs.ease_factor * 1.3).ceil() as u32;
                }
                srs.streak += 1;
                srs.ease_factor += 0.15;
            }
        }

        srs.repetitions += 1;
        srs.next_review = Some(Utc::now() + Duration::days(srs.interval as i64));

        // 사용자 통계 업데이트
        self.user_stats.record_study();
        self.save_stats()?;
        
        // 파일 업데이트
        self.save_all()?;

        Ok(())
    }

    /// 카드 삭제
    pub fn delete_card(&mut self, id: u64) -> Result<bool> {
        if self.cards.remove(&id).is_some() {
            self.save_all()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 노트에서 생성된 카드들
    pub fn cards_by_note(&self, note_id: u64) -> Vec<&Card> {
        self.cards
            .values()
            .filter(|c| c.source_note_id == Some(note_id))
            .collect()
    }

    /// 통계
    pub fn stats(&self) -> SrsStats {
        let now = Utc::now();
        let total = self.cards.len();
        let due = self.due_cards().len();
        let new = self.cards.values().filter(|c| c.srs.repetitions == 0).count();
        let learning = self.cards.values().filter(|c| c.srs.repetitions > 0 && c.srs.interval < 7).count();
        let mature = self.cards.values().filter(|c| c.srs.interval >= 7).count();

        SrsStats {
            total,
            due,
            new,
            learning,
            mature,
        }
    }

/// 사용자 통계 로드
    fn load_stats(&mut self) {
        if let Ok(data) = std::fs::read_to_string(&self.stats_path) {
            if let Ok(stats) = serde_json::from_str(&data) {
                self.user_stats = stats;
            }
        }
    }
    
    /// 사용자 통계 저장
    pub fn save_stats(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.user_stats)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;
        std::fs::write(&self.stats_path, json).map_err(LazarusError::Io)?;
        Ok(())
    }
    
    /// 카드 수
    pub fn count(&self) -> usize {
        self.cards.len()
    }
}

/// SRS 통계
#[derive(Debug, Clone, Serialize)]
pub struct SrsStats {
    pub total: usize,
    pub due: usize,
    pub new: usize,
    pub learning: usize,
    pub mature: usize,
}

/// 사용자 통계 (게이미피케이션)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserStats {
    /// 연속 학습일
    pub streak: u32,
    /// 마지막 학습 날짜 (YYYY-MM-DD)
    pub last_study_date: Option<String>,
    /// 총 복습 횟수
    pub total_reviews: u64,
    /// 총 학습일
    pub total_days: u32,
}

impl UserStats {
    /// 오늘 학습 기록
    pub fn record_study(&mut self) {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        
        match &self.last_study_date {
            Some(last) if last == &today => {
                // 오늘 이미 학습함 - 스트릭 유지
            }
            Some(last) => {
                // 어제 학습했으면 스트릭 증가, 아니면 리셋
                let yesterday = (chrono::Utc::now() - chrono::Duration::days(1))
                    .format("%Y-%m-%d").to_string();
                
                if last == &yesterday {
                    self.streak += 1;
                } else {
                    self.streak = 1;
                }
                self.total_days += 1;
                self.last_study_date = Some(today);
            }
            None => {
                // 첫 학습
                self.streak = 1;
                self.total_days = 1;
                self.last_study_date = Some(today);
            }
        }
        
        self.total_reviews += 1;
    }
    
    /// 스트릭 이모지
    pub fn streak_emoji(&self) -> &'static str {
        match self.streak {
            0 => "",
            1..=2 => "🔥",
            3..=6 => "🔥🔥",
            7..=13 => "🔥🔥🔥",
            14..=29 => "⚡",
            30..=99 => "💎",
            _ => "👑",
        }
    }
}
