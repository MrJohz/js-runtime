/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use dep_resolver::{get_resolver, Resolver};
use environment::Environment;
use manifests::PackageConfig;

use crate::parsing::Install;

fn fetch_dependencies(
    env: &impl Environment,
    package: &PackageConfig,
    accumulator: &mut Vec<Resolver>,
) -> Result<(), String> {
    for dependency in package.dependencies() {
        let resolver = get_resolver(env, package, dependency).unwrap();
        accumulator.push(resolver);
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
