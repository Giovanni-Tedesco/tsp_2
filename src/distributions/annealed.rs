use rand::distributions::Uniform;
use rand::prelude::{Distribution, ThreadRng};

use crate::city::City;
use crate::ga::FitFunc;

pub struct Annealed {
    S: f64,
    fitnesses: Vec<f64>
}


impl Annealed {
    pub fn new(fitnesses: &Vec<f64>) -> Self {
        let mut S = 0f64;
        for fit in fitnesses {
            S += fit;
        }


        return Annealed {
            S,
            fitnesses: fitnesses.clone()
        }
    }

    pub fn sample(&self, rng: &mut ThreadRng) -> usize {
        let uniform = Uniform::new(0f64, self.S);
        let mut cj = 0f64;

        let r = uniform.sample(rng);


        for (idx, fit) in self.fitnesses.iter().enumerate() {
            cj += fit;

            if r <= cj { 
                return idx;
            }

        }

        return 0;
    }
}

pub fn annealed_selection(fitnesses: &Vec<f64>) -> usize {
    let mut rng = rand::thread_rng();

    let mut S = 0f64;
    for fit in fitnesses {
        S += fit;
    }

    let uniform = Uniform::new(0f64, S);
    let r = uniform.sample(&mut rng);
    let mut cj = 0f64;

    for (idx, fit) in fitnesses.iter().enumerate() {
        cj += fit;

        if r <= cj { 
            return idx;
        }

    }

    return 0;

}