//! USB 실시간 감시 (폴링 방식)
//!
//! notify crate 대신 단순 폴링 사용 (크로스 플랫폼 호환성)

use crate::sync::detect::{LazarusUsb, UsbDetector};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info, warn};

/// USB 이벤트
#[derive(Debug, Clone)]
pub enum UsbEvent {
    /// 새 USB 연결됨
    Connected(LazarusUsb),
    /// USB 연결 해제됨
    Disconnected(String), // USB 이름
}

/// USB 감시자
pub struct UsbWatcher {
    detector: Arc<UsbDetector>,
    /// 이벤트 브로드캐스터
    event_tx: broadcast::Sender<UsbEvent>,
    /// 실행 중 여부
    running: Arc<RwLock<bool>>,
    /// 폴링 간격 (초)
    poll_interval: Duration,
}

impl UsbWatcher {
    /// 새 감시자 생성
    pub fn new(poll_interval_secs: u64) -> Self {
        let (event_tx, _) = broadcast::channel(16);

        Self {
            detector: Arc::new(UsbDetector::new()),
            event_tx,
            running: Arc::new(RwLock::new(false)),
            poll_interval: Duration::from_secs(poll_interval_secs),
        }
    }

    /// 이벤트 구독
    pub fn subscribe(&self) -> broadcast::Receiver<UsbEvent> {
        self.event_tx.subscribe()
    }

    /// 감시 시작 (백그라운드 태스크)
    pub async fn start(&self) {
        // 이미 실행 중이면 무시
        {
            let mut running = self.running.write().await;
            if *running {
                warn!("USB 감시자 이미 실행 중");
                return;
            }
            *running = true;
        }

        info!("🔌 USB 감시 시작 ({}초 간격)", self.poll_interval.as_secs());

        // 초기 스캔
        let initial = self.detector.scan().await;
        for usb in initial {
            let _ = self.event_tx.send(UsbEvent::Connected(usb));
        }

        // 폴링 루프
        let detector = Arc::clone(&self.detector);
        let event_tx = self.event_tx.clone();
        let running = Arc::clone(&self.running);
        let interval = self.poll_interval;

        tokio::spawn(async move {
            let mut known_paths: std::collections::HashSet<std::path::PathBuf> =
                std::collections::HashSet::new();

            // 초기 경로 등록
            for usb in detector.get_detected().await {
                known_paths.insert(usb.path.clone());
            }

            loop {
                // 실행 상태 확인
                if !*running.read().await {
                    info!("🔌 USB 감시 중지됨");
                    break;
                }

                tokio::time::sleep(interval).await;

                // 현재 연결된 USB 스캔
                let current = detector.scan().await;
                let current_paths: std::collections::HashSet<_> =
                    current.iter().map(|u| u.path.clone()).collect();

                // 새로 연결된 USB
                for usb in &current {
                    if !known_paths.contains(&usb.path) {
                        info!("🔌 USB 연결됨: {}", usb.name);
                        let _ = event_tx.send(UsbEvent::Connected(usb.clone()));
                    }
                }

                // 연결 해제된 USB
                for path in &known_paths {
                    if !current_paths.contains(path) {
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("USB")
                            .to_string();
                        info!("🔌 USB 연결 해제됨: {}", name);
                        let _ = event_tx.send(UsbEvent::Disconnected(name));
                    }
                }

                known_paths = current_paths;
            }
        });
    }

    /// 감시 중지
    pub async fn stop(&self) {
        *self.running.write().await = false;
    }

    /// 현재 감지된 USB 목록
    pub async fn get_detected(&self) -> Vec<LazarusUsb> {
        self.detector.get_detected().await
    }

    /// 수동 스캔
    pub async fn scan_now(&self) -> Vec<LazarusUsb> {
        self.detector.scan().await
    }

    /// USB 초기화
    pub fn init_usb(path: &std::path::Path) -> std::io::Result<()> {
        UsbDetector::init_usb(path)
    }
}

impl Default for UsbWatcher {
    fn default() -> Self {
        Self::new(5) // 5초 간격
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_watcher_creation() {
        let watcher = UsbWatcher::new(1);
        let _rx = watcher.subscribe();

        // 감시 시작
        watcher.start().await;

        // 잠시 대기
        tokio::time::sleep(Duration::from_millis(100)).await;

        // 중지
        watcher.stop().await;
    }
}
