/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use environment::Environment;
use manifests::{Dependency, PackageConfig};

mod errors;
mod resolvers;

pub use resolvers::Resolver;

pub fn get_resolver(
    env: &impl Environment,
    package: &PackageConfig,
    dep: &Dependency,
) -> Result<Resolver, errors::ResolveFailure> {
    Resolver::from_dependency(env, package, dep)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use environment::{MockEnvironment, NullEnvironment};
    use manifests::{ConfigFile, KnopfSection};

    use super::*;

    #[test]
    fn test_file_dependency_produces_file_resolver() {
        let mut env = MockEnvironment::new();
        env.expect_path_from_base()
            .returning(|_, _| Ok(PathBuf::from("/resolved")));

        let dependency = Dependency::FileDependency {
            path: String::from("../other-path"),
        };

        let resolver = get_resolver(
            &env,
            &PackageConfig::from_manifest(
                "".into(),
                ConfigFile {
                    knopf: KnopfSection {
                        name: "package".into(),
                        version: "0.0.0".into(),
                        dependencies: HashMap::new(),
                    },
                },
            ),
            &dependency,
        );
        assert!(resolver.is_ok(), "resolver was successful");

        let resolver = resolver.unwrap();
        assert_eq!(resolver.name(), "file-resolver");
    }

    #[test]
    fn git_dependency_produces_git_resolver() {
        let dependency = Dependency::GitDependency {
            git: String::from("git@github.com:user/package.git"),
        };

        let resolver = get_resolver(
            &NullEnvironment,
            &PackageConfig::from_manifest(
                "".into(),
                ConfigFile {
                    knopf: KnopfSection {
                        name: "package".into(),
                        version: "0.0.0".into(),
                        dependencies: HashMap::new(),
                    },
                },
            ),
            &dependency,
        );
        assert!(resolver.is_ok(), "resolver was successful");

        let resolver = resolver.unwrap();
        assert_eq!(resolver.name(), "git-resolver");
    }
}
