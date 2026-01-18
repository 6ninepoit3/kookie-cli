//! Delete secret command

use crate::commands::lock::ensure_unlocked;
use crate::utils::{display, input};

/// Runs the delete command
pub fn run(id_or_name: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut vault = ensure_unlocked()?;
    
    // Check if secret exists
    let secret_type = if vault.get_password(id_or_name).is_some() {
        Some("password")
    } else if vault.get_api_key(id_or_name).is_some() {
        Some("API key")
    } else if vault.get_note(id_or_name).is_some() {
        Some("note")
    } else if vault.get_db_credential(id_or_name).is_some() {
        Some("database credential")
    } else if vault.get_token(id_or_name).is_some() {
        Some("token")
    } else {
        None
    };
    
    let secret_type = match secret_type {
        Some(t) => t,
        None => {
            display::error(&format!("Secret '{}' not found.", id_or_name));
            return Ok(());
        }
    };
    
    // Confirm deletion
    if !force {
        display::warning(&format!(
            "You are about to delete the {} '{}'",
            secret_type, id_or_name
        ));
        if !input::prompt_confirm("Are you sure?", false)? {
            display::info("Aborted.");
            return Ok(());
        }
    }
    
    // Delete based on type
    let deleted_name = if vault.get_password(id_or_name).is_some() {
        vault.delete_password(id_or_name)?.name
    } else if vault.get_api_key(id_or_name).is_some() {
        vault.delete_api_key(id_or_name)?.name
    } else if vault.get_note(id_or_name).is_some() {
        vault.delete_note(id_or_name)?.name
    } else if vault.get_db_credential(id_or_name).is_some() {
        vault.delete_db_credential(id_or_name)?.name
    } else if vault.get_token(id_or_name).is_some() {
        vault.delete_token(id_or_name)?.name
    } else {
        return Ok(());
    };
    
    display::success(&format!("Deleted {} '{}'", secret_type, deleted_name));
    
    Ok(())
}
