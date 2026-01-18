//! Add secrets command

use crate::commands::lock::ensure_unlocked;
use crate::session::cache;
use crate::utils::{display, input};
use crate::vault::types::*;

/// Secret type to add
#[derive(Debug, Clone, Copy)]
pub enum AddType {
    Password,
    ApiKey,
    Note,
    DbCredential,
    Token,
}

/// Runs the add command
pub fn run(secret_type: AddType) -> Result<(), Box<dyn std::error::Error>> {
    let mut vault = ensure_unlocked()?;
    
    match secret_type {
        AddType::Password => add_password(&mut vault)?,
        AddType::ApiKey => add_api_key(&mut vault)?,
        AddType::Note => add_note(&mut vault)?,
        AddType::DbCredential => add_db_credential(&mut vault)?,
        AddType::Token => add_token(&mut vault)?,
    }
    
    Ok(())
}

fn add_password(vault: &mut crate::vault::Vault) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    display::info("Adding new password...");
    println!();
    
    let name = input::prompt_text("Name (e.g., 'github-personal'):")?;
    if name.is_empty() {
        display::error("Name is required.");
        return Ok(());
    }
    
    let description = input::prompt_optional("Description (optional):")?;
    let username = input::prompt_optional("Username (optional):")?;
    let url = input::prompt_optional("URL (optional):")?;
    
    let password = input::prompt_password("Password:")?;
    if password.is_empty() {
        display::error("Password is required.");
        return Ok(());
    }
    
    let secret = Password::new(name.clone(), password, description, username, url);
    vault.add_password(secret)?;
    
    // Refresh session
    refresh_session()?;
    
    display::success(&format!("Password '{}' added successfully!", name));
    Ok(())
}

fn add_api_key(vault: &mut crate::vault::Vault) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    display::info("Adding new API key...");
    println!();
    
    let name = input::prompt_text("Name (e.g., 'stripe-api-key'):")?;
    if name.is_empty() {
        display::error("Name is required.");
        return Ok(());
    }
    
    let description = input::prompt_optional("Description (optional):")?;
    let service = input::prompt_optional("Service (optional, e.g., 'Stripe'):")?;
    
    let key = input::prompt_password("API Key:")?;
    if key.is_empty() {
        display::error("API key is required.");
        return Ok(());
    }
    
    let secret = ApiKey::new(name.clone(), key, description, service);
    vault.add_api_key(secret)?;
    
    refresh_session()?;
    
    display::success(&format!("API key '{}' added successfully!", name));
    Ok(())
}

fn add_note(vault: &mut crate::vault::Vault) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    display::info("Adding new private note...");
    println!();
    
    let name = input::prompt_text("Name (e.g., 'recovery-codes'):")?;
    if name.is_empty() {
        display::error("Name is required.");
        return Ok(());
    }
    
    println!("Content (end with an empty line):");
    let mut content = String::new();
    loop {
        let line = input::prompt_text("")?;
        if line.is_empty() {
            break;
        }
        content.push_str(&line);
        content.push('\n');
    }
    
    if content.trim().is_empty() {
        display::error("Content is required.");
        return Ok(());
    }
    
    let secret = Note::new(name.clone(), content.trim().to_string());
    vault.add_note(secret)?;
    
    refresh_session()?;
    
    display::success(&format!("Note '{}' added successfully!", name));
    Ok(())
}

fn add_db_credential(vault: &mut crate::vault::Vault) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    display::info("Adding new database credential...");
    println!();
    
    let name = input::prompt_text("Name (e.g., 'prod-postgres'):")?;
    if name.is_empty() {
        display::error("Name is required.");
        return Ok(());
    }
    
    let description = input::prompt_optional("Description (optional):")?;
    let db_type = input::prompt_optional("Database type (postgres/mysql/mongodb):")?;
    
    let host = input::prompt_text("Host:")?;
    if host.is_empty() {
        display::error("Host is required.");
        return Ok(());
    }
    
    let port_str = input::prompt_optional("Port (optional):")?;
    let port = port_str.and_then(|p| p.parse().ok());
    
    let database = input::prompt_text("Database name:")?;
    if database.is_empty() {
        display::error("Database name is required.");
        return Ok(());
    }
    
    let username = input::prompt_text("Username:")?;
    if username.is_empty() {
        display::error("Username is required.");
        return Ok(());
    }
    
    let password = input::prompt_password("Password:")?;
    if password.is_empty() {
        display::error("Password is required.");
        return Ok(());
    }
    
    let secret = DbCredential::new(
        name.clone(),
        host,
        port,
        database,
        username,
        password,
        db_type,
        description,
    );
    vault.add_db_credential(secret)?;
    
    refresh_session()?;
    
    display::success(&format!("Database credential '{}' added successfully!", name));
    Ok(())
}

fn add_token(vault: &mut crate::vault::Vault) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    display::info("Adding new token...");
    println!();
    
    let name = input::prompt_text("Name (e.g., 'jwt-secret'):")?;
    if name.is_empty() {
        display::error("Name is required.");
        return Ok(());
    }
    
    let description = input::prompt_optional("Description (optional):")?;
    let token_type = input::prompt_optional("Token type (jwt/oauth/bearer):")?;
    
    let token = input::prompt_password("Token:")?;
    if token.is_empty() {
        display::error("Token is required.");
        return Ok(());
    }
    
    // TODO: Add expiration date parsing
    let expires_at = None;
    
    let secret = Token::new(name.clone(), token, description, token_type, expires_at);
    vault.add_token(secret)?;
    
    refresh_session()?;
    
    display::success(&format!("Token '{}' added successfully!", name));
    Ok(())
}

fn refresh_session() -> Result<(), Box<dyn std::error::Error>> {
    // Re-save session to extend timeout
    if let Some(key) = cache::get_cached_key() {
        let config = cache::load_config();
        cache::save_session(&key, config.timeout_minutes)?;
    }
    Ok(())
}
