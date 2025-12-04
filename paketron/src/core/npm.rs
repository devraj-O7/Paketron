use anyhow::Result;
use serde::Deserialize;
use crate::core::package::{Package, Installer, InstallerType};

#[derive(Deserialize, Debug)]
struct NpmSearchResult {
    objects: Vec<NpmSearchObject>,
}

#[derive(Deserialize, Debug)]
struct NpmSearchObject {
    package: NpmPackageInfo,
}

#[derive(Deserialize, Debug)]
struct NpmPackageInfo {
    name: String,
    version: String,
    description: Option<String>,
    links: NpmLinks,
}

#[derive(Deserialize, Debug)]
struct NpmLinks {
    npm: String,
    homepage: Option<String>,
}

#[derive(Deserialize, Debug)]
struct NpmPackageMetadata {
    name: String,
    description: Option<String>,
    #[serde(rename = "dist-tags")]
    dist_tags: NpmDistTags,
    versions: std::collections::HashMap<String, NpmVersion>,
    license: Option<String>,
}

#[derive(Deserialize, Debug)]
struct NpmDistTags {
    latest: String,
}

#[derive(Deserialize, Debug)]
struct NpmVersion {
    dist: NpmDist,
}

#[derive(Deserialize, Debug)]
struct NpmDist {
    tarball: String,
    shasum: String,
}

pub struct NpmClient;

impl NpmClient {
    pub async fn search(query: &str) -> Result<Vec<Package>> {
        let url = format!("https://registry.npmjs.org/-/v1/search?text={}&size=10", query);
        let response = reqwest::get(&url).await?.json::<NpmSearchResult>().await?;

        let packages = response.objects.into_iter().map(|obj| {
            Package {
                name: obj.package.name,
                version: obj.package.version,
                description: obj.package.description,
                license: None, // Search result doesn't always have license
                homepage: obj.package.links.homepage.or(Some(obj.package.links.npm)),
                installer: None, // Installer details need a separate fetch
            }
        }).collect();

        Ok(packages)
    }

    pub async fn get_package(name: &str) -> Result<Option<Package>> {
        let url = format!("https://registry.npmjs.org/{}", name);
        let response = reqwest::get(&url).await?;

        if response.status() == 404 {
            return Ok(None);
        }

        let metadata = response.json::<NpmPackageMetadata>().await?;
        let latest_version_tag = metadata.dist_tags.latest;
        let latest_version = metadata.versions.get(&latest_version_tag).unwrap();

        Ok(Some(Package {
            name: metadata.name,
            version: latest_version_tag,
            description: metadata.description,
            license: metadata.license,
            homepage: None,
            installer: Some(Installer {
                url: latest_version.dist.tarball.clone(),
                sha256: latest_version.dist.shasum.clone(), // Note: NPM uses SHA1 usually, but we'll store it here
                installer_type: InstallerType::Zip, // Treat tarball as Zip/Archive
                args: None,
            }),
        }))
    }
}
