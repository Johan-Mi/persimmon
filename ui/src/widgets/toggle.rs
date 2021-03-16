use crate::core::{UiResponse, Widget};
use constants::keybinds::CONFIRM;

pub struct Toggle {
    pub is_on: bool,
    pub on: Box<dyn Widget>,
    pub off: Box<dyn Widget>,
}

impl Widget for Toggle {
    fn handle_event(
        &mut self,
        event: &sdl2::event::Event,
    ) -> Option<crate::core::UiResponse> {
        match event {
            sdl2::event::Event::KeyDown { keycode, .. }
                if keycode == &Some(CONFIRM) =>
            {
                self.is_on = !self.is_on;
                Some(UiResponse::DoNothing)
            }
            event => {
                let active = if self.is_on {
                    &mut self.on
                } else {
                    &mut self.off
                };
                active.handle_event(event)
            }
        }
    }

    fn render(&self, boundry: &crate::core::Rect, gfx: &mut gfx::Gfx) {
        let active = if self.is_on { &self.on } else { &self.off };
        active.render(boundry, gfx);
    }
}
