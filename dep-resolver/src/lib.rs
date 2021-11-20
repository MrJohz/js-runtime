use std::path::Path;

use environment::Environment;
use manifests::Dependency;

mod errors;
mod resolvers;

use resolvers::Resolver;

pub fn get_resolver<'a, Env: Environment>(
    env: &'a Env,
    package_root: &'a Path,
    dep: &'a Dependency,
) -> Result<Box<dyn resolvers::Resolver + 'a>, errors::ResolveFailure> {
    match dep {
        Dependency::FileDependency { path } => Ok(Box::new(resolvers::FileResolver::new(
            env,
            package_root,
            path,
        ))),
        Dependency::GitDependency { git: _ } => Ok(Box::new(resolvers::GitResolver)),
    }
}

#[cfg(test)]
mod tests {
    use environment::NullEnvironment;

    use super::*;

    #[test]
    fn test_file_dependency_produces_file_resolver() {
        let dependency = Dependency::FileDependency {
            path: String::from("../other-path"),
        };

        let resolver = get_resolver(&NullEnvironment, &Path::new("."), &dependency);
        assert!(resolver.is_ok(), "resolver was successful");

        let resolver = resolver.unwrap();
        assert_eq!(resolver.name(), "file-resolver");
    }

    #[test]
    fn git_dependency_produces_git_resolver() {
        let dependency = Dependency::GitDependency {
            git: String::from("git@github.com:user/package.git"),
        };

        let resolver = get_resolver(&NullEnvironment, &Path::new("."), &dependency);
        assert!(resolver.is_ok(), "resolver was successful");

        let resolver = resolver.unwrap();
        assert_eq!(resolver.name(), "git-resolver");
    }
}
