//! 스토리지 엔진
//!
//! LazarusDB의 핵심 - WAL 기반 append-only 저장소

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::io::{BufRead, BufReader, Write};
#[cfg(unix)]
use std::os::unix::fs::FileExt;

use super::note::{Note, NoteAtom};
use super::wal::{WalReader, WalWriter, ENTRY_HEADER_SIZE};
use crate::error::{LazarusError, Result};
use crate::crypto::CryptoManager;
/// 버퍼 크기 (4KB)
const BUFFER_SIZE: usize = 4096;

/// 스토리지 엔진
pub struct StorageEngine {
    /// 데이터 파일 경로
    path: PathBuf,
    
    /// WAL 라이터
    writer: WalWriter,
    
    /// 읽기 전용 파일 핸들
    read_handle: std::fs::File,
    
    /// ID -> (헤더 오프셋) 인덱스
    index: HashMap<u64, u64>,
    
    /// 벡터 캐시 (검색용)
    vector_cache: Vec<(u64, Vec<i8>)>,
    
    /// 다음 ID
    next_id: AtomicU64,
}

impl StorageEngine {
    /// 새 엔진 생성 또는 기존 데이터 로드
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let path_str = path.to_str().ok_or_else(|| {
            LazarusError::DbInit("잘못된 경로".to_string())
        })?;

        // 디렉토리 생성
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let writer = WalWriter::open(path_str, BUFFER_SIZE)?;
        let read_handle = std::fs::File::open(&path)?;

        let mut engine = Self {
            path: path.clone(),
            writer,
            read_handle,
            index: HashMap::new(),
            vector_cache: Vec::new(),
            next_id: AtomicU64::new(1),
        };

        // 기존 데이터 복구
        engine.recover()?;

        tracing::info!(
            "StorageEngine 초기화 완료: {} 개의 노트 로드됨",
            engine.index.len()
        );

