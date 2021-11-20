mod errors;

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Dependency {
    FileDependency { path: String },
    GitDependency { git: String },
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PackageConfig {
    pub name: String,
    pub version: String, // TODO: should this be a cleverer type?

    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ConfigFile {
    pub knopf: PackageConfig,
}

pub fn parse_manifest(contents: &str) -> Result<PackageConfig, errors::ManifestError> {
    let manifest = toml::from_str::<ConfigFile>(contents)?;
    Ok(manifest.knopf)
}
