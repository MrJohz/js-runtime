use std::{
    fs::File,
    io::Error as IoError,
    path::{Path, PathBuf},
};

mod null_env;

pub use null_env::NullEnvironment;

#[mockall::automock]
pub trait Environment {
    fn path_from_base(&self, base: &Path, path: &Path) -> Result<PathBuf, IoError>;
    fn load_file(&self, path: &Path) -> Result<File, IoError>;
    fn copy_directory(&self, source: &Path, target: &Path) -> Result<(), IoError>;
}
