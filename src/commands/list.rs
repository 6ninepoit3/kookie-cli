//! List secrets command

use crate::commands::lock::ensure_unlocked;
use crate::utils::display;

/// Type filter for listing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListFilter {
    All,
    Passwords,
    ApiKeys,
    Notes,
    DbCredentials,
    Tokens,
}

/// Runs the list command
pub fn run(filter: ListFilter) -> Result<(), Box<dyn std::error::Error>> {
    let vault = ensure_unlocked()?;
    
    let mut total = 0;
    
    if filter == ListFilter::All || filter == ListFilter::Passwords {
        if !vault.data.passwords.is_empty() {
            display::list_header("Passwords", vault.data.passwords.len());
            for p in &vault.data.passwords {
                display::list_item(&p.id, &p.name, p.username.as_deref());
            }
            total += vault.data.passwords.len();
        }
    }
    
    if filter == ListFilter::All || filter == ListFilter::ApiKeys {
        if !vault.data.api_keys.is_empty() {
            display::list_header("API Keys", vault.data.api_keys.len());
            for k in &vault.data.api_keys {
                display::list_item(&k.id, &k.name, k.service.as_deref());
            }
            total += vault.data.api_keys.len();
        }
    }
    
    if filter == ListFilter::All || filter == ListFilter::Notes {
        if !vault.data.notes.is_empty() {
            display::list_header("Notes", vault.data.notes.len());
            for n in &vault.data.notes {
                display::list_item(&n.id, &n.name, None);
            }
            total += vault.data.notes.len();
        }
    }
    
    if filter == ListFilter::All || filter == ListFilter::DbCredentials {
        if !vault.data.db_credentials.is_empty() {
            display::list_header("Database Credentials", vault.data.db_credentials.len());
            for c in &vault.data.db_credentials {
                let extra = format!("{}@{}", c.username, c.host);
                display::list_item(&c.id, &c.name, Some(&extra));
            }
            total += vault.data.db_credentials.len();
        }
    }
    
    if filter == ListFilter::All || filter == ListFilter::Tokens {
        if !vault.data.tokens.is_empty() {
            display::list_header("Tokens", vault.data.tokens.len());
            for t in &vault.data.tokens {
                let extra = if t.is_expired() { Some("expired") } else { None };
                display::list_item(&t.id, &t.name, extra);
            }
            total += vault.data.tokens.len();
        }
    }
    
    if total == 0 {
        display::info("No secrets found. Use 'kookie add' to add secrets.");
    } else {
        println!();
        display::info(&format!("Total: {} secrets", total));
    }
    
    Ok(())
}
