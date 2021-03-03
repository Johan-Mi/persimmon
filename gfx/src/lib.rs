use sdl2::{
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

pub struct Gfx<'a> {
    pub canvas: Canvas<Window>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub textures: Textures<'a>,
}

pub struct Textures<'a> {
    pub tilemap: Texture<'a>,
}
