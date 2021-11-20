use manifests::PackageConfig;

use crate::parsing::Install;

pub fn run_install(package: PackageConfig, args: Install) -> Result<(), String> {
    dbg!((package, args));
    Ok(())
}
