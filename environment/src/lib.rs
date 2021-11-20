use std::{
    fs::File,
    io::Error as IoError,
    path::{Path, PathBuf},
};

mod null_env;

pub use null_env::NullEnvironment;

pub trait Environment {
    fn path_from_base(
        &self,
        base: impl AsRef<Path>,
        path: impl AsRef<Path>,
    ) -> Result<PathBuf, IoError>;
    fn load_file(&self, path: impl AsRef<Path>) -> Result<File, IoError>;
    fn copy_directory(
        &self,
        source: impl AsRef<Path>,
        target: impl AsRef<Path>,
    ) -> Result<(), IoError>;
}
