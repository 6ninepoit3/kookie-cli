//! Vault storage operations

use super::{VaultError, VaultFile};
use std::fs;
use std::path::PathBuf;

/// Returns the default vault directory path
pub fn get_vault_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".kookie")
}

/// Returns the default vault file path
pub fn get_vault_path() -> PathBuf {
    get_vault_dir().join("vault.json")
}

/// Returns the session file path
pub fn get_session_path() -> PathBuf {
    get_vault_dir().join(".session")
}

/// Returns the config file path
pub fn get_config_path() -> PathBuf {
    get_vault_dir().join("config.json")
}

/// Ensures the vault directory exists
pub fn ensure_vault_dir() -> Result<(), VaultError> {
    let dir = get_vault_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(())
}

/// Loads the vault file from disk
pub fn load_vault_file(path: &PathBuf) -> Result<VaultFile, VaultError> {
    let content = fs::read_to_string(path)?;
    let vault_file: VaultFile = serde_json::from_str(&content)?;
    Ok(vault_file)
}

/// Saves the vault file to disk
pub fn save_vault_file(path: &PathBuf, vault_file: &VaultFile) -> Result<(), VaultError> {
    ensure_vault_dir()?;
    let content = serde_json::to_string_pretty(vault_file)?;
    fs::write(path, content)?;
    Ok(())
}
