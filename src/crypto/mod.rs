//! 암호화 모듈
//!
//! XChaCha20-Poly1305 + Argon2id

use argon2::Argon2;
use chacha20poly1305::{
    aead::Aead,
    XChaCha20Poly1305, XNonce,
    KeyInit,
};
use rand::RngCore;

use crate::error::{LazarusError, Result};

/// 솔트 크기 (16 bytes)
const SALT_SIZE: usize = 16;

/// Nonce 크기 (24 bytes for XChaCha20)
const NONCE_SIZE: usize = 24;

/// 암호화 매니저
pub struct CryptoManager {
    /// 암호화 키 (32 bytes)
    key: [u8; 32],
}

impl CryptoManager {
    /// PIN에서 암호화 키 유도
    pub fn from_pin(pin: &str, salt: &[u8]) -> Result<Self> {
        let mut key = [0u8; 32];
        
        let argon2 = Argon2::default();
        argon2.hash_password_into(
            pin.as_bytes(),
            salt,
            &mut key,
        ).map_err(|e| LazarusError::Encryption)?;
        
        Ok(Self { key })
    }

    /// 새 솔트 생성
    pub fn generate_salt() -> [u8; SALT_SIZE] {
        let mut salt = [0u8; SALT_SIZE];
        rand::thread_rng().fill_bytes(&mut salt);
        salt
    }

    /// 데이터 암호화
    /// 반환: nonce (12 bytes) + ciphertext + tag (16 bytes)
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new_from_slice(&self.key)
            .map_err(|_| LazarusError::Encryption)?;

        // 랜덤 nonce 생성
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = XNonce::from_slice(&nonce_bytes);

        // 암호화
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|_| LazarusError::Encryption)?;

        // nonce + ciphertext 결합
        let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// 데이터 복호화
    /// 입력: nonce (12 bytes) + ciphertext + tag (16 bytes)
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < NONCE_SIZE {
            return Err(LazarusError::Decryption);
        }

        let cipher = XChaCha20Poly1305::new_from_slice(&self.key)
            .map_err(|_| LazarusError::Decryption)?;

        // nonce와 ciphertext 분리
        let nonce = XNonce::from_slice(&data[..NONCE_SIZE]);
        let ciphertext = &data[NONCE_SIZE..];

        // 복호화
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|_| LazarusError::Decryption)?;

        Ok(plaintext)
    }

    /// PIN 검증 (테스트용 데이터로 확인)
    pub fn verify_pin(&self, test_data: &[u8], expected: &[u8]) -> bool {
        match self.decrypt(test_data) {
            Ok(decrypted) => decrypted == expected,
            Err(_) => false,
        }
    }
}

/// 암호화된 데이터 헤더
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptedHeader {
    /// 솔트 (base64)
    pub salt: String,
    /// PIN 검증용 암호화된 테스트 데이터 (base64)
    pub verify_data: String,
}

impl EncryptedHeader {
    /// 새 헤더 생성 (PIN 설정 시)
    pub fn new(pin: &str) -> Result<Self> {
        let salt = CryptoManager::generate_salt();
        let crypto = CryptoManager::from_pin(pin, &salt)?;
        
        // 검증용 데이터 암호화
        let test_data = b"LAZARUS_PIN_OK";
        let verify_data = crypto.encrypt(test_data)?;
        
        Ok(Self {
            salt: base64_encode(&salt),
            verify_data: base64_encode(&verify_data),
        })
    }

    /// PIN 검증
    pub fn verify(&self, pin: &str) -> Result<bool> {
        let salt = base64_decode(&self.salt)?;
        let verify_data = base64_decode(&self.verify_data)?;
        
        let crypto = CryptoManager::from_pin(pin, &salt)?;
        Ok(crypto.verify_pin(&verify_data, b"LAZARUS_PIN_OK"))
    }

    /// CryptoManager 생성
    pub fn get_crypto(&self, pin: &str) -> Result<CryptoManager> {
        let salt = base64_decode(&self.salt)?;
        CryptoManager::from_pin(pin, &salt)
    }
}

