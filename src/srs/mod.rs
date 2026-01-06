//! SRS (Spaced Repetition System) 엔진
//!
//! SM-2 알고리즘 기반 간격 반복 학습
pub mod extractor;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

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
#[derive(Default)]
pub enum CardType {
    /// 기본 Q&A
    #[default]
    Basic,
    Cloze,
    Multiplechoice,
    Image,
    Definition,
}

/// SRS 학습 데이터 (FSRS 알고리즘)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrsData {
    /// 다음 복습 시간
    pub next_review: Option<DateTime<Utc>>,
    /// 마지막 복습 시간
    #[serde(default)]
    pub last_review: Option<DateTime<Utc>>,
    /// 간격 (일)
    pub interval: u32,
    /// SM-2 호환 (레거시)
    pub ease_factor: f32,
    /// 복습 횟수
    pub repetitions: u32,
    /// 연속 정답 횟수
    pub streak: u32,
    /// FSRS: 안정성 (Stability) - 90% 기억 유지 기간 (일)
    #[serde(default = "default_stability")]
    pub stability: f32,
    /// FSRS: 난이도 (Difficulty) - 0.0 ~ 1.0
    #[serde(default = "default_difficulty")]
    pub difficulty: f32,
    /// FSRS: 학습 상태
    #[serde(default)]
    pub state: CardState,
}

fn default_stability() -> f32 {
    0.0
}
fn default_difficulty() -> f32 {
    0.3
}

/// 카드 학습 상태
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CardState {
    #[default]
    New,
    Learning,
    Review,
    Relearning,
}

impl Default for SrsData {
    fn default() -> Self {
        Self {
            next_review: Some(Utc::now()),
            last_review: None,
            interval: 0,
            ease_factor: 2.5,
            repetitions: 0,
            streak: 0,
            stability: 0.0,
            difficulty: 0.3,
            state: CardState::New,
        }
    }
}

impl SrsData {
    pub fn new() -> Self {
        Self::default()
    }
}

/// FSRS 파라미터 (기본값 = Anki 커뮤니티 최적화 값)
pub struct FsrsParams {
    pub w: [f32; 17],
}

impl Default for FsrsParams {
    fn default() -> Self {
        Self {
            // FSRS v4 기본 파라미터
            w: [
                0.4,  // w0: 초기 안정성 (Again)
                0.6,  // w1: 초기 안정성 (Hard)
                2.4,  // w2: 초기 안정성 (Good)
                5.8,  // w3: 초기 안정성 (Easy)
                4.93, // w4: 난이도 기본값
                0.94, // w5: 난이도 계수
                0.86, // w6: 난이도 변화율
                0.01, // w7: 난이도 평균 회귀
                1.49, // w8: 안정성 증가 기본
                0.14, // w9: 난이도 영향
                0.94, // w10: 검색가능성 영향
                2.18, // w11: Hard 패널티
                0.05, // w12: Easy 보너스
                0.34, // w13: 짧은 간격 패널티
                1.26, // w14: 긴 간격 패널티
                0.29, // w15: Hard 안정성 계수
                2.61, // w16: Easy 안정성 계수
            ],
        }
    }
}

impl FsrsParams {
    /// 검색가능성 계산 (Retrievability)
    /// t: 마지막 복습 이후 경과 일수
    /// s: 안정성
    pub fn retrievability(&self, t: f32, s: f32) -> f32 {
        if s <= 0.0 {
            return 0.0;
        }
        (1.0 + t / (9.0 * s)).powf(-1.0)
    }

    /// 초기 안정성 계산 (새 카드)
    pub fn initial_stability(&self, rating: u8) -> f32 {
        self.w[rating as usize]
    }

    /// 초기 난이도 계산
    pub fn initial_difficulty(&self, rating: u8) -> f32 {
        let d = self.w[4] - (rating as f32 - 3.0) * self.w[5];
        d.clamp(1.0, 10.0) / 10.0 // 0.0 ~ 1.0 정규화
    }

    /// 난이도 업데이트
    pub fn next_difficulty(&self, d: f32, rating: u8) -> f32 {
        let delta = self.w[6] * (rating as f32 - 3.0);
        let mean_reversion = self.w[7] * (self.w[4] / 10.0 - d);
        (d + delta + mean_reversion).clamp(0.0, 1.0)
    }

