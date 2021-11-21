/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Eq, Debug)]
pub enum Commands {
    Install(Install),
}

#[derive(StructOpt, PartialEq, Eq, Debug, Default)]
/// Install all dependencies for the current project
pub struct Install {}

#[derive(StructOpt, PartialEq, Eq, Debug)]
/// Knopf -- A JavaScript Runtime
pub struct Cli {
    /// path to package manifest
    #[structopt(short = "p", default_value = ".")]
    pub package: PathBuf,

    #[structopt(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn from_env() -> Self {
        Self::from_args()
    }
}
