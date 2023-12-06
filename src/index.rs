/// A position on the board as an index.
///
/// Starts at 0 in the top right corner, moving right and then down, ending at 90.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SigIndex(i8);

impl SigIndex {
    pub(crate) fn all() -> impl ExactSizeIterator<Item = SigIndex> + Clone {
        (0..91).map(SigIndex)
    }

    pub(crate) fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<SigCoord> for SigIndex {
    fn from(SigCoord { row, col }: SigCoord) -> Self {
        Self(col + row_index_offset(row))
    }
}

/// A position on the board as row and column.
///
/// - The center of the playing field is at (0, 0).
/// - Rows go from -5 (top) to 5 (bottom).
/// - Columns are skewed, so that the top center (0, -5) is the top right corner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SigCoord {
    row: i8,
    col: i8,
}

impl SigCoord {
    fn new(row: i8, col: i8) -> Option<Self> {
        (row.abs() <= 5 && col.abs() <= 5 && (row - col).abs() <= 5).then_some(Self { row, col })
    }

    pub(crate) fn row(self) -> i8 {
        self.row
    }

    pub(crate) fn col(self) -> i8 {
        self.col
    }

    /// Iterates over all adjacent coordinates in clockwise order, starting at the right.
    ///
    /// Positions that are outside the playing field yield [`None`].
    pub(crate) fn adjacent_cw(self) -> [Option<SigCoord>; 6] {
        let Self { row, col } = self;
        [
            Self::new(row + 1, col),
            Self::new(row + 1, col + 1),
            Self::new(row, col + 1),
            Self::new(row - 1, col),
            Self::new(row - 1, col - 1),
            Self::new(row, col - 1),
        ]
    }
}

impl From<SigIndex> for SigCoord {
    fn from(SigIndex(index): SigIndex) -> Self {
        let symmetrical_index = index - 45;
        let row = (match symmetrical_index.abs() {
            40..=45 => 5,
            33..=39 => 4,
            25..=32 => 3,
            16..=24 => 2,
            6..=15 => 1,
            _ => 0,
        }) * symmetrical_index.signum();
        SigCoord {
            row,
            col: index - row_index_offset(row),
        }
    }
}

fn row_index_offset(row: i8) -> i8 {
    45 + row * 11 - row * (row.abs() + 1) / 2
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn coord_index_conversion() {
        let mut all_coords = BTreeSet::new();
        for index in 0..91 {
            let coord = SigCoord::from(SigIndex(index));
            assert!(all_coords.insert(coord), "coords not unique: {coord:?}");
            assert_eq!(SigIndex::from(coord), SigIndex(index));
        }
    }
}
