use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("`{name}` is not a valid command")]
    InvalidCommand { name: String },

    #[error("Unexpected end of command")]
    UnexpectedEnd,

    #[error("`{caller}`: Invalid argument `{arg}`")]
    InvalidArgument { caller: &'static str, arg: String },

    #[error("`{caller}`: Too many arguments")]
    TooManyArguments { caller: &'static str },

    #[error("`{caller}`: Wrong number of arguments, expected {expected} but got {found}")]
    WrongNumberOfArguments {
        caller: &'static str,
        expected: usize,
        found: usize,
    },
}
