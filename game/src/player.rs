use constants::TILE_SIZE;
use gfx::Gfx;
use player::Player;
use sdl2::{pixels::Color, rect::Rect};

const fn player_screen_coords(player: &Player) -> (i32, i32) {
    ((player.x * TILE_SIZE) as i32, (player.y * TILE_SIZE) as i32)
}

pub fn render_player(player: &Player, gfx: &mut Gfx) {
    gfx.canvas.set_draw_color(Color::BLUE);

    let (player_x, player_y) = player_screen_coords(player);

    let player_rect = Rect::new(player_x, player_y, 16, 16);

    gfx.canvas.fill_rect(player_rect).unwrap();
}