        Ok(engine)
    }

    /// 기존 데이터에서 인덱스 복구
    fn recover(&mut self) -> Result<()> {
        let path_str = self.path.to_str().ok_or_else(|| {
            LazarusError::DbRecovery("잘못된 경로".to_string())
        })?;

        let mut reader = match WalReader::open(path_str) {
            Ok(r) => r,
            Err(LazarusError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(());
            }
            Err(e) => return Err(e),
        };

        let mut max_id = 0u64;
        let mut recovered = 0usize;
        let mut corrupted = 0usize;

        loop {
            match reader.next_entry() {
                Ok(Some((entry_offset, entry))) => {
                    // rkyv 역직렬화 시도
                    let archived = match rkyv::check_archived_root::<NoteAtom>(&entry.data) {
                        Ok(a) => a,
                        Err(_) => continue, // 손상된 엔트리 스킵
                    };
                    let id = archived.id;
                    
                    // 삭제된 노트는 인덱스에서 제외
                    if archived.deleted {
                        self.index.remove(&id);
                        self.vector_cache.retain(|(vid, _)| *vid != id);
                    } else {
                        // 헤더 오프셋 저장 (데이터 읽을 때 여기서부터 읽음)
                        self.index.insert(id, entry_offset);
                        
                        // 벡터 캐시 업데이트
                        if let rkyv::option::ArchivedOption::Some(ref vec) = archived.vector {
                            let vec_copy: Vec<i8> = vec.iter().copied().collect();
                            self.vector_cache.retain(|(vid, _)| *vid != id);
                            self.vector_cache.push((id, vec_copy));
                        }
                    }

                    if id > max_id {
                        max_id = id;
                    }
                    recovered += 1;
                }
                Ok(None) => break,
                Err(LazarusError::DbCorruption { .. }) => {
                    corrupted += 1;
                    tracing::warn!("손상된 엔트리 발견, 건너뜀");
                    continue;
                }
                Err(e) => {
                    tracing::error!("복구 중 에러: {}", e);
                    break;
                }
            }
        }

        self.next_id.store(max_id + 1, Ordering::SeqCst);

        if corrupted > 0 {
            tracing::warn!(
                "복구 완료: {} 노트 로드, {} 손상됨",
                recovered,
                corrupted
            );
        } else {
            tracing::info!("복구 완료: {} 노트 로드", recovered);
        }

        Ok(())
    }

    /// 노트 저장
    pub fn save(&mut self, note: &Note, vector: Option<Vec<i8>>) -> Result<u64> {
        let id = if note.id == 0 {
            self.next_id.fetch_add(1, Ordering::SeqCst)
        } else {
            let current = self.next_id.load(Ordering::SeqCst);
            if note.id >= current {
                self.next_id.store(note.id + 1, Ordering::SeqCst);
            }
            note.id
        };

        // 콘텐츠 압축
        let content_bytes = note.to_markdown().into_bytes();
        let compressed = zstd::encode_all(std::io::Cursor::new(&content_bytes), 3)
            .map_err(|e| LazarusError::DbWrite(e.to_string()))?;

        // NoteAtom 생성
        let atom = NoteAtom {
            id,
            created_at: note.created_at.timestamp(),
            updated_at: note.updated_at.timestamp(),
            content: compressed,
            vector: vector.clone(),
            encrypted: note.encrypted,
            deleted: false,
        };

        // 직렬화
        let bytes = rkyv::to_bytes::<_, 256>(&atom)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;

        // WAL에 추가 - 반환값은 헤더 오프셋
        let entry_offset = self.writer.append(bytes.to_vec())?;

        // 인덱스에 헤더 오프셋 저장
        self.index.insert(id, entry_offset);

        // 벡터 캐시 업데이트
        if let Some(v) = vector {
            self.vector_cache.retain(|(vid, _)| *vid != id);
            self.vector_cache.push((id, v));
        }

        // 즉시 플러시
        self.writer.flush()?;

        tracing::debug!("노트 저장: id={}, offset={}", id, entry_offset);

        Ok(id)
    }

    /// 노트 저장 (암호화 지원)
        pub fn save_encrypted(
            &mut self, 
            note: &Note, 
            vector: Option<Vec<i8>>,
            crypto: Option<&CryptoManager>,
        ) -> Result<u64> {
            let id = if note.id == 0 {
                self.next_id.fetch_add(1, Ordering::SeqCst)
            } else {
                let current = self.next_id.load(Ordering::SeqCst);
                if note.id >= current {
                    self.next_id.store(note.id + 1, Ordering::SeqCst);
                }
                note.id
            };
    
            // 콘텐츠 압축
            let content_bytes = note.to_markdown().into_bytes();
            let compressed = zstd::encode_all(std::io::Cursor::new(&content_bytes), 3)
                .map_err(|e| LazarusError::DbWrite(e.to_string()))?;
    
            // 암호화 (필요시)
            let (final_content, is_encrypted) = if note.encrypted {
                match crypto {
                    Some(c) => {
                        let encrypted = c.encrypt(&compressed)?;
                        (encrypted, true)
                    }
                    None => {
                        return Err(LazarusError::Encryption);
                    }
                }
            } else {
                (compressed, false)
            };
    
            // NoteAtom 생성
            let atom = NoteAtom {
                id,
                created_at: note.created_at.timestamp(),
                updated_at: note.updated_at.timestamp(),
                content: final_content,
                vector: vector.clone(),
                encrypted: is_encrypted,
                deleted: false,
            };
    
            // 직렬화
            let bytes = rkyv::to_bytes::<_, 256>(&atom)
                .map_err(|e| LazarusError::Serialize(e.to_string()))?;
    
            // WAL에 추가
            let entry_offset = self.writer.append(bytes.to_vec())?;
    
            // 인덱스에 저장
            self.index.insert(id, entry_offset);
    
            // 벡터 캐시 업데이트
            if let Some(v) = vector {
                self.vector_cache.retain(|(vid, _)| *vid != id);
                self.vector_cache.push((id, v));
            }
    
            // 즉시 플러시
            self.writer.flush()?;
            tracing::debug!("노트 저장 (암호화={}): id={}", is_encrypted, id);
    
            Ok(id)
        }
    
		/// 노트 로드 (복호화 지원)
    pub fn get_decrypted(&self, id: u64, crypto: Option<&CryptoManager>) -> Result<Option<Note>> {
        let header_offset = match self.index.get(&id) {
            Some(&o) => o,
            None => return Ok(None),
        };

        // 헤더에서 길이 읽기
        let mut len_buf = [0u8; 4];
        #[cfg(unix)]
        self.read_handle.read_at(&mut len_buf, header_offset)?;
        #[cfg(not(unix))]
        {
            use std::io::{Seek, SeekFrom, Read};
            let mut handle = &self.read_handle;
            handle.seek(SeekFrom::Start(header_offset))?;
            handle.read_exact(&mut len_buf)?;
        }

        let len = u32::from_le_bytes(len_buf) as usize;
        let data_offset = header_offset + ENTRY_HEADER_SIZE as u64;

        // 데이터 읽기
        let mut buffer = vec![0u8; len];
        #[cfg(unix)]
        self.read_handle.read_at(&mut buffer, data_offset)?;
        #[cfg(not(unix))]
        {
            use std::io::{Seek, SeekFrom, Read};
            let mut handle = &self.read_handle;
            handle.seek(SeekFrom::Start(data_offset))?;
            handle.read_exact(&mut buffer)?;
        }

		// rkyv 역직렬화 (validation 포함)
        let atom = rkyv::from_bytes::<NoteAtom>(&buffer)
            .map_err(|e| LazarusError::Deserialize(e.to_string()))?;

        if atom.deleted {
            return Ok(None);
        }

        // 복호화 (필요시)
        let decompressed = if atom.encrypted {
            match crypto {
                Some(c) => {
                    let decrypted = c.decrypt(&atom.content)?;
                    zstd::decode_all(std::io::Cursor::new(&decrypted))
                        .map_err(|_| LazarusError::ZimDecompress)?
                }
                None => {
                    // 암호화됐는데 키 없음 → 내용 숨김
                    return Ok(Some(Note {
                        id: atom.id,
                        title: "🔒 암호화된 노트".to_string(),
                        content: "PIN을 입력하여 잠금을 해제하세요".to_string(),
                        tags: vec![],
                        created_at: chrono::DateTime::from_timestamp(atom.created_at, 0)
                            .unwrap_or_default()
                            .with_timezone(&chrono::Utc),
                        updated_at: chrono::DateTime::from_timestamp(atom.updated_at, 0)
                            .unwrap_or_default()
                            .with_timezone(&chrono::Utc),
                        encrypted: true,
                    }));
                }
            }
        } else {
            zstd::decode_all(std::io::Cursor::new(&atom.content))
                .map_err(|_| LazarusError::ZimDecompress)?
        };

        let content = String::from_utf8_lossy(&decompressed).to_string();
        match Note::from_markdown(atom.id, &content) {
            Some(mut note) => {
                note.encrypted = atom.encrypted;
                Ok(Some(note))
            }
            None => Ok(None),
        }
    }
    
    /// 노트 읽기
    pub fn get(&self, id: u64) -> Result<Option<Note>> {
        let header_offset = match self.index.get(&id) {
            Some(&o) => o,
            None => return Ok(None),
        };

        // 헤더에서 길이 읽기
        let mut len_buf = [0u8; 4];
        
        #[cfg(unix)]
        self.read_handle.read_at(&mut len_buf, header_offset)?;
        
        #[cfg(not(unix))]
        {
            use std::io::{Seek, SeekFrom, Read};
            let mut handle = &self.read_handle;
            handle.seek(SeekFrom::Start(header_offset))?;
            handle.read_exact(&mut len_buf)?;
        }

        let len = u32::from_le_bytes(len_buf) as usize;
        
        // 데이터 오프셋 = 헤더 오프셋 + 헤더 크기(8)
        let data_offset = header_offset + ENTRY_HEADER_SIZE as u64;

        // 데이터 읽기
        let mut buffer = vec![0u8; len];
        
        #[cfg(unix)]
        self.read_handle.read_at(&mut buffer, data_offset)?;
        
        #[cfg(not(unix))]
        {
            use std::io::{Seek, SeekFrom, Read};
            let mut handle = &self.read_handle;
            handle.seek(SeekFrom::Start(data_offset))?;
            handle.read_exact(&mut buffer)?;
        }

        // 역직렬화
        let archived = unsafe { rkyv::archived_root::<NoteAtom>(&buffer) };

        // 압축 해제
        let decompressed = zstd::decode_all(std::io::Cursor::new(&archived.content))
            .map_err(|e| LazarusError::Deserialize(e.to_string()))?;

        let markdown = String::from_utf8(decompressed)
            .map_err(|e| LazarusError::Deserialize(e.to_string()))?;

        // Note로 변환
        let note = Note::from_markdown(id, &markdown)
            .ok_or_else(|| LazarusError::Deserialize("마크다운 파싱 실패".to_string()))?;

        Ok(Some(note))
    }

    /// 노트 삭제 (soft delete)
    pub fn delete(&mut self, id: u64) -> Result<bool> {
        if !self.index.contains_key(&id) {
            return Ok(false);
        }

        // 삭제 표시된 NoteAtom 생성
        let atom = NoteAtom {
            id,
            created_at: 0,
            updated_at: chrono::Utc::now().timestamp(),
            content: Vec::new(),
            vector: None,
            encrypted: false,
            deleted: true,
        };

        let bytes = rkyv::to_bytes::<_, 256>(&atom)
            .map_err(|e| LazarusError::Serialize(e.to_string()))?;

        self.writer.append(bytes.to_vec())?;
        self.writer.flush()?;

        // 인덱스에서 제거
        self.index.remove(&id);
        self.vector_cache.retain(|(vid, _)| *vid != id);

        tracing::debug!("노트 삭제: id={}", id);

        Ok(true)
    }

    /// 모든 노트 ID 목록
    pub fn list_ids(&self) -> Vec<u64> {
        self.index.keys().copied().collect()
    }

    /// 노트 개수
    pub fn count(&self) -> usize {
        self.index.len()
    }

    /// 버퍼 플러시
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

