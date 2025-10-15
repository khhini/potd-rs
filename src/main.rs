mod bing;
mod command;
mod error;

use clap::Parser;

use crate::{
    command::{Cli, dispatch_command},
    error::PotdError,
};

#[tokio::main]
async fn main() -> Result<(), PotdError> {
    let cli = Cli::parse();
    let command = dispatch_command(cli);

    command.run().await?;
    Ok(())
}
