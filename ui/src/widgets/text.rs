use crate::core::{Rect, Widget};
use sdl2::{pixels::Color, rect::Rect as SdlRect};

pub struct Text {
    pub text: String,
}

impl Widget for Text {
    fn handle_event(&mut self, _event: &sdl2::event::Event) {}

    fn render(&self, boundry: &Rect, gfx: &mut gfx::Gfx) {
        let surface =
            gfx.fonts.regular.render("ABC").solid(Color::BLACK).unwrap();
        let (width, height) = surface.size();

        let texture = surface.as_texture(gfx.texture_creator).unwrap();

        let dst = SdlRect::new(boundry.x, boundry.y, width, height);

        gfx.canvas.copy(&texture, None, dst).unwrap();
    }
}
