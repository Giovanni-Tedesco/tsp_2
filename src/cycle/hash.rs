use crate::cycle::Cycle;
use std::hash::Hash;


impl Hash for Cycle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}