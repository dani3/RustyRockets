use vector2d::Vector2D;

use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{WindowContext, Window};
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;

const SIZE: i32 = 30;

pub struct Target {
    texture_creator : TextureCreator<WindowContext>,
    position : Vector2D<i32>
}

impl Target {
    pub fn new(canvas: &Canvas<Window>, position : Point) -> Self {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        let x = position.x - SIZE / 2;
        let y = position.y - SIZE / 2;

        Target {
            texture_creator,
            position : Vector2D::new(x, y)
        }
    }

    pub fn show(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        let mut texture = self.texture_creator.create_texture_target(None, SIZE as u32, SIZE as u32)
                                              .expect("Failed to create a texture");

        let _ = canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
            texture_canvas.clear();
        });

        let _ = canvas.copy_ex(
              &texture
            , None
            , Rect::new(self.position.x as i32, self.position.y as i32, SIZE as u32, SIZE as u32)
            , 0.0
            , Point::new(SIZE as i32 / 2, SIZE as i32 / 2)
            , false
            , false);
    }
}