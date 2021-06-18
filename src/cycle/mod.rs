mod utils;
use genetic::GeneticCustom;
use rand::{Rng, distributions::Uniform, prelude::Distribution};
mod hash;
mod eq;

use crate::city::TOTAL_CITIES;

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
        let k = 2f64 / child.float_vec.len() as f64;

        let dice = Uniform::new(-k, k);

        // - 1 / k, + 1 / k
        for i in &mut child.float_vec { 
            let x: f64 = rng.gen();

            if x < mutation_rate {
                let z = dice.sample(&mut rng);
                if *i + z > 0f64 && *i + z < 1f64 {
                    *i += z
                }
            }
        }

        child.path = generate_path(&child.float_vec);


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

    pub fn reversal(&self) -> Self {
        let mut rng = rand::thread_rng();
        let mut gene = self.clone();
        let uniform = Uniform::new(0, TOTAL_CITIES);

        let a = uniform.sample(&mut rng);
        let b = uniform.sample(&mut rng);

        gene.float_vec.swap(a, b);
        gene.path = generate_path(&gene.float_vec);

        return gene
    }
}

impl GeneticCustom for Cycle {
    fn mutate_step(&self, other: &Self, params: &genetic::AlgorithmParams) -> (Self, Self) where Self: Sized {
       let (child_1, child_2) = self.cross_over(other, params.co_factor);
        // let child_1 = self.clone();
        // let child_2 = other.clone();

        let mutated_child_1 = child_1.mutate(params.mutation_rate);
        let mutated_child_2 = child_2.mutate(params.mutation_rate);

        let ret_1 = mutated_child_1.reversal();
        let ret_2 = mutated_child_2.reversal();

        return (ret_1, ret_2);
    }
}