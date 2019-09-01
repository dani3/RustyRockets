use vector2d::Vector2D;

use std::ops::Add;

use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::{Rect, Point};
use sdl2::video::Window;
use sdl2::pixels::Color;

const HEIGHT: u32 = 50;
const WIDTH: u32 = 10;

pub struct Rocket {
    texture : Texture<'static>,
    position : Vector2D<f64>,
    velocity : Vector2D<f64>,
    acceleration : Vector2D<f64>
}

impl Rocket {
    pub fn new(canvas: &Canvas<Window>, origin: Point) -> Self {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();
        let texture: Texture<'static> = texture_creator.create_texture_target(None, WIDTH, HEIGHT)
                                     .expect("Failed to create a texture");

        let x = origin.x - HEIGHT as i32;
        let y = origin.y - WIDTH as i32 / 2;

        Rocket {
            texture,
            position : Vector2D::new(x as f64, y as f64),
            velocity : Vector2D::new(0.0, 0.0),
            acceleration : Vector2D::new(0.0, 0.0)
        }
    }

    pub fn apply_force(&mut self, force: Vector2D<f64>) {
        self.acceleration = self.acceleration.add(force);
    }

    pub fn update(&mut self) {
        // Update the velocity based on the acceleration
        self.velocity = self.velocity.add(self.acceleration);
        // Update the position based on the velocity
        self.position = self.position.add(self.velocity);

        // Clear the acceleration
        self.acceleration = Vector2D::new(0.0, 0.0);
    }

    pub fn show(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        let _ = canvas.with_texture_canvas(&mut self.texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
            texture_canvas.clear();
        });

        let _ = canvas.copy_ex(
            &self.texture, None, Rect::new(100, 100, 50, 150), 0.0, Point::new(25, 150), false, false);
    }
}