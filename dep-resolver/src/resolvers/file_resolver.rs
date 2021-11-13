use config::PackageConfig;
use mock_filesystem::Filesystem;

use crate::errors::ResolveFailure;

use super::Resolver;
use std::{collections::HashMap, path::Path};

#[derive(Debug, PartialEq, Eq)]
pub struct FileResolver<'a, FS: Filesystem> {
    fs: &'a FS,
    package_root: &'a Path,
    dependency_path: &'a str,
}

impl<'a, FS: Filesystem> Resolver for FileResolver<'a, FS> {
    fn resolve_package_config(&self) -> Result<PackageConfig, ResolveFailure> {
        let path = self.fs.canonical(self.package_root, self.dependency_path)?;
        dbg!(path);

        Ok(PackageConfig {
            name: "test".to_string(),
            version: "0.1.1".to_string(),
            dependencies: HashMap::new(),
        })
    }
}

impl<'a, FS: Filesystem> FileResolver<'a, FS> {
    pub(crate) fn new(fs: &'a FS, package_root: &'a Path, dependency_path: &'a str) -> Self
    where
        FS: Filesystem,
    {
        Self {
            fs,
            package_root,
            dependency_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mock_filesystem::MockFilesystem;

    use super::*;

    #[test]
    pub fn resolves_to_error_if_package_does_not_exist() {
        let fs = mock_filesystem::mock_fs!(
            "path/to/x" => {
            }
        );
        let fs = MockFilesystem::from_tree(fs);
        let package_root = PathBuf::from("/tmp/other");
        let resolver = FileResolver::new(&fs, &package_root, "dependency_path");

        assert!(
            resolver.resolve_package_config().is_err(),
            "Should fail to resolve"
        )
    }
}
