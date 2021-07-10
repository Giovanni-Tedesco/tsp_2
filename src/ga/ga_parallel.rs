use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::city::Map;
use crate::cycle::Cycle;
use crate::distributions::annealed::Annealed;
use crate::fitness;
use crate::city::TOTAL_CITIES;
use crate::fitness::fitness;
use crate::plot;

use genetic::{AlgorithmParams, GeneticCustom};
use genetic::abstractions::FitFunc;
use rayon::prelude::*;



pub type CycleVec = Vec<Cycle>;
pub type ParHash<T> = HashMap<T, f64>;


pub fn genetic_parallel(
    initial_population: &CycleVec,
    params: &mut AlgorithmParams,    
    cache: &mut ParHash<Cycle>,
    town_map: &Map
) -> Vec<Cycle> 
{
    let mut rng = rand::thread_rng();

    let mut population = if initial_population.is_empty() {
        let mut x: Vec<Cycle> = Vec::new();
        x.push(Cycle::new(TOTAL_CITIES));
        x
    } else {
        initial_population.clone()
    };

    println!("distance: {:?}, fitness: {:?}, cycle: {:?}"
    , fitness::tot_dist(&population[0], &town_map)
    , fitness::fitness(&population[0], &town_map)
    , population[0].path);

    plot::plot_graph(&population[0], town_map);

    let mut last_best = population[0].clone();

    // let mut i = 0;

    for i in 0..params.rounds {
    
        let fitnesses = parallel_fitness(&population, town_map, &last_best, cache);

        population.sort_by(|a, b| fitness::fitness(b, &town_map)
        .partial_cmp(&fitness::fitness(a, &town_map))
        .unwrap());

        let dist = Annealed::new(&fitnesses);

        println!("distance: {:?}, fitness: {:?}, cycle: {:?}"
        , fitness::tot_dist(&population[0], &town_map)
        , fitness::fitness(&population[0], &town_map)
        , i);



        let mut new_population: Vec<Cycle> = Vec::new();

        // This is done in an effor to avoid hitting a local-minima.
        // How much it works is questionable.
        if i % 1000 == 0 {
            params.elitism /= 2
        }

        let ins = params.elitism.min(population.len());
        
        for i in 0..ins {
            new_population.push(population[i].clone());
        }

        last_best = population[0].clone();

        // TODO: Can this be parallelized
        while new_population.len() < params.max_popuation as usize {
            let parent_1 = &population[dist.sample(&mut rng)];
            let parent_2 = &population[dist.sample(&mut rng)];

            // let (child_1, child_2 )= parent_1.cross_over(&parent_2, params.co_factor);

            // let mutated_child_1 = child_1.mutation(params.mutation_rate);
            // let mutated_child_2 = child_2.mutation(params.mutation_rate);
            let (mutated_child_1, mutated_child_2) = parent_1.mutate_step(&parent_2, params);

            new_population.push(mutated_child_1);
            new_population.push(mutated_child_2);
        }


        population = new_population;

    }

    return population;

}


pub fn parallel_fitness(population: &CycleVec, town_map: &Map, last_best: &Cycle, cache: &mut ParHash<Cycle>) -> Vec<f64> {
    
    let mut x = cache.clone();
    population.par_iter().map(|item| calc_fitness(item, &x, last_best, town_map)).collect::<Vec<f64>>()

}

// TODO: Try a 'hashbrown map' Designed specifically for parallelisation.
pub fn calc_fitness(item: &Cycle
    ,cache:  &ParHash<Cycle>
    ,last_best: &Cycle
    , town_map: &Map
) -> f64 
{
    let mut x = cache.clone();

    match x.entry(item.clone()) {
        Entry::Occupied(entry) => *entry.get(),
        Entry::Vacant(entry) => *entry.insert(fitness::fitness(item, town_map)),
    }

}