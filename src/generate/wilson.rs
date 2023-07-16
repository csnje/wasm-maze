use super::Generator;
use crate::{Cell, Dimensions, Direction, DIRECTIONS};

use js_sys::Math::random;

/// A type implementing [Wilson's algorithm](https://en.wikipedia.org/wiki/Loop-erased_random_walk)
/// to generate a maze.
#[derive(Default)]
pub(crate) struct Wilson {
    // current walk index; if None then start of the algorithm
    walk: Option<usize>,
    // stack of cell indexes for the current walk; if empty then start of new walk
    stack: Vec<usize>,
}

impl Generator for Wilson {
    /// Apply a step of the algorithm.
    fn step(&mut self, dimensions: Dimensions, cells: &mut Vec<Cell>) -> bool {
        match self.walk {
            None => {
                // start of the algorithm; select a single random cell
                // which is the destination of the first complete walk
                web_sys::console::log_1(&"create using Wilson's algorithm".into());
                let idx = (random() * cells.len() as f64) as usize;
                cells[idx].walk = Some(0);
                self.walk = Some(1);
            }
            Some(walk) => {
                match self.stack.last() {
                    None => {
                        match cells
                            .iter()
                            .enumerate()
                            .find(|(_, cell)| cell.walk.is_none())
                        {
                            None => {
                                // end of algorithm; reset data
                                web_sys::console::log_1(&"create is complete".into());
                                self.walk = None;
                                self.stack.clear();
                                return false;
                            }
                            Some((idx, _)) => {
                                // start of new walk
                                cells[idx].walk = Some(walk);
                                self.stack.push(idx);
                            }
                        }
                    }
                    Some(cell) => {
                        // continue the current walk

                        // neighbours
                        let mut neighbour = {
                            let neighbours = DIRECTIONS
                                .iter()
                                .filter_map(|direction| direction.neighbour(dimensions, *cell))
                                .collect::<Vec<_>>();

                            // pick neighbour at random
                            neighbours[(random() * neighbours.len() as f64) as usize]
                        };

                        match cells[neighbour].walk {
                            None => {
                                // add cell to current walk
                                cells[neighbour].walk = Some(walk);
                                self.stack.push(neighbour);
                            }
                            Some(neighbour_walk) => {
                                if walk == neighbour_walk {
                                    // encountered the current walk; erase the loop
                                    while *self.stack.last().unwrap() != neighbour {
                                        cells[self.stack.pop().unwrap()].walk = None;
                                    }
                                } else {
                                    // encountered a previous walk; complete the current walk
                                    web_sys::console::log_1(
                                        &format!("walk {walk} is complete").into(),
                                    );
                                    self.walk = Some(walk + 1);
                                    while let Some(last) = self.stack.pop() {
                                        match Direction::between(dimensions, last, neighbour) {
                                            Some(direction) => match direction {
                                                Direction::First => {
                                                    cells[last].remove_wall(Direction::First);
                                                    cells[neighbour].remove_wall(Direction::Third);
                                                }
                                                Direction::Second => {
                                                    cells[last].remove_wall(Direction::Second);
                                                    cells[neighbour].remove_wall(Direction::Forth);
                                                }
                                                Direction::Third => {
                                                    cells[last].remove_wall(Direction::Third);
                                                    cells[neighbour].remove_wall(Direction::First);
                                                }
                                                Direction::Forth => {
                                                    cells[last].remove_wall(Direction::Forth);
                                                    cells[neighbour].remove_wall(Direction::Second);
                                                }
                                            },
                                            None => unreachable!(),
                                        }

                                        neighbour = last;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        true
    }
}
