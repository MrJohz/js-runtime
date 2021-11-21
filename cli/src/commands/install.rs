/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashSet;

use dep_resolver::{get_resolver, Resolver};
use environment::Environment;
use manifests::PackageConfig;

use crate::parsing::Install;

fn fetch_dependencies(
    env: &impl Environment,
    package: &PackageConfig,
    accumulator: &mut HashSet<(PackageConfig, Resolver)>,
) -> Result<(), String> {
    for dependency in package.dependencies() {
        let resolver = get_resolver(env, package, dependency).unwrap();
        let manifest = resolver
            .resolve_manifest(env)
            .map_err(|err| format!("{:?}", err))?;
        fetch_dependencies(env, &manifest, accumulator)?;

        accumulator.insert((manifest, resolver));
    }

    Ok(())
}

pub fn run_install(
    env: &impl Environment,
    package: PackageConfig,
    args: Install,
) -> Result<(), String> {
    let mut dependencies = HashSet::new();
    fetch_dependencies(env, &package, &mut dependencies)?;
    dbg!((package, args, &dependencies));
    for (manifest, resolver) in dependencies {
        resolver
            .install_package(
                env,
                format!("/tmp/test-path/{}/test", manifest.name()).as_ref(),
            )
            .map_err(|err| format!("{:?}", err))?;
    }
    Ok(())
}
