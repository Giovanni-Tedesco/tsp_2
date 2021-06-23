// use std::array::IntoIter;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::iter::FromIterator;
use std::env;

use crate::city::Map;

use genetic::AlgorithmParams;
use maplit::hashmap;
use tsp_2::*;
use tsp_2::cycle::Cycle;
use tsp_2::city::City;
use tsp_2::city::TOTAL_CITIES;
use tsp_2::plot::plot_graph;

use std::rc::Rc;


fn main() {

    let args: Vec<String> = env::args().collect();
    
    let filename = &args[1];

    let town_map = utils::file_reader(filename);

    let params = AlgorithmParams {
        rounds: 5000,
        max_popuation: 500,
        mutation_rate: 0.2,
        co_factor: 0.5,
        elitism: 250,
    };

    let initial_population2: Vec<Cycle> = Vec::new();
    let mut par_cache: HashMap<Cycle, f64> = HashMap::new();
    // let mut town_map = get_map();

    let mut y = ga::ga_parallel::genetic_parallel(&initial_population2, &params, &mut par_cache, &town_map);
    

    // let mut x: Vec<Rc<Cycle>> = ga::genetic_algorithm::<Cycle>(&initial_population, &params, &fitness, &mut cache, &town_map);

    y.sort_by(|a, b| fitness::fitness(b, &town_map)
    .partial_cmp(&fitness::fitness(a, &town_map))
    .unwrap());

    println!("distance: {:?}, fitness: {:?}, cycle: {:?}, floats: {:?}"
        , fitness::tot_dist(&y[0], &town_map)
        , fitness::fitness(&y[0], &town_map)
        , y[0].path
        , y[0].float_vec
    );

    plot_graph(&y[0], &town_map);

}


fn get_map() -> Map {
    hashmap!{
    0usize => City{x:6734f64, y:1453f64},
    1  => City{x:2233f64, y:10f64},
    2  => City{x:5530f64, y:1424f64},
    3  => City{x:401f64,  y:841f64},
    4  => City{x:3082f64, y:1644f64},
    5  => City{x:7608f64, y:4458f64},
    6  => City{x:7573f64, y:3716f64},
    7  => City{x:7265f64, y:1268f64},
    8  => City{x:6898f64, y:1885f64},
    9 => City{x:1112f64, y:2049f64},
    10 => City{x:5468f64, y:2606f64},
    11 => City{x:5989f64, y:2873f64},
    12 => City{x:4706f64, y:2674f64},
    13 => City{x:4612f64, y:2035f64},
    14 => City{x:6347f64, y:2683f64},
    15 => City{x:6107f64, y:669f64},
    16 => City{x:7611f64, y:5184f64},
    17 => City{x:7462f64, y:3590f64},
    18 => City{x:7732f64, y:4723f64},
    19 => City{x:5900f64, y:3561f64},
    20 => City{x:4483f64, y:3369f64},
    21 => City{x:6101f64, y:1110f64},
    22 => City{x:5199f64, y:2182f64},
    23 => City{x:1633f64, y:2809f64},
    24 => City{x:4307f64, y:2322f64},
    25 => City{x:675f64,  y:1006f64},
    26 => City{x:7555f64, y:4819f64},
    27 => City{x:7541f64, y:3981f64},
    28 => City{x:3177f64, y:756f64},
    29 => City{x:7352f64, y:4506f64},
    30 => City{x:7545f64, y:2801f64},
    31 => City{x:3245f64, y:3305f64},
    32 => City{x:6426f64, y:3173f64},
    33 => City{x:4608f64, y:1198f64},
    34 => City{x:23f64,   y:2216f64},
    35 => City{x:7248f64, y:3779f64},
    36 => City{x:7762f64, y:4595f64},
    37 => City{x:7392f64, y:2244f64},
    38 => City{x:3484f64, y:2829f64},
    39 => City{x:6271f64, y:2135f64},
    40 => City{x:4985f64, y:140f64},
    41 => City{x:1916f64, y:1569f64},
    42 => City{x:7280f64, y:4899f64},
    43 => City{x:7509f64, y:3239f64},
    44 => City{x:10f64,   y:2676f64},
    45 => City{x:6807f64, y:2993f64},
    46 => City{x:5185f64, y:3258f64},
    47 => City{x:3023f64, y:1942f64}
    }


}

