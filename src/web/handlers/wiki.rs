//! 위키 핸들러 (다중 ZIM 지원)
use std::path::PathBuf;

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;

use crate::error::{LazarusError, Result};
use crate::web::state::AppState;
use crate::i18n::all_translations;

#[derive(Deserialize)]
pub struct WikiSearchParams {
    pub q: Option<String>,
    pub zim: Option<String>,  // ZIM 선택
}

/// GET /wiki/search
pub async fn search_wiki(
    State(state): State<AppState>,
    Query(params): Query<WikiSearchParams>,
) -> Result<Response> {
    let zim_names = state.zim_names().await;
    
    if zim_names.is_empty() {
        return Ok(Html(render_no_zim()).into_response());
    }

    // 선택된 ZIM 또는 첫 번째
    let selected_name = params.zim.clone()
        .unwrap_or_else(|| zim_names.first().cloned().unwrap_or_default());

    let selected_zim = state.get_zim_by_name(&selected_name).await
        .or(state.get_zim().await)
        .ok_or_else(|| LazarusError::ZimNotFound { 
            title: "ZIM 파일이 로드되지 않음".to_string() 
        })?;

    let zim = selected_zim.read().await;
    let query = params.q.clone().unwrap_or_default();
    
    let results = if query.is_empty() {
        zim.list_articles(50)?
    } else {
        zim.search_fuzzy(&query, 50)?
    };

    let html = render_search_results(&query, &results, &zim_names, &selected_name);
    Ok(Html(html).into_response())
}

/// ZIM 없음 페이지
fn render_no_zim() -> String {
    r#"
<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <title>위키 - Lazarus</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <nav class="navbar">
        <a href="/" class="logo">📚 Lazarus</a>
        <div class="nav-links">
            <a href="/notes">노트</a>
            <a href="/search">검색</a>
        </div>
    </nav>
    <main class="container">
        <div class="empty-state">
            <h2>📖 ZIM 파일 없음</h2>
            <p>ZIM 파일을 로드하려면 서버 시작 시 --zim 옵션을 사용하세요:</p>
            <pre>lazarus --zim /path/to/wikipedia.zim</pre>
            <p>또는 여러 ZIM 파일:</p>
            <pre>lazarus --zim wiki1.zim --zim wiki2.zim</pre>
            <p>또는 디렉토리 지정:</p>
            <pre>lazarus --zim-dir /path/to/zims/</pre>
        </div>
    </main>
</body>
</html>
"#.to_string()
}

