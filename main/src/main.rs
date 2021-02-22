mod repl;

use game::Game;
use repl::repl;

fn main() {
    let game = Game::new();

    if let Err(err) = repl(|input| {
        if input.is_empty() {
            return;
        }

        let command = match input.parse() {
            Ok(command) => command,
            Err(err) => {
                eprintln!("Command error: {}", err);
                return;
            }
        };

        game.run_command(command);
    }) {
        eprintln!("IO error: {}", err);
    }
}
