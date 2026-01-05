//! 애플리케이션 상태

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::i18n::{Lang, Translations, get_translations};
use crate::crypto::{SecurityConfig, CryptoManager};
use crate::zim::ZimReader;
use crate::db::{StorageEngine, BackupManager};
use crate::search::SearchEngine;
use crate::srs::SrsEngine;
use crate::error::Result;

/// ZIM 정보
pub struct ZimInfo {
    pub name: String,
    pub path: PathBuf,
    pub reader: Arc<RwLock<ZimReader>>,
}

/// 애플리케이션 상태
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<RwLock<StorageEngine>>,
    pub search: Arc<RwLock<SearchEngine>>,
    pub zims: Arc<RwLock<Vec<ZimInfo>>>,
    pub srs: Arc<RwLock<SrsEngine>>,
    pub data_dir: PathBuf,
    pub zim_dir: PathBuf,
    pub version: &'static str,
    pub backup: Arc<RwLock<BackupManager>>,
    pub security: Arc<RwLock<SecurityConfig>>,
    pub crypto: Arc<RwLock<Option<CryptoManager>>>,
    pub lang: Arc<RwLock<Lang>>,
}

impl AppState {
    /// 새 상태 생성
    pub async fn new(data_dir: PathBuf, zim_paths: Vec<PathBuf>) -> Result<Self> {
        let db_path = data_dir.join("notes.lazarus");
        let index_path = data_dir.join("index");
        let srs_path = data_dir.join("srs.jsonl");
        let backup_dir = data_dir.join("backups");
        let security_path = data_dir.join("security.json");
        let zim_dir = data_dir.join("zims");

        // ZIM 디렉토리 생성
        if !zim_dir.exists() {
            std::fs::create_dir_all(&zim_dir)?;
        }

        let db = StorageEngine::open(&db_path)?;
        let search = SearchEngine::open(&index_path)?;
        let srs = SrsEngine::open(&srs_path)?;
        let backup = BackupManager::new(&db_path, &backup_dir);
        // 보안 설정 로드
            let security = SecurityConfig::load(&security_path)?;
            tracing::info!("보안 설정: PIN {}", if security.pin_enabled { "활성화" } else { "비활성화" });
        // 시작 시 자동 백업
        if let Err(e) = backup.backup() {
            tracing::warn!("시작 시 백업 실패: {}", e);
        }

        // 여러 ZIM 파일 로드
        let mut zims = Vec::new();

        // CLI에서 지정한 ZIM 파일들
        for path in zim_paths {
            if let Ok(reader) = ZimReader::open(&path) {
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                tracing::info!("ZIM 로드 완료: {} ({})", name, path.display());
                zims.push(ZimInfo {
                    name,
                    path: path.clone(),
                    reader: Arc::new(RwLock::new(reader)),
                });
            }
        }

        // ZIM 디렉토리에서 추가 로드
        if let Ok(entries) = std::fs::read_dir(&zim_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "zim").unwrap_or(false) {
                    let name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    // 이미 로드된 ZIM인지 확인
                    if !zims.iter().any(|z| z.name == name) {
                        if let Ok(reader) = ZimReader::open(&path) {
                            tracing::info!("ZIM 디렉토리에서 로드: {} ({})", name, path.display());
                            zims.push(ZimInfo {
                                name,
                                path: path.clone(),
                                reader: Arc::new(RwLock::new(reader)),
                            });
                        }
                    }
                }
            }
        }

        if zims.is_empty() {
            tracing::info!("로드된 ZIM 파일 없음");
        } else {
            tracing::info!("총 {}개의 ZIM 파일 로드됨", zims.len());
        }

        tracing::info!("SRS 엔진 초기화: {}개의 카드", srs.count());

        Ok(Self {
                    lang: Arc::new(RwLock::new(Lang::En)),
                    db: Arc::new(RwLock::new(db)),
                    search: Arc::new(RwLock::new(search)),
                    zims: Arc::new(RwLock::new(zims)),
                    srs: Arc::new(RwLock::new(srs)),
                    data_dir,
                    zim_dir,
                    version: env!("CARGO_PKG_VERSION"),
                    backup: Arc::new(RwLock::new(backup)),
                    security: Arc::new(RwLock::new(security)),
                    crypto: Arc::new(RwLock::new(None)),  // PIN 입력 전까지 None
                })
    }
    /// 현재 언어 가져오기
        pub async fn get_lang(&self) -> Lang {
            *self.lang.read().await
        }
    
        /// 언어 설정
        pub async fn set_lang(&self, lang: Lang) {
            *self.lang.write().await = lang;
        }
    
        /// 번역 가져오기
        pub async fn translations(&self) -> Translations {
            get_translations(*self.lang.read().await)
        }
        
    /// 첫 번째 ZIM 리더 가져오기
    pub async fn get_zim(&self) -> Option<Arc<RwLock<ZimReader>>> {
        let zims = self.zims.read().await;
        zims.first().map(|z| z.reader.clone())
    }

    /// 이름으로 ZIM 리더 가져오기
    pub async fn get_zim_by_name(&self, name: &str) -> Option<Arc<RwLock<ZimReader>>> {
        let zims = self.zims.read().await;
        zims.iter()
            .find(|z| z.name == name)
            .map(|z| z.reader.clone())
    }

    /// 모든 ZIM 이름 목록
    pub async fn zim_names(&self) -> Vec<String> {
        let zims = self.zims.read().await;
        zims.iter().map(|z| z.name.clone()).collect()
    }

    /// ZIM 정보 목록
    pub async fn zim_list(&self) -> Vec<(String, String)> {
        let zims = self.zims.read().await;
        zims.iter().map(|z| (z.name.clone(), z.path.display().to_string())).collect()
    }

    /// ZIM 파일 동적 추가
    pub async fn add_zim(&self, path: PathBuf) -> Result<String> {
        let reader = ZimReader::open(&path)?;
        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut zims = self.zims.write().await;

        // 이미 있으면 제거 후 다시 추가
        zims.retain(|z| z.name != name);

        tracing::info!("ZIM 추가: {} ({})", name, path.display());
        zims.push(ZimInfo {
            name: name.clone(),
            path,
            reader: Arc::new(RwLock::new(reader)),
        });

        Ok(name)
    }

    /// ZIM 제거
    pub async fn remove_zim(&self, name: &str) -> bool {
        let mut zims = self.zims.write().await;
        let before = zims.len();
        zims.retain(|z| z.name != name);
        zims.len() < before
    }

    /// ZIM 디렉토리 새로고침
    pub async fn reload_zims(&self) -> Result<Vec<String>> {
        let mut added = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&self.zim_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "zim").unwrap_or(false) {
                    let name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    // 이미 로드된 ZIM인지 확인
                    let exists = {
                        let zims = self.zims.read().await;
                        zims.iter().any(|z| z.name == name)
                    };

                    if !exists {
                        if let Ok(reader) = ZimReader::open(&path) {
                            tracing::info!("ZIM 새로고침으로 로드: {}", name);
                            let mut zims = self.zims.write().await;
                            zims.push(ZimInfo {
                                name: name.clone(),
                                path: path.clone(),
                                reader: Arc::new(RwLock::new(reader)),
                            });
                            added.push(name);
                        }
                    }
                }
            }
        }

        Ok(added)
    }
}
