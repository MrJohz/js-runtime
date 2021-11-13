use std::collections::HashMap;

pub enum Dependency {
    FileDependency { path: String },
}

pub struct ConfigFile {
    pub name: String,
    pub version: String, // TODO: should this be a cleverer type?

    pub dependencies: HashMap<String, Dependency>,
}