/// DB 압축 (Compaction)
	/// 삭제된 레코드 제거, 최신 상태만 유지
	pub fn compact(&mut self) -> Result<CompactResult> {
	    let before_size = std::fs::metadata(&self.path)
	        .map(|m| m.len())
	        .unwrap_or(0);
	    
	    let record_count = self.index.len();
	    
	    // 현재 모든 노트 읽기
	    let mut notes: Vec<Note> = Vec::new();
	    for &id in self.index.keys() {
	        if let Some(note) = self.get(id)? {
	            notes.push(note);
	        }
	    }
	    
	    // 임시 파일에 새로 쓰기
	    let temp_path = self.path.with_extension("lazarus.tmp");
	    let temp_path_str = temp_path.to_str().ok_or_else(|| {
	        LazarusError::DbInit("잘못된 경로".to_string())
	    })?;
	    
	    {
	        let mut temp_writer = WalWriter::open(temp_path_str, BUFFER_SIZE)?;
	        
	        for note in &notes {
	            // 콘텐츠 압축
	            let content_bytes = note.to_markdown().into_bytes();
	            let compressed = zstd::encode_all(std::io::Cursor::new(&content_bytes), 3)
	                .map_err(|e| LazarusError::DbWrite(e.to_string()))?;
	            
	            let atom = NoteAtom {
	                id: note.id,
	                created_at: note.created_at.timestamp(),
	                updated_at: note.updated_at.timestamp(),
	                content: compressed,
	                vector: None,
	                encrypted: note.encrypted,
	                deleted: false,
	            };
	            
	            let data = rkyv::to_bytes::<_, 256>(&atom)
	                .map_err(|e| LazarusError::Serialize(e.to_string()))?;
	            temp_writer.append(data.to_vec())?;
	        }
	        
	        temp_writer.flush()?;
	    }
	    
	    // 기존 파일 교체
	    std::fs::rename(&temp_path, &self.path).map_err(LazarusError::Io)?;
	    
	    // 엔진 재초기화
	    let path_str = self.path.to_str().ok_or_else(|| {
	        LazarusError::DbInit("잘못된 경로".to_string())
	    })?;
	    self.writer = WalWriter::open(path_str, BUFFER_SIZE)?;
	    self.read_handle = std::fs::File::open(&self.path)?;
	    self.index.clear();
	    self.vector_cache.clear();
	    self.recover()?;
	    
	    let after_size = std::fs::metadata(&self.path)
	        .map(|m| m.len())
	        .unwrap_or(0);
	    
	    let saved = if before_size > after_size {
	        before_size - after_size
	    } else {
	        0
	    };
	    
	    tracing::info!(
	        "Compaction 완료: {} -> {} ({} 절약, {}개 레코드)",
	        format_size(before_size),
	        format_size(after_size),
	        format_size(saved),
	        record_count
	    );
	    
	    Ok(CompactResult {
	        before_size,
	        after_size,
	        saved_bytes: saved,
	        record_count,
	    })
	}
}

