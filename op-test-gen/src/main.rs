use crate::cli::Cli;
use clap::Parser;
use color_eyre::eyre;

mod cli;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    cli.run().await?;
    Ok(())
}
