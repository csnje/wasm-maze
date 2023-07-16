use crate::Dimensions;

/// A type providing directions.
#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub(crate) enum Direction {
    First = 0b1,
    Second = 0b10,
    Third = 0b100,
    Forth = 0b1000,
}

impl Direction {
    /// Next to this `Direction`.
    pub(crate) fn next(&self) -> Direction {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::Third,
            Self::Third => Self::Forth,
            Self::Forth => Self::First,
        }
    }

    /// Previous to this `Direction`.
    pub(crate) fn prev(&self) -> Self {
        match self {
            Self::First => Self::Forth,
            Self::Second => Self::First,
            Self::Third => Self::Second,
            Self::Forth => Self::Third,
        }
    }

    /// Determines neighbouring cell in this `Direction`. `None` if outside of dimensions.
    pub(crate) fn neighbour(&self, dimensions: Dimensions, cell: usize) -> Option<usize> {
        match self {
            Direction::First => cell.checked_sub(dimensions.0),
            Direction::Second => cell.checked_add(1).filter(|val| val % dimensions.0 != 0),
            Direction::Third => cell
                .checked_add(dimensions.0)
                .filter(|val| *val < dimensions.0 * dimensions.1),
            Direction::Forth => cell.checked_sub(1).filter(|_| cell % dimensions.0 != 0),
        }
    }

    /// Determines `Direction` from cell to neighbouring cell. `None` if cells are not neighbours.
    pub(crate) fn between(dimensions: Dimensions, from: usize, to: usize) -> Option<Direction> {
        from.checked_sub(dimensions.0)
            .filter(|val| *val == to)
            .map(|_| Self::First)
            .or_else(|| {
                from.checked_add(dimensions.0)
                    .filter(|val| *val == to)
                    .map(|_| Self::Third)
            })
            .or_else(|| {
                from.checked_add(1)
                    .filter(|val| *val == to && to % dimensions.0 != 0)
                    .map(|_| Self::Second)
            })
            .or_else(|| {
                from.checked_sub(1)
                    .filter(|val| *val == to && from % dimensions.0 != 0)
                    .map(|_| Self::Forth)
            })
    }
}

/// Array of all `Direction`s.
pub(crate) const DIRECTIONS: &[Direction] = &[
    Direction::First,
    Direction::Second,
    Direction::Third,
    Direction::Forth,
];
