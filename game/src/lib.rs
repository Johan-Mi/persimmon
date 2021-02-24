use command::{subcommands::Query, Command};
use creatures::CreatureKind;
use player::Player;

pub struct Game {
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }

    pub fn run_command(&self, command: Command) {
        match command {
            Command::Query(query) => match query {
                Query::CreatureKind { name } => {
                    if let Ok(kind) = name.parse::<CreatureKind>() {
                        let name = kind.as_str();
                        let evolution = kind
                            .evolves_into()
                            .map(|k| k.as_str())
                            .unwrap_or("None");

                        println!("{}", name);
                        println!("  - Evolves into: {}", evolution);
                    } else {
                        eprintln!("`{}` is not a creature", name);
                    }
                }
            },
        }
    }
}
