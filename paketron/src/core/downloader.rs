use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

pub struct Downloader;

impl Downloader {
    pub async fn download(url: &str, target_path: &PathBuf) -> Result<()> {
        let response = reqwest::get(url).await?;
        let content = response.bytes().await?;
        let mut file = File::create(target_path)?;
        file.write_all(&content)?;
        Ok(())
    }
}
