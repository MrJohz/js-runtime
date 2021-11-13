use std::{
    io::Error,
    path::{Path, PathBuf},
};

pub trait Filesystem {
    fn canonical(
        &self,
        path: impl AsRef<Path>,
        other_path: impl AsRef<Path>,
    ) -> Result<PathBuf, Error>;

    fn load_manifest(&self, path: impl AsRef<Path>) -> Result<manifests::PackageConfig, Error>;
}
