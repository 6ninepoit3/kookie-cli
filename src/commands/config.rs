//! Configuration command

use crate::session::cache::{self, SessionConfig};
use crate::utils::display;

/// Runs the config command
pub fn run(timeout: Option<u32>, show: bool) -> Result<(), Box<dyn std::error::Error>> {
    if show {
        let config = cache::load_config();
        println!();
        display::info("Current configuration:");
        println!("  Unlock timeout: {} minutes", config.timeout_minutes);
        println!();
        return Ok(());
    }
    
    if let Some(minutes) = timeout {
        let config = SessionConfig {
            timeout_minutes: minutes,
        };
        cache::save_config(&config)?;
        
        if minutes == 0 {
            display::success("Timeout disabled. Password will be required for every operation.");
        } else {
            display::success(&format!(
                "Unlock timeout set to {} minutes.",
                minutes
            ));
        }
    } else {
        display::info("Usage: kookie config --timeout <minutes>");
        display::info("       kookie config --show");
    }
    
    Ok(())
}
