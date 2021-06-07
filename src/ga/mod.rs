// use std::collections::BTreeMap;
use crate::city::City;
use crate::city::TOTAL_CITIES;
// use crate::tour::Tour;
use crate::cycle::Cycle;
use crate::city::Map;

use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;
// use std::cmp::Ordering;


use rand::distributions::WeightedIndex;
// use rand::Rng;
use rand::distributions::{Distribution};

use genetic::AlgorithmParams;
use genetic::distributions::standard_weighted::StandardWeighted;
use genetic::{abstractions::CustomDistribution};
// use crate::utils::BoltzmannParams;

use genetic::distributions::boltzmann::Boltzmann;


use genetic::GeneticCustom;


pub type GenHash<T> = HashMap<Rc<T>, f64>;


pub fn genetic_algorithm<T>(
    initial_population: &Vec<Rc<Cycle>>,
    params: &AlgorithmParams,    
    fitness: &Box<dyn Fn(&Cycle, &Map) -> f64>,
    cache: &mut GenHash<Cycle>,
    towns_map: &Map
) -> Vec<Rc<Cycle>> 
where
T: GeneticCustom + Eq + Hash
{

    let mut population = if initial_population.is_empty() {
        let mut x: Vec<Rc<Cycle>> = Vec::new();
        x.push(Rc::new(Cycle::new(TOTAL_CITIES)));
        x
    } else {
        initial_population.clone()
    };

    for i in 0..params.rounds {

        let mut rng = rand::thread_rng();

        // println!("{:?}", population);

        let mut x: Vec<f64> = Vec::new();        
        
        for item in &population {
            x.push(calc_fitness(item, fitness, cache, towns_map));
        }

        // println!("{:?}", x);


        let dist = WeightedIndex::new(x).unwrap();
        // let boltzmann_params = Boltzmann {
        //     distribution: None,
        //     t_coefficient: 1f64,
        //     f_max: 1f64,
        //     generation: i as f64,
        //     max_generation: params.rounds as f64,
        // };

        // let std_weighted = StandardWeighted{
        //     distribution: None
        // };

        // println!("Population: {:?}" ,population);
        // let sampler= std_weighted.new(&population, fitness, cache);
        // let dist = sampler.distribution.unwrap();


        // let dist = Boltzmann::boltzmann_selection(&population, boltzmann_params, fitness, cache);        
        // let sampler = boltzmann_params.new(&population, fitness, cache);
        // let dist = sampler.distribution.unwrap();


        let mut new_population: Vec<Rc<Cycle>> = Vec::new();

        while new_population.len() < params.max_popuation as usize {
            let parent_1 = &population[dist.sample(&mut rng)];
            let parent_2 = &population[dist.sample(&mut rng)];

            // let (child_1, child_2 )= parent_1.cross_over(&parent_2, params.co_factor);

            // let mutated_child_1 = child_1.mutation(params.mutation_rate);
            // let mutated_child_2 = child_2.mutation(params.mutation_rate);
            let (mutated_child_1, mutated_child_2) = parent_1.mutate_step(&parent_2, params);

            new_population.push(Rc::new(mutated_child_1));
            new_population.push(Rc::new(mutated_child_2));
        }

        population = new_population;

    }

    return population;

}

pub fn calc_fitness<T>(
        item: &Rc<T>,
        fitness: &Box<dyn Fn(&T, &Map) -> f64>,
        cache: &mut HashMap<Rc<T>, f64>,
        towns_map: &Map
    ) -> f64
    where
        T: Hash + Eq
    {
        match cache.entry(item.clone()) {
            Entry::Vacant(entry) => *entry.insert(fitness(item, towns_map)),
            Entry::Occupied(entry) => *entry.get()
        }

}