/// 검색 결과 HTML 렌더링
fn render_search_results(
    query: &str,
    results: &[crate::zim::DirEntry],
    zim_names: &[String],
    selected_zim: &str,
) -> String {
    let zim_selector = if zim_names.len() > 1 {
        let options: String = zim_names.iter().map(|name| {
            let selected = if name == selected_zim { " selected" } else { "" };
            format!(r#"<option value="{}"{}>{}</option>"#, name, selected, name)
        }).collect();

        format!(r#"
            <select id="zim-select" onchange="changeZim(this.value)" class="zim-selector">
                {}
            </select>
        "#, options)
    } else {
        String::new()
    };

    let results_html: String = results.iter().map(|entry| {
        let encoded_url = urlencoding::encode(&entry.url);
        format!(r#"
            <a href="/wiki/{}?zim={}" class="search-result">
                <div class="result-title">{}</div>
                <div class="result-url">{}</div>
            </a>
        "#, encoded_url, selected_zim, entry.title, entry.url)
    }).collect();

    let result_count = if query.is_empty() {
        "최근 문서".to_string()
    } else {
        format!("\"{}\" 검색 결과: {}건", query, results.len())
    };

    format!(r#"
<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <title>위키 검색 - Lazarus</title>
    <link rel="stylesheet" href="/static/style.css">
    <style>
        .wiki-header {{
            display: flex;
            gap: 1rem;
            align-items: center;
            margin-bottom: 1.5rem;
        }}
        .wiki-header h1 {{
            margin: 0;
            flex: 1;
        }}
        .zim-selector {{
            padding: 0.5rem 1rem;
            font-size: 1rem;
            border-radius: var(--radius);
            border: 1px solid var(--border);
            background: var(--bg-secondary);
            color: var(--text);
        }}
        .search-form {{
            display: flex;
            gap: 0.5rem;
            margin-bottom: 1.5rem;
        }}
        .search-form input {{
            flex: 1;
            padding: 0.75rem 1rem;
            font-size: 1rem;
            border-radius: var(--radius);
            border: 1px solid var(--border);
            background: var(--bg-secondary);
            color: var(--text);
        }}
        .search-result {{
            display: block;
            padding: 1rem;
            margin-bottom: 0.5rem;
            background: var(--bg-secondary);
            border-radius: var(--radius);
            text-decoration: none;
            color: var(--text);
            border: 1px solid var(--border);
        }}
        .search-result:hover {{
            border-color: var(--accent);
        }}
        .result-title {{
            font-weight: 600;
            margin-bottom: 0.25rem;
        }}
        .result-url {{
            font-size: 0.875rem;
            color: var(--text-secondary);
        }}
        .result-count {{
            color: var(--text-secondary);
            margin-bottom: 1rem;
        }}
    </style>
</head>
<body>
    <nav class="navbar">
        <a href="/" class="logo">📚 Lazarus</a>
        <div class="nav-links">
            <a href="/notes">노트</a>
            <a href="/search">검색</a>
            <a href="/wiki/" class="active">위키</a>
        </div>
    </nav>
    <main class="container">
        <div class="wiki-header">
            <h1>🔍 위키 검색</h1>
            {}
        </div>
        <form class="search-form" method="get" action="/wiki/search">
            <input type="hidden" name="zim" value="{}">
            <input type="text" name="q" value="{}" placeholder="검색어 입력..." autofocus>
            <button type="submit" class="btn btn-primary">검색</button>
        </form>
        <div class="result-count">{}</div>
        <div class="search-results">
            {}
        </div>
    </main>
    <script>
        function changeZim(name) {{
            const url = new URL(window.location);
            url.searchParams.set('zim', name);
            window.location = url;
        }}
    </script>
</body>
</html>
"#, zim_selector, selected_zim, query, result_count, results_html)
}

/// GET /wiki/*path
pub async fn get_article(
    State(state): State<AppState>,
    path: Option<Path<String>>,
    Query(params): Query<WikiSearchParams>,
) -> Result<Response> {
    let path_str = path.map(|p| p.0).unwrap_or_default();
    let zim_names = state.zim_names().await;

    if zim_names.is_empty() {
        return Ok(Html(render_no_zim()).into_response());
    }

    // 선택된 ZIM 또는 첫 번째
    let selected_name = params.zim.clone()
        .unwrap_or_else(|| zim_names.first().cloned().unwrap_or_default());

    let selected_zim = state.get_zim_by_name(&selected_name).await
        .or(state.get_zim().await)
        .ok_or_else(|| LazarusError::ZimNotFound { 
            title: "ZIM 파일이 로드되지 않음".to_string() 
        })?;

    let zim = selected_zim.read().await;

    // 경로 정리
    let url = path_str.trim_start_matches('/');

    // 빈 경로면 검색 페이지로 리다이렉트
    if url.is_empty() {
        return Ok(axum::response::Redirect::to(&format!("/wiki/search?zim={}", selected_name)).into_response());
    }

    // URL 디코딩
    let decoded_url = urlencoding::decode(url)
        .map(|s| s.to_string())
        .unwrap_or_else(|_| url.to_string());

    // 먼저 이진 탐색으로 빠르게 시도
    if let Some(content) = zim.get_content_fast(&decoded_url)? {
        let content_type = guess_mime(&decoded_url);
        // HTML이면 네비게이션 추가
        if content_type.contains("html") {
            let html = String::from_utf8_lossy(&content);
            let title = decoded_url.split('/').last().unwrap_or(&decoded_url);
            let wrapped = wrap_wiki_html(&html, title, &zim_names, &selected_name);
            return Ok(Html(wrapped).into_response());
        }
        return Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, content_type)],
            content,
        ).into_response());
    }

    // 느린 검색으로 폴백
    if let Some(entry) = zim.find_by_url('A', &decoded_url)? {
            if let Some(content) = zim.get_content('A', &decoded_url)? {
            let content_type = guess_mime(&decoded_url);

            // HTML이면 네비게이션 추가
            if content_type.contains("html") {
                let html = String::from_utf8_lossy(&content);
                let wrapped = wrap_wiki_html(&html, &entry.title, &zim_names, &selected_name);
                return Ok(Html(wrapped).into_response());
            }

            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type)],
                content,
            ).into_response());
        }
    }

    // 404
    Ok((
        StatusCode::NOT_FOUND,
        Html(format!(r#"
<!DOCTYPE html>
<html><head><title>404 - Lazarus</title><link rel="stylesheet" href="/static/style.css"></head>
<body>
    <nav class="navbar"><a href="/" class="logo">📚 Lazarus</a></nav>
    <main class="container">
        <h1>문서를 찾을 수 없음</h1>
        <p>요청한 문서가 없습니다: {}</p>
        <a href="/wiki/search?zim={}" class="btn btn-primary">검색으로 돌아가기</a>
    </main>
</body></html>
"#, decoded_url, selected_name)),
    ).into_response())
}

/// HTML 문서에 네비게이션 추가
fn wrap_wiki_html(content: &str, title: &str, zim_names: &[String], selected_zim: &str) -> String {
    let zim_selector = if zim_names.len() > 1 {
        let options: String = zim_names.iter().map(|name| {
            let selected = if name == selected_zim { " selected" } else { "" };
            format!(r#"<option value="{}"{}>{}</option>"#, name, selected, name)
        }).collect();
        format!(r#"
            <select onchange="changeZim(this.value)" class="zim-select">
                {}
            </select>
        "#, options)
    } else {
        String::new()
    };

    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - Lazarus Wiki</title>
    <link rel="stylesheet" href="/static/style.css">
    <style>
        :root {{
            --wiki-nav-height: 50px;
        }}
        .wiki-nav {{
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            height: var(--wiki-nav-height);
            background: var(--bg);
            padding: 0 1rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
            border-bottom: 1px solid var(--border);
            z-index: 1000;
        }}
        .wiki-nav-left {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }}
        .wiki-nav-center {{
            flex: 1;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 1rem;
        }}
        .wiki-nav-right {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }}
        .wiki-nav a, .wiki-nav button {{
            color: var(--text);
            text-decoration: none;
            padding: 0.4rem 0.75rem;
            border-radius: var(--radius);
            border: none;
            background: transparent;
            cursor: pointer;
            font-size: 0.9rem;
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }}
        .wiki-nav a:hover, .wiki-nav button:hover {{
            background: var(--bg-secondary);
        }}
        .wiki-nav .logo {{
            font-weight: bold;
            font-size: 1rem;
        }}
        .nav-btn {{
            background: var(--bg-secondary) !important;
            border: 1px solid var(--border) !important;
        }}
        .nav-btn:hover {{
            background: var(--accent) !important;
            color: white !important;
        }}
        .wiki-title {{
            color: var(--text-secondary);
            font-size: 0.85rem;
            max-width: 300px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }}
        .zim-select {{
            padding: 0.3rem 0.5rem;
            border-radius: var(--radius);
            border: 1px solid var(--border);
            background: var(--bg-secondary);
            color: var(--text);
            font-size: 0.85rem;
        }}
        .wiki-content {{
            margin-top: calc(var(--wiki-nav-height) + 1rem);
            padding: 1rem;
            max-width: 900px;
            margin-left: auto;
            margin-right: auto;
        }}
        .wiki-content img {{
            max-width: 100%;
            height: auto;
        }}
        .wiki-content a {{
            color: var(--accent);
        }}
        .wiki-content h1, .wiki-content h2, .wiki-content h3 {{
            margin-top: 1.5rem;
            margin-bottom: 0.5rem;
        }}
        .wiki-content p {{
            line-height: 1.7;
            margin-bottom: 1rem;
        }}
        .wiki-content table {{
            border-collapse: collapse;
            width: 100%;
            margin: 1rem 0;
        }}
        .wiki-content th, .wiki-content td {{
            border: 1px solid var(--border);
            padding: 0.5rem;
            text-align: left;
        }}
        .wiki-content th {{
            background: var(--bg-secondary);
        }}
        .wiki-content pre, .wiki-content code {{
            background: var(--bg-secondary);
            padding: 0.2rem 0.4rem;
            border-radius: 4px;
            font-family: monospace;
        }}
        .wiki-content pre {{
            padding: 1rem;
            overflow-x: auto;
        }}
        .wiki-content blockquote {{
            border-left: 3px solid var(--accent);
            margin: 1rem 0;
            padding-left: 1rem;
            color: var(--text-secondary);
        }}
        .breadcrumb {{
            font-size: 0.8rem;
            color: var(--text-secondary);
            margin-bottom: 1rem;
        }}
        .breadcrumb a {{
            color: var(--text-secondary);
        }}
        .breadcrumb a:hover {{
            color: var(--accent);
        }}
        /* 선택 팝업 */
        .selection-popup {{
            display: none;
            position: absolute;
            background: var(--bg);
            border: 1px solid var(--border);
            border-radius: var(--radius);
            padding: 0.5rem;
            box-shadow: 0 4px 12px rgba(0,0,0,0.3);
            z-index: 2000;
            gap: 0.25rem;
        }}
        .selection-popup.show {{
            display: flex;
        }}
        .selection-popup button {{
            padding: 0.4rem 0.6rem;
            border: none;
            background: var(--bg-secondary);
            color: var(--text);
            border-radius: 4px;
            cursor: pointer;
            font-size: 0.8rem;
            white-space: nowrap;
        }}
        .selection-popup button:hover {{
            background: var(--accent);
            color: white;
        }}
        /* 카드 생성 모달 */
        .modal-overlay {{
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0,0,0,0.6);
            z-index: 3000;
            align-items: center;
            justify-content: center;
        }}
        .modal-overlay.show {{
            display: flex;
        }}
        .modal {{
            background: var(--bg);
            border-radius: var(--radius-lg);
            padding: 1.5rem;
            width: 90%;
            max-width: 500px;
            max-height: 80vh;
            overflow-y: auto;
        }}
        .modal h3 {{
            margin: 0 0 1rem 0;
        }}
        .modal label {{
            display: block;
            margin-bottom: 0.5rem;
            font-weight: 500;
        }}
        .modal input, .modal textarea, .modal select {{
            width: 100%;
            padding: 0.5rem;
            border: 1px solid var(--border);
            border-radius: var(--radius);
            background: var(--bg-secondary);
            color: var(--text);
            margin-bottom: 1rem;
            font-family: inherit;
        }}
        .modal textarea {{
            min-height: 80px;
            resize: vertical;
        }}
        .modal-buttons {{
            display: flex;
            gap: 0.5rem;
            justify-content: flex-end;
        }}
        .modal-buttons button {{
            padding: 0.5rem 1rem;
            border: none;
            border-radius: var(--radius);
            cursor: pointer;
        }}
        .modal-buttons .btn-primary {{
            background: var(--accent);
            color: white;
        }}
        .modal-buttons .btn-secondary {{
            background: var(--bg-secondary);
            color: var(--text);
        }}
        /* 토스트 */
        .toast {{
            position: fixed;
            bottom: 2rem;
            left: 50%;
            transform: translateX(-50%);
            background: var(--accent);
            color: white;
            padding: 0.75rem 1.5rem;
            border-radius: var(--radius);
            z-index: 4000;
            display: none;
        }}
        .toast.show {{
            display: block;
            animation: fadeInOut 2s ease;
        }}
        @keyframes fadeInOut {{
            0% {{ opacity: 0; }}
            15% {{ opacity: 1; }}
            85% {{ opacity: 1; }}
            100% {{ opacity: 0; }}
        }}
        /* 스플릿뷰 */
        #split-container {{
            display: block;
        }}
        #editor-panel {{
            display: none;
            position: fixed;
            top: var(--wiki-nav-height);
            right: 0;
            bottom: 0;
            width: 50%;
            border-left: 2px solid var(--border);
            background: var(--bg);
            z-index: 900;
        }}
        #editor-frame {{
            width: 100%;
            height: 100%;
            border: none;
        }}
        #split-container.split-active #editor-panel {{
            display: block;
        }}
        #split-container.split-active .wiki-content {{
            width: 50%;
            margin-right: 50%;
        }}
        /* 모바일 */
        @media (max-width: 768px) {{
            .wiki-nav-center {{
                display: none;
            }}
            .wiki-title {{
                max-width: 150px;
            }}
        }}
    </style>