    /// 안정성 업데이트 (복습 후)
    pub fn next_stability(&self, s: f32, d: f32, r: f32, rating: u8) -> f32 {
        if rating == 0 {
            // Again: 안정성 리셋
            return self.w[0];
        }

        let hard_penalty = if rating == 1 { self.w[15] } else { 1.0 };
        let easy_bonus = if rating == 3 { self.w[16] } else { 1.0 };

        let new_s = s
            * (self.w[8].exp()
                * (d * 10.0 + 1.0).powf(-self.w[9])
                * ((self.w[10] * (1.0 - r)).exp() - 1.0)
                * hard_penalty
                * easy_bonus);

        new_s.max(0.1) // 최소 안정성
    }

    /// 다음 간격 계산 (목표 검색가능성 = 90%)
    pub fn next_interval(&self, s: f32) -> u32 {
        let target_r = 0.9; // 90% 기억 유지 목표
        let interval = 9.0 * s * (1.0 / target_r - 1.0);
        interval.round().max(1.0) as u32
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
    /// 복습 로그 (개인화용)
    review_logs: Vec<ReviewLog>,
    logs_path: String,
    /// 개인화된 파라미터 (None이면 기본값)
    pub custom_params: Option<[f32; 17]>,
    params_path: String,
}

impl SrsEngine {
    /// 새 엔진 생성 또는 파일에서 로드
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file_path = path.as_ref().to_string_lossy().to_string();
        let stats_path = file_path.replace(".jsonl", "_stats.json");
        let logs_path = file_path.replace(".jsonl", "_logs.jsonl");
        let params_path = file_path.replace(".jsonl", "_params.json");

        let mut engine = Self {
            cards: HashMap::new(),
            next_id: 1,
            file_path,
            user_stats: UserStats::default(),
            stats_path,
            review_logs: Vec::new(),
            logs_path,
            custom_params: None,
            params_path,
        };

        if path.as_ref().exists() {
            engine.load()?;
        }
        engine.load_stats();
        engine.load_logs();
        engine.load_params();

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

        let json =
            serde_json::to_string(card).map_err(|e| LazarusError::Serialize(e.to_string()))?;
        writeln!(file, "{}", json).map_err(LazarusError::Io)?;

        Ok(())
    }

