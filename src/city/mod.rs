use std::collections::HashMap;

use rand::{distributions::{Uniform, uniform}, prelude::Distribution};


pub type Map = HashMap<usize, City>;

pub const TOTAL_CITIES: usize = 237;
#[derive(Debug)]
pub struct City {
    pub x: f64,
    pub y: f64
}

impl City {
    pub fn new(x: f64, y: f64) -> Self {
        City {
            x,
            y
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        f64::sqrt((self.x - other.x).powf(2f64) + (self.y - other.y).powf(2f64))
    }

    pub fn generate_cities(n: usize) -> Map {
        let mut rng = rand::thread_rng();
        let dice = Uniform::new(0f64, 100f64);

        let mut ret_map: Map = HashMap::new();

        for i in 0..n {
            let x = dice.sample(&mut rng);
            let y = dice.sample(&mut rng);

            let city = City {
                x,
                y
            };

            ret_map.insert(i, city);
        }

        return ret_map;

    }
}