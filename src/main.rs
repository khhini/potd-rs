use clap::Parser;
use potd_rs::{
    adapters::cli::dispatch_command,
    core::{entities::cli::Cli, error::PotdError},
};

#[tokio::main]
async fn main() -> Result<(), PotdError> {
    let cli = Cli::parse();
    let command = dispatch_command(cli);

    command.run().await?;
    Ok(())
}
