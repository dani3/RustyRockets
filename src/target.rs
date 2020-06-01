use crate::sprite::Sprite;

use vector2d::Vector2D;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Target {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub position: Vector2D<f64>,
}

impl Target {
    pub fn new(origin: Point, width: u32, height: u32) -> Self {
        let x = origin.x - (width as i32 / 2);
        let y = origin.y - (height as i32 / 2);

        Target {
            name: String::from("Target"),
            position: Vector2D::new(x as f64, y as f64),
            width,
            height,
        }
    }

    pub fn get_position(&self) -> Vector2D<f64> {
        Vector2D::new(self.position.x, self.position.y)
    }
}

impl Sprite for Target {
    fn draw(&self, canvas: &mut Canvas<Window>, texture: &mut Texture) {
        let _ = canvas.with_texture_canvas(texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
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
            Point::new(0, 0),
            false,
            false,
        );
    }
}
