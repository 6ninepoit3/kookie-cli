//! Display utilities for formatting output

use crate::vault::types::*;
use colored::*;

/// Prints a success message
pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

/// Prints an error message
pub fn error(msg: &str) {
    println!("{} {}", "✗".red().bold(), msg);
}

/// Prints a warning message
pub fn warning(msg: &str) {
    println!("{} {}", "!".yellow().bold(), msg);
}

/// Prints an info message
pub fn info(msg: &str) {
    println!("{} {}", "ℹ".blue().bold(), msg);
}

/// Prints a secret value (masked by default)
pub fn print_secret(label: &str, value: &str, show: bool) {
    let display = if show {
        value.to_string()
    } else {
        "••••••••".to_string()
    };
    println!("  {}: {}", label.dimmed(), display.yellow());
}

/// Formats a password for display
pub fn display_password(password: &Password, show_secret: bool) {
    println!();
    println!("{}", "═".repeat(50).dimmed());
    println!("{} {}", "ID:".dimmed(), password.id.cyan());
    println!("{} {}", "Name:".dimmed(), password.name.white().bold());
    
    if let Some(desc) = &password.description {
        println!("{} {}", "Description:".dimmed(), desc);
    }
    if let Some(username) = &password.username {
        println!("{} {}", "Username:".dimmed(), username.green());
    }
    
    print_secret("Password", &password.password, show_secret);
    
    if let Some(url) = &password.url {
        println!("{} {}", "URL:".dimmed(), url.blue().underline());
    }
    
    println!("{} {}", "Created:".dimmed(), password.created_at.format("%Y-%m-%d %H:%M"));
    println!("{}", "═".repeat(50).dimmed());
}

/// Formats an API key for display
pub fn display_api_key(api_key: &ApiKey, show_secret: bool) {
    println!();
    println!("{}", "═".repeat(50).dimmed());
    println!("{} {}", "ID:".dimmed(), api_key.id.cyan());
    println!("{} {}", "Name:".dimmed(), api_key.name.white().bold());
    
    if let Some(desc) = &api_key.description {
        println!("{} {}", "Description:".dimmed(), desc);
    }
    if let Some(service) = &api_key.service {
        println!("{} {}", "Service:".dimmed(), service.green());
    }
    
    print_secret("Key", &api_key.key, show_secret);
    
    println!("{} {}", "Created:".dimmed(), api_key.created_at.format("%Y-%m-%d %H:%M"));
    println!("{}", "═".repeat(50).dimmed());
}

/// Formats a note for display
pub fn display_note(note: &Note, show_content: bool) {
    println!();
    println!("{}", "═".repeat(50).dimmed());
    println!("{} {}", "ID:".dimmed(), note.id.cyan());
    println!("{} {}", "Name:".dimmed(), note.name.white().bold());
    
    if show_content {
        println!("{}", "Content:".dimmed());
        println!("{}", note.content.yellow());
    } else {
        println!("{} {}", "Content:".dimmed(), "••••••••".yellow());
    }
    
    println!("{} {}", "Created:".dimmed(), note.created_at.format("%Y-%m-%d %H:%M"));
    println!("{}", "═".repeat(50).dimmed());
}

/// Formats a database credential for display
pub fn display_db_credential(cred: &DbCredential, show_secret: bool) {
    println!();
    println!("{}", "═".repeat(50).dimmed());
    println!("{} {}", "ID:".dimmed(), cred.id.cyan());
    println!("{} {}", "Name:".dimmed(), cred.name.white().bold());
    
    if let Some(desc) = &cred.description {
        println!("{} {}", "Description:".dimmed(), desc);
    }
    if let Some(db_type) = &cred.db_type {
        println!("{} {}", "Type:".dimmed(), db_type.green());
    }
    
    println!("{} {}", "Host:".dimmed(), cred.host);
    if let Some(port) = cred.port {
        println!("{} {}", "Port:".dimmed(), port);
    }
    println!("{} {}", "Database:".dimmed(), cred.database);
    println!("{} {}", "Username:".dimmed(), cred.username.green());
    
    print_secret("Password", &cred.password, show_secret);
    
    if show_secret {
        println!("{} {}", "Connection String:".dimmed(), cred.connection_string().blue());
    }
    
    println!("{} {}", "Created:".dimmed(), cred.created_at.format("%Y-%m-%d %H:%M"));
    println!("{}", "═".repeat(50).dimmed());
}

/// Formats a token for display
pub fn display_token(token: &Token, show_secret: bool) {
    println!();
    println!("{}", "═".repeat(50).dimmed());
    println!("{} {}", "ID:".dimmed(), token.id.cyan());
    println!("{} {}", "Name:".dimmed(), token.name.white().bold());
    
    if let Some(desc) = &token.description {
        println!("{} {}", "Description:".dimmed(), desc);
    }
    if let Some(token_type) = &token.token_type {
        println!("{} {}", "Type:".dimmed(), token_type.green());
    }
    
    print_secret("Token", &token.token, show_secret);
    
    if let Some(expires) = token.expires_at {
        let status = if token.is_expired() {
            "EXPIRED".red()
        } else {
            "valid".green()
        };
        println!("{} {} ({})", "Expires:".dimmed(), expires.format("%Y-%m-%d %H:%M"), status);
    }
    
    println!("{} {}", "Created:".dimmed(), token.created_at.format("%Y-%m-%d %H:%M"));
    println!("{}", "═".repeat(50).dimmed());
}

/// Prints a list header
pub fn list_header(secret_type: &str, count: usize) {
    println!();
    println!("{} {} ({})", "📋".cyan(), secret_type.white().bold(), count);
    println!("{}", "─".repeat(50).dimmed());
}

/// Prints a list item summary
pub fn list_item(id: &str, name: &str, extra: Option<&str>) {
    print!("  {} ", "•".dimmed());
    print!("{} ", name.white().bold());
    if let Some(e) = extra {
        print!("{} ", format!("({})", e).dimmed());
    }
    println!("{}", format!("[{}]", &id[..8]).cyan().dimmed());
}
