use std::io::{self, Result as IoResult, Write};

pub(crate) fn repl<F>(mut func: F) -> IoResult<()>
where
    F: FnMut(&str),
{
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.is_empty() {
            break;
        }

        let input = input.trim();

        func(input)
    }

    Ok(())
}
