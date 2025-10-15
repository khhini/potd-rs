use clap::Parser;

use crate::core::entities::command::Commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[command(name = "potd-rs")]
pub struct Cli {
    #[command[subcommand]]
    pub command: Commands,
}
