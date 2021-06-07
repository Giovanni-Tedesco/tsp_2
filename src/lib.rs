pub mod cycle;
pub mod ga;
pub mod city;
pub mod fitness;

#[macro_use] extern crate maplit;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
