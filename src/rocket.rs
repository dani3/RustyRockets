use vector2d::Vector2D;

use rand::Rng;

use std::f64::consts::PI;

use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::{Rect, Point};
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;

const HEIGHT: u32 = 25;
const WIDTH: u32 = 5;

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub struct Rocket {
    texture_creator : TextureCreator<WindowContext>,
    position : Vector2D<f64>,
    velocity : Vector2D<f64>,
    acceleration : Vector2D<f64>
}

impl Rocket {
    pub fn new(canvas: &Canvas<Window>, origin: Point) -> Self {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        let x = origin.x - WIDTH as i32 / 2;
        let y = origin.y - HEIGHT as i32;

        let mut rng = rand::thread_rng();

        let vx = map_range((-5000.0, 5000.0), (-5.0, 5.0), rng.gen_range(-5000, 5000) as f64);
        let vy = map_range((-5000.0, 0.0), (-5.0, 0.0), rng.gen_range(-5000, 0) as f64);

        println!("{:?}", Vector2D::new(vx as f64, vy as f64).angle() * 180.0/PI);
        println!("{:?}", Vector2D::new(vx as f64, vy as f64));

        Rocket {
            texture_creator,
            position : Vector2D::new(x as f64, y as f64),
            velocity : Vector2D::new(vx as f64, vy as f64),
            acceleration : Vector2D::new(0.0, 0.0)
        }
    }

    pub fn apply_force(&mut self, force: Vector2D<f64>) {
        self.acceleration += force;
    }

    pub fn update(&mut self) {
        // Update the velocity based on the acceleration
        self.velocity += self.acceleration;
        // Update the position based on the velocity
        self.position += self.velocity;

        // Clear the acceleration
        self.acceleration = Vector2D::new(0.0, 0.0);
    }

    pub fn show(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        let mut texture = self.texture_creator.create_texture_target(None, WIDTH, HEIGHT)
                                              .expect("Failed to create a texture");

        let _ = canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
            texture_canvas.clear();
        });

        let _ = canvas.copy_ex(
              &texture
            , None
            , Rect::new(self.position.x as i32, self.position.y as i32, WIDTH, HEIGHT)
            , 90.0 + self.velocity.angle() * 180.0 / PI
            , Point::new(WIDTH as i32 / 2, HEIGHT as i32)
            , false
            , false);
    }
}
