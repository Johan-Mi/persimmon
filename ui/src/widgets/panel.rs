use crate::core::{Rect, Widget};
use sdl2::{event::Event, pixels::Color, rect::Rect as SdlRect};
use world::tile::Tile;

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
        let panel_rect =
            SdlRect::new(boundry.x, boundry.y, boundry.width, boundry.height);

        gfx.canvas.set_draw_color(Color::WHITE);
        gfx.canvas.fill_rect(panel_rect).unwrap();

        let corner_rect = Tile::UiPanelCorners.rect();

        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x(), corner_rect.y(), 8, 8),
                SdlRect::new(boundry.x, boundry.y, 8, 8),
            )
            .unwrap();
        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x() + 8, corner_rect.y(), 8, 8),
                SdlRect::new(
                    boundry.x + boundry.width as i32 - 8,
                    boundry.y,
                    8,
                    8,
                ),
            )
            .unwrap();
        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x() + 8, corner_rect.y() + 8, 8, 8),
                SdlRect::new(
                    boundry.x + boundry.width as i32 - 8,
                    boundry.y + boundry.height as i32 - 8,
                    8,
                    8,
                ),
            )
            .unwrap();
        gfx.canvas
            .copy(
                &gfx.textures.tilemap,
                SdlRect::new(corner_rect.x(), corner_rect.y() + 8, 8, 8),
                SdlRect::new(
                    boundry.x,
                    boundry.y + boundry.height as i32 - 8,
                    8,
                    8,
                ),
            )
            .unwrap();

        if let Some(contained) = &self.contained {
            let inner_rect = Rect {
                x: boundry.x + 8,
                y: boundry.y + 8,
                width: boundry.width - 16,
                height: boundry.height - 16,
            };

            contained.render(&inner_rect, gfx);
        }
    }
}