/// Base64 인코딩
fn base64_encode(data: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(data.len() * 4 / 3 + 4);
    for byte in data {
        write!(s, "{:02x}", byte).unwrap();
    }
    s
}

/// Base64 디코딩 (실제로는 hex)
fn base64_decode(s: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(s.len() / 2);
    for i in (0..s.len()).step_by(2) {
        let byte = u8::from_str_radix(&s[i..i+2], 16)
            .map_err(|_| LazarusError::Decryption)?;
        bytes.push(byte);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let salt = CryptoManager::generate_salt();
        let crypto = CryptoManager::from_pin("1234", &salt).unwrap();
        
        let plaintext = b"Hello, Lazarus!";
        let encrypted = crypto.encrypt(plaintext).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_wrong_pin() {
        let salt = CryptoManager::generate_salt();
        let crypto1 = CryptoManager::from_pin("1234", &salt).unwrap();
        let crypto2 = CryptoManager::from_pin("5678", &salt).unwrap();
        
        let plaintext = b"Secret data";
        let encrypted = crypto1.encrypt(plaintext).unwrap();
        
        // 다른 PIN으로 복호화 시도 → 실패해야 함
        assert!(crypto2.decrypt(&encrypted).is_err());
    }

    #[test]
    fn test_header_verify() {
        let header = EncryptedHeader::new("1234").unwrap();
        
        assert!(header.verify("1234").unwrap());
        assert!(!header.verify("5678").unwrap());
    }
    #[test]
    fn test_encrypt_produces_different_output() {
        let salt = CryptoManager::generate_salt();
        let crypto = CryptoManager::from_pin("123456", &salt).unwrap();
        let plaintext = b"Same message";
        
        let encrypted1 = crypto.encrypt(plaintext).unwrap();
        let encrypted2 = crypto.encrypt(plaintext).unwrap();
        
        // 다른 nonce로 다른 결과
        assert_ne!(encrypted1, encrypted2);
    }

    #[test]
    fn test_empty_plaintext() {
        let salt = CryptoManager::generate_salt();
        let crypto = CryptoManager::from_pin("123456", &salt).unwrap();
        let plaintext = b"";
        
        let encrypted = crypto.encrypt(plaintext).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_salt_uniqueness() {
        let salt1 = CryptoManager::generate_salt();
        let salt2 = CryptoManager::generate_salt();
        
        assert_ne!(salt1, salt2);
    }
}

use std::path::Path;
use std::fs;

/// 보안 설정 (PIN 잠금)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityConfig {
    /// PIN 설정됨 여부
    pub pin_enabled: bool,
    /// 암호화 헤더 (PIN 설정 시)
    pub header: Option<EncryptedHeader>,
}

impl SecurityConfig {
    /// 기본값 (PIN 없음)
    pub fn default() -> Self {
        Self {
            pin_enabled: false,
            header: None,
        }
    }

    /// 파일에서 로드
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)
            .map_err(|e| LazarusError::Io(e))?;
        serde_json::from_str(&content)
            .map_err(|e| LazarusError::JsonParse(e))
    }

    /// 파일에 저장
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| LazarusError::JsonParse(e))?;
        fs::write(path, content)
            .map_err(|e| LazarusError::Io(e))?;
        Ok(())
    }

    /// PIN 설정
    pub fn set_pin(&mut self, pin: &str) -> Result<()> {
        let header = EncryptedHeader::new(pin)?;
        self.pin_enabled = true;
        self.header = Some(header);
        Ok(())
    }

    /// PIN 해제
    pub fn remove_pin(&mut self) {
        self.pin_enabled = false;
        self.header = None;
    }

    /// PIN 검증
    pub fn verify_pin(&self, pin: &str) -> Result<bool> {
        match &self.header {
            Some(header) => header.verify(pin),
            None => Ok(true), // PIN 없으면 항상 성공
        }
    }

    /// CryptoManager 가져오기
    pub fn get_crypto(&self, pin: &str) -> Result<Option<CryptoManager>> {
        match &self.header {
            Some(header) => Ok(Some(header.get_crypto(pin)?)),
            None => Ok(None),
        }
    }
}

