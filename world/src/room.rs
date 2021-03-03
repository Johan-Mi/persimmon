use crate::tile::Tile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Room {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Room {
    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.width + x]
    }
}

impl Default for Room {
    fn default() -> Self {
        let width = 32;
        let height = 24;

        Self {
            width,
            height,
            tiles: vec![Tile::Ground; width * height],
        }
    }
}
