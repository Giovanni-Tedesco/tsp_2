use crate::cycle::Cycle;

impl PartialEq for Cycle {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Cycle {

}