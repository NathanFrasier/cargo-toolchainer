//! A cli tool for automatically updating your rust-toolchian.toml file
#![forbid(unsafe_code)]
#![deny(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

use clap::{Parser, Subcommand, Args};

use cli::*;

mod cli;
mod update_channel;
mod utils;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Toolchainer(Cli)
}


#[derive(Args)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Update(UpdateArgs),
}

fn main() {
    let CargoCli::Toolchainer(cli) = CargoCli::parse();
    match cli.command {
        Commands::Update(args) => update(args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        CargoCli::command().debug_assert();
    }
}
