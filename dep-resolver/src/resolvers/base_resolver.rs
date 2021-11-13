use manifests::PackageConfig;

use crate::errors::ResolveFailure;

pub trait Resolver {
    fn resolve_manifest(&self) -> Result<PackageConfig, ResolveFailure>;
}
