use std::{
    io::Error,
    panic,
    path::{Path, PathBuf},
};

use super::Filesystem;

pub struct NullFilesystem;
impl Filesystem for NullFilesystem {
    fn canonical(&self, _: impl AsRef<Path>, _: impl AsRef<Path>) -> Result<PathBuf, Error> {
        panic!("Using the null filesystem in any way will produce an error.")
    }
}
