use clap::{Parser, Subcommand};
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
#[clap(rename_all = "kebab_case", infer_subcommands = true)]
pub enum Commands {
    /// Uses a forge script to generate a test vector
    #[command(visible_alias = "s")]
    Script {
        /// Path to the forge script
        #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
        path: Vec<PathBuf>,
    },
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Commands::Script { path } => {
                println!("Running scripts: {:?}", path);
            }
        }
    }
}
