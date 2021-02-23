use crate::CommandError;

pub(crate) fn get_one_arg<'a>(
    caller: &'static str,
    args: &'a [&str],
) -> Result<&'a str, CommandError> {
    match args {
        [the_arg] => Ok(the_arg),

        args => Err(CommandError::WrongNumberOfArguments {
            caller,
            expected: 1,
            found: args.len(),
        }),
    }
}

#[macro_export]
macro_rules! def_subcommands {
    ($type:ty: $typename:literal {$(
        $subname:literal => $parsefunc:ident
    ),*$(,)?}) => {
        impl $type {
            pub(crate) fn parse(
                args: &[&str]
            ) -> ::std::result::Result<Self, $crate::CommandError> {
                match args {
                    [] => ::std::result::Result::Err(
                        $crate::CommandError::UnexpectedEnd),

                    $([$subname, args @ ..] => $parsefunc(args),)*

                    [arg, ..] => ::std::result::Result::Err(
                        $crate::CommandError::InvalidArgument {
                        caller: $typename,
                        arg: arg.to_string(),
                    }),
                }
            }
        }
    }
}
