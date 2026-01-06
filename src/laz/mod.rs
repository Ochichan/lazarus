//! .laz 패키지 포맷
//!
//! Lazarus 학습 패키지 - ZIP 기반 오프라인 교육 콘텐츠 포맷

mod reader;
mod writer;

pub use reader::VerifyResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// MIME 타입
pub const MIMETYPE: &str = "application/x-lazarus";

/// 패키지 메타데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMeta {
    /// 패키지 고유 ID
    pub uuid: String,
    /// 제목
    pub title: String,
    /// 작성자
    pub author: String,
    /// 버전 (증가하는 정수)
    pub version: u32,
    /// 생성 시간 (Unix timestamp)
    pub created_at: i64,
    /// 수정 시간 (Unix timestamp)
    pub updated_at: i64,
    /// 언어 코드 (ko, en, ar 등)
    pub language: String,
    /// 라이선스
    pub license: String,
    /// 의존성 (선수 과목)
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// 설명
    #[serde(default)]
    pub description: String,
}

impl Default for PackageMeta {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            title: String::new(),
            author: String::new(),
            version: 1,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
            language: "ko".to_string(),
            license: "CC-BY-SA".to_string(),
            dependencies: Vec::new(),
            description: String::new(),
        }
    }
}

/// 파일 무결성 매니페스트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// 파일별 SHA-256 해시
    pub files: HashMap<String, String>,
    /// 매니페스트 생성 시간
    pub generated_at: i64,
}

/// 커리큘럼 (학습 순서)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curriculum {
    /// 챕터 목록
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// 챕터 ID
    pub id: String,
    /// 챕터 제목
    pub title: String,
    /// 포함된 노트 ID 목록 (순서대로)
    pub notes: Vec<String>,
    /// 하위 챕터
    #[serde(default)]
    pub children: Vec<Chapter>,
}

/// SRS 플래시카드
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrsCard {
    /// 카드 ID
    pub id: String,
    /// 카드 타입
    pub card_type: CardType,
    /// 질문 (빈칸은 {{answer}} 형태)
    pub question: String,
    /// 정답
    pub answer: String,
    /// 원본 노트 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_note_id: Option<String>,
    /// 원본 블록 ID (Editor.js block)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_block_id: Option<String>,
    /// 힌트
    #[serde(default)]
    pub hints: Vec<String>,
    /// 태그
    #[serde(default)]
    pub tags: Vec<String>,
    /// SRS 데이터
    #[serde(default)]
    pub srs_data: SrsData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SrsData {
    /// 다음 복습 시간
    pub next_review: Option<i64>,
    /// 간격 (일)
    pub interval: u32,
    /// 난이도 계수
    pub ease_factor: f32,
    /// 복습 횟수
    pub repetitions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum CardType {
    /// 기본 Q&A
    #[default]
    Basic, Cloze, Multiplechoice, Image
}

/// 노트 콘텐츠
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteContent {
    /// 노트 ID
    pub id: String,
    /// 제목
    pub title: String,
    /// 본문 (마크다운 또는 Editor.js JSON)
    pub content: String,
    /// 태그
    #[serde(default)]
    pub tags: Vec<String>,
    /// 생성 시간
    pub created_at: i64,
    /// 수정 시간
    pub updated_at: i64,
}

/// 전체 .laz 패키지
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazPackage {
    pub meta: PackageMeta,
    pub manifest: Manifest,
    pub curriculum: Curriculum,
    pub srs: Vec<SrsCard>,
    pub content: HashMap<String, NoteContent>,
    /// 에셋 파일 경로 목록
    pub assets: Vec<String>,
}

impl LazPackage {
    /// 새 패키지 생성
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            meta: PackageMeta {
                title: title.to_string(),
                author: author.to_string(),
                ..Default::default()
            },
            manifest: Manifest {
                files: HashMap::new(),
                generated_at: chrono::Utc::now().timestamp(),
            },
            curriculum: Curriculum {
                chapters: Vec::new(),
            },
            srs: Vec::new(),
            content: HashMap::new(),
            assets: Vec::new(),
        }
    }

    /// 노트 추가
    pub fn add_note(&mut self, note: NoteContent) {
        self.content.insert(note.id.clone(), note);
    }

    /// SRS 카드 추가
    pub fn add_card(&mut self, card: SrsCard) {
        self.srs.push(card);
    }
}
