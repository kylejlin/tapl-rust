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

pub trait Position {
    fn position(self) -> FilePositionRange;
}
