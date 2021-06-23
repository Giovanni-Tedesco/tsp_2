use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::city::{self, City, Map};

pub fn file_reader(filename: &String) -> Map{

    let mut city_map: Map = HashMap::new();

    if let Ok(lines) = read_lines(filename) {

        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(" ").collect();

                let key: usize = split[0].parse().unwrap();
                let x_coord: f64 = split[1].parse().unwrap();
                let y_coord: f64 = split[2].parse().unwrap();

                let ins_city = City {
                    x: x_coord,
                    y: y_coord
                };

                city_map.insert(key - 1, ins_city);

            }
        }

    }

    return city_map;

}


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}