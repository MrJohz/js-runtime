use std::path::Path;

use manifests::ConfigFile;

use crate::errors::ResolveFailure;

/// A trait for resolving packages
///
/// Different packages can be resolved in different ways (e.g. from the local filesystem,
/// from a git repo, from a central repository).  For each package resolution, we principally
/// need the package manifest file (`resolve_manifest`) and a way to install the package in
/// the correct place (`install_package`).
pub trait Resolver {
    /// Resolve the manifest for a given package.
    fn resolve_manifest(&self) -> Result<ConfigFile, ResolveFailure>;

    /// Install the package to the correct location.
    ///
    /// TODO: how/when do post-install scripts run here?
    fn install_package(&self, target: &Path) -> Result<(), ResolveFailure>;
}
