use crate::{Cell, Dimensions};

pub(crate) trait Solver {
    /// Apply a step of the algorithm.
    fn step(
        &mut self,
        dimensions: Dimensions,
        cells: &mut Vec<Cell>,
        from: usize,
        to: usize,
    ) -> bool;
}
