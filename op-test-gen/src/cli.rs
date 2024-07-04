use anvil::cmd::NodeArgs;
use clap::{Parser, Subcommand};
use color_eyre::eyre;
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
        #[arg(long, use_value_delimiter = true, value_delimiter = ',')]
        path: Vec<PathBuf>,

        #[command(flatten)]
        node_args: NodeArgs,
    },
}

impl Cli {
    pub async fn run(&self) -> eyre::Result<()> {
        match &self.command {
            Commands::Script { path, node_args } => {
                println!("Running scripts: {:?}", path);
                node_args.clone().run().await?;
                Ok(())
            }
        }
    }
}
