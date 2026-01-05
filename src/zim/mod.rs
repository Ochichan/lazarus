//! ZIM 파일 리더
//!
//! 80GB 위키백과를 메모리 효율적으로 읽기 위한 Mmap 기반 리더

use memmap2::Mmap;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::error::{LazarusError, Result};

/// Levenshtein 거리 계산 (두 문자열 간 편집 거리)
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut prev_row: Vec<usize> = (0..=len2).collect();
    let mut curr_row: Vec<usize> = vec![0; len2 + 1];

    for i in 1..=len1 {
        curr_row[0] = i;
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };
            curr_row[j] = (prev_row[j] + 1)
                .min(curr_row[j - 1] + 1)
                .min(prev_row[j - 1] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[len2]
}

/// 단어가 쿼리와 유사한지 확인 (fuzzy match)
fn fuzzy_match(query: &str, text: &str, max_distance: usize) -> bool {
    let query_lower = query.to_lowercase();
    let text_lower = text.to_lowercase();

    // 정확히 포함하면 바로 true
    if text_lower.contains(&query_lower) {
        return true;
    }

    // 단어별로 검사
    for word in text_lower.split(|c: char| !c.is_alphanumeric()) {
        if word.is_empty() {
            continue;
        }

        // 길이가 너무 다르면 스킵
        let len_diff = (word.len() as isize - query_lower.len() as isize).abs() as usize;
        if len_diff > max_distance {
            continue;
        }

        // Levenshtein 거리 계산
        if levenshtein_distance(&query_lower, word) <= max_distance {
            return true;
        }
    }

    false
}

/// ZIM 파일 매직 넘버
const ZIM_MAGIC: u32 = 72173914; // 0x44D495A

/// ZIM 헤더 (고정 80바이트)
#[derive(Debug)]
pub struct ZimHeader {
    pub magic: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub uuid: [u8; 16],
    pub article_count: u32,
    pub cluster_count: u32,
    pub url_ptr_pos: u64,
    pub title_ptr_pos: u64,
    pub cluster_ptr_pos: u64,
    pub mime_list_pos: u64,
    pub main_page: u32,
    pub layout_page: u32,
    pub checksum_pos: u64,
}

impl ZimHeader {
    /// 바이트 슬라이스에서 헤더 파싱
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < 80 {
            return Err(LazarusError::ZimOpen {
                path: "헤더가 너무 짧음".to_string(),
            });
        }

        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

        if magic != ZIM_MAGIC {
            return Err(LazarusError::ZimOpen {
                path: format!("잘못된 매직 넘버: {:#x}", magic),
            });
        }

        Ok(Self {
            magic,
            major_version: u16::from_le_bytes([data[4], data[5]]),
            minor_version: u16::from_le_bytes([data[6], data[7]]),
            uuid: data[8..24].try_into().unwrap(),
            article_count: u32::from_le_bytes([data[24], data[25], data[26], data[27]]),
            cluster_count: u32::from_le_bytes([data[28], data[29], data[30], data[31]]),
            url_ptr_pos: u64::from_le_bytes(data[32..40].try_into().unwrap()),
            title_ptr_pos: u64::from_le_bytes(data[40..48].try_into().unwrap()),
            cluster_ptr_pos: u64::from_le_bytes(data[48..56].try_into().unwrap()),
            mime_list_pos: u64::from_le_bytes(data[56..64].try_into().unwrap()),
            main_page: u32::from_le_bytes([data[64], data[65], data[66], data[67]]),
            layout_page: u32::from_le_bytes([data[68], data[69], data[70], data[71]]),
            checksum_pos: u64::from_le_bytes(data[72..80].try_into().unwrap()),
        })
    }
}

/// ZIM 리더
pub struct ZimReader {
    /// 파일 경로
    path: PathBuf,
    /// Memory-mapped 파일
    mmap: Mmap,
    /// 파싱된 헤더
    pub header: ZimHeader,
}

impl ZimReader {
    /// ZIM 파일 열기
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        let file = File::open(&path).map_err(|e| LazarusError::ZimOpen {
            path: format!("{}: {}", path.display(), e),
        })?;

        let mmap = unsafe {
            Mmap::map(&file).map_err(|e| LazarusError::ZimOpen {
                path: format!("mmap 실패: {}", e),
            })?
        };

        let header = ZimHeader::parse(&mmap)?;

