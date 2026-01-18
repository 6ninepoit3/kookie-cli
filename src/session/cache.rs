//! Session caching for unlock timeout
//!
//! This module manages the temporary session that keeps the vault unlocked
//! for a configurable duration without re-entering the master password.

use crate::vault::storage;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;

/// Session configuration
#[derive(Serialize, Deserialize, Clone)]
pub struct SessionConfig {
    /// Timeout in minutes (0 = always ask for password)
    pub timeout_minutes: u32,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout_minutes: 10, // Default 10 minutes
        }
    }
}

/// Session data stored on disk
#[derive(Serialize, Deserialize)]
struct SessionData {
    /// Encrypted key (encrypted with a machine-specific key)
    key_data: String,
    /// Expiration time
    expires_at: DateTime<Utc>,
}

/// Gets a simple machine-specific key for session encryption
/// This is not meant to be highly secure, just to prevent trivial reading
fn get_machine_key() -> [u8; 32] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    
    // Use username and home dir as entropy
    if let Some(home) = dirs::home_dir() {
        home.to_string_lossy().hash(&mut hasher);
    }
    if let Ok(username) = std::env::var("USERNAME").or_else(|_| std::env::var("USER")) {
        username.hash(&mut hasher);
    }
    
    // Add a constant salt
    "kookie_session_v1".hash(&mut hasher);
    
    let hash = hasher.finish();
    let mut key = [0u8; 32];
    
    // Expand hash to 32 bytes
    for i in 0..4 {
        let bytes = hash.to_le_bytes();
        key[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
    }
    
    key
}

/// Saves a session with the encryption key
pub fn save_session(key: &[u8; 32], timeout_minutes: u32) -> Result<(), std::io::Error> {
    if timeout_minutes == 0 {
        return Ok(()); // Don't save session if timeout is 0
    }

    let machine_key = get_machine_key();
    
    // XOR the key with machine key for basic obfuscation
    let mut obfuscated = [0u8; 32];
    for i in 0..32 {
        obfuscated[i] = key[i] ^ machine_key[i];
    }
    
    let session = SessionData {
        key_data: BASE64.encode(obfuscated),
        expires_at: Utc::now() + Duration::minutes(timeout_minutes as i64),
    };

    let path = storage::get_session_path();
    let content = serde_json::to_string(&session)?;
    fs::write(path, content)?;
    
    Ok(())
}

/// Gets the cached key if session is still valid
pub fn get_cached_key() -> Option<[u8; 32]> {
    let path = storage::get_session_path();
    
    if !path.exists() {
        return None;
    }
    
    let content = fs::read_to_string(&path).ok()?;
    let session: SessionData = serde_json::from_str(&content).ok()?;
    
    // Check if expired
    if session.expires_at < Utc::now() {
        let _ = clear_session();
        return None;
    }
    
    // Decode and de-obfuscate
    let obfuscated = BASE64.decode(&session.key_data).ok()?;
    if obfuscated.len() != 32 {
        return None;
    }
    
    let machine_key = get_machine_key();
    let mut key = [0u8; 32];
    for i in 0..32 {
        key[i] = obfuscated[i] ^ machine_key[i];
    }
    
    Some(key)
}

/// Clears the session
pub fn clear_session() -> Result<(), std::io::Error> {
    let path = storage::get_session_path();
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Loads session configuration
pub fn load_config() -> SessionConfig {
    let path = storage::get_config_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
    }
    SessionConfig::default()
}

/// Saves session configuration
pub fn save_config(config: &SessionConfig) -> Result<(), std::io::Error> {
    storage::ensure_vault_dir().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let path = storage::get_config_path();
    let content = serde_json::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}
