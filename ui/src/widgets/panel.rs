use crate::core::{Rect, UiResponse, Widget};
use sdl2::{event::Event, pixels::Color, rect::Rect as SdlRect};
use world::tile::Tile;

pub struct Panel {
    pub contained: Option<Box<dyn Widget>>,
}

impl Widget for Panel {
    fn handle_event(&mut self, event: &Event) -> Option<UiResponse> {
        if let Some(contained) = &mut self.contained {
            contained.handle_event(event)
        } else {
            None
        }
    }

    fn render(&self, boundry: &Rect, gfx: &mut gfx::Gfx) {
        let inner_rect = Rect {
            x: boundry.x + 4,
            y: boundry.y + 4,
            width: boundry.width - 8,
            height: boundry.height - 8,
        };

        gfx.canvas.set_draw_color(Color::WHITE);
        gfx.canvas
            .fill_rect(SdlRect::from(inner_rect.clone()))
            .unwrap();

        let corner_rect = Tile::UiPanelCorners.rect();

        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x(), corner_rect.y(), 4, 4),
                SdlRect::new(boundry.x, boundry.y, 4, 4),
            )
            .unwrap();
        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x() + 4, corner_rect.y(), 4, 4),
                SdlRect::new(
                    boundry.x + boundry.width as i32 - 4,
                    boundry.y,
                    4,
                    4,
                ),
            )
            .unwrap();
        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x() + 4, corner_rect.y() + 4, 4, 4),
                SdlRect::new(
                    boundry.x + boundry.width as i32 - 4,
                    boundry.y + boundry.height as i32 - 4,
                    4,
                    4,
                ),
            )
            .unwrap();
        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x(), corner_rect.y() + 4, 4, 4),
                SdlRect::new(
                    boundry.x,
                    boundry.y + boundry.height as i32 - 4,
                    4,
                    4,
                ),
            )
            .unwrap();

        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x() + 3, corner_rect.y(), 1, 4),
                SdlRect::new(boundry.x + 4, boundry.y, inner_rect.width, 4),
            )
            .unwrap();
        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x() + 3, corner_rect.y() + 4, 1, 4),
                SdlRect::new(
                    boundry.x + 4,
                    boundry.y + boundry.height as i32 - 4,
                    inner_rect.width,
                    4,
                ),
            )
            .unwrap();
        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x(), corner_rect.y() + 3, 4, 1),
                SdlRect::new(boundry.x, boundry.y + 4, 4, inner_rect.height),
            )
            .unwrap();
        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x + 4, corner_rect.y() + 3, 4, 1),
                SdlRect::new(
                    boundry.x + boundry.width as i32 - 4,
                    boundry.y + 4,
                    4,
                    inner_rect.height,
                ),
            )
            .unwrap();

        if let Some(contained) = &self.contained {
            contained.render(&inner_rect, gfx);
        }
    }
}
