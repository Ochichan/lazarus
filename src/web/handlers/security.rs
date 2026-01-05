//! 보안 핸들러 (PIN 잠금)

use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::crypto::{CryptoManager, SecurityConfig};
use crate::error::Result;
use crate::web::state::AppState;
use crate::i18n::all_translations;

/// PIN 상태 응답
#[derive(Serialize)]
pub struct PinStatus {
    pub enabled: bool,
    pub locked: bool,
}

/// PIN 요청
#[derive(Deserialize)]
pub struct PinRequest {
    pub pin: String,
}

/// PIN 설정 요청
#[derive(Deserialize)]
pub struct SetPinRequest {
    pub current_pin: Option<String>,
    pub new_pin: String,
}

/// API 응답
#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

/// GET /api/security/status - PIN 상태 확인
pub async fn get_status(
    State(state): State<AppState>,
) -> Result<Json<PinStatus>> {
    let security = state.security.read().await;
    let crypto = state.crypto.read().await;
    
    Ok(Json(PinStatus {
        enabled: security.pin_enabled,
        locked: security.pin_enabled && crypto.is_none(),
    }))
}

/// POST /api/security/unlock - PIN으로 잠금 해제
pub async fn unlock(
    State(state): State<AppState>,
    Json(req): Json<PinRequest>,
) -> Result<Json<ApiResponse>> {
    let security = state.security.read().await;
    
    if !security.pin_enabled {
        return Ok(Json(ApiResponse {
            success: true,
            message: "PIN이 설정되지 않았습니다".to_string(),
        }));
    }
    
    // PIN 검증
    if !security.verify_pin(&req.pin)? {
        return Ok(Json(ApiResponse {
            success: false,
            message: "잘못된 PIN입니다".to_string(),
        }));
    }
    
    // CryptoManager 생성 및 저장
    if let Some(crypto) = security.get_crypto(&req.pin)? {
        // 백업에도 암호화 매니저 연결
        let mut backup = state.backup.write().await;
        backup.set_crypto(Some(crypto.clone()));
        
        let mut crypto_lock = state.crypto.write().await;
        *crypto_lock = Some(crypto);
    }
    
    Ok(Json(ApiResponse {
        success: true,
        message: "잠금이 해제되었습니다".to_string(),
    }))
}

/// POST /api/security/lock - 잠금
pub async fn lock(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse>> {
    let mut crypto = state.crypto.write().await;
    *crypto = None;
    
    // 백업 암호화도 해제
    let mut backup = state.backup.write().await;
    backup.set_crypto(None);
    
    Ok(Json(ApiResponse {
        success: true,
        message: "잠금되었습니다".to_string(),
    }))
}

/// POST /api/security/set-pin - PIN 설정/변경
pub async fn set_pin(
    State(state): State<AppState>,
    Json(req): Json<SetPinRequest>,
) -> Result<Json<ApiResponse>> {
    // PIN 유효성 검사 (6-32자리 영숫자)
    if req.new_pin.len() < 6 || req.new_pin.len() > 32 {
        return Ok(Json(ApiResponse {
            success: false,
            message: "PIN must be 6-32 characters".to_string(),
        }));
    }
    if !req.new_pin.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Ok(Json(ApiResponse {
            success: false,
            message: "PIN must be alphanumeric only".to_string(),
        }));
    }
    
    let mut security = state.security.write().await;
    
    // 기존 PIN이 있으면 검증
    if security.pin_enabled {
        match &req.current_pin {
            Some(current) => {
                if !security.verify_pin(current)? {
                    return Ok(Json(ApiResponse {
                        success: false,
                        message: "현재 PIN이 잘못되었습니다".to_string(),
                    }));
                }
            }
            None => {
                return Ok(Json(ApiResponse {
                    success: false,
                    message: "현재 PIN을 입력해주세요".to_string(),
                }));
            }
        }
    }
    
    // 새 PIN 설정
    security.set_pin(&req.new_pin)?;
    
    // 파일에 저장
    let security_path = state.data_dir.join("security.json");
    security.save(&security_path)?;
    
    // CryptoManager 업데이트
    if let Some(crypto) = security.get_crypto(&req.new_pin)? {
        let mut crypto_lock = state.crypto.write().await;
        *crypto_lock = Some(crypto);
    }
    
    Ok(Json(ApiResponse {
        success: true,
        message: "PIN이 설정되었습니다".to_string(),
    }))
}

