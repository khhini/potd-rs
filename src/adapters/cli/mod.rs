use crate::core::entities::{cli::Cli, command::Commands};
use crate::core::port::Command;

pub mod bing;

pub fn dispatch_command(cli: Cli) -> Box<dyn Command> {
    match cli.command {
        Commands::Bing(cmd) => Box::new(cmd),
    }
}
