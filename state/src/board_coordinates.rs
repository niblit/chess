#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoardCoordinates {
    row: usize,
    col: usize,
}

impl BoardCoordinates {
    pub fn new(row: usize, col: usize) -> Self {
        assert!(
            row <= 7 && col <= 7,
            "row and col must point to a square inside the board"
        );
        Self { row, col }
    }
    pub fn row(&self) -> usize {
        self.row
    }
    pub fn col(&self) -> usize {
        self.col
    }
}
