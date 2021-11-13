use manifests::PackageConfig;
use mock_filesystem::Filesystem;

use crate::errors::ResolveFailure;
use crate::Resolver;

use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct FileResolver<'a, FS: Filesystem> {
    fs: &'a FS,
    package_root: &'a Path,
    dependency_path: &'a str,
}

impl<'a, FS: Filesystem> Resolver for FileResolver<'a, FS> {
    fn resolve_manifest(&self) -> Result<PackageConfig, ResolveFailure> {
        let path = self.fs.canonical(self.package_root, self.dependency_path)?;
        let manifest = self.fs.load_manifest(&path)?;

        Ok(manifest)
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

    use super::*;

    #[test]
    fn resolves_to_error_if_package_dir_does_not_exist() {
        let fs = mock_filesystem::mock_fs!(
            "projects" => { "project-a" => {} }
        );
        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&fs, &package_root, "dependency_path");

        assert!(
            resolver.resolve_manifest().is_err(),
            "Should fail to resolve"
        )
    }

    #[test]
    fn resolves_to_error_if_package_file_does_not_exist() {
        let fs = mock_filesystem::mock_fs!(
            "projects" => {
                "project-a" => {}
                "project-b" => {}
            }
        );
        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&fs, &package_root, "../project-b");

        assert!(
            resolver.resolve_manifest().is_err(),
            "Should fail to resolve"
        )
    }
}
