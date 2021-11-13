use super::Resolver;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct FileResolver<'a> {
    package_root: &'a Path,
    dependency_path: &'a str,
}

impl<'a> Resolver for FileResolver<'a> {
    #[cfg(test)]
    fn as_file_resolver<'b>(&'b self) -> Option<&'b FileResolver<'b>> {
        Some(self)
    }
}

impl<'a> FileResolver<'a> {
    pub(crate) fn new(package_root: &'a Path, dependency_path: &'a str) -> Self {
        Self {
            package_root,
            dependency_path,
        }
    }
}
