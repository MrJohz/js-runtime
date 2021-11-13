use std::path::Path;

use config::Dependency;

mod errors;
mod resolvers;

pub fn get_resolver<'a>(
    package_root: &'a Path,
    dep: &'a Dependency,
) -> Result<Box<dyn resolvers::Resolver + 'a>, errors::ResolveFailure> {
    match dep {
        Dependency::FileDependency { path } => {
            Ok(Box::new(resolvers::FileResolver::new(package_root, path)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_dependency_produces_file_resolver() {
        let dependency = Dependency::FileDependency {
            path: String::from("../other-path"),
        };

        let resolver = get_resolver(&Path::new("."), &dependency);
        assert!(resolver.is_ok(), "resolver was successful");
        assert_eq!(
            resolver.unwrap().as_file_resolver(),
            Some(&resolvers::FileResolver::new(
                &Path::new("."),
                "../other-path"
            ))
        );
    }
}
