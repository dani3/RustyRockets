use rand::Rng;
use vector2d::Vector2D;

pub struct Dna {
    pub genes: Vec<Vector2D<f64>>,
}

impl Dna {
    pub fn new(capacity: usize, optional_genes: Option<Vec<Vector2D<f64>>>) -> Self {
        if let Some(genes) = optional_genes {
            return Dna { genes };
        } else {
            let mut genes = Vec::new();
            let mut rng = rand::thread_rng();

            for _ in 0..capacity {
                let vx = rng.gen_range(-0.2, 0.2) as f64;
                let vy = rng.gen_range(-0.2, 0.2) as f64;

                genes.push(Vector2D::new(vx, vy));
            }

            Dna { genes }
        }
    }

    /// Crosses over two different DNAs producing a new Dna
    pub fn crossover(&self, partner_dna: &Dna) -> Dna {
        let mut rng = rand::thread_rng();

        let mut new_genes: Vec<Vector2D<f64>> = Vec::new();
        for i in 0..self.genes.len() {
            let middle = rng.gen_range(0, self.genes.len());
            if i > middle {
                new_genes.push(self.genes[i]);
            } else {
                new_genes.push(partner_dna.genes[i]);
            }
        }

        Dna::new(self.genes.len(), Some(new_genes))
    }

    /// Generates some random mutation in a specific gene
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..self.genes.len() {
            if rng.gen_range(0.0, 1.0) < 0.01 {
                let vx = rng.gen_range(-0.2, 0.2) as f64;
                let vy = rng.gen_range(-0.2, 0.2) as f64;

                self.genes[i] = Vector2D::new(vx, vy);
            }
        }
    }
}
