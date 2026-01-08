//! USB 동기화 모듈

pub mod detect;
pub mod jsonl;
pub mod manifest;
pub mod state;

pub use detect::{LazarusUsb, UsbDetector};
pub use jsonl::{read_jsonl, write_jsonl};
pub use manifest::UsbManifest;
pub use state::{
    export_notes, import_notes, sync_notes, sync_posts, sync_qna, SyncError, SyncResult,
};

/// 패키지 동기화
pub fn sync_packages(
    usb_path: &std::path::Path,
    local_packages: &[crate::db::PackageSummary],
    packages_dir: &std::path::Path,
) -> std::io::Result<(Vec<String>, usize, usize)> {
    use std::fs;

    let usb_packages_dir = usb_path.join("packages");
    fs::create_dir_all(&usb_packages_dir)?;

    let mut downloaded = Vec::new();
    let mut uploaded = 0;

    // USB → Local (다운로드)
    if usb_packages_dir.exists() {
        for entry in fs::read_dir(&usb_packages_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "laz").unwrap_or(false) {
                let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let local_path = packages_dir.join(filename);

                if !local_path.exists() {
                    fs::copy(&path, &local_path)?;
                    downloaded.push(filename.to_string());
                }
            }
        }
    }

    // Local → USB (업로드)
    for pkg in local_packages {
        let local_path = packages_dir.join(&pkg.filename);
        let usb_pkg_path = usb_packages_dir.join(&pkg.filename);

        if local_path.exists() && !usb_pkg_path.exists() {
            fs::copy(&local_path, &usb_pkg_path)?;
            uploaded += 1;
        }
    }

    Ok((downloaded, uploaded, 0))
}
