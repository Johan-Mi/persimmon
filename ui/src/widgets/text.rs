use crate::core::{Rect, Widget};
use sdl2::pixels::Color;

pub struct Text {
    pub text: String,
}

impl Widget for Text {
    fn handle_event(&mut self, _event: &sdl2::event::Event) {}

    fn render(&self, _boundry: &Rect, gfx: &mut gfx::Gfx) {
        let text_surface =
            gfx.fonts.regular.render("ABC").solid(Color::BLACK).unwrap();
        let text_texture =
            text_surface.as_texture(gfx.texture_creator).unwrap();
        gfx.canvas.copy(&text_texture, None, None).unwrap();
    }
}
