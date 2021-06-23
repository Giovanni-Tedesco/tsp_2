pub mod cycle;
pub mod ga;
pub mod city;
pub mod fitness;
pub mod distributions;
pub mod utils;

// Ant Colony Simulation
pub mod antcolony;

// Plotting
pub mod plot;

#[macro_use] extern crate maplit;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