</head>
<body>
	<div id="split-container">
    <nav class="wiki-nav">
        <div class="wiki-nav-left">
            <a href="/" class="logo">📚</a>
            <button onclick="goBack()" class="nav-btn" title="Back">←</button>
            <button onclick="goForward()" class="nav-btn" title="Forward">→</button>
        </div>
        <div class="wiki-nav-center">
            <a href="/notes">📝 Notes</a>
            <a href="/search">🔍 Search</a>
            <a href="/wiki/search?zim={selected_zim}">📖 Wiki</a>
            <a href="/srs">🧠 SRS</a>
        </div>
        <div class="wiki-nav-right">
            <button id="split-btn" onclick="toggleSplitView()" class="nav-btn" title="New Note">📝+</button>
            {zim_selector}
            <span class="wiki-title" title="{title}">{title}</span>
        </div>
    </nav>

    <!-- 선택 팝업 -->
    <div id="selection-popup" class="selection-popup">
        <button onclick="createCard('basic')">📝 Basic</button>
        <button onclick="createCard('cloze')">📦 Cloze</button>
        <button onclick="createCard('definition')">📖 Definition</button>
    </div>

    <!-- 카드 생성 모달 -->
    <div id="card-modal" class="modal-overlay">
        <div class="modal">
            <h3>🧠 Create Flashcard</h3>
            <label>Type</label>
            <select id="card-type">
                <option value="basic">Basic (Q&A)</option>
                <option value="cloze">Cloze (Fill-in)</option>
                <option value="definition">Definition</option>
            </select>
            <label>Question</label>
            <textarea id="card-question" placeholder="Enter question..."></textarea>
            <label>Answer</label>
            <textarea id="card-answer" placeholder="Enter answer..."></textarea>
            <div class="modal-buttons">
                <button class="btn-secondary" onclick="closeCardModal()">Cancel</button>
                <button class="btn-primary" onclick="saveCard()">Create Card</button>
            </div>
        </div>
    </div>

    <!-- 토스트 -->
    <div id="toast" class="toast"></div>

    <main class="wiki-content">
        <div class="breadcrumb">
            <a href="/">Home</a> → <a href="/wiki/search?zim={selected_zim}">Wiki</a> → {title}
        </div>
        {content}
    </main>
    
    <!-- 스플릿뷰 에디터 -->
    <div id="editor-panel">
        <iframe id="editor-frame" src="about:blank"></iframe>
    </div>
    </div><!-- end split-container -->

    <script>
