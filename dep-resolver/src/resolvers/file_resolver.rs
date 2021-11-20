use manifests::ConfigFile;
use mock_filesystem::Filesystem;

use crate::errors::ResolveFailure;
use crate::Resolver;

use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct FileResolver<'a, FS: Filesystem> {
    fs: &'a FS,
    package_root: &'a Path,
    dependency_path: &'a str,
}

impl<'a, FS: Filesystem> Resolver for FileResolver<'a, FS> {
    fn resolve_manifest(&self) -> Result<ConfigFile, ResolveFailure> {
        let path = self.fs.canonical(self.package_root, self.dependency_path)?;
        let mut manifest_file = self.fs.load_file(&path.join("knopf.toml"))?;

        let mut buffer = Vec::new();
        manifest_file.read_to_end(&mut buffer)?;

        let manifest = toml::from_slice(&buffer)?;

        Ok(manifest)
    }

    fn install_package(&self, target: &Path) -> Result<(), ResolveFailure> {
        let source_dir = self.fs.canonical(self.package_root, self.dependency_path)?;
        self.fs.copy_directory(source_dir, target)?;
        Ok(())
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
    use std::{collections::HashMap, path::PathBuf};

    use manifests::PackageConfig;

    use super::*;

    #[test]
    fn resolves_to_error_if_package_dir_does_not_exist() {
        let fs = mock_filesystem::mock_tree!(
            "projects" => { "project-a" => {} }
        );
        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&fs, &package_root, "../project-b");

        assert!(
            resolver.resolve_manifest().is_err(),
            "Should fail to resolve"
        )
    }

    #[test]
    fn resolves_to_error_if_manifest_file_does_not_exist() {
        let fs = mock_filesystem::mock_tree!(
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

    #[test]
    fn resolves_to_relative_manifest_file() {
        let manifest_file = include_str!("../../data/simple-manifest.toml");
        let fs = mock_filesystem::mock_tree!(
            "projects" => {
                "project-a" => {}
                "project-b" => {
                    "knopf.toml" => manifest_file
                }
            }
        );

        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&fs, &package_root, "../project-b");

        assert_eq!(
            resolver.resolve_manifest().unwrap(),
            ConfigFile {
                knopf: PackageConfig {
                    name: "test-project".into(),
                    version: "0.0.0".into(),
                    dependencies: HashMap::new(),
                }
            }
        )
    }

    #[test]
    fn resolves_to_absolute_manifest_file() {
        let manifest_file = include_str!("../../data/simple-manifest.toml");
        let fs = mock_filesystem::mock_tree!(
            "projects" => {
                "project-a" => {}
                "project-b" => {
                    "knopf.toml" => manifest_file
                }
            }
        );

        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&fs, &package_root, "/projects/project-b");

        assert_eq!(
            resolver.resolve_manifest().unwrap(),
            ConfigFile {
                knopf: PackageConfig {
                    name: "test-project".into(),
                    version: "0.0.0".into(),
                    dependencies: HashMap::new(),
                }
            }
        )
    }

    #[test]
    fn installs_package_to_target_directory() {
        let fs = mock_filesystem::mock_tree!(
            "projects" => {
                "project-a" => {}
                "project-b" => { "testfile.txt" => "ok" }
            }
            "lib" => {}
        );

        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&fs, &package_root, "/projects/project-b");

        assert_eq!(
            resolver
                .install_package("/lib/package-id".as_ref())
                .unwrap(),
            ()
        );

        assert_eq!(
            fs,
            mock_filesystem::mock_tree!(
                "projects" => {
                    "project-a" => {}
                    "project-b" => { "testfile.txt" => "ok" }
                }
                "lib" => {}
            )
        );
    }
}
