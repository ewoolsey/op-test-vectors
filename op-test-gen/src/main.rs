use crate::cli::Cli;
use clap::Parser;

mod cli;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