const wikiTitle = "{title}";
        const wikiUrl = window.location.href;
        let selectedText = '';
        let splitMode = false;

        // 네비게이션
        function goBack() {{ window.history.back(); }}
        function goForward() {{ window.history.forward(); }}
        function changeZim(name) {{ window.location = '/wiki/search?zim=' + name; }}

        // 스플릿뷰 토글
        function toggleSplitView() {{
            splitMode = !splitMode;
            const container = document.getElementById('split-container');
            const wikiContent = document.querySelector('.wiki-content');
            const splitBtn = document.getElementById('split-btn');
            
            if (splitMode) {{
                // 스플릿뷰 활성화
                container.classList.add('split-active');
                wikiContent.classList.add('split-wiki');
                splitBtn.textContent = '✕';
                splitBtn.title = 'Close editor';
                
                // 에디터 iframe 로드
                const editorFrame = document.getElementById('editor-frame');
                const initialContent = encodeURIComponent(`# ${{wikiTitle}}\n\nSource: [Wikipedia](${{wikiUrl}})\n\n`);
                editorFrame.src = `/notes/new?embed=1&title=${{encodeURIComponent(wikiTitle)}}&wiki_content=${{initialContent}}`;
            }} else {{
                // 스플릿뷰 비활성화
                container.classList.remove('split-active');
                wikiContent.classList.remove('split-wiki');
                splitBtn.textContent = '📝+';
                splitBtn.title = 'New Note';
            }}
        }}

        // 텍스트 선택 감지
        document.addEventListener('mouseup', (e) => {{
            const selection = window.getSelection();
            const text = selection.toString().trim();
            const popup = document.getElementById('selection-popup');
            
            if (text.length > 3 && text.length < 1000) {{
                selectedText = text;
                const range = selection.getRangeAt(0);
                const rect = range.getBoundingClientRect();
                popup.style.left = (rect.left + window.scrollX) + 'px';
                popup.style.top = (rect.bottom + window.scrollY + 5) + 'px';
                popup.classList.add('show');
            }} else {{
                popup.classList.remove('show');
            }}
        }});

        // 팝업 외부 클릭 시 닫기
        document.addEventListener('mousedown', (e) => {{
            const popup = document.getElementById('selection-popup');
            if (!popup.contains(e.target)) {{
                popup.classList.remove('show');
            }}
        }});

        // 카드 생성 모달 열기
        function createCard(type) {{
            document.getElementById('selection-popup').classList.remove('show');
            document.getElementById('card-type').value = type;
            
            const questionEl = document.getElementById('card-question');
            const answerEl = document.getElementById('card-answer');
            
            if (type === 'basic') {{
                questionEl.value = '';
                answerEl.value = selectedText;
            }} else if (type === 'cloze') {{
                questionEl.value = selectedText.replace(/(\S+)$/, '{{{{$1}}}}');
                answerEl.value = selectedText.match(/(\S+)$/)?.[1] || '';
            }} else if (type === 'definition') {{
                questionEl.value = wikiTitle;
                answerEl.value = selectedText;
            }}
            
            document.getElementById('card-modal').classList.add('show');
        }}

        function closeCardModal() {{
            document.getElementById('card-modal').classList.remove('show');
        }}

        // 카드 저장
        async function saveCard() {{
            const type = document.getElementById('card-type').value;
            const question = document.getElementById('card-question').value;
            const answer = document.getElementById('card-answer').value;
            
            if (!question || !answer) {{
                showToast('❌ Question and answer required');
                return;
            }}
            
            try {{
                const res = await fetch('/api/srs/cards', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{
                        question,
                        answer,
                        card_type: type,
                        source_wiki_url: wikiUrl
                    }})
                }});
                
                if (res.ok) {{
                    showToast('✅ Card created!');
                    closeCardModal();
                }} else {{
                    showToast('❌ Failed to create card');
                }}
            }} catch (err) {{
                showToast('❌ Error: ' + err.message);
            }}
        }}

        // 선택한 텍스트를 에디터에 추가
        function addToNote() {{
            document.getElementById('selection-popup').classList.remove('show');
            if (!splitMode) {{
                toggleSplitView();
            }}
            // 에디터에 텍스트 전달
            setTimeout(() => {{
                const frame = document.getElementById('editor-frame');
                frame.contentWindow.postMessage({{
                    type: 'appendText',
                    text: selectedText
                }}, '*');
            }}, 500);
        }}

        function showToast(msg) {{
            const toast = document.getElementById('toast');
            toast.textContent = msg;
            toast.classList.add('show');
            setTimeout(() => toast.classList.remove('show'), 2000);
        }}

        // 위키 내부 링크 수정
        document.querySelectorAll('a').forEach(a => {{
            const href = a.getAttribute('href');
            if (href && !href.startsWith('http') && !href.startsWith('/') && !href.startsWith('#')) {{
                a.href = '/wiki/' + href.replace(/^\.\.?\//g, '').replace(/^\/+/, '') + '?zim={selected_zim}';
            }}
        }});

        // 키보드 단축키
        document.addEventListener('keydown', (e) => {{
            if (e.altKey && e.key === 'ArrowLeft') goBack();
            if (e.altKey && e.key === 'ArrowRight') goForward();
            if (e.key === 'Escape') {{
                if (splitMode) {{
                    toggleSplitView();
                }} else {{
                    closeCardModal();
                }}
            }}
            if (e.ctrlKey && e.key === 'e') {{
                e.preventDefault();
                toggleSplitView();
            }}
        }});
    </script>
</body>
</html>
"#,
        title = title,
        selected_zim = selected_zim,
        zim_selector = zim_selector,
        content = content
    )
}

