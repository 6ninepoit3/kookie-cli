//! Kookie - A secure, local-first, encrypted secret manager for developers
//!
//! # Usage
//!
//! ```bash
//! # Initialize vault
//! kookie init
//!
//! # Add secrets
//! kookie add --password
//! kookie add --api-key
//! kookie add --note
//! kookie add --db
//! kookie add --token
//!
//! # List secrets
//! kookie list
//! kookie list --passwords
//!
//! # Get a secret
//! kookie get <name-or-id>
//! kookie get <name-or-id> --copy
//!
//! # Delete a secret
//! kookie delete <name-or-id>
//!
//! # Lock/unlock
//! kookie lock
//! kookie unlock
//!
//! # Generate secrets
//! kookie generate jwt
//! kookie generate key --length 32
//! kookie generate password --length 16
//!
//! # Configure
//! kookie config --timeout 10
//! kookie config --show
//! ```

use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod crypto;
mod session;
mod utils;
mod vault;

/// 🍪 Kookie - A secure, local-first, encrypted secret manager for developers
#[derive(Parser)]
#[command(name = "kookie")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new encrypted vault
    Init {
        /// Force reinitialization (deletes existing vault)
        #[arg(short, long)]
        force: bool,
    },
    
    /// Lock the vault (clear session)
    Lock,
    
    /// Unlock the vault for a duration
    Unlock {
        /// Timeout in minutes (overrides config)
        #[arg(short, long)]
        timeout: Option<u32>,
    },
    
    /// Add a new secret
    Add {
        /// Add a password
        #[arg(long, group = "secret_type")]
        password: bool,
        
        /// Add an API key
        #[arg(long, group = "secret_type")]
        api_key: bool,
        
        /// Add a private note
        #[arg(long, group = "secret_type")]
        note: bool,
        
        /// Add database credentials
        #[arg(long, group = "secret_type")]
        db: bool,
        
        /// Add a token
        #[arg(long, group = "secret_type")]
        token: bool,
    },
    
    /// List stored secrets
    List {
        /// Show only passwords
        #[arg(long)]
        passwords: bool,
        
        /// Show only API keys
        #[arg(long)]
        api_keys: bool,
        
        /// Show only notes
        #[arg(long)]
        notes: bool,
        
        /// Show only database credentials
        #[arg(long)]
        db: bool,
        
        /// Show only tokens
        #[arg(long)]
        tokens: bool,
    },
    
    /// Get a specific secret by name or ID
    Get {
        /// Name or ID of the secret
        name_or_id: String,
        
        /// Copy the secret value to clipboard
        #[arg(short, long)]
        copy: bool,
    },
    
    /// Delete a secret
    Delete {
        /// Name or ID of the secret
        name_or_id: String,
        
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
    
    /// Generate random secrets
    Generate {
        #[command(subcommand)]
        gen_type: GenerateType,
    },
    
    /// Configure kookie settings
    Config {
        /// Set unlock timeout in minutes (0 to disable)
        #[arg(short, long)]
        timeout: Option<u32>,
        
        /// Show current configuration
        #[arg(short, long)]
        show: bool,
    },
    
    /// Install kookie to system PATH
    Install {
        /// Force overwrite if already installed
        #[arg(short, long)]
        force: bool,
    },
    
    /// Uninstall kookie from system
    Uninstall,
}

#[derive(Subcommand)]
enum GenerateType {
    /// Generate a JWT secret (256-bit)
    Jwt {
        /// Copy to clipboard
        #[arg(short, long)]
        copy: bool,
    },
    
    /// Generate a random key
    Key {
        /// Length in bytes (default: 32)
        #[arg(short, long)]
        length: Option<usize>,
        
        /// Copy to clipboard
        #[arg(short, long)]
        copy: bool,
    },
    
    /// Generate a random password
    Password {
        /// Length in characters (default: 16)
        #[arg(short, long)]
        length: Option<usize>,
        
        /// Include symbols
        #[arg(short, long)]
        symbols: bool,
        
        /// Copy to clipboard
        #[arg(short, long)]
        copy: bool,
    },
    
    /// Generate an API key with kk_ prefix
    #[command(name = "api-key")]
    ApiKey {
        /// Copy to clipboard
        #[arg(short, long)]
        copy: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Init { force } => commands::init::run(force),
        
        Commands::Lock => commands::lock::lock(),
        
        Commands::Unlock { timeout } => commands::lock::unlock(timeout),
        
        Commands::Add { password, api_key, note, db, token } => {
            let add_type = if password {
                commands::add::AddType::Password
            } else if api_key {
                commands::add::AddType::ApiKey
            } else if note {
                commands::add::AddType::Note
            } else if db {
                commands::add::AddType::DbCredential
            } else if token {
                commands::add::AddType::Token
            } else {
                println!("{}", "Please specify a secret type:".yellow());
                println!("  kookie add --password");
                println!("  kookie add --api-key");
                println!("  kookie add --note");
                println!("  kookie add --db");
                println!("  kookie add --token");
                return;
            };
            commands::add::run(add_type)
        }
        
        Commands::List { passwords, api_keys, notes, db, tokens } => {
            let filter = if passwords {
                commands::list::ListFilter::Passwords
            } else if api_keys {
                commands::list::ListFilter::ApiKeys
            } else if notes {
                commands::list::ListFilter::Notes
            } else if db {
                commands::list::ListFilter::DbCredentials
            } else if tokens {
                commands::list::ListFilter::Tokens
            } else {
                commands::list::ListFilter::All
            };
            commands::list::run(filter)
        }
        
        Commands::Get { name_or_id, copy } => commands::get::run(&name_or_id, copy),
        
        Commands::Delete { name_or_id, force } => commands::delete::run(&name_or_id, force),
        
        Commands::Generate { gen_type } => {
            match gen_type {
                GenerateType::Jwt { copy } => {
                    commands::generate::run(commands::generate::GenerateType::Jwt, None, copy, false)
                }
                GenerateType::Key { length, copy } => {
                    commands::generate::run(commands::generate::GenerateType::Key, length, copy, false)
                }
                GenerateType::Password { length, symbols, copy } => {
                    commands::generate::run(commands::generate::GenerateType::Password, length, copy, symbols)
                }
                GenerateType::ApiKey { copy } => {
                    commands::generate::run(commands::generate::GenerateType::ApiKey, None, copy, false)
                }
            }
        }
        
        Commands::Config { timeout, show } => commands::config::run(timeout, show),
        
        Commands::Install { force } => commands::install::run(force),
        
        Commands::Uninstall => commands::install::uninstall(),
    };
    
    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
