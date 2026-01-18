//! Key Derivation Function using Argon2id
//!
//! Argon2id is a memory-hard password hashing function that is resistant to
//! GPU cracking attacks. It combines data-independent memory access (Argon2i)
//! with data-dependent memory access (Argon2d) for optimal security.

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, Params, Version,
};
use rand::rngs::OsRng;
use thiserror::Error;

/// Key derivation errors
#[derive(Error, Debug)]
pub enum KdfError {
    #[error("Failed to derive key: {0}")]
    DerivationError(String),
    #[error("Invalid salt")]
    InvalidSalt,
}

/// Argon2id parameters for key derivation
/// - Memory: 64 MB (65536 KB)
/// - Iterations: 3
/// - Parallelism: 4
/// - Output length: 32 bytes (256 bits)
const MEMORY_COST: u32 = 65536; // 64 MB
const TIME_COST: u32 = 3;
const PARALLELISM: u32 = 4;
const OUTPUT_LEN: usize = 32;

/// Derives a 256-bit encryption key from a password using Argon2id
///
/// # Arguments
/// * `password` - The master password
/// * `salt` - A 22+ character base64-encoded salt string
///
/// # Returns
/// A 32-byte (256-bit) key suitable for AES-256-GCM
pub fn derive_key(password: &str, salt: &str) -> Result<[u8; 32], KdfError> {
    let salt = SaltString::from_b64(salt).map_err(|_| KdfError::InvalidSalt)?;

    let params = Params::new(MEMORY_COST, TIME_COST, PARALLELISM, Some(OUTPUT_LEN))
        .map_err(|e| KdfError::DerivationError(e.to_string()))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| KdfError::DerivationError(e.to_string()))?;

    let hash_output = hash.hash.ok_or_else(|| KdfError::DerivationError("No hash output".into()))?;
    
    let bytes = hash_output.as_bytes();
    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes[..32]);
    
    Ok(key)
}

/// Generates a new random salt for key derivation
///
/// # Returns
/// A base64-encoded salt string suitable for Argon2
pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation_consistency() {
        let password = "test_password_123";
        let salt = generate_salt();
        
        let key1 = derive_key(password, &salt).unwrap();
        let key2 = derive_key(password, &salt).unwrap();
        
        assert_eq!(key1, key2, "Same password and salt should produce same key");
    }

    #[test]
    fn test_different_passwords_different_keys() {
        let salt = generate_salt();
        
        let key1 = derive_key("password1", &salt).unwrap();
        let key2 = derive_key("password2", &salt).unwrap();
        
        assert_ne!(key1, key2, "Different passwords should produce different keys");
    }

    #[test]
    fn test_different_salts_different_keys() {
        let password = "same_password";
        
        let key1 = derive_key(password, &generate_salt()).unwrap();
        let key2 = derive_key(password, &generate_salt()).unwrap();
        
        assert_ne!(key1, key2, "Different salts should produce different keys");
    }
}
