/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Dependency {
    FileDependency { path: String },
    GitDependency { git: String },
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct KnopfSection {
    pub name: String,
    pub version: String, // TODO: should this be a cleverer type?

    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ConfigFile {
    pub knopf: KnopfSection,
}
