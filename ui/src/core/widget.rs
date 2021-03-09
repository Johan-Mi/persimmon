use crate::core::rect::Rect;
use gfx::Gfx;
use sdl2::event::Event;

pub trait Widget {
    fn handle_event(&mut self, event: &Event);

    fn render(&self, boundry: &Rect, gfx: &mut Gfx);
}
