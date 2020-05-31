use crate::constants::*;
use crate::obstacle::Obstacle;
use crate::rocket::Rocket;
use crate::target::Target;

use rand::seq::SliceRandom;
use sdl2::rect::Point;

pub const ROCKET_HEIGHT: u32 = 20;
pub const ROCKET_WIDTH: u32 = 15;

pub struct Population {
    pub rockets: Vec<Rocket>,
    origin: Point,
    mating_pool: Vec<usize>,
    generation: u32,
}

impl Population {
    pub fn new(capacity: usize, x: i32, y: i32) -> Self {
        let mut rockets = Vec::new();
        for i in 0..capacity {
            let mut name = String::from("Rocket#");
            name.push_str(&i.to_string());
            rockets.push(Rocket::new(
                name,
                Point::new(x, y),
                ROCKET_HEIGHT,
                ROCKET_WIDTH,
                None,
            ));
        }

        Population {
            rockets,
            origin: Point::new(x, y),
            mating_pool: Vec::new(),
            generation: 0,
        }
    }

    /// Evaluates every rocket based on its fitness
    pub fn evaluate(&mut self, target: &Target, obstacle: &Obstacle) {
        let mut max_fitness = 0.0;
        self.mating_pool = Vec::new();
        self.generation += 1;

        println!("\nGeneration #{:} finished:", self.generation);

        let mut average = 0.0;
        let mut num_reached = 0;

        // Iterate over the entire population
        for i in 0..POPULATION_SIZE {
            // Calculate each one's fitness
            self.rockets[i].calculate_fitness(target, obstacle);

            // And calculate the maximum fitness
            if self.rockets[i].fitness > max_fitness {
                max_fitness = self.rockets[i].fitness;
            }

            average += self.rockets[i].fitness;
            if self.rockets[i].reached {
                num_reached += 1;
            }
        }

        println!(
            " - Average fitness: {:.2}",
            average / POPULATION_SIZE as f64
        );
        println!(" - Maximum fitness: {:.2}", max_fitness);
        println!(" - {:} rockets hit the target\n", num_reached);

        for i in 0..POPULATION_SIZE {
            self.rockets[i].fitness /= max_fitness;
        }

        for i in 0..POPULATION_SIZE {
            let n = (self.rockets[i].fitness * 100.0) as i32;
            for _ in 0..n {
                self.mating_pool.push(i);
            }
        }
    }

    /// Runs a natural selection on the current mating pool
    pub fn natural_selection(&mut self) {
        for i in 0..POPULATION_SIZE {
            // Choose two random parents
            let a = self.mating_pool.choose(&mut rand::thread_rng()).unwrap();
            let b = self.mating_pool.choose(&mut rand::thread_rng()).unwrap();

            // Pick each one's Dna
            let parent_a = &self.rockets[*a].dna;
            let parent_b = &self.rockets[*b].dna;

            // Cross both DNAs over
            let mut child = parent_a.crossover(parent_b);
            // Apply some random low-probability mutation
            child.mutate();

            let mut name = String::from("Rocket#");
            name.push_str(&i.to_string());

            self.rockets[i] = Rocket::new(
                name,
                Point::new(self.origin.x(), self.origin.y()),
                ROCKET_HEIGHT,
                ROCKET_WIDTH,
                Some(child),
            );
        }
    }
}
