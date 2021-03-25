use constants::TEAM_SIZE;
use creatures::Creature;
use misc::direction::Direction;

pub struct Player {
    pub x: u32,
    pub y: u32,
    dir: Direction,

    team: [Option<Creature>; TEAM_SIZE],
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Direction::Right,

            team: <[_; TEAM_SIZE]>::default(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
