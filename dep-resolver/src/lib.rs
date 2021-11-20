use std::path::Path;

use manifests::Dependency;
use mock_filesystem::Filesystem;

mod errors;
mod resolvers;

use resolvers::Resolver;

pub fn get_resolver<'a, FS: Filesystem>(
    fs: &'a FS,
    package_root: &'a Path,
    dep: &'a Dependency,
) -> Result<Box<dyn resolvers::Resolver + 'a>, errors::ResolveFailure> {
    match dep {
        Dependency::FileDependency { path } => Ok(Box::new(resolvers::FileResolver::new(
            fs,
            package_root,
            path,
        ))),
        Dependency::GitDependency { git: _ } => Ok(Box::new(resolvers::GitResolver)),
    }
}

#[cfg(test)]
mod tests {
    use mock_filesystem::NullFilesystem;

    use super::*;

    #[test]
    fn test_file_dependency_produces_file_resolver() {
        let dependency = Dependency::FileDependency {
            path: String::from("../other-path"),
        };

        let resolver = get_resolver(&NullFilesystem, &Path::new("."), &dependency);
        assert!(resolver.is_ok(), "resolver was successful");

        let resolver = resolver.unwrap();
        assert_eq!(resolver.name(), "file-resolver");
    }

    #[test]
    fn git_dependency_produces_git_resolver() {
        let dependency = Dependency::GitDependency {
            git: String::from("git@github.com:user/package.git"),
        };

        let resolver = get_resolver(&NullFilesystem, &Path::new("."), &dependency);
        assert!(resolver.is_ok(), "resolver was successful");

        let resolver = resolver.unwrap();
        assert_eq!(resolver.name(), "git-resolver");
    }
}
