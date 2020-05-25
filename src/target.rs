use crate::constants::*;

use vector2d::Vector2D;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

const SIZE: i32 = 15;

const TARGET_ORIGIN_X: i32 = SCREEN_WIDTH as i32 / 2;
const TARGET_ORIGIN_Y: i32 = 50;

pub struct Target {
    texture_creator: TextureCreator<WindowContext>,
    position: Vector2D<i32>,
}

impl Target {
    pub fn new(canvas: &Canvas<Window>) -> Self {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        let x = TARGET_ORIGIN_X - (SIZE / 2);
        let y = TARGET_ORIGIN_Y - (SIZE / 2);

        Target {
            texture_creator,
            position: Vector2D::new(x, y),
        }
    }

    pub fn show(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        let mut texture = self
            .texture_creator
            .create_texture_target(None, SIZE as u32, SIZE as u32)
            .expect("Failed to create a texture");

        let _ = canvas.with_texture_canvas(&mut texture, |texture_canvas| {
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

    pub fn get_position(&self) -> Vector2D<i32> {
        Vector2D::new(TARGET_ORIGIN_X, TARGET_ORIGIN_Y)
    }
}
