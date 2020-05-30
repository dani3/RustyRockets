use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use vector2d::Vector2D;

use crate::sprite::Sprite;

pub struct Obstacle {
    pub width: u32,
    pub height: u32,
    pub position: Vector2D<f64>,
}

impl Obstacle {
    pub fn new(origin: Point, width: u32, height: u32) -> Self {
        let x = origin.x - (width as i32 / 2);
        let y = origin.y - (height as i32 / 2);

        Obstacle {
            position: Vector2D::new(x as f64, y as f64),
            width,
            height,
        }
    }

    pub fn is_inside(&self, position: Point) -> bool {
        if (position.y >= self.position.y as i32)
            && (position.y <= (self.position.y as i32 + self.height as i32))
        {
            if (position.x >= self.position.x as i32)
                && (position.x <= (self.position.x as i32 + self.width as i32))
            {
                return true;
            }
        }

        false
    }
}

impl Sprite for Obstacle {
    fn draw(&self, canvas: &mut Canvas<Window>, texture: &mut Texture) {
        let _ = canvas.with_texture_canvas(texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));
            texture_canvas.clear();
        });

        let _ = canvas.copy_ex(
            &texture,
            None,
            Rect::new(
                self.position.x as i32,
                self.position.y as i32,
                self.width,
                self.height,
            ),
            0.0,
            Point::new(self.width as i32 / 2, self.height as i32 / 2),
            false,
            false,
        );
    }
}
