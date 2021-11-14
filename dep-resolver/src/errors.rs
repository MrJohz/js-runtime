#[derive(Debug)]
pub enum ResolveFailure {
    IoError(std::io::Error),
    ManifestParseError(toml::de::Error),
}

impl From<std::io::Error> for ResolveFailure {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<toml::de::Error> for ResolveFailure {
    fn from(error: toml::de::Error) -> Self {
        Self::ManifestParseError(error)
    }
}