/// MIME 타입 추측
fn guess_mime(url: &str) -> &'static str {
    let lower = url.to_lowercase();
    if lower.ends_with(".html") || lower.ends_with(".htm") || !lower.contains('.') {
        "text/html; charset=utf-8"
    } else if lower.ends_with(".css") {
        "text/css"
    } else if lower.ends_with(".js") {
        "application/javascript"
    } else if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".json") {
        "application/json"
    } else {
        "application/octet-stream"
    }
}

/// GET /api/wiki/list - ZIM 목록 API
pub async fn list_zims(
    State(state): State<AppState>,
) -> Result<axum::Json<Vec<String>>> {
    Ok(axum::Json(state.zim_names().await))
}

/// POST /api/zim/reload - ZIM 디렉토리 새로고침
pub async fn reload_zims(
    State(state): State<AppState>,
) -> Result<axum::Json<ReloadResult>> {
    let added = state.reload_zims().await?;
    Ok(axum::Json(ReloadResult {
        success: true,
        added,
        total: state.zim_names().await.len(),
    }))
}

/// POST /api/zim/add - 경로로 ZIM 추가
pub async fn add_zim(
    State(state): State<AppState>,
    axum::Json(req): axum::Json<AddZimRequest>,
) -> Result<axum::Json<AddZimResult>> {
    let path = PathBuf::from(&req.path);

    // 경로 검증: path traversal 방지
    let canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            return Ok(axum::Json(AddZimResult {
                success: false,
                name: None,
                error: Some("Invalid path".to_string()),
            }));
        }
    };

    // .zim 확장자 확인
    if !canonical.to_string_lossy().to_lowercase().ends_with(".zim") {
        return Ok(axum::Json(AddZimResult {
            success: false,
            name: None,
            error: Some("Only .zim files allowed".to_string()),
        }));
    }

    // 민감한 경로 차단
    let path_str = canonical.to_string_lossy();
    let blocked = ["/etc", "/proc", "/sys", "/dev", "/root", "/boot"];
    for prefix in blocked {
        if path_str.starts_with(prefix) {
            return Ok(axum::Json(AddZimResult {
                success: false,
                name: None,
                error: Some("Access denied".to_string()),
            }));
        }
    }

    if !canonical.exists() {
        return Ok(axum::Json(AddZimResult {
            success: false,
            name: None,
            error: Some("File not found".to_string()),
        }));
    }

    match state.add_zim(canonical).await {
        Ok(name) => Ok(axum::Json(AddZimResult {
            success: true,
            name: Some(name),
            error: None,
        })),
        Err(e) => Ok(axum::Json(AddZimResult {
            success: false,
            name: None,
            error: Some(e.to_string()),
        })),
    }
}

