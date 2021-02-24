use constants::TEAM_SIZE;
use creatures::Creature;

pub struct Player {
    team: [Option<Creature>; TEAM_SIZE],
}

impl Player {
    pub fn new() -> Self {
        Self {
            team: <[_; TEAM_SIZE]>::default(),
        }
    }
}
