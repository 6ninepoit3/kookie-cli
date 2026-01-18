//! Self-install command
//!
//! Allows kookie to install itself to the system PATH

use crate::utils::display;
use std::env;
use std::fs;
use std::path::PathBuf;

#[cfg(windows)]
use std::process::Command;

/// Gets the install directory based on OS
fn get_install_dir() -> PathBuf {
    #[cfg(windows)]
    {
        // Windows: %LOCALAPPDATA%\kookie
        if let Some(local_app_data) = env::var_os("LOCALAPPDATA") {
            return PathBuf::from(local_app_data).join("kookie");
        }
        // Fallback
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".kookie")
            .join("bin")
    }
    
    #[cfg(not(windows))]
    {
        // Linux/macOS: ~/.local/bin
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".local")
            .join("bin")
    }
}

/// Gets the binary name for the current OS
fn get_binary_name() -> &'static str {
    #[cfg(windows)]
    {
        "kookie.exe"
    }
    #[cfg(not(windows))]
    {
        "kookie"
    }
}

/// Adds a directory to the user's PATH on Windows
#[cfg(windows)]
fn add_to_path_windows(install_dir: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    use winreg::enums::*;
    use winreg::RegKey;
    
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
    
    let current_path: String = env_key.get_value("Path").unwrap_or_default();
    let install_dir_str = install_dir.to_string_lossy();
    
    // Check if already in PATH
    if current_path.to_lowercase().contains(&install_dir_str.to_lowercase()) {
        return Ok(false); // Already in PATH
    }
    
    // Add to PATH
    let new_path = if current_path.is_empty() {
        install_dir_str.to_string()
    } else {
        format!("{};{}", current_path, install_dir_str)
    };
    
    env_key.set_value("Path", &new_path)?;
    
    // Broadcast WM_SETTINGCHANGE to notify other applications
    // This is done via PowerShell since we don't want to add Windows API deps
    let _ = Command::new("powershell")
        .args([
            "-Command",
            "[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User'), 'User')"
        ])
        .output();
    
    Ok(true)
}

/// Adds to PATH on Unix systems
#[cfg(not(windows))]
fn add_to_path_unix(install_dir: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let install_dir_str = install_dir.to_string_lossy();
    
    // Check if already in PATH
    if let Ok(path) = env::var("PATH") {
        if path.contains(&*install_dir_str) {
            return Ok(false);
        }
    }
    
    // Determine shell config file
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let shell = env::var("SHELL").unwrap_or_default();
    
    let config_file = if shell.contains("zsh") {
        home.join(".zshrc")
    } else if shell.contains("fish") {
        home.join(".config").join("fish").join("config.fish")
    } else {
        // Default to bashrc
        home.join(".bashrc")
    };
    
    // Read current config
    let current_content = fs::read_to_string(&config_file).unwrap_or_default();
    
    // Check if already configured
    if current_content.contains(".local/bin") || current_content.contains(&*install_dir_str) {
        return Ok(false);
    }
    
    // Append to config
    let line = if shell.contains("fish") {
        format!("\n# Kookie CLI\nset -gx PATH {} $PATH\n", install_dir_str)
    } else {
        format!("\n# Kookie CLI\nexport PATH=\"{}:$PATH\"\n", install_dir_str)
    };
    
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config_file)?;
    
    use std::io::Write;
    file.write_all(line.as_bytes())?;
    
    Ok(true)
}

/// Runs the install command
pub fn run(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    display::info("🍪 Installing Kookie CLI...");
    println!();
    
    // Get current executable path
    let current_exe = env::current_exe()?;
    let install_dir = get_install_dir();
    let binary_name = get_binary_name();
    let dest_path = install_dir.join(binary_name);
    
    // Check if already installed in the right place
    if current_exe == dest_path {
        display::success("Kookie is already installed!");
        println!("  Location: {}", dest_path.display());
        return Ok(());
    }
    
    // Check if destination exists
    if dest_path.exists() && !force {
        display::warning(&format!("Kookie already exists at {}", dest_path.display()));
        display::info("Use 'kookie install --force' to overwrite.");
        return Ok(());
    }
    
    // Create install directory
    if !install_dir.exists() {
        println!("📁 Creating directory: {}", install_dir.display());
        fs::create_dir_all(&install_dir)?;
    }
    
    // Copy binary
    println!("📋 Copying to: {}", dest_path.display());
    fs::copy(&current_exe, &dest_path)?;
    
    // Make executable on Unix
    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest_path, perms)?;
    }
    
    // Add to PATH
    #[cfg(windows)]
    let path_updated = add_to_path_windows(&install_dir)?;
    
    #[cfg(not(windows))]
    let path_updated = add_to_path_unix(&install_dir)?;
    
    if path_updated {
        display::success("Added to PATH!");
    } else {
        display::info("Already in PATH.");
    }
    
    println!();
    display::success("🎉 Installation complete!");
    println!();
    println!("  Installed to: {}", dest_path.display());
    println!();
    
    #[cfg(windows)]
    {
        display::warning("Please restart your terminal for PATH changes to take effect.");
    }
    
    #[cfg(not(windows))]
    {
        display::info("Run 'source ~/.bashrc' or restart your terminal.");
    }
    
    println!();
    println!("Get started:");
    println!("  kookie init            # Initialize your vault");
    println!("  kookie add --password  # Add a password");
    println!("  kookie --help          # See all commands");
    println!();
    
    Ok(())
}

/// Runs the uninstall command
pub fn uninstall() -> Result<(), Box<dyn std::error::Error>> {
    let install_dir = get_install_dir();
    let binary_name = get_binary_name();
    let installed_path = install_dir.join(binary_name);
    
    if !installed_path.exists() {
        display::info("Kookie is not installed in the standard location.");
        return Ok(());
    }
    
    println!();
    display::warning(&format!("Removing: {}", installed_path.display()));
    
    fs::remove_file(&installed_path)?;
    
    // Try to remove directory if empty
    if let Ok(entries) = fs::read_dir(&install_dir) {
        if entries.count() == 0 {
            let _ = fs::remove_dir(&install_dir);
        }
    }
    
    display::success("Kookie has been uninstalled.");
    display::info("Your vault data in ~/.kookie is preserved.");
    display::info("You may need to manually remove the PATH entry.");
    
    Ok(())
}
