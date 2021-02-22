use crate::CommandError;

pub enum Query {
    CreatureKind { name: String },
}

impl Query {
    pub(crate) fn parse(args: &[&str]) -> Result<Self, CommandError> {
        match args {
            [] => Err(CommandError::UnexpectedEnd),

            ["creature_kind", args @ ..] | ["ck", args @ ..] => match args {
                [] => Err(CommandError::UnexpectedEnd),

                [name] => Ok(Query::CreatureKind {
                    name: name.to_string(),
                }),

                _ => Err(CommandError::TooManyArguments {
                    subcommand: "query creature_kind",
                }),
            },

            [arg, ..] => Err(CommandError::InvalidArgument {
                subcommand: "query",
                arg: arg.to_string(),
            }),
        }
    }
}
