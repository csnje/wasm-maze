use super::Generator;
use crate::{Cell, Dimensions, Direction, DIRECTIONS};

use js_sys::Math::random;

/// A type implementing a randomised [depth first search](https://en.wikipedia.org/wiki/Depth-first_search)
/// algorithm to generate a maze.
#[derive(Default)]
pub(crate) struct RandomisedDepthFirstSearch {
    initialised: bool,
    // stack of cell indexes
    stack: Vec<usize>,
}

impl Generator for RandomisedDepthFirstSearch {
    /// Apply a step of the algorithm.
    fn step(&mut self, dimensions: Dimensions, cells: &mut Vec<Cell>) -> bool {
        const WALK: usize = 0;

        if !self.initialised {
            // start of the algorithm; select a random from cell
            web_sys::console::log_1(&"create using randomised depth first search algorithm".into());
            let from = (random() * cells.len() as f64) as usize;
            cells[from].walk = Some(WALK);
            self.initialised = true;
            self.stack.push(from);
        } else {
            // loop used to backtrack search path in one step
            loop {
                match self.stack.pop() {
                    None => {
                        // end of algorithm; reset data
                        web_sys::console::log_1(&"create is complete".into());
                        self.initialised = false;
                        self.stack.clear();
                        return false;
                    }
                    Some(cell) => {
                        let neighbour = {
                            // unvisited neighbours
                            let neighbours = DIRECTIONS
                                .iter()
                                .filter_map(|direction| direction.neighbour(dimensions, cell))
                                .filter(|neighbour| cells[*neighbour].walk.is_none())
                                .collect::<Vec<_>>();

                            // pick neighbour (if any) at random
                            match neighbours.len() {
                                0 => None,
                                len => Some(neighbours[(random() * len as f64) as usize]),
                            }
                        };

                        if let Some(neighbour) = neighbour {
                            cells[neighbour].walk = Some(WALK);
                            match Direction::between(dimensions, cell, neighbour) {
                                Some(direction) => match direction {
                                    Direction::First => {
                                        cells[cell].remove_wall(Direction::First);
                                        cells[neighbour].remove_wall(Direction::Third);
                                    }
                                    Direction::Second => {
                                        cells[cell].remove_wall(Direction::Second);
                                        cells[neighbour].remove_wall(Direction::Forth);
                                    }
                                    Direction::Third => {
                                        cells[cell].remove_wall(Direction::Third);
                                        cells[neighbour].remove_wall(Direction::First);
                                    }
                                    Direction::Forth => {
                                        cells[cell].remove_wall(Direction::Forth);
                                        cells[neighbour].remove_wall(Direction::Second);
                                    }
                                },
                                None => unreachable!(),
                            }
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
