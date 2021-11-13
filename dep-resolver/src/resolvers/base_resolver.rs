use config::PackageConfig;

use crate::errors::ResolveFailure;

pub trait Resolver {
    fn resolve_package_config(&self) -> Result<PackageConfig, ResolveFailure>;
}
