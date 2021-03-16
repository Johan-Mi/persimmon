use crate::state::GameState;
use command::{subcommands::Query, Command};
use constants::{
    FRAMERATE, TILE_SIZE, WINDOW_PIXEL_HEIGHT, WINDOW_PIXEL_WIDTH,
    WINDOW_TILE_HEIGHT, WINDOW_TILE_WIDTH,
};
use creatures::CreatureKind;
use gfx::{Fonts, Gfx, Textures};
use player::Player;
use sdl2::{
    event::{Event, WindowEvent},
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use ui::{
    core::{Position, Rect as UiRect, Widget},
    widgets::{
        text::{Text, TextStyle},
        Panel, Positioned, Toggle,
    },
};
use world::room::Room;

pub struct Game {
    player: Player,
    state: GameState,

    rooms: HashMap<String, Room>,
    current_room: String,

    menu: Box<dyn Widget>,
}

impl Game {
    pub fn new() -> Self {
        let default_room = Room::default();
        let mut rooms = HashMap::with_capacity(1);
        rooms.insert("default".to_string(), default_room);

        let menu = Box::new(Positioned {
            x: Position::Absolute(8),
            y: Position::Absolute(8),
            width: 160,
            height: 100,
            contained: Box::new(Toggle {
                is_on: false,
                on: Box::new(Panel {
                    contained: Some(Box::new(Text {
                        text: "Toggle: on\nPress `I`".to_string(),
                        style: TextStyle::Regular,
                    })),
                }),
                off: Box::new(Panel {
                    contained: Some(Box::new(Text {
                        text: "Toggle: off\nPress `I`".to_string(),
                        style: TextStyle::Regular,
                    })),
                }),
            }),
        });

        Self {
            player: Player::new(),
            state: GameState::Overworld,
            rooms,
            current_room: "default".to_string(),
            menu,
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

        let font_context = sdl2::ttf::init().unwrap();

        let font_regular = font_context
            .load_font("assets/fonts/PersimmonRegular.ttf", 8)
            .unwrap();

        let fonts = Fonts {
            regular: font_regular,
        };

        let mut gfx = Gfx {
            canvas,
            texture_creator: &texture_creator,
            textures,
            font_context: &font_context,
            fonts,
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
                        win_event: WindowEvent::Resized(..),
                        ..
                    } => {
                        gfx.canvas
                            .set_logical_size(
                                WINDOW_PIXEL_WIDTH,
                                WINDOW_PIXEL_HEIGHT,
                            )
                            .unwrap();
                        gfx.canvas.set_draw_color(Color::BLACK);
                        gfx.canvas.clear();
                    }

                    _ => {
                        self.menu.handle_event(&event);
                    }
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
        let room = self.rooms.get(&self.current_room).unwrap();

        for y in 0..WINDOW_TILE_HEIGHT {
            for x in 0..WINDOW_TILE_WIDTH {
                let tile = room.get_tile(x as usize, y as usize);

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

        let boundry = UiRect {
            x: 0,
            y: 0,
            width: WINDOW_PIXEL_WIDTH,
            height: WINDOW_PIXEL_HEIGHT,
        };
        self.menu.render(&boundry, gfx);
    }
}
