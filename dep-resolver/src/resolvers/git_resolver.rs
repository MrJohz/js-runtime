/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use environment::Environment;

#[derive(Debug, PartialEq, Eq, Hash)]
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
        _: &impl Environment,
    ) -> Result<manifests::PackageConfig, crate::errors::ResolveFailure> {
        todo!()
    }

    pub fn install_package(
        &self,
        _: &impl Environment,
        _: &std::path::Path,
    ) -> Result<(), crate::errors::ResolveFailure> {
        todo!()
    }
}