/// DELETE /api/zim/:name - ZIM 제거
pub async fn remove_zim(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<axum::Json<RemoveResult>> {
    let removed = state.remove_zim(&name).await;
    Ok(axum::Json(RemoveResult {
        success: removed,
        name,
    }))
}

/// GET /api/zim/dir - ZIM 디렉토리 경로
pub async fn get_zim_dir(
    State(state): State<AppState>,
) -> Result<axum::Json<ZimDirResult>> {
    Ok(axum::Json(ZimDirResult {
        path: state.zim_dir.display().to_string(),
    }))
}

#[derive(serde::Serialize)]
pub struct ReloadResult {
    pub success: bool,
    pub added: Vec<String>,
    pub total: usize,
}

#[derive(serde::Deserialize)]
pub struct AddZimRequest {
    pub path: String,
}

#[derive(serde::Serialize)]
pub struct AddZimResult {
    pub success: bool,
    pub name: Option<String>,
    pub error: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RemoveResult {
    pub success: bool,
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct ZimDirResult {
    pub path: String,
}

/// GET /wiki/manage - ZIM 관리 페이지
pub async fn manage_zims(
    State(state): State<AppState>,
) -> Result<Html<String>> {
    let zim_list = state.zim_list().await;
    let zim_dir = state.zim_dir.display().to_string();
    let lang = state.get_lang().await;
    let t = all_translations(lang);

    let empty_text = t.get("wiki.no_zim_loaded").cloned().unwrap_or_default();
    let remove_text = t.get("wiki.remove").cloned().unwrap_or_default();

    let zim_rows: String = if zim_list.is_empty() {
        format!(r#"<tr><td colspan="3" class="empty">{}</td></tr>"#, empty_text)
    } else {
        zim_list.iter().map(|(name, path)| {
            format!(r#"
                <tr>
                    <td><strong>{}</strong></td>
                    <td class="path">{}</td>
                    <td>
                        <button class="btn btn-danger btn-sm" onclick="removeZim('{}')">{}</button>
                    </td>
                </tr>
            "#, name, path, name, remove_text)
        }).collect()
    };

    let html = format!(r#"
<!DOCTYPE html>
<html lang="{}">
<head>
    <meta charset="UTF-8">
    <title>{} - Lazarus</title>
    <link rel="stylesheet" href="/static/style.css">
    <style>
        .manage-container {{
            max-width: 900px;
            margin: 0 auto;
            padding: 2rem;
        }}
        .section {{
            background: var(--bg-secondary);
            border-radius: var(--radius);
            padding: 1.5rem;
            margin-bottom: 1.5rem;
            border: 1px solid var(--border);
        }}
        .section h2 {{
            margin-top: 0;
            margin-bottom: 1rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }}
        .zim-table {{
            width: 100%;
            border-collapse: collapse;
        }}
        .zim-table th, .zim-table td {{
            padding: 0.75rem;
            text-align: left;
            border-bottom: 1px solid var(--border);
        }}
        .zim-table th {{
            color: var(--text-secondary);
            font-weight: 600;
        }}
        .zim-table .path {{
            font-family: monospace;
            font-size: 0.875rem;
            color: var(--text-secondary);
            word-break: break-all;
        }}
        .zim-table .empty {{
            text-align: center;
            color: var(--text-secondary);
            padding: 2rem;
        }}
        .dir-path {{
            background: var(--bg);
            padding: 0.75rem 1rem;
            border-radius: var(--radius);
            font-family: monospace;
            margin-bottom: 1rem;
            border: 1px solid var(--border);
        }}
        .dir-hint {{
            color: var(--text-secondary);
            font-size: 0.875rem;
            margin-top: 0.5rem;
        }}
        .action-row {{
            display: flex;
            gap: 0.5rem;
            margin-top: 1rem;
        }}
        .add-form {{
            display: flex;
            gap: 0.5rem;
        }}
        .add-form input {{
            flex: 1;
            padding: 0.75rem 1rem;
            border-radius: var(--radius);
            border: 1px solid var(--border);
            background: var(--bg);
            color: var(--text);
            font-family: monospace;
        }}
        .btn-sm {{
            padding: 0.25rem 0.75rem;
            font-size: 0.875rem;
        }}
        .btn-danger {{
            background: #dc2626;
        }}
        .btn-danger:hover {{
            background: #b91c1c;
        }}
        .toast {{
            position: fixed;
            bottom: 2rem;
            right: 2rem;
            padding: 1rem 1.5rem;
            background: var(--accent);
            color: white;
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
        .toast.error {{
            background: #dc2626;
        }}
        .toast.success {{
            background: #16a34a;
        }}
        .stats {{
            display: flex;
            gap: 2rem;
            margin-bottom: 1rem;
        }}
        .stat {{
            text-align: center;
        }}
        .stat-value {{
            font-size: 2rem;
            font-weight: bold;
            color: var(--accent);
        }}
        .stat-label {{
            color: var(--text-secondary);
            font-size: 0.875rem;
        }}
        .lang-selector {{
            position: fixed;
            bottom: 1rem;
            right: 1rem;
        }}
    </style>
</head>
<body>
    <nav class="navbar">
        <a href="/" class="logo">📚 Lazarus</a>
        <div class="nav-links">
            <a href="/notes">{}</a>
            <a href="/search">{}</a>
            <a href="/wiki/search">{}</a>
        </div>
    </nav>
    <main class="manage-container">
        <h1>📖 {}</h1>
        <div class="section">
            <h2>📊 {}</h2>
            <div class="stats">
                <div class="stat">
                    <div class="stat-value" id="zim-count">{}</div>
                    <div class="stat-label">{}</div>
                </div>
            </div>
        </div>
        <div class="section">
            <h2>📂 {}</h2>
            <div class="dir-path">{}</div>
            <div class="dir-hint">
                💡 {}
            </div>
            <div class="action-row">
                <button class="btn btn-primary" onclick="reloadZims()">🔄 {}</button>
                <button class="btn btn-secondary" onclick="openDir()">📁 {}</button>
            </div>
        </div>
        <div class="section">
            <h2>➕ {}</h2>
            <div class="add-form">
                <input type="text" id="zim-path" placeholder="{}">
                <button class="btn btn-primary" onclick="addZim()">{}</button>
            </div>
            <div class="dir-hint">
                💡 {}
            </div>
        </div>
        <div class="section">
            <h2>📚 {}</h2>
            <table class="zim-table">
                <thead>
                    <tr>
                        <th>{}</th>
                        <th>{}</th>
                        <th>{}</th>
                    </tr>
                </thead>
                <tbody id="zim-list">
                    {}
                </tbody>
            </table>
        </div>
    </main>
    <section class="lang-selector">
        <form action="/api/lang" method="post" style="display: inline;">
            <button type="submit" name="lang" value="en" class="btn btn-sm {}">EN</button>
            <button type="submit" name="lang" value="ko" class="btn btn-sm {}">한국어</button>
        </form>
    </section>
    <div id="toast" class="toast"></div>
    <script>
        const t = {{
            zim_added: "{}",
            no_new_zim: "{}",
            refresh_failed: "{}",
            enter_path: "{}",
            add_failed: "{}",
            remove_confirm: "{}",
            zim_removed: "{}",
            remove_failed: "{}",
            error: "{}",
            open_folder: "{}"
        }};

        function showToast(message, type = 'success') {{
            const toast = document.getElementById('toast');
            toast.textContent = message;
            toast.className = 'toast ' + type + ' show';
            setTimeout(() => toast.classList.remove('show'), 3000);
        }}
        async function reloadZims() {{
            try {{
                const res = await fetch('/api/zim/reload', {{ method: 'POST' }});
                const data = await res.json();
                if (data.success) {{
                    if (data.added.length > 0) {{
                        showToast(data.added.length + t.zim_added + data.added.join(', '), 'success');
                    }} else {{
                        showToast(t.no_new_zim, 'success');
                    }}
                    setTimeout(() => location.reload(), 1000);
                }} else {{
                    showToast(t.refresh_failed, 'error');
                }}
            }} catch (e) {{
                showToast(t.error + e.message, 'error');
            }}
        }}
        async function addZim() {{
            const path = document.getElementById('zim-path').value.trim();
            if (!path) {{
                showToast(t.enter_path, 'error');
                return;
            }}
            try {{
                const res = await fetch('/api/zim/add', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{ path }})
                }});
                const data = await res.json();
                if (data.success) {{
                    showToast(t.zim_added + data.name, 'success');
                    setTimeout(() => location.reload(), 1000);
                }} else {{
                    showToast(data.error || t.add_failed, 'error');
                }}
            }} catch (e) {{
                showToast(t.error + e.message, 'error');
            }}
        }}
        async function removeZim(name) {{
            if (!confirm(t.remove_confirm + ' "' + name + '"?')) return;
            try {{
                const res = await fetch('/api/zim/' + encodeURIComponent(name), {{ method: 'DELETE' }});
                const data = await res.json();
                if (data.success) {{
                    showToast(t.zim_removed + name, 'success');
                    setTimeout(() => location.reload(), 1000);
                }} else {{
                    showToast(t.remove_failed, 'error');
                }}
            }} catch (e) {{
                showToast(t.error + e.message, 'error');
            }}
        }}
        function openDir() {{
            showToast(t.open_folder + '\\n' + '{}', 'success');
        }}
        document.getElementById('zim-path').addEventListener('keypress', (e) => {{
            if (e.key === 'Enter') addZim();
        }});
    </script>
</body>
</html>
"#,
        lang.code(),
        t.get("wiki.manage").cloned().unwrap_or_default(),
        t.get("nav.notes").cloned().unwrap_or_default(),
        t.get("nav.search").cloned().unwrap_or_default(),
        t.get("nav.wiki").cloned().unwrap_or_default(),
        t.get("wiki.manage").cloned().unwrap_or_default(),
        t.get("wiki.status").cloned().unwrap_or_default(),
        zim_list.len(),
        t.get("wiki.loaded_zims").cloned().unwrap_or_default(),
        t.get("wiki.directory").cloned().unwrap_or_default(),
        zim_dir,
        t.get("wiki.directory_hint").cloned().unwrap_or_default(),
        t.get("wiki.refresh").cloned().unwrap_or_default(),
        t.get("wiki.open_folder").cloned().unwrap_or_default(),
        t.get("wiki.add").cloned().unwrap_or_default(),
        t.get("wiki.add_placeholder").cloned().unwrap_or_default(),
        t.get("wiki.add_btn").cloned().unwrap_or_default(),
        t.get("wiki.add_hint").cloned().unwrap_or_default(),
        t.get("wiki.loaded_files").cloned().unwrap_or_default(),
        t.get("wiki.name").cloned().unwrap_or_default(),
        t.get("wiki.path").cloned().unwrap_or_default(),
        t.get("wiki.action").cloned().unwrap_or_default(),
        zim_rows,
        if lang.code() == "en" { "btn-primary" } else { "btn-secondary" },
        if lang.code() == "ko" { "btn-primary" } else { "btn-secondary" },
        // JS translations
        t.get("wiki.zim_added").cloned().unwrap_or_default(),
        t.get("wiki.no_new_zim").cloned().unwrap_or_default(),
        t.get("wiki.refresh_failed").cloned().unwrap_or_default(),
        t.get("wiki.enter_path").cloned().unwrap_or_default(),
        t.get("wiki.add_failed").cloned().unwrap_or_default(),
        t.get("wiki.remove_confirm").cloned().unwrap_or_default(),
        t.get("wiki.zim_removed").cloned().unwrap_or_default(),
        t.get("wiki.remove_failed").cloned().unwrap_or_default(),
        t.get("common.error").cloned().unwrap_or_default(),
        t.get("wiki.open_folder_msg").cloned().unwrap_or_default(),
        zim_dir
    );
    Ok(Html(html))
}
