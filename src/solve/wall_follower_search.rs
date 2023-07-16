use super::Solver;

use crate::direction::Direction;
use crate::Dimensions;

use std::marker::PhantomData;

/// Trait for turning used in `WallFollower`.
pub(crate) trait WallFollowerSearchTurnDirection {
    /// Initial turn.
    fn initial(direction: Direction) -> Direction;

    /// Subsequent turn.
    fn subsequent(direction: Direction) -> Direction;
}

/// A type implementing `WallFollowerSearchTurnDirection` for right turns.
#[derive(Default)]
pub(crate) struct Right;

impl WallFollowerSearchTurnDirection for Right {
    /// Initial turn.
    fn initial(direction: Direction) -> Direction {
        direction.next()
    }

    /// Subsequent turn.
    fn subsequent(direction: Direction) -> Direction {
        direction.prev()
    }
}

/// A type implementing `WallFollowerSearchTurnDirection` for left turns.
#[derive(Default)]
pub(crate) struct Left;

impl WallFollowerSearchTurnDirection for Left {
    /// Initial turn.
    fn initial(direction: Direction) -> Direction {
        direction.prev()
    }

    /// Subsequent turn.
    fn subsequent(direction: Direction) -> Direction {
        direction.next()
    }
}

/// A type implementing a wall following search algorithm to solve a maze.
#[derive(Default)]
pub(crate) struct WallFollowerSearch<T: WallFollowerSearchTurnDirection> {
    phantom: PhantomData<T>,
    // current cell index and direction; if None then start of the algorithm
    cell_and_direction: Option<(usize, Direction)>,
}

impl<T: WallFollowerSearchTurnDirection> Solver for WallFollowerSearch<T> {
    fn step(
        &mut self,
        dimensions: Dimensions,
        cells: &mut Vec<crate::Cell>,
        from: usize,
        to: usize,
    ) -> bool {
        // loop used to backtrack search path in one step
        loop {
            match self.cell_and_direction {
                None => {
                    // start of the algorithm
                    web_sys::console::log_1(&"solve using wall follower search algorithm".into());
                    self.cell_and_direction = Some((from, Direction::First));
                    break;
                }
                Some((cell, direction)) => {
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

                        self.cell_and_direction = None;
                        return false;
                    }

                    // neighbour depending on turn direction
                    let mut direction = T::initial(direction);
                    let neighbour = loop {
                        if !cells[cell].has_wall(direction) {
                            break direction
                                .neighbour(dimensions, cell)
                                .expect("should have neighbour");
                        }
                        direction = T::subsequent(direction);
                    };

                    let backtrack = if cells[neighbour].solution.previous.is_none() {
                        cells[neighbour].solution.previous = Some(cell);
                        false
                    } else {
                        true
                    };

                    self.cell_and_direction = Some((neighbour, direction));

                    if !backtrack {
                        break;
                    }
                }
            }
        }

        true
    }
}
