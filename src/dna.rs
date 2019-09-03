use vector2d::Vector2D;

use rand::Rng;

pub struct DNA {
    genes : Vec<Vector2D<f64>>
}

impl DNA {
    pub fn new(lifespan: isize) -> Self {
        let mut genes = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0 .. lifespan {
            let vx = rng.gen_range(-0.5, 0.5) as f64;
            let vy = rng.gen_range(-0.5, 0.5) as f64;

            genes.push(Vector2D::new(vx, vy));
        }

        DNA {
            genes
        }
    }

    pub fn get_genes(&self) -> &Vec<Vector2D<f64>> {
        &self.genes
    }
}