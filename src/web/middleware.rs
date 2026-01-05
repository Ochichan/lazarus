//! 인증 미들웨어

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::web::state::AppState;

/// PIN 잠금 체크 미들웨어
/// 
/// PIN이 설정되어 있고 잠금 상태면 401 반환
/// 예외: /api/security/*, /static/*, / 
pub async fn require_unlock(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path();
    
    // 예외 경로 (인증 없이 허용)
    if is_public_path(path) {
        return next.run(request).await;
    }
    
    // PIN 설정 확인
    let security = state.security.read().await;
    if !security.pin_enabled {
        // PIN 없으면 통과
        return next.run(request).await;
    }
    drop(security);  // 락 해제
    
    // crypto가 있으면 잠금 해제된 상태
    let crypto = state.crypto.read().await;
    if crypto.is_some() {
        return next.run(request).await;
    }
    
    // 잠금 상태 → 401
    (
        StatusCode::UNAUTHORIZED,
        axum::Json(serde_json::json!({
            "success": false,
            "message": "PIN required. Please unlock first.",
            "locked": true
        }))
    ).into_response()
}

/// 인증 없이 접근 가능한 경로
fn is_public_path(path: &str) -> bool {
    // 보안 관련 (unlock/lock/status)
    if path.starts_with("/api/security") {
        return true;
    }
    // 정적 파일
    if path.starts_with("/static") {
        return true;
    }
    // 메인 페이지 (잠금 UI 표시용)
    if path == "/" {
        return true;
    }
    // 보안 페이지
    if path == "/security" {
        return true;
    }
    // favicon
    if path == "/favicon.ico" {
        return true;
    }
    
    false
}
