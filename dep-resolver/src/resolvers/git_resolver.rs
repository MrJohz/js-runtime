use environment::Environment;

use crate::Resolver;

pub struct GitResolver<'a, Env: Environment> {
    _env: &'a Env,
    _url: &'a str,
}

impl<'a, Env: Environment> GitResolver<'a, Env> {
    pub fn new(env: &'a Env, url: &'a str) -> Self {
        Self {
            _env: env,
            _url: url,
        }
    }
}

impl<'a, Env: Environment> Resolver for GitResolver<'a, Env> {
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
