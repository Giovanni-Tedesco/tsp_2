// use std::collections::BTreeMap;
use crate::city::City;
use crate::city::TOTAL_CITIES;
use crate::distributions::annealed::Annealed;
// use crate::tour::Tour;
use crate::cycle::Cycle;
use crate::city::Map;
use crate::fitness;
use crate::fitness::fitness;
use crate::fitness::tot_dist;

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
pub type FitFunc = Box<dyn Fn(&Cycle, &Map) -> f64>;


pub fn genetic_algorithm<T>(
    initial_population: &Vec<Rc<Cycle>>,
    params: &AlgorithmParams,    
    fitness: &FitFunc,
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

    println!("distance: {:?}, fitness: {:?}, cycle: {:?}"
    , fitness::tot_dist(&population[0], &towns_map)
    , fitness::fitness(&population[0], &towns_map)
    , population[0].path);

    let mut last_best = population[0].clone();

    for i in 0..params.rounds {


        let mut rng = rand::thread_rng();

        // println!("{:?}", population);

        let mut x: Vec<f64> = Vec::new();        

        
        for item in &population {
            x.push(calc_fitness(item, fitness, cache, towns_map, &last_best))
        }

        population.sort_by(|a, b| fitness::fitness(b, &towns_map)
        .partial_cmp(&fitness::fitness(a, &towns_map))
        .unwrap());
        // println!("{:?}", x);


        // let dist = WeightedIndex::new(x).unwrap();
        let dist = Annealed::new(&x);

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

        let ins = params.elitism.min(population.len());

        for i in 0..ins {
            new_population.push(population[i].clone());
        }

        last_best = population[0].clone();

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

pub fn calc_fitness (
        item: &Rc<Cycle>,
        fitness: &FitFunc,
        cache: &mut HashMap<Rc<Cycle>, f64>,
        towns_map: &Map,
        last_best: &Cycle
    ) -> f64
    where
    {
        let curr = fitness::tot_dist(item, towns_map);
        let best = fitness::tot_dist(last_best, towns_map);

        let fit = best / curr;

        match cache.entry(item.clone()) {
            Entry::Vacant(entry) => *entry.insert(fitness(item, towns_map)),
            Entry::Occupied(entry) => *entry.get()
        }

}

fn boltzmann_fit<T>(item: &Rc<Cycle>,
    fitness: &FitFunc
    , cache: &mut GenHash<Cycle>   
    , towns_map: &Map
    , params: &Boltzmann
) -> f64
where
    T: Hash + Eq
{
    match cache.entry(item.clone()) {
        Entry::Vacant(entry) => *entry.insert(boltzmann_probability(item, fitness, towns_map, params)),
        Entry::Occupied(entry) => *entry.get()
    }

}

fn boltzmann_probability (x: &Rc<Cycle>,
    fitness: &FitFunc,
    towns_map: &Map,
    params: &Boltzmann) -> f64 
where
{
    f64::exp(-((params.f_max - fitness(x, towns_map)))/get_t_boltzmann(params))

}

fn get_t_boltzmann(params: &Boltzmann) -> f64 {

    (params.t_coefficient).powf((1f64 + 100f64*params.generation) / params.max_generation)

}