    /// 전체 저장 (덮어쓰기)
    pub fn save_all(&self) -> Result<()> {
        let mut file = File::create(&self.file_path).map_err(LazarusError::Io)?;

        for card in self.cards.values() {
            let json =
                serde_json::to_string(card).map_err(|e| LazarusError::Serialize(e.to_string()))?;
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
            .filter(|c| c.srs.next_review.map(|r| r <= now).unwrap_or(true))
            .collect()
    }

    /// 복습 결과 처리 (FSRS 알고리즘)
    pub fn review(&mut self, card_id: u64, result: ReviewResult) -> Result<()> {
        let card = self
            .cards
            .get_mut(&card_id)
            .ok_or_else(|| LazarusError::NotFound(format!("카드 ID: {}", card_id)))?;

        let rating = match result {
            ReviewResult::Again => 0,
            ReviewResult::Hard => 1,
            ReviewResult::Good => 2,
            ReviewResult::Easy => 3,
        };
        let now = Utc::now();

        // 로그용 데이터 저장 (변경 전)
        let stability_before = card.srs.stability;
        let difficulty_before = card.srs.difficulty;
        let state_before = card.srs.state;
        let elapsed_days = card
            .srs
            .last_review
            .map(|lr| (now - lr).num_hours() as f32 / 24.0)
            .unwrap_or(0.0);

        let srs = &mut card.srs;
        let params = FsrsParams::default();

        match srs.state {
            CardState::New => {
                // 새 카드: 초기 안정성/난이도 설정
                srs.stability = params.initial_stability(rating);
                srs.difficulty = params.initial_difficulty(rating);
                srs.state = if rating == 0 {
                    CardState::Learning
                } else {
                    CardState::Review
                };
                if rating >= 2 {
                    srs.streak = 1;
                }
            }
            CardState::Learning | CardState::Relearning => {
                if rating >= 2 {
                    // Good/Easy: Review 상태로 전환
                    srs.stability = params.initial_stability(rating);
                    srs.state = CardState::Review;
                } else {
                    // Again/Hard: Learning 유지
                    srs.stability = params.w[0];
                }
                srs.difficulty = params.next_difficulty(srs.difficulty, rating);
            }
            CardState::Review => {
                // 검색가능성 계산
                let r = params.retrievability(elapsed_days, srs.stability);

                if rating == 0 {
                    // Again: Relearning으로
                    srs.stability = params.w[0];
                    srs.state = CardState::Relearning;
                    srs.streak = 0;
                } else {
                    // Hard/Good/Easy: 안정성 업데이트
                    srs.stability = params.next_stability(srs.stability, srs.difficulty, r, rating);
                    srs.streak += 1;
                }
                srs.difficulty = params.next_difficulty(srs.difficulty, rating);
            }
        }

        // 다음 간격 계산
        srs.interval = if srs.state == CardState::Learning || srs.state == CardState::Relearning {
            match rating {
                0 => 0, // 즉시 다시
                1 => 0, // 10분 후 (일 단위라 0)
                _ => 1, // 하루
            }
        } else {
            params.next_interval(srs.stability)
        };

        srs.next_review = Some(now + Duration::days(srs.interval as i64));
        srs.last_review = Some(now);
        srs.repetitions += 1;

        // SM-2 호환 (레거시)
        srs.ease_factor = 1.3 + srs.difficulty * 1.7; // 1.3 ~ 3.0 매핑

        // 복습 로그 기록 (FSRS 개인화용)
        let log = ReviewLog {
            card_id,
            timestamp: now,
            rating,
            stability_before,
            difficulty_before,
            elapsed_days,
            state: state_before,
        };
        self.append_log(log)?;

        // 통계 업데이트
        self.user_stats.record_study();
        self.save_stats()?;
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
        let new = self
            .cards
            .values()
            .filter(|c| c.srs.repetitions == 0)
            .count();
        let learning = self
            .cards
            .values()
            .filter(|c| c.srs.repetitions > 0 && c.srs.interval < 7)
            .count();
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

    /// 복습 로그 로드
    fn load_logs(&mut self) {
        if let Ok(data) = std::fs::read_to_string(&self.logs_path) {
            for line in data.lines() {
                if let Ok(log) = serde_json::from_str::<ReviewLog>(line) {
                    self.review_logs.push(log);
                }
            }
            tracing::info!("FSRS 로그: {}개 로드됨", self.review_logs.len());
        }
    }

    /// 복습 로그 추가
    fn append_log(&mut self, log: ReviewLog) -> Result<()> {
        // 메모리에 추가
        self.review_logs.push(log.clone());

        // 파일에 추가
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.logs_path)
            .map_err(LazarusError::Io)?;

        let json =
            serde_json::to_string(&log).map_err(|e| LazarusError::Serialize(e.to_string()))?;
        writeln!(file, "{}", json).map_err(LazarusError::Io)?;

        Ok(())
    }

    /// 개인화 파라미터 로드
    fn load_params(&mut self) {
        if let Ok(data) = std::fs::read_to_string(&self.params_path) {
            if let Ok(params) = serde_json::from_str::<[f32; 17]>(&data) {
                self.custom_params = Some(params);
                tracing::info!("FSRS 개인화 파라미터 로드됨");
            }
        }
    }

    /// 개인화 파라미터 저장
    pub fn save_params(&self) -> Result<()> {
        if let Some(params) = &self.custom_params {
            let json = serde_json::to_string_pretty(params)
                .map_err(|e| LazarusError::Serialize(e.to_string()))?;
            std::fs::write(&self.params_path, json).map_err(LazarusError::Io)?;
        }
        Ok(())
    }

    /// 복습 로그 수
    pub fn log_count(&self) -> usize {
        self.review_logs.len()
    }

    /// FSRS 파라미터 최적화 (경사하강법)
    pub fn optimize_params(&mut self) -> Result<OptimizationResult> {
        let logs = &self.review_logs;

        if logs.len() < 100 {
            return Err(LazarusError::NotFound(format!(
                "최소 100개의 복습 기록 필요 (현재: {}개)",
                logs.len()
            )));
        }

        // 기본 파라미터로 시작
        let mut params = FsrsParams::default().w;
        let learning_rate = 0.01;
        let iterations = 100;

        for _ in 0..iterations {
            let mut gradients = [0.0f32; 17];
            let mut total_loss = 0.0f32;
            let mut count = 0;

            for log in logs.iter() {
                // Review 상태의 로그만 사용 (학습 데이터로 의미 있음)
                if log.state != CardState::Review {
                    continue;
                }
                if log.stability_before <= 0.0 {
                    continue;
                }

                // 예측 검색가능성
                let predicted_r =
                    (1.0 + log.elapsed_days / (9.0 * log.stability_before)).powf(-1.0);

                // 실제 결과 (Again=0, 나머지=1)
                let actual = if log.rating == 0 { 0.0 } else { 1.0 };

                // 손실 (Binary Cross Entropy 근사)
                let error = predicted_r - actual;
                total_loss += error * error;
                count += 1;

                // w8 (안정성 증가 기본) 그래디언트
                gradients[8] += error * 0.1;
                // w9 (난이도 영향) 그래디언트
                gradients[9] += error * log.difficulty_before * 0.1;
                // w10 (검색가능성 영향) 그래디언트
                gradients[10] += error * (1.0 - predicted_r) * 0.1;
            }

            if count == 0 {
                break;
            }

            // 파라미터 업데이트 (경사하강)
            for i in 0..17 {
                params[i] -= learning_rate * gradients[i] / count as f32;
                // 범위 제한
                params[i] = params[i].clamp(0.01, 10.0);
            }
        }

        // RMSE 계산
        let rmse = self.calculate_rmse(&params);

        // 예상 기억률 계산
        let predicted_retention = self.calculate_retention(&params);

        // 저장
        self.custom_params = Some(params);
        self.save_params()?;

        Ok(OptimizationResult {
            params,
            log_count: logs.len(),
            rmse,
            predicted_retention,
        })
    }

    /// RMSE 계산
    fn calculate_rmse(&self, params: &[f32; 17]) -> f32 {
        let mut sum_sq = 0.0f32;
        let mut count = 0;

        for log in &self.review_logs {
            if log.state != CardState::Review || log.stability_before <= 0.0 {
                continue;
            }

            let predicted = (1.0 + log.elapsed_days / (9.0 * log.stability_before)).powf(-1.0);
            let actual = if log.rating == 0 { 0.0 } else { 1.0 };
            sum_sq += (predicted - actual).powi(2);
            count += 1;
        }

        if count > 0 {
            (sum_sq / count as f32).sqrt()
        } else {
            0.0
        }
    }

    /// 평균 예상 기억률
    fn calculate_retention(&self, _params: &[f32; 17]) -> f32 {
        let mut sum = 0.0f32;
        let mut count = 0;

        for card in self.cards.values() {
            if card.srs.stability > 0.0 {
                let elapsed = card
                    .srs
                    .last_review
                    .map(|lr| (Utc::now() - lr).num_hours() as f32 / 24.0)
                    .unwrap_or(0.0);
                let r = (1.0 + elapsed / (9.0 * card.srs.stability)).powf(-1.0);
                sum += r;
                count += 1;
            }
        }

        if count > 0 {
            sum / count as f32
        } else {
            0.9
        }
    }

    /// 현재 사용 중인 파라미터
    pub fn current_params(&self) -> FsrsParams {
        match &self.custom_params {
            Some(w) => FsrsParams { w: *w },
            None => FsrsParams::default(),
        }
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

/// 복습 로그 (FSRS 개인화용)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewLog {
    /// 카드 ID
    pub card_id: u64,
    /// 복습 시간
    pub timestamp: DateTime<Utc>,
    /// 응답 (0=Again, 1=Hard, 2=Good, 3=Easy)
    pub rating: u8,
    /// 복습 전 안정성
    pub stability_before: f32,
    /// 복습 전 난이도
    pub difficulty_before: f32,
    /// 마지막 복습 이후 경과 일수
    pub elapsed_days: f32,
    /// 복습 전 상태
    pub state: CardState,
}

/// FSRS 최적화 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// 최적화된 파라미터
    pub params: [f32; 17],
    /// 총 로그 수
    pub log_count: usize,
    /// RMSE (낮을수록 좋음)
    pub rmse: f32,
    /// 예상 기억률
    pub predicted_retention: f32,
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
                    .format("%Y-%m-%d")
                    .to_string();

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn make_test_engine() -> SrsEngine {
        let dir = tempdir().unwrap();
        let srs_path = dir.path().join("test_srs.jsonl");
        // dir을 leak해서 테스트 동안 유지
        let path_str = srs_path.to_str().unwrap().to_string();
        std::mem::forget(dir);
        SrsEngine::open(&path_str).unwrap()
    }

    fn make_card(question: &str, answer: &str, card_type: CardType) -> Card {
        Card {
            id: 0,
            card_type,
            question: question.to_string(),
            answer: answer.to_string(),
            source_note_id: None,
            source_wiki_url: None,
            hints: vec![],
            tags: vec![],
            srs: SrsData::new(),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_add_card() {
        let mut engine = make_test_engine();

        let card = make_card("What is 2+2?", "4", CardType::Basic);
        let id = engine.add_card(card).unwrap();
        assert!(id > 0);

        let card = engine.get_card(id).unwrap();
        assert_eq!(card.question, "What is 2+2?");
        assert_eq!(card.answer, "4");
    }

    #[test]
    fn test_review_good() {
        let mut engine = make_test_engine();

        let card = make_card("Q?", "A!", CardType::Basic);
        let id = engine.add_card(card).unwrap();

        // 초기 상태
        let card = engine.get_card(id).unwrap();
        assert_eq!(card.srs.repetitions, 0);

        // Good 복습
        engine.review(id, ReviewResult::Good).unwrap();

        let card = engine.get_card(id).unwrap();
        assert_eq!(card.srs.repetitions, 1);
        assert!(card.srs.interval >= 1);
    }

    #[test]
    fn test_review_again_resets() {
        let mut engine = make_test_engine();
        let card = make_card("Q?", "A!", CardType::Basic);
        let id = engine.add_card(card).unwrap();

        // Easy 복습으로 Review 상태로
        engine.review(id, ReviewResult::Easy).unwrap();

        let card = engine.get_card(id).unwrap();
        assert_eq!(card.srs.state, CardState::Review);
        assert!(card.srs.streak > 0);

        // Again → Relearning, streak 리셋
        engine.review(id, ReviewResult::Again).unwrap();

        let card = engine.get_card(id).unwrap();
        assert_eq!(card.srs.streak, 0);
        assert_eq!(card.srs.state, CardState::Relearning);
    }

    #[test]
    fn test_ease_factor_bounds() {
        let mut engine = make_test_engine();

        let card = make_card("Q?", "A!", CardType::Basic);
        let id = engine.add_card(card).unwrap();

        // 계속 Hard → ease_factor 감소
        for _ in 0..20 {
            engine.review(id, ReviewResult::Hard).unwrap();
        }

        let card = engine.get_card(id).unwrap();
        // ease_factor는 1.3 아래로 안 내려감
        assert!(card.srs.ease_factor >= 1.3);
    }

    #[test]
    fn test_card_types() {
        let mut engine = make_test_engine();

        let id1 = engine
            .add_card(make_card("Q", "A", CardType::Basic))
            .unwrap();
        let id2 = engine
            .add_card(make_card("Q", "A", CardType::Cloze))
            .unwrap();
        let id3 = engine
            .add_card(make_card("Q", "A", CardType::Definition))
            .unwrap();

        assert_eq!(engine.get_card(id1).unwrap().card_type, CardType::Basic);
        assert_eq!(engine.get_card(id2).unwrap().card_type, CardType::Cloze);
        assert_eq!(
            engine.get_card(id3).unwrap().card_type,
            CardType::Definition
        );
    }

    #[test]
    fn test_streak_emoji() {
        let mut stats = UserStats::default();

        assert_eq!(stats.streak_emoji(), "");

        stats.streak = 1;
        assert_eq!(stats.streak_emoji(), "🔥");

        stats.streak = 7;
        assert_eq!(stats.streak_emoji(), "🔥🔥🔥");

        stats.streak = 30;
        assert_eq!(stats.streak_emoji(), "💎");

        stats.streak = 100;
        assert_eq!(stats.streak_emoji(), "👑");
    }

    #[test]
    fn test_delete_card() {
        let mut engine = make_test_engine();

        let card = make_card("Q", "A", CardType::Basic);
        let id = engine.add_card(card).unwrap();
        assert!(engine.get_card(id).is_some());

        engine.delete_card(id).unwrap();
        assert!(engine.get_card(id).is_none());
    }
}
