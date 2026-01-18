//! Initialize vault command

use crate::utils::{display, input};
use crate::vault::Vault;

/// Runs the init command
pub fn run(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut vault = Vault::new();
    
    if vault.exists() && !force {
        display::error("Vault already exists at ~/.kookie/vault.json");
        display::info("Use --force to reinitialize (this will delete all secrets!)");
        return Ok(());
    }
    
    if vault.exists() && force {
        display::warning("This will delete all existing secrets!");
        if !input::prompt_confirm("Are you sure you want to continue?", false)? {
            display::info("Aborted.");
            return Ok(());
        }
    }
    
    println!();
    display::info("Initializing new kookie vault...");
    println!();
    
    // Prompt for master password
    let password = input::prompt_new_password("Enter master password:")?;
    
    // Initialize vault
    if force {
        vault.init_force(&password)?;
    } else {
        vault.init(&password)?;
    }
    
    println!();
    display::success("Vault initialized successfully!");
    display::info("Your encrypted vault is stored at ~/.kookie/vault.json");
    display::info("Remember your master password - it cannot be recovered!");
    
    Ok(())
}
