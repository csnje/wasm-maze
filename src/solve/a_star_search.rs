use super::Solver;
use crate::geometry::taxicab_distance;
use crate::{Cell, Dimensions, DIRECTIONS};

use std::collections::BinaryHeap;
use std::marker::PhantomData;

/// Trait for the heuristic used in `AStarSearch`.
pub(crate) trait AStarSearchHeuristic {
    /// Calculate heuristic value.
    fn heuristic(dimensions: Dimensions, from: usize, to: usize) -> usize;
}

/// A type implementing `AStarSearchHeuristic` for the value zero.
///
/// When used in `AStarSearch` this will make the A* search algorithm equivalent to
/// [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).
#[derive(Default)]
pub(crate) struct Zero;

impl AStarSearchHeuristic for Zero {
    /// Calculate heuristic value.
    fn heuristic(_: Dimensions, _: usize, _: usize) -> usize {
        0
    }
}

/// A type implementing `AStarSearchHeuristic` for the taxicab distance between cells.
#[derive(Default)]
pub(crate) struct TaxicabDistance;

impl AStarSearchHeuristic for TaxicabDistance {
    /// Calculate heuristic value.
    fn heuristic(dimensions: Dimensions, from: usize, to: usize) -> usize {
        taxicab_distance(dimensions, from, to)
    }
}

/// A type implementing the [A* search algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
/// to solve a maze.
///
/// When the heristic is not used (i.e. is zero) this will be equivalent to
/// [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).
#[derive(Default)]
pub(crate) struct AStarSearch<T: AStarSearchHeuristic> {
    initialised: bool,
    phantom: PhantomData<T>,
    // shortest distance so far for each cell
    distances: Vec<Option<usize>>,
    // fringe (or frontier) priority queue of the shortest distance
    // plus a heuristic estimate of the remaining distance for cells
    fringe: BinaryHeap<AStarSearchState>,
}

impl<T: AStarSearchHeuristic> Solver for AStarSearch<T> {
    /// Apply a step of the algorithm.
    fn step(
        &mut self,
        dimensions: Dimensions,
        cells: &mut Vec<Cell>,
        from: usize,
        to: usize,
    ) -> bool {
        if !self.initialised {
            // start of the algorithm
            web_sys::console::log_1(&"solve using A* search algorithm".into());

            self.distances.resize(cells.len(), None);
            self.distances[from] = Some(0);
            self.fringe.push(AStarSearchState {
                cost: T::heuristic(dimensions, from, to),
                cell: from,
            });

            self.initialised = true;
        } else {
            match self.fringe.pop() {
                Some(AStarSearchState { cost: _, cell }) => {
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
                        self.distances.clear();
                        self.fringe.clear();
                        return false;
                    }

                    // housekeeping; remove all additional entries of cell from fringe
                    self.fringe.retain(|state| state.cell != cell);

                    // accessible unvisited neighbours
                    let neighbours = DIRECTIONS
                        .iter()
                        .filter(|direction| !cells[cell].has_wall(**direction))
                        .filter_map(|direction| direction.neighbour(dimensions, cell))
                        .filter(|neighbour| {
                            *neighbour != from && cells[*neighbour].solution.previous.is_none()
                        })
                        .collect::<Vec<_>>();

                    for neighbour in neighbours {
                        let distance = self.distances[cell].unwrap() + 1; // move 1 additional cell
                        if self.distances[neighbour].map_or(true, |val| distance < val) {
                            cells[neighbour].solution.previous = Some(cell);
                            self.distances[neighbour] = Some(distance);
                            self.fringe.push(AStarSearchState {
                                cost: distance + T::heuristic(dimensions, neighbour, to),
                                cell: neighbour,
                            });
                        }
                    }
                }
                None => unreachable!(),
            }
        }

        true
    }
}

/// A type holding state for the A* search algorithm.
#[derive(Eq, PartialEq)]
struct AStarSearchState {
    cost: usize,
    cell: usize,
}

impl Ord for AStarSearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.cell.cmp(&other.cell))
    }
}

impl PartialOrd for AStarSearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
