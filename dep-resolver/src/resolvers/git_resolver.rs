use crate::Resolver;

pub struct GitResolver;

impl Resolver for GitResolver {
    fn name(&self) -> &str {
        "git-resolver"
    }

    fn resolve_manifest(&self) -> Result<manifests::ConfigFile, crate::errors::ResolveFailure> {
        todo!()
    }

    fn install_package(&self, _: &std::path::Path) -> Result<(), crate::errors::ResolveFailure> {
        todo!()
    }
}
