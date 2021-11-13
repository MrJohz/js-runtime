#[derive(Debug)]
pub enum ResolveFailure {
    IoError(std::io::Error),
}

impl From<std::io::Error> for ResolveFailure {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
