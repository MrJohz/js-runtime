/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::{Path, PathBuf};

pub use errors::ManifestError as Error;
pub use parsing::{ConfigFile, Dependency, KnopfSection};

mod errors;
mod parsing;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PackageConfig {
    manifest: ConfigFile,
    location: PathBuf,
}

impl PackageConfig {
    pub fn name(&self) -> &str {
        &self.manifest.knopf.name
    }

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