        tracing::info!(
            "ZIM 열기 성공: {} (문서: {}, 클러스터: {})",
            path.display(),
            header.article_count,
            header.cluster_count
        );

        Ok(Self { path, mmap, header })
    }

    /// 파일 크기
    pub fn size(&self) -> usize {
        self.mmap.len()
    }

    /// 원시 바이트 접근
    pub fn data(&self) -> &[u8] {
        &self.mmap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zim_header_size() {
        // ZIM 헤더는 80바이트
        assert!(std::mem::size_of::<ZimHeader>() <= 128);
    }
}

/// Directory Entry 타입
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntryType {
    /// 콘텐츠 (문서, 이미지 등)
    Content,
    /// 리다이렉트
    Redirect,
    /// 삭제됨
    Deleted,
}

/// Directory Entry
#[derive(Debug)]
pub struct DirEntry {
    pub mime_type: u16,
    pub namespace: char,
    pub url: String,
    pub title: String,
    pub entry_type: EntryType,
    /// Content entry: 클러스터/블롭 번호
    pub cluster_number: Option<u32>,
    pub blob_number: Option<u32>,
    /// Redirect entry: 리다이렉트 대상 인덱스
    pub redirect_index: Option<u32>,
}

impl ZimReader {
    /// URL 포인터 리스트에서 특정 인덱스의 오프셋 가져오기
    fn get_url_offset(&self, index: u32) -> u64 {
        let pos = self.header.url_ptr_pos as usize + (index as usize * 8);
        u64::from_le_bytes(self.mmap[pos..pos + 8].try_into().unwrap())
    }

    /// 클러스터 포인터 가져오기
    fn get_cluster_offset(&self, cluster_num: u32) -> u64 {
        let pos = self.header.cluster_ptr_pos as usize + (cluster_num as usize * 8);
        u64::from_le_bytes(self.mmap[pos..pos + 8].try_into().unwrap())
    }

    /// 디렉토리 엔트리 파싱
    pub fn read_dir_entry(&self, index: u32) -> Result<DirEntry> {
        let offset = self.get_url_offset(index) as usize;
        let data = &self.mmap[offset..];

        let mime_type = u16::from_le_bytes([data[0], data[1]]);

        // MIME 타입으로 엔트리 종류 판별
        let entry_type = match mime_type {
            0xFFFF => EntryType::Redirect,
            0xFFFE => EntryType::Deleted,
            _ => EntryType::Content,
        };

        let _param_len = data[2];
        let namespace = data[3] as char;

        let (cluster_number, blob_number, redirect_index, url_start) = match entry_type {
            EntryType::Content => {
                let revision = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                let cluster = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
                let blob = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
                let _ = revision; // 사용하지 않음
                (Some(cluster), Some(blob), None, 16)
            }
            EntryType::Redirect => {
                let redirect = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                (None, None, Some(redirect), 8)
            }
            EntryType::Deleted => (None, None, None, 4),
        };

        // URL 읽기 (null-terminated)
        let url = read_null_terminated(&data[url_start..]);
        let title_start = url_start + url.len() + 1;
        let title = read_null_terminated(&data[title_start..]);

        Ok(DirEntry {
            mime_type,
            namespace,
            url,
            title,
            entry_type,
            cluster_number,
            blob_number,
            redirect_index,
        })
    }

    /// 문서 개수
    pub fn article_count(&self) -> u32 {
        self.header.article_count
    }

