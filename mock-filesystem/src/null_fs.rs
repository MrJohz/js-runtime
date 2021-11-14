use std::{
    io::Error,
    panic,
    path::{Path, PathBuf},
};

use crate::{Filesystem, MFile};

pub struct NullFilesystem;
impl Filesystem for NullFilesystem {
    fn canonical(&self, _: impl AsRef<Path>, _: impl AsRef<Path>) -> Result<PathBuf, Error> {
        panic!("Using the null filesystem in any way will produce an error.")
    }

    fn load_file(&self, _: impl AsRef<Path>) -> Result<MFile, Error> {
        panic!("Using the null filesystem in any way will produce an error.")
    }
}
