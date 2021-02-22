mod error;
pub mod subcommands;
mod utils;

use std::str::FromStr;
use subcommands::Query;

pub use error::CommandError;

pub enum Command {
    Query(Query),
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();

        let (subcommand, args) =
            parts.split_first().ok_or(CommandError::UnexpectedEnd)?;

        match *subcommand {
            "query" | "q" => Query::parse(args).map(Command::Query),

            _ => Err(CommandError::InvalidCommand {
                name: s.to_string(),
            }),
        }
    }
}
