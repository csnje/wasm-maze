use super::Solver;
use crate::{Dimensions, DIRECTIONS};

use js_sys::Math::random;

/// A type implementing a randomised [depth first search](https://en.wikipedia.org/wiki/Depth-first_search)
/// algorithm to solve a maze.
#[derive(Default)]
pub(crate) struct RandomisedDepthFirstSearch {
    initialised: bool,
    // stack of cell indexes
    stack: Vec<usize>,
}

impl Solver for RandomisedDepthFirstSearch {
    fn step(
        &mut self,
        dimensions: Dimensions,
        cells: &mut Vec<crate::Cell>,
        from: usize,
        to: usize,
    ) -> bool {
        if !self.initialised {
            // start of the algorithm
            web_sys::console::log_1(&"solve using randomised depth first search algorithm".into());
            self.initialised = true;
        } else {
            // loop used to backtrack search path in one step
            loop {
                match self.stack.pop() {
                    None => {
                        // reset stack; applies if first search or previous search exhausted
                        self.stack.push(from);
                    }
                    Some(cell) => {
                        if cell == to {
                            // end of algorithm; flag path and reset data
                            web_sys::console::log_1(&"solve is complete".into());

                            let mut cell = to;
                            while cell != from {
                                cells[cell].solution.result = true;
                                cell = cells[cell]
                                    .solution
                                    .previous
                                    .expect("should have previous cell");
                            }

                            self.initialised = false;
                            self.stack.clear();
                            return false;
                        }

                        let neighbour = {
                            // accessible unvisited neighbours
                            let neighbours = DIRECTIONS
                                .iter()
                                .filter(|direction| !cells[cell].has_wall(**direction))
                                .filter_map(|direction| direction.neighbour(dimensions, cell))
                                .filter(|neighbour| {
                                    *neighbour != from
                                        && cells[*neighbour].solution.previous.is_none()
                                })
                                .collect::<Vec<_>>();

                            // pick neighbour (if any) at random
                            match neighbours.len() {
                                0 => None,
                                len => Some(neighbours[(random() * len as f64) as usize]),
                            }
                        };

                        if let Some(neighbour) = neighbour {
                            cells[neighbour].solution.previous = Some(cell);
                            self.stack.push(cell);
                            self.stack.push(neighbour);
                            break;
                        }
                    }
                }
            }
        }

        true
    }
}
