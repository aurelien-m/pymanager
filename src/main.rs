use clap::{Parser, Subcommand};
mod utils;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {},
    List {},
}

fn main() {
    utils::set_workspace();
    let _ = Cli::parse();
}
