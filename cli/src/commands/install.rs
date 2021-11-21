use dep_resolver::{get_resolver, Resolver};
use environment::Environment;
use manifests::PackageConfig;

use crate::parsing::Install;

fn fetch_dependencies(
    env: &impl Environment,
    package: &PackageConfig,
    accumulator: &mut Vec<Box<&dyn Resolver>>,
) -> Result<(), String> {
    for dependency in package.dependencies() {
        let resolver = get_resolver(env, package.location(), &dependency).unwrap();
    }

    Ok(())
}

pub fn run_install(
    env: &impl Environment,
    package: PackageConfig,
    args: Install,
) -> Result<(), String> {
    let mut dependencies = Vec::new();
    fetch_dependencies(env, &package, &mut dependencies)?;
    dbg!((package, args, dependencies));
    Ok(())
}
