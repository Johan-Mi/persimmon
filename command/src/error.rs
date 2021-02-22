use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("`{name}` is not a valid command")]
    InvalidCommand { name: String },

    #[error("Unexpected end of command")]
    UnexpectedEnd,

    #[error("Subcommand `{subcommand}`: Invalid argument `{arg}`")]
    InvalidArgument {
        subcommand: &'static str,
        arg: String,
    },

    #[error("Subcommand `{subcommand}`: Too many arguments")]
    TooManyArguments { subcommand: &'static str },

    #[error("Subcommand `{subcommand}`: Wrong number of arguments, expected {expected} but got {found}")]
    WrongNumberOfArguments {
        subcommand: &'static str,
        expected: usize,
        found: usize,
    },
}
