use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Dependency {
    FileDependency { path: String },
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
