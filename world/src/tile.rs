use constants::TILE_SIZE;
use sdl2::rect::Rect;

#[derive(Clone, Copy)]
pub enum Tile {
    Ground,
    Rock,
}

impl Tile {
    pub fn rect(&self) -> Rect {
        Rect::new(*self as i32 * TILE_SIZE as i32, 0, TILE_SIZE, TILE_SIZE)
    }
}
