use crate::core::package::Package;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub trait Registry {
    fn get_package(&self, name: &str) -> Option<Package>;
    fn list_packages(&self) -> Vec<Package>;
    fn add_package(&mut self, package: Package) -> Result<()>;
    fn remove_package(&mut self, name: &str) -> Result<()>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RegistryData {
    packages: Vec<Package>,
}

pub struct LocalRegistry {
    path: PathBuf,
    packages: Vec<Package>,
}

impl LocalRegistry {
    pub fn new(path: PathBuf) -> Result<Self> {
        let packages = if path.exists() {
            let content = fs::read_to_string(&path)?;
            let data: RegistryData = serde_json::from_str(&content)?;
            data.packages
        } else {
            Vec::new()
        };

        Ok(Self { path, packages })
    }

    fn save(&self) -> Result<()> {
        let data = RegistryData {
            packages: self.packages.clone(),
        };
        let content = serde_json::to_string_pretty(&data)?;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, content)?;
        Ok(())
    }
}

impl Registry for LocalRegistry {
    fn get_package(&self, name: &str) -> Option<Package> {
        self.packages.iter().find(|p| p.name == name).cloned()
    }

    fn list_packages(&self) -> Vec<Package> {
        self.packages.clone()
    }

    fn add_package(&mut self, package: Package) -> Result<()> {
        if let Some(pos) = self.packages.iter().position(|p| p.name == package.name) {
            self.packages[pos] = package;
        } else {
            self.packages.push(package);
        }
        self.save()
    }

    fn remove_package(&mut self, name: &str) -> Result<()> {
        if let Some(pos) = self.packages.iter().position(|p| p.name == name) {
            self.packages.remove(pos);
            self.save()
        } else {
            Ok(())
        }
    }
}