    /// 특정 URL로 엔트리 찾기 (선형 탐색 - 나중에 이진 탐색으로 개선)
    pub fn find_by_url(&self, namespace: char, url: &str) -> Result<Option<DirEntry>> {
        for i in 0..self.header.article_count {
            let entry = self.read_dir_entry(i)?;
            if entry.namespace == namespace && entry.url == url {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }

    /// URL로 엔트리 찾기 (이진 탐색)
    pub fn find_by_url_binary(&self, target_url: &str) -> Result<Option<DirEntry>> {
        let mut low: u32 = 0;
        let mut high: u32 = self.header.article_count.saturating_sub(1);

        while low <= high {
            let mid = low + (high - low) / 2;

            let entry = match self.read_dir_entry(mid) {
                Ok(e) => e,
                Err(_) => {
                    // 읽기 실패하면 범위 줄이기
                    if mid == 0 {
                        break;
                    }
                    high = mid - 1;
                    continue;
                }
            };

            match target_url.cmp(&entry.url) {
                std::cmp::Ordering::Equal => return Ok(Some(entry)),
                std::cmp::Ordering::Less => {
                    if mid == 0 {
                        break;
                    }
                    high = mid - 1;
                }
                std::cmp::Ordering::Greater => {
                    low = mid + 1;
                }
            }
        }

        Ok(None)
    }

    /// URL로 콘텐츠 가져오기 (이진 탐색 버전)
    pub fn get_content_fast(&self, target_url: &str) -> Result<Option<Vec<u8>>> {
        // 먼저 이진 탐색으로 시도
        if let Some(entry) = self.find_by_url_binary(target_url)? {
            return self.get_content_from_entry(&entry);
        }

        // 실패하면 여러 네임스페이스에서 선형 탐색
        for ns in ['C', 'A', '-'] {
            if let Some(content) = self.get_content(ns, target_url)? {
                return Ok(Some(content));
            }
        }

        Ok(None)
    }

    /// 엔트리에서 콘텐츠 가져오기
    fn get_content_from_entry(&self, entry: &DirEntry) -> Result<Option<Vec<u8>>> {
        match entry.entry_type {
            EntryType::Redirect => {
                if let Some(redirect_idx) = entry.redirect_index {
                    let redirect_entry = self.read_dir_entry(redirect_idx)?;
                    return self.get_content_from_entry(&redirect_entry);
                }
                Ok(None)
            }
            EntryType::Deleted => Ok(None),
            EntryType::Content => {
                if let (Some(cluster), Some(blob)) = (entry.cluster_number, entry.blob_number) {
                    let data = self.read_blob(cluster, blob)?;
                    Ok(Some(data))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// 메인 페이지 가져오기
    pub fn main_page(&self) -> Result<Option<DirEntry>> {
        if self.header.main_page == 0xFFFFFFFF {
            return Ok(None);
        }
        Ok(Some(self.read_dir_entry(self.header.main_page)?))
    }
}

/// Null-terminated 문자열 읽기
fn read_null_terminated(data: &[u8]) -> String {
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
    String::from_utf8_lossy(&data[..end]).to_string()
}

/// 클러스터 압축 타입
#[derive(Debug, Clone, Copy)]
pub enum ClusterCompression {
    None,
    Zstd,
    Lzma, // XZ
    Unknown(u8),
}

impl ZimReader {
    /// 클러스터에서 블롭 데이터 읽기
    pub fn read_blob(&self, cluster_num: u32, blob_num: u32) -> Result<Vec<u8>> {
        let cluster_offset = self.get_cluster_offset(cluster_num) as usize;

        // 클러스터 정보 바이트
        let info_byte = self.mmap[cluster_offset];
        let compression = match info_byte & 0x0F {
            0 | 1 => ClusterCompression::None,
            4 => ClusterCompression::Lzma,
            5 => ClusterCompression::Zstd,
            other => ClusterCompression::Unknown(other),
        };

        let extended = (info_byte & 0x10) != 0;
        let offset_size: usize = if extended { 8 } else { 4 };

        // 다음 클러스터 오프셋으로 클러스터 크기 계산
        let next_cluster_offset = if cluster_num + 1 < self.header.cluster_count {
            self.get_cluster_offset(cluster_num + 1) as usize
        } else {
            self.header.checksum_pos as usize
        };

        let cluster_data = &self.mmap[cluster_offset + 1..next_cluster_offset];

        // 압축 해제
        let decompressed = match compression {
            ClusterCompression::None => cluster_data.to_vec(),
            ClusterCompression::Zstd => zstd::decode_all(std::io::Cursor::new(cluster_data))
                .map_err(|e| LazarusError::ZimDecompress)?,
            ClusterCompression::Lzma => {
                // XZ/LZMA 지원 (나중에 추가)
                return Err(LazarusError::ZimDecompress);
            }
            ClusterCompression::Unknown(t) => {
                tracing::warn!("알 수 없는 압축 타입: {}", t);
                return Err(LazarusError::ZimDecompress);
            }
        };

        // 블롭 오프셋 테이블 읽기
        let blob_offset = if extended {
            u64::from_le_bytes(
                decompressed[blob_num as usize * 8..(blob_num as usize + 1) * 8]
                    .try_into()
                    .unwrap(),
            ) as usize
        } else {
            u32::from_le_bytes(
                decompressed[blob_num as usize * 4..(blob_num as usize + 1) * 4]
                    .try_into()
                    .unwrap(),
            ) as usize
        };

        let next_blob_offset = if extended {
            u64::from_le_bytes(
                decompressed[(blob_num as usize + 1) * 8..(blob_num as usize + 2) * 8]
                    .try_into()
                    .unwrap(),
            ) as usize
        } else {
            u32::from_le_bytes(
                decompressed[(blob_num as usize + 1) * 4..(blob_num as usize + 2) * 4]
                    .try_into()
                    .unwrap(),
            ) as usize
        };

        Ok(decompressed[blob_offset..next_blob_offset].to_vec())
    }

    /// URL로 콘텐츠 가져오기
    pub fn get_content(&self, namespace: char, url: &str) -> Result<Option<Vec<u8>>> {
        let entry = match self.find_by_url(namespace, url)? {
            Some(e) => e,
            None => return Ok(None),
        };

        // 리다이렉트 처리
        let entry = match entry.entry_type {
            EntryType::Redirect => {
                if let Some(redirect_idx) = entry.redirect_index {
                    self.read_dir_entry(redirect_idx)?
                } else {
                    return Ok(None);
                }
            }
            EntryType::Deleted => return Ok(None),
            EntryType::Content => entry,
        };

        if let (Some(cluster), Some(blob)) = (entry.cluster_number, entry.blob_number) {
            let data = self.read_blob(cluster, blob)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    /// 첫 N개 문서 목록 가져오기 (탐색용)
    pub fn list_articles(&self, limit: usize) -> Result<Vec<DirEntry>> {
        let mut articles = Vec::new();
        let count = std::cmp::min(self.header.article_count as usize, limit * 50);

        for i in 0..count {
            if let Ok(entry) = self.read_dir_entry(i as u32) {
                // HTML 문서만 필터링
                if (entry.namespace == 'A' || entry.namespace == 'C')
                    && entry.entry_type == EntryType::Content
                    && !entry.url.ends_with(".png")
                    && !entry.url.ends_with(".jpg")
                    && !entry.url.ends_with(".jpeg")
                    && !entry.url.ends_with(".gif")
                    && !entry.url.ends_with(".svg")
                    && !entry.url.ends_with(".css")
                    && !entry.url.ends_with(".js")
                    && !entry.url.ends_with(".woff")
                    && !entry.url.ends_with(".woff2")
                    && !entry.url.ends_with(".ttf")
                {
                    articles.push(entry);
                    if articles.len() >= limit {
                        break;
                    }
                }
            }
        }

        Ok(articles)
    }
}

impl ZimReader {
    /// 제목으로 검색 (부분 일치)
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<DirEntry>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for i in 0..self.header.article_count {
            if let Ok(entry) = self.read_dir_entry(i) {
                // HTML 문서만 검색
                if (entry.namespace == 'A' || entry.namespace == 'C')
                    && entry.entry_type == EntryType::Content
                    && !entry.url.ends_with(".png")
                    && !entry.url.ends_with(".jpg")
                    && !entry.url.ends_with(".css")
                    && !entry.url.ends_with(".js")
                {
                    let title_lower = entry.title.to_lowercase();
                    let url_lower = entry.url.to_lowercase();

                    if title_lower.contains(&query_lower) || url_lower.contains(&query_lower) {
                        results.push(entry);
                        if results.len() >= limit {
                            break;
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Fuzzy 검색 (오타 허용)
    pub fn search_fuzzy(&self, query: &str, limit: usize) -> Result<Vec<DirEntry>> {
        // 먼저 정확한 검색 시도
        let exact_results = self.search(query, limit)?;
        if !exact_results.is_empty() {
            return Ok(exact_results);
        }

        // 결과 없으면 fuzzy 검색
        let mut results = Vec::new();

        for i in 0..self.header.article_count {
            if let Ok(entry) = self.read_dir_entry(i) {
                if (entry.namespace == 'A' || entry.namespace == 'C')
                    && entry.entry_type == EntryType::Content
                    && !entry.url.ends_with(".png")
                    && !entry.url.ends_with(".jpg")
                    && !entry.url.ends_with(".css")
                    && !entry.url.ends_with(".js")
                {
                    // Edit distance 2까지 허용
                    if fuzzy_match(query, &entry.title, 2) {
                        results.push(entry);
                        if results.len() >= limit {
                            break;
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}
