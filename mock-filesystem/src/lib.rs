mod filesystem_trait;

mod mock_filesystem;
mod null_filesystem;
mod real_filesystem;

pub use filesystem_trait::Filesystem;
pub use mock_filesystem::MockFilesystem;
pub use null_filesystem::NullFilesystem;
pub use real_filesystem::RealFilesystem;
