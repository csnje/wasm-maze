use crate::Dimensions;

/// Row and columns for cell index.
pub(crate) fn row_and_col(dimensions: Dimensions, idx: usize) -> (usize, usize) {
    (idx / dimensions.0, idx % dimensions.0)
}

/// [Taxicab (Manhattan) distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
/// between two cells.
pub(crate) fn taxicab_distance(dimensions: Dimensions, from: usize, to: usize) -> usize {
    let (first_row, first_col) = row_and_col(dimensions, from);
    let (second_row, second_col) = row_and_col(dimensions, to);
    (first_row.max(second_row) - first_row.min(second_row))
        + (first_col.max(second_col) - first_col.min(second_col))
}
