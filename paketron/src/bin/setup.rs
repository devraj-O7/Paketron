use std::env;
use std::fs;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    println!("Paketron Installer");
    println!("==================");

    // 1. Determine Installation Directory
    let program_files = env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string());
    let install_dir = PathBuf::from(program_files).join("Paketron");
    
    println!("Installing to: {:?}", install_dir);

    if !install_dir.exists() {
        fs::create_dir_all(&install_dir).context("Failed to create installation directory")?;
    }

    // 2. Copy Binary
    // Assumes paketron.exe is in the same directory as this installer
    let current_exe = env::current_exe()?;
    let current_dir = current_exe.parent().unwrap();
    let source_path = current_dir.join("paketron.exe");
    let target_path = install_dir.join("paketron.exe");

    if source_path.exists() {
        println!("Copying paketron.exe...");
        fs::copy(&source_path, &target_path).context("Failed to copy executable")?;
    } else {
        println!("Warning: paketron.exe not found in current directory. Skipping copy.");
        // For development testing, we might just copy the installer itself as a placeholder if paketron.exe is missing
        // or just warn.
    }

    // 3. Update PATH
    println!("Updating PATH environment variable...");
    add_to_path(&install_dir)?;

    println!("Installation successful!");
    println!("Please restart your terminal to use 'paketron'.");
    
    // Keep window open
    println!("Press Enter to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());

    Ok(())
}

fn add_to_path(path: &PathBuf) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
    
    let current_path: String = env_key.get_value("Path")?;
    let path_str = path.to_str().unwrap();

    if !current_path.contains(path_str) {
        let new_path = format!("{};{}", current_path, path_str);
        env_key.set_value("Path", &new_path)?;
        println!("Added to PATH.");
        
        // Notify system of environment change (optional, but good practice)
        // unsafe {
        //     use windows::Win32::UI::WindowsAndMessaging::{SendMessageTimeoutA, HWND_BROADCAST, WM_SETTINGCHANGE, SMTO_ABORTIFHUNG};
        //     let lparam = std::ffi::CString::new("Environment").unwrap();
        //     SendMessageTimeoutA(HWND_BROADCAST, WM_SETTINGCHANGE, 0, windows::core::PCSTR(lparam.as_ptr() as *const u8), SMTO_ABORTIFHUNG, 5000, None);
        // }
    } else {
        println!("Already in PATH.");
    }

    Ok(())
}
