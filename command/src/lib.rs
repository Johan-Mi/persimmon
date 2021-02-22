use std::str::FromStr;
use thiserror::Error;

pub enum Command {
    // TODO
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(CommandError::InvalidCommand {
            name: s.to_string(),
        })
    }
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("`{name}` is not a valid command")]
    InvalidCommand { name: String },
}
