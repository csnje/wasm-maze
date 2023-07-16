use crate::{Cell, Dimensions};

pub(crate) trait Generator {
    /// Apply a step of the algorithm.
    fn step(&mut self, dimensions: Dimensions, cells: &mut Vec<Cell>) -> bool;
}
