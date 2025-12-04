use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub installer: Option<Installer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Installer {
    pub url: String,
    pub sha256: String,
    #[serde(rename = "type")]
    pub installer_type: InstallerType,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InstallerType {
    Exe,
    Msi,
    Zip,
}
