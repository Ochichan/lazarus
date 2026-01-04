//! 통합 에러 타입
//! 
//! 모든 모듈에서 사용하는 에러 타입을 정의합니다.

use std::io;
use thiserror::Error;

/// Lazarus 통합 에러 타입
#[derive(Error, Debug)]
pub enum LazarusError {
    // === 데이터베이스 에러 ===
    #[error("데이터베이스 초기화 실패: {0}")]
    DbInit(String),

    #[error("데이터 쓰기 실패: {0}")]
    DbWrite(String),

    #[error("데이터 읽기 실패: ID {id}")]
    DbRead { id: u64 },

    #[error("데이터 손상 감지: CRC 불일치 (예상: {expected:08x}, 실제: {actual:08x})")]
    DbCorruption { expected: u32, actual: u32 },

    #[error("복구 실패: {0}")]
    DbRecovery(String),

    // === 검색 에러 ===
    #[error("인덱스 생성 실패: {0}")]
    IndexCreate(String),

    #[error("검색 실패: {0}")]
    SearchFailed(String),

    // === ZIM 에러 ===
    #[error("ZIM 파일 열기 실패: {path}")]
    ZimOpen { path: String },

    #[error("ZIM 문서를 찾을 수 없음: {title}")]
    ZimNotFound { title: String },

    #[error("ZIM 압축 해제 실패")]
    ZimDecompress,

    // === 파일 시스템 에러 ===
    #[error("파일 I/O 에러: {0}")]
    Io(#[from] io::Error),

    #[error("경로가 존재하지 않음: {0}")]
    PathNotFound(String),

    #[error("권한 부족: {0}")]
    Permission(String),

    // === 직렬화 에러 ===
    #[error("직렬화 실패: {0}")]
    Serialize(String),

    #[error("역직렬화 실패: {0}")]
    Deserialize(String),

    #[error("YAML 파싱 실패: {0}")]
    YamlParse(#[from] serde_yaml::Error),

    #[error("JSON 파싱 실패: {0}")]
    JsonParse(#[from] serde_json::Error),

    // === 동기화 에러 ===
    #[error("USB 동기화 실패: {0}")]
    SyncFailed(String),

    #[error("버전 충돌: 로컬 {local}, 원격 {remote}")]
    SyncConflict { local: u64, remote: u64 },

    // === 암호화 에러 ===
    #[error("암호화 실패")]
    Encryption,

    #[error("복호화 실패: 잘못된 비밀번호")]
    Decryption,

    // === 웹 서버 에러 ===
    #[error("서버 시작 실패: {0}")]
    ServerStart(String),

    #[error("잘못된 요청: {0}")]
    BadRequest(String),

    #[error("리소스를 찾을 수 없음: {0}")]
    NotFound(String),

    // === 설정 에러 ===
    #[error("설정 파일 로드 실패: {0}")]
    ConfigLoad(String),

    #[error("잘못된 설정: {0}")]
    ConfigInvalid(String),
}

/// Result 타입 별칭
pub type Result<T> = std::result::Result<T, LazarusError>;

// === Tantivy 에러 변환 ===
impl From<tantivy::TantivyError> for LazarusError {
    fn from(e: tantivy::TantivyError) -> Self {
        LazarusError::SearchFailed(e.to_string())
    }
}

impl From<tantivy::query::QueryParserError> for LazarusError {
    fn from(e: tantivy::query::QueryParserError) -> Self {
        LazarusError::SearchFailed(e.to_string())
    }
}

// === HTTP 응답 변환 (Axum용) ===
impl axum::response::IntoResponse for LazarusError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        let (status, message) = match &self {
            LazarusError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            LazarusError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            LazarusError::Permission(_) => (StatusCode::FORBIDDEN, self.to_string()),
            LazarusError::Decryption => (StatusCode::UNAUTHORIZED, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        // 로깅
        tracing::error!("에러 응답: {} - {}", status, message);

        (status, message).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = LazarusError::DbCorruption {
            expected: 0x12345678,
            actual: 0xDEADBEEF,
        };
        assert!(err.to_string().contains("CRC 불일치"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let lazarus_err: LazarusError = io_err.into();
        assert!(matches!(lazarus_err, LazarusError::Io(_)));
    }
}
