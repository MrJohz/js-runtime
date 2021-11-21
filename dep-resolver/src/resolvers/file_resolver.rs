use environment::Environment;
use manifests::PackageConfig;

use crate::errors::ResolveFailure;
use crate::Resolver;

use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct FileResolver<'a, Env: Environment> {
    env: &'a Env,
    dependency_path: PathBuf,
}

impl<'a, Env: Environment> Resolver for FileResolver<'a, Env> {
    fn name(&self) -> &str {
        "file-resolver"
    }

    fn resolve_manifest(&self) -> Result<PackageConfig, ResolveFailure> {
        let manifest_data = self
            .env
            .read_file(&self.dependency_path.join("knopf.toml"))?;

        let manifest =
            PackageConfig::from_file_contents(self.dependency_path.clone().into(), &manifest_data)?;

        Ok(manifest)
    }

    fn install_package(&self, target: &Path) -> Result<(), ResolveFailure> {
        self.env.copy_directory(&self.dependency_path, target)?;
        Ok(())
    }
}

impl<'a, Env: Environment> FileResolver<'a, Env> {
    pub(crate) fn new(
        env: &'a Env,
        package_root: &'a Path,
        dependency_path: &'a str,
    ) -> Result<Self, ResolveFailure>
    where
        Env: Environment,
    {
        Ok(Self {
            env,
            dependency_path: env.path_from_base(package_root, dependency_path.as_ref())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use environment::MockEnvironment;
    use manifests::{ConfigFile, KnopfSection, PackageConfig};
    use mockall::predicate;

    use super::*;

    #[test]
    fn resolves_to_error_if_package_dir_does_not_exist() {
        let mut env = MockEnvironment::new();
        env.expect_path_from_base()
            .with(
                predicate::eq("/projects/project-a".as_ref()),
                predicate::eq("../project-b".as_ref()),
            )
            .returning(|_, _| Err(std::io::Error::from(std::io::ErrorKind::NotFound)));

        let package_root = PathBuf::from("/projects/project-a");
        let resolver = FileResolver::new(&env, &package_root, "../project-b");

        assert!(resolver.is_err(), "Should fail to resolve")
    }

    #[test]
    fn resolves_to_error_if_manifest_file_does_not_exist() {
        let package_root = PathBuf::from("/projects/project-a");
        let mut env = MockEnvironment::new();
        env.expect_path_from_base()
            .returning(|_, _| Ok(PathBuf::from("/projects/project-b")));
        env.expect_read_file()
            .with(predicate::eq("/projects/project-b/knopf.toml".as_ref()))
            .returning(|_| Err(std::io::Error::from(std::io::ErrorKind::NotFound)));

        let resolver = FileResolver::new(&env, &package_root, "../project-b").unwrap();

        assert!(
            resolver.resolve_manifest().is_err(),
            "Should fail to resolve"
        );
    }

    #[test]
    fn resolves_to_relative_manifest_file() {
        let package_root = PathBuf::from("/projects/project-a");
        let mut env = MockEnvironment::new();
        env.expect_path_from_base()
            .returning(|_, _| Ok(PathBuf::from("/projects/project-b")));
        env.expect_read_file()
            .returning(|_| Ok(include_bytes!("../../data/simple-manifest.toml").to_vec()));
        let resolver = FileResolver::new(&env, &package_root, "../project-b").unwrap();

        assert_eq!(
            resolver.resolve_manifest().unwrap(),
            PackageConfig::from_manifest(
                "/projects/project-b".into(),
                ConfigFile {
                    knopf: KnopfSection {
                        name: "test-project".into(),
                        version: "0.0.0".into(),
                        dependencies: HashMap::new(),
                    }
                }
            )
        )
    }

    #[test]
    fn installs_package_to_target_directory() {
        let package_root = PathBuf::from("/projects/project-a");
        let mut env = MockEnvironment::new();
        env.expect_path_from_base()
            .returning(|_, _| Ok(PathBuf::from("/projects/project-b")));
        env.expect_copy_directory()
            .with(
                predicate::eq("/projects/project-b".as_ref()),
                predicate::eq("/lib/package-id".as_ref()),
            )
            .returning(|_, _| Ok(()));
        let resolver = FileResolver::new(&env, &package_root, "/projects/project-b").unwrap();

        assert_eq!(
            resolver
                .install_package("/lib/package-id".as_ref())
                .unwrap(),
            ()
        );
    }
}
