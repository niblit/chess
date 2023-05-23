#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BoardCoordinates {
    row: usize,
    col: usize,
}

impl BoardCoordinates {
    pub fn new(row: usize, col: usize) -> Self {
        assert!(row <= 7 && col <= 7);
        Self { row, col }
    }
    pub fn row(&self) -> u8 {
        self.row as u8
    }
    pub fn col(&self) -> u8 {
        self.col as u8
    }
}