/// POST /api/security/remove-pin - PIN 제거
pub async fn remove_pin(
    State(state): State<AppState>,
    Json(req): Json<PinRequest>,
) -> Result<Json<ApiResponse>> {
    let mut security = state.security.write().await;
    
    if !security.pin_enabled {
        return Ok(Json(ApiResponse {
            success: false,
            message: "PIN이 설정되지 않았습니다".to_string(),
        }));
    }
    
    // PIN 검증
    if !security.verify_pin(&req.pin)? {
        return Ok(Json(ApiResponse {
            success: false,
            message: "잘못된 PIN입니다".to_string(),
        }));
    }
    
    // PIN 제거
    security.remove_pin();
    
    // 파일에 저장
    let security_path = state.data_dir.join("security.json");
    security.save(&security_path)?;
    
    // CryptoManager 제거
    let mut crypto_lock = state.crypto.write().await;
    *crypto_lock = None;
    
    Ok(Json(ApiResponse {
        success: true,
        message: "PIN이 제거되었습니다".to_string(),
    }))
}

/// GET /security - 보안 설정 페이지
pub async fn security_page(
    State(state): State<AppState>,
) -> Result<Html<String>> {
    let security = state.security.read().await;
    let crypto = state.crypto.read().await;
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let pin_enabled = security.pin_enabled;
    let is_locked = pin_enabled && crypto.is_none();

    let status_icon = if is_locked { "🔒" } else if pin_enabled { "🔓" } else { "🔐" };
    let status_text = if is_locked { 
        t.get("security.locked").cloned().unwrap_or_default()
    } else if pin_enabled { 
        t.get("security.pin_enabled").cloned().unwrap_or_default()
    } else { 
        t.get("security.pin_not_set").cloned().unwrap_or_default()
    };

    let buttons = if is_locked {
        format!(r#"<button class="btn btn-primary" onclick="unlock()">🔓 {}</button>"#,
            t.get("security.unlock").cloned().unwrap_or_default())
    } else if pin_enabled {
        format!(r#"<button class="btn btn-secondary" onclick="lockNow()">🔒 {}</button>
               <button class="btn btn-danger" onclick="removePin()">{}</button>"#,
            t.get("security.lock").cloned().unwrap_or_default(),
            t.get("security.remove_pin").cloned().unwrap_or_default())
    } else {
        format!(r#"<button class="btn btn-primary" onclick="setPin()">{}</button>"#,
            t.get("security.set_pin").cloned().unwrap_or_default())
    };

    let html = format!(r#"
<!DOCTYPE html>
<html lang="{}">
<head>
    <meta charset="UTF-8">
    <title>{} - Lazarus</title>
    <link rel="stylesheet" href="/static/style.css">
    <style>
        .security-container {{
            max-width: 500px;
            margin: 2rem auto;
            padding: 2rem;
        }}
        .security-card {{
            background: var(--bg-secondary);
            border-radius: var(--radius);
            padding: 2rem;
            border: 1px solid var(--border);
        }}
        .pin-input {{
            display: flex;
            gap: 0.5rem;
            justify-content: center;
            margin: 1.5rem 0;
        }}
        .pin-digit {{
            width: 50px;
            height: 60px;
            font-size: 1.5rem;
            text-align: center;
            border: 2px solid var(--border);
            border-radius: var(--radius);
            background: var(--bg);
            color: var(--text);
        }}
        .pin-digit:focus {{
            border-color: var(--accent);
            outline: none;
        }}
        .status {{
            text-align: center;
            margin-bottom: 1.5rem;
        }}
        .status-icon {{
            font-size: 3rem;
            margin-bottom: 0.5rem;
        }}
        .btn-group {{
            display: flex;
            gap: 0.5rem;
            justify-content: center;
            margin-top: 1.5rem;
        }}
        .toast {{
            position: fixed;
            bottom: 2rem;
            right: 2rem;
            padding: 1rem 1.5rem;
            border-radius: var(--radius);
            opacity: 0;
            transform: translateY(20px);
            transition: all 0.3s;
            z-index: 1000;
        }}
        .toast.show {{
            opacity: 1;
            transform: translateY(0);
        }}
        .toast.success {{ background: #16a34a; color: white; }}
        .toast.error {{ background: #dc2626; color: white; }}
        .lang-selector {{
            position: fixed;
            bottom: 1rem;
            right: 1rem;
        }}
        .btn-sm {{
            padding: 0.25rem 0.75rem;
            font-size: 0.875rem;
        }}
    </style>
</head>
<body>
    <nav class="navbar">
        <a href="/" class="logo">📚 Lazarus</a>
        <div class="nav-links">
            <a href="/notes">{}</a>
            <a href="/search">{}</a>
            <a href="/wiki/">{}</a>
        </div>
    </nav>

    <main class="security-container">
        <h1>🔐 {}</h1>

        <div class="security-card">
            <div class="status">
                <div class="status-icon">{}</div>
                <div>{}</div>
            </div>

            <div id="pin-section">
                <label>{}</label>
                <div class="pin-input">
                    <input type="password" class="pin-digit" maxlength="1" data-index="0">
                    <input type="password" class="pin-digit" maxlength="1" data-index="1">
                    <input type="password" class="pin-digit" maxlength="1" data-index="2">
                    <input type="password" class="pin-digit" maxlength="1" data-index="3">
                    <input type="password" class="pin-digit" maxlength="1" data-index="4">
                    <input type="password" class="pin-digit" maxlength="1" data-index="5">
                </div>

                <div class="btn-group">
                    {}
                </div>
            </div>
        </div>
    </main>

    <section class="lang-selector">
        <form action="/api/lang" method="post" style="display: inline;">
            <button type="submit" name="lang" value="en" class="btn btn-sm {}"">EN</button>
            <button type="submit" name="lang" value="ko" class="btn btn-sm {}">한국어</button>
        </form>
    </section>

    <div id="toast" class="toast"></div>

    <script>
        const t = {{
            pin_min: "{}",
            enter_current_pin: "{}"
        }};
        const digits = document.querySelectorAll('.pin-digit');

        digits.forEach((digit, index) => {{
            digit.addEventListener('input', (e) => {{
                if (e.target.value && index < digits.length - 1) {{
                    digits[index + 1].focus();
                }}
            }});
            digit.addEventListener('keydown', (e) => {{
                if (e.key === 'Backspace' && !e.target.value && index > 0) {{
                    digits[index - 1].focus();
                }}
            }});
        }});

        function getPin() {{
            return Array.from(digits).map(d => d.value).join('');
        }}

        function clearPin() {{
            digits.forEach(d => d.value = '');
            digits[0].focus();
        }}

        function showToast(message, type = 'success') {{
            const toast = document.getElementById('toast');
            toast.textContent = message;
            toast.className = 'toast ' + type + ' show';
            setTimeout(() => toast.classList.remove('show'), 3000);
        }}

        async function unlock() {{
            const pin = getPin();
            if (pin.length < 4) {{
                showToast(t.pin_min, 'error');
                return;
            }}

            const res = await fetch('/api/security/unlock', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ pin }})
            }});
            const data = await res.json();

            if (data.success) {{
                showToast(data.message, 'success');
                setTimeout(() => location.href = '/', 1000);
            }} else {{
                showToast(data.message, 'error');
                clearPin();
            }}
        }}

        async function setPin() {{
            const pin = getPin();
            if (pin.length < 4) {{
                showToast(t.pin_min, 'error');
                return;
            }}

            const res = await fetch('/api/security/set-pin', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ new_pin: pin }})
            }});
            const data = await res.json();

            showToast(data.message, data.success ? 'success' : 'error');
            if (data.success) {{
                setTimeout(() => location.reload(), 1000);
            }}
        }}

        async function removePin() {{
            const pin = getPin();
            if (pin.length < 4) {{
                showToast(t.enter_current_pin, 'error');
                return;
            }}
            const res = await fetch('/api/security/remove-pin', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ pin }})
            }});
            const data = await res.json();
            showToast(data.message, data.success ? 'success' : 'error');
            if (data.success) {{
                setTimeout(() => location.reload(), 1000);
            }}
        }}

        async function lockNow() {{
            const res = await fetch('/api/security/lock', {{ method: 'POST' }});
            const data = await res.json();
            showToast(data.message, 'success');
            setTimeout(() => location.reload(), 1000);
        }}

        digits[0].focus();
    </script>
</body>
</html>
"#,
        lang.code(),
        t.get("security.title").cloned().unwrap_or_default(),
        t.get("nav.notes").cloned().unwrap_or_default(),
        t.get("nav.search").cloned().unwrap_or_default(),
        t.get("nav.wiki").cloned().unwrap_or_default(),
        t.get("security.title").cloned().unwrap_or_default(),
        status_icon,
        status_text,
        t.get("security.pin_input").cloned().unwrap_or_default(),
        buttons,
        if lang.code() == "en" { "btn-primary" } else { "btn-secondary" },
        if lang.code() == "ko" { "btn-primary" } else { "btn-secondary" },
        t.get("security.pin_min_length").cloned().unwrap_or_default(),
        t.get("security.enter_current_pin").cloned().unwrap_or_default(),
    );
    Ok(Html(html))
}
