use anyhow::Result;
use crate::core::package::Package;

use crate::core::npm::NpmClient;

pub async fn search(query: &str) -> Result<()> {
    println!("Searching for '{}' on NPM...", query);

    let packages = NpmClient::search(query).await?;

    if packages.is_empty() {
        println!("No packages found matching '{}'", query);
    } else {
        println!("{:<20} {:<10} {}", "Name", "Version", "Description");
        println!("{:-<20} {:-<10} {:-<40}", "", "", "");
        for pkg in packages {
            println!(
                "{:<20} {:<10} {}",
                pkg.name
                    .chars()
                    .take(20)
                    .collect::<String>(), // Truncate name
                pkg.version,
                pkg.description
                    .unwrap_or_default()
                    .chars()
                    .take(40)
                    .collect::<String>() // Truncate description
            );
        }
    }

    Ok(())
}

fn mock_search_remote(query: &str) -> Vec<Package> {
    let db = vec![
        Package {
            name: "7zip".to_string(),
            version: "23.01".to_string(),
            description: Some("7-Zip file archiver".to_string()),
            license: Some("LGPL".to_string()),
            homepage: Some("https://7-zip.org".to_string()),
            installer: None,
        },
        Package {
            name: "vscode".to_string(),
            version: "1.85.0".to_string(),
            description: Some("Visual Studio Code".to_string()),
            license: Some("MIT".to_string()),
            homepage: Some("https://code.visualstudio.com".to_string()),
            installer: None,
        },
        Package {
            name: "rust".to_string(),
            version: "1.75.0".to_string(),
            description: Some("Rust Programming Language".to_string()),
            license: Some("MIT/Apache-2.0".to_string()),
            homepage: Some("https://rust-lang.org".to_string()),
            installer: None,
        },
    ];

    db.into_iter()
        .filter(|p| p.name.contains(query) || p.description.as_deref().unwrap_or("").contains(query))
        .collect()
}
