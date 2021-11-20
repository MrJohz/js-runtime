use environment::Environment;
use manifests::ConfigFile;

use crate::errors::ResolveFailure;
use crate::Resolver;

use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct FileResolver<'a, Env: Environment> {
    env: &'a Env,
    package_root: &'a Path,
    dependency_path: &'a str,
}

impl<'a, Env: Environment> Resolver for FileResolver<'a, Env> {
    fn name(&self) -> &str {
        "file-resolver"
    }

    fn resolve_manifest(&self) -> Result<ConfigFile, ResolveFailure> {
        let path = self
            .env
            .path_from_base(self.package_root, self.dependency_path)?;
        let mut manifest_file = self.env.load_file(&path.join("knopf.toml"))?;

        let mut buffer = Vec::new();
        manifest_file.read_to_end(&mut buffer)?;

        let manifest = toml::from_slice(&buffer)?;

        Ok(manifest)
    }

    fn install_package(&self, target: &Path) -> Result<(), ResolveFailure> {
        let source_dir = self
            .env
            .path_from_base(self.package_root, self.dependency_path)?;
        self.env.copy_directory(source_dir, target)?;
        Ok(())
    }
}

impl<'a, Env: Environment> FileResolver<'a, Env> {
    pub(crate) fn new(env: &'a Env, package_root: &'a Path, dependency_path: &'a str) -> Self
    where
        Env: Environment,
    {
        Self {
            env,
            package_root,
            dependency_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use environment::NullEnvironment;
    use manifests::PackageConfig;

    use super::*;

    #[test]
    fn resolves_to_error_if_package_dir_does_not_exist() {
        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&NullEnvironment, &package_root, "../project-b");

        assert!(
            resolver.resolve_manifest().is_err(),
            "Should fail to resolve"
        )
    }

    #[test]
    fn resolves_to_error_if_manifest_file_does_not_exist() {
        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&NullEnvironment, &package_root, "../project-b");

        assert!(
            resolver.resolve_manifest().is_err(),
            "Should fail to resolve"
        )
    }

    #[test]
    fn resolves_to_relative_manifest_file() {
        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&NullEnvironment, &package_root, "../project-b");

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
        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&NullEnvironment, &package_root, "/projects/project-b");

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
        let package_root = PathBuf::from("/projects/projects-a");
        let resolver = FileResolver::new(&NullEnvironment, &package_root, "/projects/project-b");

        assert_eq!(
            resolver
                .install_package("/lib/package-id".as_ref())
                .unwrap(),
            ()
        );
    }
}
