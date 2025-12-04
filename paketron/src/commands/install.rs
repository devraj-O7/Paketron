use crate::core::package::{Package, Installer, InstallerType};
use crate::core::registry::{Registry, LocalRegistry};
use crate::core::downloader::Downloader;
use crate::core::installer::Installer as CoreInstaller;
use anyhow::{Result, anyhow};
use std::path::PathBuf;

pub async fn install(name: &str) -> Result<()> {
    println!("Installing {}...", name);

    // 1. Initialize Registry
    // In a real app, this path should be determined dynamically
    let registry_path = PathBuf::from("C:\\ProgramData\\Paketron\\registry.json");
    let mut registry = LocalRegistry::new(registry_path)?;

    if registry.get_package(name).is_some() {
        println!("Package {} is already installed.", name);
        return Ok(());
    }

    // 2. Resolve Package (Mock Remote)
    let package = resolve_remote_package(name).ok_or_else(|| anyhow!("Package not found"))?;

    // 3. Download
    let installer = package.installer.as_ref().ok_or_else(|| anyhow!("No installer found"))?;
    let temp_dir = std::env::temp_dir();
    let target_path = temp_dir.join(format!("{}-installer.exe", name));
    
    println!("Downloading from {}...", installer.url);
    Downloader::download(&installer.url, &target_path).await?;

    // 4. Verify (Skip for now)
    // verify_checksum(&target_path, &installer.sha256)?;

    // 5. Install
    println!("Executing installer...");
    CoreInstaller::install(&target_path, &installer.installer_type, &installer.args)?;

    // 6. Update Registry
    registry.add_package(package)?;
    println!("Successfully installed {}", name);

    Ok(())
}

fn resolve_remote_package(name: &str) -> Option<Package> {
    // Mock data
    if name == "7zip" {
        Some(Package {
            name: "7zip".to_string(),
            version: "23.01".to_string(),
            description: Some("7-Zip file archiver".to_string()),
            license: Some("LGPL".to_string()),
            homepage: Some("https://7-zip.org".to_string()),
            installer: Some(Installer {
                url: "https://www.7-zip.org/a/7z2301-x64.exe".to_string(),
                sha256: "mock_sha256".to_string(),
                installer_type: InstallerType::Exe,
                args: Some(vec!["/S".to_string()]),
            }),
        })
    } else {
        None
    }
}
