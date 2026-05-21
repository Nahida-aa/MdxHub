use std::{
    cmp::Ordering,
    fmt::{self, Debug},
    ops::{Add, AddAssign, Range, Sub},
};

/// A zero-indexed point in a text buffer consisting of a row and column.
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    pub row: u32,
    pub column: u32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}:{})", self.row, self.column)
    }
}

impl Point {
    pub fn new(row: u32, column: u32) -> Self {
        Point { row, column }
    }
}

impl AddAssign<Self> for Point {
    fn add_assign(&mut self, other: Self) {
        if other.row == 0 {
            self.column += other.column;
        } else {
            self.row += other.row;
            self.column = other.column;
        }
    }
}
