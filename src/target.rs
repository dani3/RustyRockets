use crate::constants::*;
use crate::sprite::Sprite;

use vector2d::Vector2D;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

const SIZE: i32 = 15;

const TARGET_ORIGIN_X: i32 = SCREEN_WIDTH as i32 / 2;
const TARGET_ORIGIN_Y: i32 = 50;

pub struct Target {
    pub name: String,
    position: Vector2D<i32>,
    pub height: u32,
    pub width: u32,
}

impl Target {
    pub fn new() -> Self {
        let x = TARGET_ORIGIN_X - (SIZE / 2);
        let y = TARGET_ORIGIN_Y - (SIZE / 2);

        Target {
            position: Vector2D::new(x, y),
            name: String::from("Target"),
            height: SIZE as u32,
            width: SIZE as u32,
        }
    }

    pub fn get_position(&self) -> Vector2D<i32> {
        Vector2D::new(TARGET_ORIGIN_X, TARGET_ORIGIN_Y)
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
                SIZE as u32,
                SIZE as u32,
            ),
            0.0,
            Point::new(0, 0),
            false,
            false,
        );
    }
}
