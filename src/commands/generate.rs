//! Generate secrets command

use crate::utils::{clipboard, display, generators};

/// Type of key to generate
#[derive(Debug, Clone, Copy)]
pub enum GenerateType {
    Jwt,
    Key,
    Password,
    ApiKey,
}

/// Runs the generate command
pub fn run(gen_type: GenerateType, length: Option<usize>, copy: bool, symbols: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (label, value) = match gen_type {
        GenerateType::Jwt => {
            let secret = generators::generate_jwt_secret();
            ("JWT Secret (256-bit)".to_string(), secret)
        }
        GenerateType::Key => {
            let len = length.unwrap_or(32);
            let key = generators::generate_random_key(len);
            (format!("Random Key ({} bytes)", len), key)
        }
        GenerateType::Password => {
            let len = length.unwrap_or(16);
            let password = generators::generate_password(len, symbols);
            (format!("Random Password ({} chars)", len), password)
        }
        GenerateType::ApiKey => {
            let key = generators::generate_api_key();
            ("API Key".to_string(), key)
        }
    };
    
    println!();
    println!("{}: {}", label, value);
    println!();
    
    if copy {
        clipboard::copy_to_clipboard(&value)?;
        display::success("Copied to clipboard!");
    }
    
    Ok(())
}
