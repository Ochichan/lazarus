//! Write-Ahead Log 구현
//!
//! Append-only 구조로 데이터 무결성 보장

use crc32fast::Hasher;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use crate::error::{LazarusError, Result};

/// WAL 파일 매직 바이트
pub const MAGIC_BYTES: &[u8; 8] = b"LAZARUS\x01";

/// WAL 파일 버전
pub const VERSION: u8 = 1;

/// 엔트리 헤더 크기 (길이 4 + CRC 4)
pub const ENTRY_HEADER_SIZE: usize = 8;

/// WAL 엔트리
#[derive(Debug)]
pub struct WalEntry {
    /// 데이터 길이
    pub len: u32,
    /// CRC32 체크섬
    pub crc: u32,
    /// 실제 데이터
    pub data: Vec<u8>,
}

impl WalEntry {
    /// 새 엔트리 생성
    pub fn new(data: Vec<u8>) -> Self {
        let len = data.len() as u32;
        let mut hasher = Hasher::new();
        hasher.update(&data);
        let crc = hasher.finalize();

        Self { len, crc, data }
    }

    /// 바이트로 직렬화
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(ENTRY_HEADER_SIZE + self.data.len());
        bytes.extend_from_slice(&self.len.to_le_bytes());
        bytes.extend_from_slice(&self.crc.to_le_bytes());
        bytes.extend_from_slice(&self.data);
        bytes
    }

    /// CRC 검증
    pub fn verify(&self) -> bool {
        let mut hasher = Hasher::new();
        hasher.update(&self.data);
        hasher.finalize() == self.crc
    }
}

/// WAL 리더
pub struct WalReader {
    reader: BufReader<File>,
    current_offset: u64,
    file_len: u64,
}

impl WalReader {
    /// WAL 파일 열기
    pub fn open(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let file_len = file.metadata()?.len();
        let mut reader = BufReader::new(file);

        // 매직 바이트 확인
        let mut magic = [0u8; 8];
        reader.read_exact(&mut magic)?;

        if &magic != MAGIC_BYTES {
            return Err(LazarusError::DbCorruption {
                expected: 0x4C415A41, // "LAZA"
                actual: u32::from_le_bytes(magic[0..4].try_into().unwrap()),
            });
        }

        Ok(Self {
            reader,
            current_offset: 8, // 매직 바이트 이후
            file_len,
        })
    }

    /// 다음 엔트리 읽기
    pub fn next_entry(&mut self) -> Result<Option<(u64, WalEntry)>> {
        if self.current_offset >= self.file_len {
            return Ok(None);
        }

        // 헤더 읽기
        let mut header = [0u8; ENTRY_HEADER_SIZE];
        if let Err(e) = self.reader.read_exact(&mut header) {
            // EOF나 불완전한 헤더는 복구 시 마지막 엔트리일 수 있음
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                return Ok(None);
            }
            return Err(e.into());
        }

        let len = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let crc = u32::from_le_bytes(header[4..8].try_into().unwrap());

        // 데이터 읽기
        let mut data = vec![0u8; len as usize];
        self.reader.read_exact(&mut data)?;

        let entry_offset = self.current_offset;
        self.current_offset += ENTRY_HEADER_SIZE as u64 + len as u64;

        let entry = WalEntry { len, crc, data };

        // CRC 검증
        if !entry.verify() {
            return Err(LazarusError::DbCorruption {
                expected: crc,
                actual: {
                    let mut h = Hasher::new();
                    h.update(&entry.data);
                    h.finalize()
                },
            });
        }

        Ok(Some((entry_offset, entry)))
    }

    /// 현재 오프셋
    pub fn offset(&self) -> u64 {
        self.current_offset
    }
}

/// WAL 라이터
pub struct WalWriter {
    file: File,
    buffer: Vec<u8>,
    buffer_size: usize,
    current_offset: u64,
}

impl WalWriter {
    /// WAL 파일 열기 또는 생성
    pub fn open(path: &str, buffer_size: usize) -> Result<Self> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)?;

        let file_len = file.metadata()?.len();

        // 새 파일이면 매직 바이트 쓰기
        let current_offset = if file_len == 0 {
            file.write_all(MAGIC_BYTES)?;
            file.sync_data()?;
            8
        } else {
            file_len
        };

        Ok(Self {
            file,
            buffer: Vec::with_capacity(buffer_size),
            buffer_size,
            current_offset,
        })
    }

    /// 엔트리 추가
    pub fn append(&mut self, data: Vec<u8>) -> Result<u64> {
        let entry = WalEntry::new(data);
        let bytes = entry.to_bytes();

        let entry_offset = self.current_offset + self.buffer.len() as u64;

        self.buffer.extend_from_slice(&bytes);

        // 버퍼가 차면 플러시
        if self.buffer.len() >= self.buffer_size {
            self.flush()?;
        }

        Ok(entry_offset)
    }

    /// 버퍼 플러시
    pub fn flush(&mut self) -> Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        self.file.write_all(&self.buffer)?;
        self.file.sync_data()?;

        self.current_offset += self.buffer.len() as u64;
        self.buffer.clear();

        Ok(())
    }

    /// 현재 오프셋
    pub fn offset(&self) -> u64 {
        self.current_offset + self.buffer.len() as u64
    }
}

impl Drop for WalWriter {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_wal_entry_crc() {
        let entry = WalEntry::new(b"hello world".to_vec());
        assert!(entry.verify());
    }

    #[test]
    fn test_wal_write_read() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap();

        // 쓰기
        {
            let mut writer = WalWriter::open(path, 4096).unwrap();
            writer.append(b"first entry".to_vec()).unwrap();
            writer.append(b"second entry".to_vec()).unwrap();
            writer.flush().unwrap();
        }

        // 읽기
        {
            let mut reader = WalReader::open(path).unwrap();

            let (_, entry1) = reader.next_entry().unwrap().unwrap();
            assert_eq!(entry1.data, b"first entry");

            let (_, entry2) = reader.next_entry().unwrap().unwrap();
            assert_eq!(entry2.data, b"second entry");

            assert!(reader.next_entry().unwrap().is_none());
        }
    }
}
