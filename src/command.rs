use async_trait::async_trait;

use crate::{bing::BingCommand, error::PotdError};
use clap::{Parser, Subcommand};

#[async_trait]
pub trait Command {
    async fn run(&self) -> Result<(), PotdError>;
    fn dry_run(&self) -> bool;
}

#[derive(Subcommand)]
pub enum Commands {
    Bing(BingCommand),
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[command(name = "potd-rs")]
pub struct Cli {
    #[command[subcommand]]
    command: Commands,
}

pub fn dispatch_command(cli: Cli) -> Box<dyn Command> {
    match cli.command {
        Commands::Bing(cmd) => Box::new(cmd),
    }
}
