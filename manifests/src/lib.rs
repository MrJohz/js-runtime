use std::path::{Path, PathBuf};

pub use errors::ManifestError as Error;
pub use parsing::{ConfigFile, Dependency, KnopfSection};

mod errors;
mod parsing;

#[derive(Debug, PartialEq, Eq)]
pub struct PackageConfig {
    manifest: ConfigFile,
    location: PathBuf,
}

impl PackageConfig {
    pub fn location(&self) -> &Path {
        &self.location
    }

    pub fn dependencies(&self) -> impl Iterator<Item = &Dependency> {
        self.manifest.knopf.dependencies.values()
    }

    pub fn from_manifest(location: PathBuf, manifest: ConfigFile) -> Self {
        Self { manifest, location }
    }

    pub fn from_file_contents(
        location: PathBuf,
        contents: &[u8],
    ) -> Result<Self, errors::ManifestError> {
        let manifest = toml::from_slice(contents)?;
        Ok(Self { manifest, location })
    }
}
