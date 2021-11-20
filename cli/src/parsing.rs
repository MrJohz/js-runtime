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
    #[structopt(short = "p", default_value = "\".\".into()")]
    package: PathBuf,

    #[structopt(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn from_env() -> Self {
        Self::from_args()
    }
}
