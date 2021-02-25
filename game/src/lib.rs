use command::{subcommands::Query, Command};
use constants::{FRAMERATE, WINDOW_HEIGHT, WINDOW_WIDTH};
use creatures::CreatureKind;
use player::Player;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas,
    video::Window,
};
use std::time::{Duration, Instant};

pub struct Game {
    player: Player,

    i: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),

            i: 0,
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
            .window("Persimmon", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));

        canvas.clear();
        canvas.present();

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

                    _ => {}
                }
            }

            self.update();

            self.render(&mut canvas);
            canvas.present();

            let time_spent = now.elapsed();
            std::thread::sleep(
                Duration::new(0, 1_000_000_000u32 / FRAMERATE) - time_spent,
            );
        }
    }

    pub fn update(&mut self) {
        self.i = (self.i + 1) % 255;
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(self.i, 64, 255 - self.i));
        canvas.clear();
    }
}
