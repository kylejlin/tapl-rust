use std::ops::{Add, AddAssign};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct FilePositionRange {
    pub start: FilePosition,
    pub end: FilePosition,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct FilePosition {
    /// First line is line `1`.
    pub line: usize,

    /// First column is column `0`.
    pub column: usize,

    pub index: usize,
}

impl Add<Displacement> for FilePosition {
    type Output = Self;

    fn add(self, rhs: Displacement) -> Self {
        FilePosition {
            line: self.line + rhs.lines,
            column: if rhs.lines > 0 {
                rhs.columns
            } else {
                self.column + rhs.columns
            },
            index: self.index + rhs.len,
        }
    }
}

impl AddAssign<Displacement> for FilePosition {
    fn add_assign(&mut self, rhs: Displacement) {
        *self = *self + rhs;
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Displacement {
    pub lines: usize,
    pub columns: usize,
    pub len: usize,
}

impl Add<Self> for Displacement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Displacement {
            lines: self.lines + rhs.lines,
            columns: if rhs.lines > 0 {
                rhs.columns
            } else {
                self.columns + rhs.columns
            },
            len: self.len + rhs.len,
        }
    }
}

impl AddAssign<Displacement> for Displacement {
    fn add_assign(&mut self, rhs: Displacement) {
        *self = *self + rhs;
    }
}
