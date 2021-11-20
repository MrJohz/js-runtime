use manifests::parse_manifest;

mod commands;
mod parsing;

fn main() {
    let args = parsing::Cli::from_env();
    let package = parse_manifest("[knopf]").unwrap();
    match args.command {
        Some(parsing::Commands::Install(args)) => commands::run_install(package, args),
        None => commands::run_install(package, parsing::Install::default()),
    }
    .unwrap();
}
