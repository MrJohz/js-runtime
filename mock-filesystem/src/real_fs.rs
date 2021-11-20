use std::{fs::canonicalize, io::Error};

use crate::{Filesystem, MFile};

pub struct RealFilesystem {}
impl Filesystem for RealFilesystem {
    fn canonical(
        &self,
        path: impl AsRef<std::path::Path>,
        other_path: impl AsRef<std::path::Path>,
    ) -> Result<std::path::PathBuf, Error> {
        canonicalize(path.as_ref().join(other_path))
    }

    fn load_file(&self, _: impl AsRef<std::path::Path>) -> Result<MFile, Error> {
        todo!()
    }

    fn copy_directory(
        &self,
        _: impl AsRef<std::path::Path>,
        _: impl AsRef<std::path::Path>,
    ) -> Result<(), Error> {
        todo!()
    }
}
