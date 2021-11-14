use manifests::ConfigFile;

use crate::errors::ResolveFailure;

pub trait Resolver {
    fn resolve_manifest(&self) -> Result<ConfigFile, ResolveFailure>;
}
