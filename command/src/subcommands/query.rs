use crate::{def_subcommands, utils::get_n_args, CommandError};

pub enum Query {
    CreatureKind { name: String },
}

def_subcommands! {
    Query: "query" {
        "creature_kind" => parsesub_creature_kind,
        "ck" => parsesub_creature_kind,
    }
}

pub fn parsesub_creature_kind(args: &[&str]) -> Result<Query, CommandError> {
    let [name] = get_n_args("query creature_kind", args)?;

    Ok(Query::CreatureKind {
        name: name.to_string(),
    })
}
