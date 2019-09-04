use crate::target::Target;
use crate::dna::DNA;
use crate::constants::LIFESPAN;

use vector2d::Vector2D;

use std::f64::consts::PI;
use std::f64;

use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::{Rect, Point};
use sdl2::video::{WindowContext, Window};
use sdl2::pixels::Color;

const HEIGHT: u32 = 25;
const WIDTH: u32 = 5;

pub struct Rocket {
    texture_creator : TextureCreator<WindowContext>,
    position : Vector2D<f64>,
    velocity : Vector2D<f64>,
    acceleration : Vector2D<f64>,
    dna : DNA,
    count : usize,
    fitness : f64
}

impl Rocket {
    pub fn new(canvas: &Canvas<Window>, origin: Point) -> Self {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        let x = origin.x - WIDTH as i32 / 2;
        let y = origin.y - HEIGHT as i32;

        Rocket {
            texture_creator,
            position : Vector2D::new(x as f64, y as f64),
            velocity : Vector2D::new(0.0, 0.0),
            acceleration : Vector2D::new(0.0, 0.0),
            dna : DNA::new(LIFESPAN),
            count : 0,
            fitness : 0.0
        }
    }

    /// Applies some force
    fn apply_force(&mut self, force: Vector2D<f64>) {
        self.acceleration += force;
    }

    /// Updates the position based on the acceleration and velocity.
    pub fn update(&mut self) {
        self.apply_force(self.dna.get_genes()[self.count]);
        self.count += 1;

        // Update the velocity based on the acceleration
        self.velocity += self.acceleration;
        // Update the position based on the velocity
        self.position += self.velocity;

        // Clear the acceleration
        self.acceleration = Vector2D::new(0.0, 0.0);
    }

    /// Draws the rocket
    pub fn show(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        let mut texture = self.texture_creator.create_texture_target(None, WIDTH, HEIGHT)
                                              .expect("Failed to create a texture");

        let _ = canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));
            texture_canvas.clear();
        });

        let angle;
        if self.velocity.angle() == 0.0 {
            angle = 0.0;
        } else {
            angle = 90.0 + (self.velocity.angle() * 180.0 / PI);
        }

        let _ = canvas.copy_ex(
              &texture
            , None
            , Rect::new(self.position.x as i32, self.position.y as i32, WIDTH, HEIGHT)
            , angle
            , Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2)
            , false
            , false);
    }

    fn calulate_distance_to_target(&self, target : &Target) -> f64 {
        let vx = self.position.x;
        let vy = self.position.y;

        let ux = target.get_position().x as f64;
        let uy = target.get_position().y as f64;

        ((ux - vx).powi(2) + (uy - vy).powi(2)).sqrt()
    }

    pub fn calculate_fitness(&mut self, target : &Target) {
        let dist = self.calulate_distance_to_target(target);

        if dist == 0.0 {
            self.fitness = 1.0;
        } else {
            self.fitness = 1.0 / dist;
        }
    }
}
