use crate::{utils::get_one_arg, CommandError};

pub enum Query {
    CreatureKind { name: String },
}

impl Query {
    pub(crate) fn parse(args: &[&str]) -> Result<Self, CommandError> {
        match args {
            [] => Err(CommandError::UnexpectedEnd),

            ["creature_kind", args @ ..] | ["ck", args @ ..] => {
                let name = get_one_arg("query creature_kind", args)?;

                Ok(Query::CreatureKind {
                    name: name.to_string(),
                })
            }

            [arg, ..] => Err(CommandError::InvalidArgument {
                subcommand: "query",
                arg: arg.to_string(),
            }),
        }
    }
}
