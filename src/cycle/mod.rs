mod utils;
use genetic::GeneticCustom;
use rand::{Rng, distributions::Uniform, prelude::Distribution};
mod hash;
mod eq;

use self::utils::{generate_floats, generate_path};


#[derive(Debug, Clone)]
pub struct Cycle {
    pub path: Vec<usize>,
    pub float_vec: Vec<f64>
}

impl Cycle {
    /// Generate a completely random cycle.
    pub fn new(n: usize) -> Self {
        let float_vec = generate_floats(n);
        let path = generate_path(&float_vec);

        return Cycle {
            float_vec,
            path
        }
    }

    // TODO: Improve this... it kind of sucks
    pub fn mutate(&self, mutation_rate: f64) -> Self {
        let mut child = self.clone();
        let mut rng = rand::thread_rng();
        let dice = Uniform::new(1, child.path.len());

        let pos_1 = dice.sample(&mut rng);
        let pos_2 = dice.sample(&mut rng);

        child.path.swap(pos_1, pos_2);

        return child;

    }

    // TODO: Also Improve this... it kinda sucks as well
    pub fn cross_over(&self, other: &Self , co_factor: f64) -> (Self, Self) {
        let mut c_1 = self.clone();
        let mut c_2 = other.clone();

        let mut rng = rand::thread_rng();

        for i in 0..c_1.float_vec.len() {
            let x: f64 = rng.gen();

            if x < co_factor {
                let temp = c_1.float_vec[i];
                c_1.float_vec[i] = c_2.float_vec[i];
                c_2.float_vec[i] = temp;
            }
        }

        c_1.path = generate_path(&c_1.float_vec);
        c_2.path =  generate_path(&c_2.float_vec);

        return (c_1, c_2)
    }
}

impl GeneticCustom for Cycle {
    fn mutate_step(&self, other: &Self, params: &genetic::AlgorithmParams) -> (Self, Self) where Self: Sized {
       let (child_1, child_2) = self.cross_over(other, params.co_factor);


        let mutated_child_1 = child_1.mutate(params.mutation_rate);
        let mutated_child_2 = child_2.mutate(params.mutation_rate);

        return (mutated_child_1, mutated_child_2);
    }
}