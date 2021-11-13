use std::{fs::canonicalize, io::Error};

use super::Filesystem;

pub struct RealFilesystem {}
impl Filesystem for RealFilesystem {
    fn canonical(
        &self,
        path: impl AsRef<std::path::Path>,
        other_path: impl AsRef<std::path::Path>,
    ) -> Result<std::path::PathBuf, Error> {
        canonicalize(path.as_ref().join(other_path))
    }

    fn load_manifest(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<manifests::PackageConfig, Error> {
        todo!()
    }
}
