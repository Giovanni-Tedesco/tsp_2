use std::env;

use tsp_2::utils;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("{:?}", utils::file_reader(filename));

}