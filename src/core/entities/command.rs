use clap::Subcommand;

use crate::core::entities::bing::BingCommand;

#[derive(Subcommand)]
pub enum Commands {
    Bing(BingCommand),
}
