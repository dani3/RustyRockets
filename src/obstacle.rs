use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;

use vector2d::Vector2D;

pub struct Obstacle {
    width : u32,
    height : u32,
    pub position : Vector2D<f64>
}

impl Obstacle {
    pub fn new(origin: Point, width: u32, height: u32) -> Self {
        let x = origin.x - (width as i32 / 2);
        let y = origin.y - (height as i32 / 2);

        Obstacle {
            position : Vector2D::new(x as f64, y as f64),
            width,
            height
        }
    }

    pub fn show(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(200, 200, 200));

        let _ = canvas.fill_rect(
                    Rect::new(self.position.x as i32, self.position.y as i32, self.width, self.height));
    }

    pub fn is_inside(&self, position: Point) -> bool {
        if (position.y >= self.position.y as i32) && (position.y <= (self.position.y as i32 + self.height as i32))
        {
            if (position.x >= self.position.x as i32) && (position.x <= (self.position.x as i32 + self.width as i32))
            {
                return true;
            }
        }

        false
    }
}
