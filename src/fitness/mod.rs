use crate::city::Map;
use crate::cycle::Cycle;

pub fn fitness(cycle: &Cycle, town_map: &Map) -> f64 {

    // Calculate the total length of the tour...
    let mut total = 0f64;
    let mut curr_cycle = cycle.clone();

    // if contains_all_cities(t) == false {
    //     return 0f64
    // }
    curr_cycle.path.push(0);

    for (i, city) in curr_cycle.path.iter().enumerate() {
        let next_city = match curr_cycle.path.get(i + 1) {
            Some(next_city) => {
                town_map.get(next_city).unwrap()
            }
            None => {
                let x = *cycle.path.get(0).as_ref().unwrap();
                town_map.get(x).unwrap()
            }
        };

        let curr_city = town_map.get(city).unwrap();

        total += curr_city.distance(next_city);
    }

    let x = 1f64/total + 1f64;

    if x.is_infinite() {
        return 0f64;
    } else {
        return 1f64/total;
    }
}

pub fn tot_dist(cycle: &Cycle, town_map: &Map) -> f64 {
    // Calculate the total length of the tour...
    let mut total = 0f64;

    // if contains_all_cities(t) == false {
    //     return 0f64
    // }

    let mut curr_cycle = cycle.clone();
    curr_cycle.path.push(0);

    for (i, city) in curr_cycle.path.iter().enumerate() {
        let next_city = match curr_cycle.path.get(i + 1) {
            Some(next_city) => {
                town_map.get(next_city).unwrap()
            }
            None => {
                let x = *cycle.path.get(0).as_ref().unwrap();
                town_map.get(x).unwrap()
            }
        };

        let curr_city = town_map.get(city).unwrap();

        total += curr_city.distance(next_city);
    }


    return total;

}