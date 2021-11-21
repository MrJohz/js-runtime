use std::{
    io::{Error as IoError, ErrorKind as IoErrorKind},
    path::{Path, PathBuf},
};

use environment::{Environment, LiveEnvironment};
use manifests::PackageConfig;

mod commands;
mod parsing;

fn open_package(env: &impl Environment, path: &Path) -> Result<(PathBuf, Vec<u8>), IoError> {
    let path = if path.file_name() == Some("knopf.toml".as_ref()) {
        path.parent()
    } else {
        Some(path)
    };
    let path = path.ok_or(IoError::from(IoErrorKind::NotFound))?;
    let path = env.canonical(&path)?;

    let file_contents = env.read_file(&path.join("knopf.toml"))?;

    Ok((path, file_contents))
}

fn main() {
    let args = parsing::Cli::from_env();
    let env = LiveEnvironment::default();

    let (manifest_location, manifest_data) = open_package(&env, &args.package).unwrap();
    let package = PackageConfig::from_file_contents(manifest_location, &manifest_data).unwrap();

    match args.command {
        Some(parsing::Commands::Install(args)) => commands::run_install(&env, package, args),
        None => commands::run_install(&env, package, parsing::Install::default()),
    }
    .unwrap();
}
