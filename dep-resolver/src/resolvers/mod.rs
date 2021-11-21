/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod file_resolver;
mod git_resolver;

use std::path::Path;

use environment::Environment;
use manifests::{Dependency, PackageConfig};

use crate::errors::ResolveFailure;

use self::{file_resolver::FileResolver, git_resolver::GitResolver};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Resolver {
    FileResolver(FileResolver),
    GitResolver(GitResolver),
}

impl Resolver {
    pub fn from_dependency(
        env: &impl Environment,
        package: &PackageConfig,
        dep: &Dependency,
    ) -> Result<Self, ResolveFailure> {
        match dep {
            Dependency::FileDependency { path } => Ok(Self::FileResolver(FileResolver::new(
                env,
                package.location(),
                path,
            )?)),
            Dependency::GitDependency { git: git_url } => {
                Ok(Self::GitResolver(GitResolver::new(env, git_url)))
            }
        }
    }

    pub fn name(&self) -> &str {
        match &self {
            Self::FileResolver(resolver) => resolver.name(),
            Self::GitResolver(resolver) => resolver.name(),
        }
    }

    pub fn resolve_manifest(
        &self,
        env: &impl Environment,
    ) -> Result<PackageConfig, ResolveFailure> {
        match &self {
            Self::FileResolver(resolver) => resolver.resolve_manifest(env),
            Self::GitResolver(resolver) => resolver.resolve_manifest(env),
        }
    }

    pub fn install_package(
        &self,
        env: &impl Environment,
        target: &Path,
    ) -> Result<(), ResolveFailure> {
        match &self {
            Self::FileResolver(resolver) => resolver.install_package(env, target),
            Self::GitResolver(resolver) => resolver.install_package(env, target),
        }
    }
}
