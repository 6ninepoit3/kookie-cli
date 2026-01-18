//! Get secret command

use crate::commands::lock::ensure_unlocked;
use crate::utils::{clipboard, display};

/// Runs the get command
pub fn run(id_or_name: &str, copy: bool) -> Result<(), Box<dyn std::error::Error>> {
    let vault = ensure_unlocked()?;
    
    // Search in all secret types
    if let Some(p) = vault.get_password(id_or_name) {
        display::display_password(p, true);
        if copy {
            clipboard::copy_to_clipboard(&p.password)?;
            display::success("Password copied to clipboard!");
        }
        return Ok(());
    }
    
    if let Some(k) = vault.get_api_key(id_or_name) {
        display::display_api_key(k, true);
        if copy {
            clipboard::copy_to_clipboard(&k.key)?;
            display::success("API key copied to clipboard!");
        }
        return Ok(());
    }
    
    if let Some(n) = vault.get_note(id_or_name) {
        display::display_note(n, true);
        if copy {
            clipboard::copy_to_clipboard(&n.content)?;
            display::success("Note content copied to clipboard!");
        }
        return Ok(());
    }
    
    if let Some(c) = vault.get_db_credential(id_or_name) {
        display::display_db_credential(c, true);
        if copy {
            clipboard::copy_to_clipboard(&c.connection_string())?;
            display::success("Connection string copied to clipboard!");
        }
        return Ok(());
    }
    
    if let Some(t) = vault.get_token(id_or_name) {
        display::display_token(t, true);
        if copy {
            clipboard::copy_to_clipboard(&t.token)?;
            display::success("Token copied to clipboard!");
        }
        return Ok(());
    }
    
    display::error(&format!("Secret '{}' not found.", id_or_name));
    display::info("Use 'kookie list' to see all secrets.");
    
    Ok(())
}
