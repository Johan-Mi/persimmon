use command::Command;

pub struct Game {
    // TODO
}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_command(&self, command: Command) {
        println!("Successfully ran a command");
    }
}
