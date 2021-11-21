use environment::Environment;

#[derive(Debug, PartialEq, Eq)]
pub struct GitResolver {
    _url: String,
}

impl GitResolver {
    pub fn new(_env: &impl Environment, url: &str) -> Self {
        Self { _url: url.into() }
    }
}

impl GitResolver {
    pub fn name(&self) -> &str {
        "git-resolver"
    }

    pub fn resolve_manifest(
        &self,
    ) -> Result<manifests::PackageConfig, crate::errors::ResolveFailure> {
        todo!()
    }

    pub fn install_package(
        &self,
        _: &std::path::Path,
    ) -> Result<(), crate::errors::ResolveFailure> {
        todo!()
    }
}
