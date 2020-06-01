use crate::sprite::Sprite;

use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Background {
    pub name: String,
    pub height: u32,
    pub width: u32,
}

impl Background {
    pub fn new(height: u32, width: u32) -> Self {
        Background {
            name: String::from("Background"),
            height,
            width,
        }
    }
}

impl Sprite for Background {
    fn draw(&self, canvas: &mut Canvas<Window>, texture: &mut Texture) {
        let _ = canvas.copy_ex(
            &texture,
            None,
            Rect::new(0, 0, self.width, self.height),
            0.0,
            Point::new(self.width as i32 / 2, self.height as i32 / 2),
            false,
            false,
        );
    }
}
