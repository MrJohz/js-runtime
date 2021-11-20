#[derive(Debug, PartialEq, Eq)]
pub enum ManifestError {
    ParseError(toml::de::Error),
}

impl From<toml::de::Error> for ManifestError {
    fn from(error: toml::de::Error) -> Self {
        Self::ParseError(error)
    }
}
