use crate::core::{Position, Rect, Widget};
use sdl2::{event::Event, pixels::Color, rect::Rect as SdlRect};

pub struct Panel {
    pub x: Position,
    pub y: Position,
    pub width: u32,
    pub height: u32,

    pub contained: Option<Box<dyn Widget>>,
}

impl Widget for Panel {
    fn handle_event(&mut self, event: &Event) {
        if let Some(contained) = &mut self.contained {
            contained.handle_event(event);
        }
    }

    fn render(&self, boundry: &Rect, gfx: &mut gfx::Gfx) {
        let width = self.width;
        let height = self.height;

        let x = match self.x {
            Position::Left => boundry.x,
            Position::Right => boundry.x + (boundry.width - width) as i32,
            Position::Center => {
                boundry.x + (boundry.width as i32 - width as i32) / 2
            }
            Position::Absolute(x) => x,
        };

        let y = match self.y {
            Position::Left => boundry.y,
            Position::Right => boundry.y + (boundry.height - height) as i32,
            Position::Center => {
                boundry.y + (boundry.height - height) as i32 / 2
            }
            Position::Absolute(y) => y,
        };

        let rect = SdlRect::new(x, y, width, height);

        gfx.canvas.set_draw_color(Color::WHITE);
        gfx.canvas.fill_rect(rect).unwrap();

        if let Some(contained) = &self.contained {
            contained.render(
                &Rect {
                    x,
                    y,
                    width,
                    height,
                },
                gfx,
            );
        }
    }
}
