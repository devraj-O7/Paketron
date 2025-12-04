use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use crate::core::package::InstallerType;

pub struct Installer;

impl Installer {
    pub fn install(path: &PathBuf, installer_type: &InstallerType, args: &Option<Vec<String>>) -> Result<()> {
        let mut command = match installer_type {
            InstallerType::Exe => Command::new(path),
            InstallerType::Msi => {
                let mut cmd = Command::new("msiexec");
                cmd.arg("/i").arg(path);
                cmd
            },
            InstallerType::Zip => {
                println!("Extracting package...");
                let file = std::fs::File::open(path)?;
                let tar = flate2::read::GzDecoder::new(file);
                let mut archive = tar::Archive::new(tar);
                
                // Extract to current directory or a specific location?
                // For now, let's extract to a 'packages' directory in the current folder
                let target_dir = std::env::current_dir()?.join("packages");
                archive.unpack(&target_dir)?;
                
                println!("Extracted to {:?}", target_dir);
                
                // Return the command to run? Or just finish?
                // For NPM packages, there isn't a single "installer" command usually.
                // We'll just return a dummy command that does nothing.
                Command::new("cmd").arg("/c").arg("echo Package extracted")
            }
        };

        if let Some(args) = args {
            command.args(args);
        }

        let status = command.status()?;
        if !status.success() {
            anyhow::bail!("Installation failed");
        }
        Ok(())
    }
}
