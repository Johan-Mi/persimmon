use std::convert::TryInto;

use crate::CommandError;

pub fn get_n_args<'a, const N: usize>(
    caller: &'static str,
    args: &'a [&str],
) -> Result<[&'a str; N], CommandError> {
    args.try_into()
        .map_err(|_| CommandError::WrongNumberOfArguments {
            caller,
            expected: 1,
            found: args.len(),
        })
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

#[macro_export]
macro_rules! def_commands {
    ($(
        $cmdtype:ident ($($cmdname:literal),*)
    ),*$(,)?) => {
        pub enum Command {
            $($cmdtype($crate::subcommands::$cmdtype)),*
        }

        def_commands!(@a $($($cmdtype $cmdname)*)*);
    };

    (@a $($cmdtype:ident $cmdname:literal)*) => {
        impl ::std::str::FromStr for Command {
            type Err = $crate::CommandError;

            fn from_str(
                s: &str
            ) -> ::std::result::Result<Self, Self::Err> {
                let parts: ::std::vec::Vec<_> = s.split_whitespace().collect();

                let (command, args) = parts
                    .split_first()
                    .ok_or($crate::CommandError::UnexpectedEnd)?;

                match *command {
                    $($cmdname => {
                        $crate::subcommands::$cmdtype::parse(args)
                            .map(Command::$cmdtype)
                    })*

                    _ => Err(CommandError::InvalidCommand {
                        name: s.to_string(),
                    }),
                }
            }
        }
    };
}
