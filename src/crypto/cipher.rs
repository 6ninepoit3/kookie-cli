//! AES-256-GCM Encryption/Decryption
//!
//! AES-256-GCM provides authenticated encryption, ensuring both
//! confidentiality and integrity of the encrypted data.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use thiserror::Error;

/// Cipher errors
#[derive(Error, Debug)]
pub enum CipherError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed - wrong password or corrupted data")]
    DecryptionFailed,
    #[error("Invalid ciphertext format")]
    InvalidFormat,
}

/// Nonce size for AES-GCM (96 bits = 12 bytes)
const NONCE_SIZE: usize = 12;

/// Encrypts plaintext using AES-256-GCM
///
/// # Arguments
/// * `key` - 32-byte encryption key
/// * `plaintext` - Data to encrypt
///
/// # Returns
/// Base64-encoded string containing: nonce || ciphertext || tag
pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<String, CipherError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CipherError::EncryptionFailed)?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CipherError::EncryptionFailed)?;

    // Combine nonce and ciphertext
    let mut combined = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(&combined))
}

/// Decrypts ciphertext using AES-256-GCM
///
/// # Arguments
/// * `key` - 32-byte encryption key
/// * `ciphertext_b64` - Base64-encoded ciphertext (nonce || ciphertext || tag)
///
/// # Returns
/// Decrypted plaintext bytes
pub fn decrypt(key: &[u8; 32], ciphertext_b64: &str) -> Result<Vec<u8>, CipherError> {
    let combined = BASE64
        .decode(ciphertext_b64)
        .map_err(|_| CipherError::InvalidFormat)?;

    if combined.len() < NONCE_SIZE {
        return Err(CipherError::InvalidFormat);
    }

    let (nonce_bytes, ciphertext) = combined.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CipherError::DecryptionFailed)?;

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CipherError::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [0x42u8; 32];
        let plaintext = b"Hello, World! This is a secret message.";

        let encrypted = encrypt(&key, plaintext).unwrap();
        let decrypted = decrypt(&key, &encrypted).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = [0x42u8; 32];
        let key2 = [0x43u8; 32];
        let plaintext = b"Secret data";

        let encrypted = encrypt(&key1, plaintext).unwrap();
        let result = decrypt(&key2, &encrypted);

        assert!(result.is_err());
    }

    #[test]
    fn test_different_encryptions_different_output() {
        let key = [0x42u8; 32];
        let plaintext = b"Same message";

        let encrypted1 = encrypt(&key, plaintext).unwrap();
        let encrypted2 = encrypt(&key, plaintext).unwrap();

        // Due to random nonce, each encryption should produce different output
        assert_ne!(encrypted1, encrypted2);
    }
}
