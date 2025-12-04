use anyhow::{Result, anyhow};
use crate::core::registry::{Registry, LocalRegistry};
use std::path::PathBuf;

pub async fn uninstall(name: &str) -> Result<()> {
    println!("Uninstalling {}...", name);

    // 1. Initialize Registry
    let registry_path = PathBuf::from("C:\\ProgramData\\Paketron\\registry.json");
    let mut registry = LocalRegistry::new(registry_path)?;

    // 2. Check if installed
    if registry.get_package(name).is_none() {
        println!("Package {} is not installed.", name);
        return Ok(());
    }

    // 3. Execute Uninstaller (Mock)
    // In a real app, we would look up the uninstall string or run the installer with /uninstall
    println!("Executing uninstaller for {}...", name);
    // CoreInstaller::uninstall(...)

    // 4. Remove from Registry
    registry.remove_package(name)?;
    println!("Successfully uninstalled {}", name);

    Ok(())
}
