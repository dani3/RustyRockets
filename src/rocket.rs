use crate::constants::*;
use crate::dna::DNA;
use crate::obstacle::Obstacle;
use crate::sprite::Sprite;
use crate::target::Target;

use vector2d::Vector2D;

use std::f64;
use std::f64::consts::PI;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

const MAX_REWARD: f64 = 20.0;
const MIN_REWARD: f64 = 10.0;

const OBSTACLE_PASSED_REWARD: f64 = 2.0;
const OBSTACLE_NOT_PASSED_PENALTY: f64 = 0.25;

const CRASH_PENALTY: f64 = 0.10;

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub struct Rocket {
    position: Vector2D<f64>,
    velocity: Vector2D<f64>,
    acceleration: Vector2D<f64>,
    count: usize,
    time_reached: usize,
    crashed: bool,
    pub height: u32,
    pub width: u32,
    pub name: String,
    pub reached: bool,
    pub dna: DNA,
    pub fitness: f64,
}

impl Rocket {
    pub fn new(name: String, origin: Point, h: u32, w: u32, dna: Option<DNA>) -> Self {
        let x = origin.x - w as i32 / 2;
        let y = origin.y - h as i32;

        Rocket {
            position: Vector2D::new(x as f64, y as f64),
            velocity: Vector2D::new(0.0, 0.0),
            acceleration: Vector2D::new(0.0, 0.0),
            dna: dna.unwrap_or(DNA::new(None)),
            height: h,
            width: w,
            name,
            crashed: false,
            reached: false,
            time_reached: 0,
            count: 0,
            fitness: 0.0,
        }
    }

    /// Applies some force
    fn apply_force(&mut self, force: Vector2D<f64>) {
        self.acceleration += force;
    }

    /// Updates the position based on the acceleration and velocity.
    pub fn update(&mut self, target: &Target, obstacle: &Obstacle) {
        if obstacle.is_inside(Point::new(self.position.x as i32, self.position.y as i32)) {
            self.crashed = true;
        } else if (self.position.x > SCREEN_WIDTH as f64)
            || (self.position.x < 0.0)
            || (self.position.y < 0.0)
        {
            self.crashed = true;
        } else {
            let dist = self.calulate_distance_to_target(target);

            if dist < 10.0 {
                self.reached = true;
                self.time_reached = self.count;
            } else if (self.position.x > SCREEN_WIDTH as f64) || (self.position.x < 0.0) {
                self.crashed = true;
            } else if ((self.position.y as i32) < 0)
                || ((self.position.y as u32) + self.height > SCREEN_HEIGHT as u32)
            {
                self.crashed = true;
            } else {
                self.apply_force(self.dna.get_genes()[self.count]);
                self.count += 1;

                // Update the velocity based on the acceleration
                self.velocity += self.acceleration;
                // Update the position based on the velocity
                self.position += self.velocity;

                // Clear the acceleration
                self.acceleration = Vector2D::new(0.0, 0.0);
            }
        }
    }

    fn calulate_distance_to_target(&self, target: &Target) -> f64 {
        let vx = self.position.x;
        let vy = self.position.y;

        let ux = target.get_position().x as f64;
        let uy = target.get_position().y as f64;

        ((ux - vx).powi(2) + (uy - vy).powi(2)).sqrt()
    }

    /// Calculates the fitness based on the distance to the target
    pub fn calculate_fitness(&mut self, target: &Target, obstacle: &Obstacle) {
        let dist = self.calulate_distance_to_target(target);

        if dist > SCREEN_HEIGHT as f64 {
            // Penalise if the rocket is out of bounds
            self.fitness = 1.0;
        } else {
            self.fitness = map_range(
                (10.0, SCREEN_HEIGHT as f64),
                (SCREEN_HEIGHT as f64, 0.0),
                dist,
            );
        }

        if self.reached {
            // Reward those who reached the target and those who were faster
            let time_reward = map_range(
                (LIFESPAN as f64 / 5.0, LIFESPAN as f64),
                (MAX_REWARD, MIN_REWARD),
                self.time_reached as f64,
            );

            self.fitness *= time_reward;
        } else if self.crashed {
            // Penalty if they crashed
            self.fitness *= CRASH_PENALTY;
        }

        if self.position.y < obstacle.position.y {
            // Reward if they went passed the obstacle
            self.fitness *= OBSTACLE_PASSED_REWARD;
        } else if self.position.y >= obstacle.position.y {
            // Penalty if the did not go passed the obstacle
            self.fitness *= OBSTACLE_NOT_PASSED_PENALTY;
        }
    }
}

impl Sprite for Rocket {
    fn draw(&self, canvas: &mut Canvas<Window>, texture: &mut Texture) {
        let _ = canvas.with_texture_canvas(texture, |texture_canvas| {
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
            &texture,
            None,
            Rect::new(
                self.position.x as i32,
                self.position.y as i32,
                self.width,
                self.height,
            ),
            angle,
            Point::new(self.width as i32 / 2, self.height as i32 / 2),
            false,
            false,
        );
    }
}
