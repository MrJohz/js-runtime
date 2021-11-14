mod filesystem_trait;

pub mod mock_fs;
mod null_fs;
mod real_fs;

pub use filesystem_trait::Filesystem;
pub use filesystem_trait::MFile;
pub use mock_fs::MockFilesystem;
pub use null_fs::NullFilesystem;
pub use real_fs::RealFilesystem;

pub use mock_filesystem_macros::mock_tree;
