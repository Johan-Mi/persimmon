use crate::gfx::{Gfx, Textures};
use command::{subcommands::Query, Command};
use constants::{
    FRAMERATE, TILE_SIZE, WINDOW_PIXEL_HEIGHT, WINDOW_PIXEL_WIDTH,
    WINDOW_TILE_HEIGHT, WINDOW_TILE_WIDTH,
};
use creatures::CreatureKind;
use player::Player;
use sdl2::{
    event::{Event, WindowEvent},
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
};
use std::time::{Duration, Instant};
use world::tile::Tile;

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

    pub fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Persimmon", WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let tilemap = texture_creator
            .load_texture("assets/textures/tilemap.png")
            .unwrap();

        let textures = Textures { tilemap };

        let mut gfx = Gfx {
            canvas,
            texture_creator: &texture_creator,
            textures,
        };

        gfx.canvas.set_draw_color(Color::RGB(0, 0, 0));

        gfx.canvas.clear();
        gfx.canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            let now = Instant::now();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'running;
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }

                    Event::Window {
                        win_event: WindowEvent::Resized(new_width, new_height),
                        ..
                    } => {
                        let scale_x =
                            new_width as f32 / WINDOW_PIXEL_WIDTH as f32;
                        let scale_y =
                            new_height as f32 / WINDOW_PIXEL_HEIGHT as f32;

                        let scale = scale_x.min(scale_y);

                        gfx.canvas.clear();

                        gfx.canvas.set_scale(scale, scale).unwrap();
                    }

                    _ => {}
                }
            }

            self.update();

            self.render(&mut gfx);
            gfx.canvas.present();

            let time_spent = now.elapsed();
            std::thread::sleep(
                Duration::new(0, 1_000_000_000u32 / FRAMERATE)
                    .checked_sub(time_spent)
                    .unwrap_or_default(),
            );
        }
    }

    fn update(&mut self) {
        // Do nothing currently
    }

    fn render(&self, gfx: &mut Gfx) {
        for y in 0..WINDOW_TILE_HEIGHT {
            for x in 0..WINDOW_TILE_WIDTH {
                let tile = if x % 2 == y % 2 {
                    Tile::Ground
                } else {
                    Tile::Rock
                };

                let src_rect = tile.rect();
                let dst_rect = Rect::new(
                    (x * TILE_SIZE) as i32,
                    (y * TILE_SIZE) as i32,
                    TILE_SIZE,
                    TILE_SIZE,
                );

                gfx.canvas
                    .copy(&gfx.textures.tilemap, src_rect, dst_rect)
                    .unwrap();
            }
        }
    }
}
