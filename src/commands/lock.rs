//! Lock and unlock commands

use crate::session::{self, cache};
use crate::utils::{display, input};
use crate::vault::Vault;

/// Runs the lock command
pub fn lock() -> Result<(), Box<dyn std::error::Error>> {
    cache::clear_session()?;
    display::success("Vault locked. Master password will be required for next access.");
    Ok(())
}

/// Runs the unlock command
pub fn unlock(timeout: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
    let mut vault = Vault::new();
    
    if !vault.exists() {
        display::error("Vault not initialized. Run 'kookie init' first.");
        return Ok(());
    }
    
    // Get timeout from config or argument
    let config = cache::load_config();
    let timeout_minutes = timeout.unwrap_or(config.timeout_minutes);
    
    // Check if already unlocked
    if let Some(_key) = cache::get_cached_key() {
        display::info("Vault is already unlocked.");
        return Ok(());
    }
    
    // Prompt for password
    let password = input::prompt_password("Enter master password:")?;
    
    // Try to unlock
    match vault.unlock(&password) {
        Ok(()) => {
            // Save session
            if timeout_minutes > 0 {
                // We need to get the key from the vault - but it's private
                // So we'll re-derive it here
                let vault_file = crate::vault::storage::load_vault_file(&vault.path)?;
                let key = crate::crypto::kdf::derive_key(&password, &vault_file.salt)?;
                session::save_session(&key, timeout_minutes)?;
                
                display::success(&format!(
                    "Vault unlocked for {} minutes.",
                    timeout_minutes
                ));
            } else {
                display::success("Vault unlocked (session disabled).");
            }
            Ok(())
        }
        Err(e) => {
            display::error(&format!("Failed to unlock: {}", e));
            Ok(())
        }
    }
}

/// Ensures the vault is unlocked, prompting for password if needed
/// Returns the unlocked vault
pub fn ensure_unlocked() -> Result<Vault, Box<dyn std::error::Error>> {
    let mut vault = Vault::new();
    
    if !vault.exists() {
        return Err("Vault not initialized. Run 'kookie init' first.".into());
    }
    
    // Check for cached session
    if let Some(key) = cache::get_cached_key() {
        // Load vault with cached key
        let vault_file = crate::vault::storage::load_vault_file(&vault.path)?;
        let decrypted = crate::crypto::decrypt(&key, &vault_file.encrypted_data)
            .map_err(|_| "Session expired or corrupted. Please unlock again.")?;
        vault.data = serde_json::from_slice(&decrypted)?;
        return Ok(vault);
    }
    
    // Prompt for password
    let password = input::prompt_password("Enter master password:")?;
    vault.unlock(&password)?;
    
    // Save session for convenience
    let config = cache::load_config();
    if config.timeout_minutes > 0 {
        let vault_file = crate::vault::storage::load_vault_file(&vault.path)?;
        let key = crate::crypto::kdf::derive_key(&password, &vault_file.salt)?;
        session::save_session(&key, config.timeout_minutes)?;
    }
    
    Ok(vault)
}
