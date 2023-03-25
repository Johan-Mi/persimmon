use constants::TILE_SIZE;
use sdl2::rect::Rect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Tile {
    UiPanelCorners,

    Ground,
    Rock,
}

impl Tile {
    #[must_use]
    pub fn rect(&self) -> Rect {
        Rect::new(*self as i32 * TILE_SIZE as i32, 0, TILE_SIZE, TILE_SIZE)
    }
}
