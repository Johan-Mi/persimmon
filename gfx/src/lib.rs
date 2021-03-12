use sdl2::{
    render::{Canvas, Texture, TextureCreator},
    ttf::{Font, Sdl2TtfContext},
    video::{Window, WindowContext},
};

pub struct Gfx<'a> {
    pub canvas: Canvas<Window>,

    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub textures: Textures<'a>,

    pub font_context: &'a Sdl2TtfContext,
    pub fonts: Fonts<'a>,
}

pub struct Textures<'a> {
    pub tilemap: Texture<'a>,
}

pub struct Fonts<'a> {
    pub regular: Font<'a, 'a>,
}
