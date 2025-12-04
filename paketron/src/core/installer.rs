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
                // TODO: Implement zip extraction
                println!("Zip extraction not yet implemented");
                return Ok(());
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
