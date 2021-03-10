use crate::core::{Position, Rect, Widget};
use sdl2::{event::Event, pixels::Color, rect::Rect as SdlRect};

pub struct Panel {
    pub contained: Option<Box<dyn Widget>>,
}

impl Widget for Panel {
    fn handle_event(&mut self, event: &Event) {
        if let Some(contained) = &mut self.contained {
            contained.handle_event(event);
        }
    }

    fn render(&self, boundry: &Rect, gfx: &mut gfx::Gfx) {
        let rect =
            SdlRect::new(boundry.x, boundry.y, boundry.width, boundry.height);

        gfx.canvas.set_draw_color(Color::WHITE);
        gfx.canvas.fill_rect(rect).unwrap();

        if let Some(contained) = &self.contained {
            contained.render(boundry, gfx);
        }
    }
}
