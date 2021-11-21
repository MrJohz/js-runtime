use std::{
    fmt::Debug,
    fs::File,
    io::Error as IoError,
    path::{Path, PathBuf},
};

mod live_env;
mod null_env;

pub use live_env::LiveEnvironment;
pub use null_env::NullEnvironment;

#[mockall::automock]
pub trait Environment: Debug {
    fn canonical(&self, path: &Path) -> Result<PathBuf, IoError>;
    fn path_from_base(&self, base: &Path, path: &Path) -> Result<PathBuf, IoError>;
    fn read_file(&self, path: &Path) -> Result<Vec<u8>, IoError>;
    fn copy_directory(&self, source: &Path, target: &Path) -> Result<(), IoError>;
}
