use crate::rocket::Rocket;
use crate::target::Target;
use crate::constants::*;

use rand::seq::SliceRandom;

use sdl2::render::Canvas;
use sdl2::rect::Point;
use sdl2::video::Window;

const POPULATION_SIZE : usize = 500;

const POPULATION_ORIGIN_X: i32 = SCREEN_WIDTH as i32 / 2;
const POPULATION_ORIGIN_Y: i32 = SCREEN_HEIGHT as i32;

pub struct Population {
    rockets : Vec<Rocket>,
    mating_pool : Vec<usize>
}

impl Population {
    pub fn new(canvas: &Canvas<Window>) -> Self {
        let mut rockets = Vec::new();
        for _ in 0 .. POPULATION_SIZE {
            rockets.push(
                Rocket::new(
                      canvas
                    , Point::new(POPULATION_ORIGIN_X, POPULATION_ORIGIN_Y)
                    , None));
        }

        Population {
            rockets,
            mating_pool : Vec::new()
        }
    }

    /// Updates and draws every rocket
    pub fn run(&mut self, canvas: &mut Canvas<Window>, target : &Target) {
        for i in 0 .. POPULATION_SIZE {
            self.rockets[i].update(target);
            self.rockets[i].show(canvas);
        }
    }

    /// Evaluates every rocket based on its fitness
    pub fn evaluate(&mut self, target : &Target) {
        let mut max_fitness = 0.0;
        self.mating_pool = Vec::new();

        println!("\nNew population has been born:");

        let mut average = 0.0;
        let mut num_reached = 0;

        // Iterate over the entire population
        for i in 0 .. POPULATION_SIZE {
            // Calculate each one's fitness
            self.rockets[i].calculate_fitness(target);

            // And calculate the maximum fitness
            if self.rockets[i].fitness > max_fitness {
                max_fitness = self.rockets[i].fitness;
            }

            average += self.rockets[i].fitness;
            if self.rockets[i].reached {
                num_reached += 1;
            }
        }

        println!(" - Average fitness of the previous generation: {:.2}", average / POPULATION_SIZE as f64);
        println!(" - Maximum fitness of the previous generation: {:.2}", max_fitness);
        println!(" - {:} rockets hit the target", num_reached);

        for i in 0 .. POPULATION_SIZE {
            self.rockets[i].fitness /= max_fitness;
        }

        for i in 0 .. POPULATION_SIZE {
            let n = (self.rockets[i].fitness * 100.0) as i32;
            for _ in 0 .. n {
                self.mating_pool.push(i);
            }
        }
    }

    /// Runs a natural selection on the current mating pool
    pub fn natural_selection(&mut self, canvas: &Canvas<Window>) {
        for i in 0 .. POPULATION_SIZE {
            // Choose two random parents
            let a = self.mating_pool.choose(&mut rand::thread_rng()).unwrap();
            let b = self.mating_pool.choose(&mut rand::thread_rng()).unwrap();

            // Pick each one's DNA
            let parent_a = &self.rockets[*a].dna;
            let parent_b = &self.rockets[*b].dna;

            // Cross both DNAs over
            let mut child = parent_a.crossover(parent_b);
            // Apply some random low-probability mutation
            child.mutate();

            self.rockets[i] = Rocket::new(
                  &canvas
                , Point::new(POPULATION_ORIGIN_X, POPULATION_ORIGIN_Y)
                , Some(child));
        }
    }
}