/// Compaction 결과
#[derive(Debug, serde::Serialize)]
pub struct CompactResult {
	pub before_size: u64,
	pub after_size: u64,
	pub saved_bytes: u64,
	pub record_count: usize,
}

/// 사이즈 포맷팅
fn format_size(bytes: u64) -> String {
	if bytes >= 1024 * 1024 {
	    format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
	} else if bytes >= 1024 {
	    format!("{:.2} KB", bytes as f64 / 1024.0)
	} else {
	    format!("{} B", bytes)
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_engine_basic() {
        let tmp = TempDir::new().unwrap();
        let db_path = tmp.path().join("test.lazarus");

        let mut engine = StorageEngine::open(&db_path).unwrap();

        let note = Note::new(0, "테스트".to_string(), "내용".to_string());
        let id = engine.save(&note, None).unwrap();

        assert_eq!(id, 1);

        let loaded = engine.get(id).unwrap().unwrap();
        assert_eq!(loaded.title, "테스트");
        assert_eq!(loaded.content, "내용");
    }

    #[test]
    fn test_engine_recovery() {
        let tmp = TempDir::new().unwrap();
        let db_path = tmp.path().join("test.lazarus");

        // 첫 번째 세션
        {
            let mut engine = StorageEngine::open(&db_path).unwrap();
            let note = Note::new(0, "복구 테스트".to_string(), "데이터".to_string());
            engine.save(&note, None).unwrap();
        }

        // 두 번째 세션 - 복구
        {
            let engine = StorageEngine::open(&db_path).unwrap();
            assert_eq!(engine.count(), 1);
            
            let loaded = engine.get(1).unwrap().unwrap();
            assert_eq!(loaded.title, "복구 테스트");
        }
    }
}
