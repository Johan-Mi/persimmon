mod error;
pub mod subcommands;
mod utils;

pub use error::CommandError;

def_commands! {
    Query ("query", "q"),
}
