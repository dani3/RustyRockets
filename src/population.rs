use crate::rocket::Rocket;

use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::video::Window;

const POPULATION_SIZE : usize = 100;

pub struct Population {
    rockets : Vec<Rocket>
}

impl Population {
    pub fn new(canvas: &Canvas<Window>, origin: Point) -> Self {
        let mut rockets = Vec::new();
        for _ in 0..POPULATION_SIZE {
            rockets.push(Rocket::new(canvas, origin));
        }

        Population {
            rockets
        }
    }

    pub fn run(&mut self, canvas: &mut Canvas<Window>) {
        for i in 0 .. POPULATION_SIZE {
            self.rockets[i].update();
            self.rockets[i].show(canvas);
        }
    }
}