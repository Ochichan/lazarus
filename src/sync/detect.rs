//! USB 자동 감지
//!
//! 마운트 포인트 감시 + lazarus.sync 파일로 Lazarus USB 식별

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// USB 마운트 포인트 (OS별)
#[cfg(target_os = "linux")]
const MOUNT_POINTS: &[&str] = &["/media", "/mnt", "/run/media"];

#[cfg(target_os = "macos")]
const MOUNT_POINTS: &[&str] = &["/Volumes"];

#[cfg(target_os = "windows")]
const MOUNT_POINTS: &[&str] = &["D:\\", "E:\\", "F:\\", "G:\\", "H:\\"];

/// Lazarus USB 마커 파일
const LAZARUS_MARKER: &str = "lazarus.sync";

/// 감지된 USB 정보
#[derive(Debug, Clone)]
pub struct LazarusUsb {
    /// USB 경로
    pub path: PathBuf,
    /// USB 이름 (폴더명)
    pub name: String,
    /// 매니페스트 존재 여부
    pub has_manifest: bool,
    /// 노트 개수
    pub note_count: usize,
    /// 게시글 개수
    pub post_count: usize,
    /// Q&A 개수
    pub qna_count: usize,
    /// 패키지 개수
    pub package_count: usize,
}

impl LazarusUsb {
    /// USB 경로에서 정보 로드
    pub fn from_path(path: &Path) -> Option<Self> {
        // lazarus.sync 파일 확인
        let marker_path = path.join(LAZARUS_MARKER);
        if !marker_path.exists() {
            return None;
        }

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("USB")
            .to_string();

        // 각 폴더의 파일 개수 세기
        let note_count = count_files(&path.join("notes"), "laz");
        let post_count = count_lines(&path.join("bulletin/posts.jsonl"));
        let qna_count = count_lines(&path.join("qna/questions.jsonl"));
        let package_count = count_files(&path.join("packages"), "laz");

        let has_manifest = path.join("manifest.json").exists();

        Some(Self {
            path: path.to_path_buf(),
            name,
            has_manifest,
            note_count,
            post_count,
            qna_count,
            package_count,
        })
    }

    /// 총 콘텐츠 수
    pub fn total_content(&self) -> usize {
        self.note_count + self.post_count + self.qna_count + self.package_count
    }

    /// USB가 비어있는지
    pub fn is_empty(&self) -> bool {
        self.total_content() == 0
    }
}

/// 파일 개수 세기 (확장자 필터)
fn count_files(dir: &Path, extension: &str) -> usize {
    if !dir.exists() {
        return 0;
    }

    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext == extension)
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0)
}

/// JSONL 파일 줄 수 세기
fn count_lines(file: &Path) -> usize {
    if !file.exists() {
        return 0;
    }

    std::fs::read_to_string(file)
        .map(|content| content.lines().filter(|l| !l.trim().is_empty()).count())
        .unwrap_or(0)
}

/// USB 감지기
pub struct UsbDetector {
    /// 현재 감지된 Lazarus USB 목록
    detected: Arc<RwLock<Vec<LazarusUsb>>>,
    /// 이전에 본 경로 (중복 알림 방지)
    seen_paths: Arc<RwLock<HashSet<PathBuf>>>,
}

impl UsbDetector {
    /// 새 감지기 생성
    pub fn new() -> Self {
        Self {
            detected: Arc::new(RwLock::new(Vec::new())),
            seen_paths: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// 현재 연결된 Lazarus USB 스캔
    pub async fn scan(&self) -> Vec<LazarusUsb> {
        let mut found = Vec::new();

        for mount_point in MOUNT_POINTS {
            let mount_path = Path::new(mount_point);
            if !mount_path.exists() {
                continue;
            }

            // 마운트 포인트 하위 폴더 탐색
            if let Ok(entries) = std::fs::read_dir(mount_path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(usb) = LazarusUsb::from_path(&path) {
                            info!("🔌 Lazarus USB 발견: {} ({})", usb.name, path.display());
                            found.push(usb);
                        }
                    }
                }
            }

            // Linux /run/media/username 패턴 처리
            #[cfg(target_os = "linux")]
            if mount_point == &"/run/media" {
                if let Ok(users) = std::fs::read_dir(mount_path) {
                    for user in users.filter_map(|e| e.ok()) {
                        if let Ok(devices) = std::fs::read_dir(user.path()) {
                            for device in devices.filter_map(|e| e.ok()) {
                                let path = device.path();
                                if path.is_dir() {
                                    if let Some(usb) = LazarusUsb::from_path(&path) {
                                        info!(
                                            "🔌 Lazarus USB 발견: {} ({})",
                                            usb.name,
                                            path.display()
                                        );
                                        found.push(usb);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 감지 목록 업데이트
        *self.detected.write().await = found.clone();

        found
    }

    /// 새로 연결된 USB 확인 (이전에 안 본 것만)
    pub async fn scan_new(&self) -> Vec<LazarusUsb> {
        let all = self.scan().await;
        let mut seen = self.seen_paths.write().await;

        let new_usbs: Vec<LazarusUsb> = all
            .into_iter()
            .filter(|usb| !seen.contains(&usb.path))
            .collect();

        // seen에 추가
        for usb in &new_usbs {
            seen.insert(usb.path.clone());
        }

        new_usbs
    }

    /// 현재 감지된 USB 목록 반환
    pub async fn get_detected(&self) -> Vec<LazarusUsb> {
        self.detected.read().await.clone()
    }

    /// 특정 경로가 Lazarus USB인지 확인
    pub fn is_lazarus_usb(path: &Path) -> bool {
        path.join(LAZARUS_MARKER).exists()
    }

    /// USB 초기화 (lazarus.sync 생성)
    pub fn init_usb(path: &Path) -> std::io::Result<()> {
        // 마커 파일 생성
        let marker_path = path.join(LAZARUS_MARKER);
        std::fs::write(&marker_path, "# Lazarus USB\n# Do not delete this file\n")?;

        // 폴더 구조 생성
        std::fs::create_dir_all(path.join("notes"))?;
        std::fs::create_dir_all(path.join("bulletin"))?;
        std::fs::create_dir_all(path.join("qna"))?;
        std::fs::create_dir_all(path.join("packages"))?;

        // 빈 manifest.json 생성
        let manifest = serde_json::json!({
            "version": "0.4.0",
            "created_at": chrono::Utc::now().to_rfc3339(),
            "device_name": hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_else(|| "Unknown".to_string()),
        });
        std::fs::write(
            path.join("manifest.json"),
            serde_json::to_string_pretty(&manifest)?,
        )?;

        info!("✅ USB 초기화 완료: {}", path.display());
        Ok(())
    }
}

impl Default for UsbDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_lazarus_usb_detection() {
        let dir = tempdir().unwrap();
        let usb_path = dir.path();

        // 마커 없으면 None
        assert!(LazarusUsb::from_path(usb_path).is_none());

        // 마커 생성
        std::fs::write(usb_path.join(LAZARUS_MARKER), "").unwrap();

        // 이제 감지됨
        let usb = LazarusUsb::from_path(usb_path).unwrap();
        assert!(usb.is_empty());
    }

    #[test]
    fn test_init_usb() {
        let dir = tempdir().unwrap();
        let usb_path = dir.path();

        UsbDetector::init_usb(usb_path).unwrap();

        assert!(usb_path.join(LAZARUS_MARKER).exists());
        assert!(usb_path.join("notes").exists());
        assert!(usb_path.join("manifest.json").exists());
    }
}
