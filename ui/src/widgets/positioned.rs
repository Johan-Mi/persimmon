use crate::core::{Position, Rect, Widget};

pub struct Positioned {
    pub x: Position,
    pub y: Position,
    pub width: u32,
    pub height: u32,

    pub contained: Box<dyn Widget>,
}

impl Widget for Positioned {
    fn handle_event(&mut self, event: &sdl2::event::Event) {
        self.contained.handle_event(event);
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

        self.contained.render(
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
