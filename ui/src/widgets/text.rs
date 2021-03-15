use crate::core::{Rect, Widget};
use sdl2::{pixels::Color, rect::Rect as SdlRect};

const TEXT_COLOR: Color = Color::BLACK;
const SHADOW_COLOR: Color = Color::RGB(0xc0, 0xc0, 0xc0);

pub struct Text {
    pub text: String,
    pub style: TextStyle,
}

impl Widget for Text {
    fn handle_event(&mut self, _event: &sdl2::event::Event) {}

    fn render(&self, boundry: &Rect, gfx: &mut gfx::Gfx) {
        let font = match self.style {
            TextStyle::Regular => &gfx.fonts.regular,
        };

        let surface = font
            .render(&self.text)
            .blended_wrapped(TEXT_COLOR, boundry.width)
            .unwrap();
        let shadow_surface = font
            .render(&self.text)
            .blended_wrapped(SHADOW_COLOR, boundry.width)
            .unwrap();
        let (width, height) = surface.size();

        let texture = surface.as_texture(gfx.texture_creator).unwrap();
        let shadow_texture =
            shadow_surface.as_texture(gfx.texture_creator).unwrap();

        let dst = SdlRect::new(boundry.x, boundry.y, width, height);
        let shadow_dst = SdlRect::new(boundry.x, boundry.y + 1, width, height);

        gfx.canvas.copy(&shadow_texture, None, shadow_dst).unwrap();
        gfx.canvas.copy(&texture, None, dst).unwrap();
    }
}

pub enum TextStyle {
    Regular,
}
