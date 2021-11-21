#[derive(Debug)]
pub enum ResolveFailure {
    IoError(std::io::Error),
    ManifestParseError(manifests::Error),
}

impl From<std::io::Error> for ResolveFailure {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<manifests::Error> for ResolveFailure {
    fn from(error: manifests::Error) -> Self {
        Self::ManifestParseError(error)
    }
}
