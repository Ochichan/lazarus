//! JSONL (JSON Lines) 유틸리티

use serde::{de::DeserializeOwned, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// JSONL 파일 읽기
pub fn read_jsonl<T: DeserializeOwned>(path: &Path) -> Result<Vec<T>, std::io::Error> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut items = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str(&line) {
            Ok(item) => items.push(item),
            Err(e) => {
                tracing::warn!("JSONL 파싱 오류 (스킵): {}", e);
            }
        }
    }

    Ok(items)
}

/// JSONL 파일에 추가
pub fn append_jsonl<T: Serialize>(path: &Path, item: &T) -> Result<(), std::io::Error> {
    // 부모 디렉토리 생성
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    let json = serde_json::to_string(item)?;
    writeln!(file, "{}", json)?;

    Ok(())
}

/// JSONL 파일 전체 쓰기 (덮어쓰기)
pub fn write_jsonl<T: Serialize>(path: &Path, items: &[T]) -> Result<(), std::io::Error> {
    // 부모 디렉토리 생성
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;

    for item in items {
        let json = serde_json::to_string(item)?;
        writeln!(file, "{}", json)?;
    }

    Ok(())
}

/// JSONL 항목 수 카운트 (전체 로드 없이)
pub fn count_jsonl(path: &Path) -> usize {
    if !path.exists() {
        return 0;
    }

    match File::open(path) {
        Ok(file) => BufReader::new(file)
            .lines()
            .filter(|l| l.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false))
            .count(),
        Err(_) => 0,
    }
}
