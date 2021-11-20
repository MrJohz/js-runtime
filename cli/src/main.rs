use std::{
    fs::File,
    io::{Error as IoError, Read},
    path::Path,
};

use manifests::parse_manifest;

mod commands;
mod parsing;

fn read_manifest(path: &Path) -> Result<String, IoError> {
    let mut file = if path.file_name() == Some("knopf.toml".as_ref()) {
        File::open(path)?
    } else {
        File::open(path.join("knopf.toml"))?
    };
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn main() {
    let args = parsing::Cli::from_env();
    let manifest_data = read_manifest(&args.package).unwrap();
    let package = parse_manifest(&manifest_data).unwrap();
    match args.command {
        Some(parsing::Commands::Install(args)) => commands::run_install(package, args),
        None => commands::run_install(package, parsing::Install::default()),
    }
    .unwrap();
}
