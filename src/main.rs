// Temporarily allow warnings for v0.2 release
// TODO: Clean up dead code in v0.3
#![allow(dead_code, unused_imports, unused_variables, unused_assignments)]

mod links;
mod crypto;
mod curriculum;
mod db;
mod error;
mod i18n;
mod laz;
mod search;
mod srs;
mod sync;
mod web;
mod zim;

use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::error::Result;

#[derive(Parser, Debug)]
#[command(name = "lazarus")]
#[command(about = "경량 개인 지식 관리 시스템")]
struct Args {
    /// HTTP 서버 포트
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// 데이터 저장 경로
    #[arg(short, long, default_value = "./data")]
    data: PathBuf,

    /// ZIM 파일 경로 (여러 개 지정 가능)
    #[arg(long)]
    zim: Vec<PathBuf>,

    /// ZIM 파일이 있는 디렉토리
    #[arg(long)]
    zim_dir: Option<PathBuf>,

    /// 바인드 주소
    #[arg(short, long, default_value = "127.0.0.1")]
    bind: String,

    /// 로그 레벨 (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 로깅 초기화
    init_logging(&args.log_level);

    tracing::info!("🚀 Lazarus 시작");
    tracing::info!("   데이터 경로: {}", args.data.display());
    tracing::info!("   서버 주소: {}:{}", args.bind, args.port);

    // 데이터 디렉토리 생성
    if !args.data.exists() {
        std::fs::create_dir_all(&args.data)?;
        tracing::info!("   데이터 디렉토리 생성됨");
    }

    // ZIM 파일 경로 수집
    let mut zim_paths = args.zim.clone();

    // --zim-dir 옵션이 있으면 해당 디렉토리의 모든 .zim 파일 추가
    if let Some(ref zim_dir) = args.zim_dir {
        if zim_dir.exists() && zim_dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(zim_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "zim").unwrap_or(false) {
                        tracing::info!("   ZIM 발견: {}", path.display());
                        zim_paths.push(path);
                    }
                }
            }
        }
    }

    let state = web::state::AppState::new(args.data.clone(), zim_paths).await?;

    // 링크 인덱스 빌드 (이거 추가!)
    state.build_link_index().await?;

    // 라우터 생성
    let app = web::router::create_router(state);

    // 서버 시작
    let addr = format!("{}:{}", args.bind, args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    tracing::info!("🌐 http://{} 에서 실행 중", addr);
    tracing::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 그레이스풀 셧다운
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| error::LazarusError::ServerStart(e.to_string()))?;

    tracing::info!("👋 Lazarus 종료");
    Ok(())
}

/// 로깅 초기화
fn init_logging(level: &str) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().compact())
        .init();
}

/// 셧다운 시그널 대기
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Ctrl+C 핸들러 설치 실패");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("SIGTERM 핸들러 설치 실패")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { tracing::info!("Ctrl+C 수신"); },
        _ = terminate => { tracing::info!("SIGTERM 수신"); },
    }
}
