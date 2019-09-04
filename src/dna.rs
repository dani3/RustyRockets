use crate::constants::*;

use vector2d::Vector2D;

use rand::Rng;

pub struct DNA {
    genes : Vec<Vector2D<f64>>
}

impl DNA {
    pub fn new(optional_genes : Option<Vec<Vector2D<f64>>>) -> Self {
        if let Some(genes) = optional_genes {
            return DNA {
                genes
            };

        } else {

            let mut genes = Vec::new();
            let mut rng = rand::thread_rng();
            for _ in 0 .. LIFESPAN {
                let vx = rng.gen_range(-0.5, 0.5) as f64;
                let vy = rng.gen_range(-0.5, 0.5) as f64;

                genes.push(Vector2D::new(vx, vy));
            }

            DNA {
                genes
            }
        }
    }

    pub fn get_genes(&self) -> &Vec<Vector2D<f64>> {
        &self.genes
    }

    pub fn crossover(&self, partner_dna : &DNA) -> DNA {
        let mut rng = rand::thread_rng();

        let mut new_genes : Vec<Vector2D<f64>> = Vec::new();
        let mid = rng.gen_range(0, LIFESPAN);

        for i in 0 .. LIFESPAN {
            if i > mid {
                new_genes.push(self.genes[i as usize]);
            } else {
                new_genes.push(partner_dna.genes[i as usize]);
            }
        }

        DNA::new(Some(new_genes))
    }
}