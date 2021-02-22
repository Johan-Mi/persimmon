use crate::CommandError;

pub(crate) fn get_one_arg<'a>(
    subcommand: &'static str,
    args: &'a [&str],
) -> Result<&'a str, CommandError> {
    match args {
        [the_arg] => Ok(the_arg),

        args => Err(CommandError::WrongNumberOfArguments {
            subcommand,
            expected: 1,
            found: args.len(),
        }),
    }
}
