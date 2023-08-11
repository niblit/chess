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

#[cfg(test)]
mod tests {
    use super::BoardCoordinates;

    #[test]
    #[should_panic]
    fn greater_row() {
        BoardCoordinates::new(8, 0);
    }

    #[test]
    #[should_panic]
    fn greater_col() {
        BoardCoordinates::new(0, 8);
    }

    #[test]
    #[should_panic]
    fn greater_row_and_col() {
        BoardCoordinates::new(8, 8);
    }

    #[test]
    fn valid_coordinates() {
        for r in 0..=7 {
            for c in 0..=7 {
                BoardCoordinates::new(r, c);
            }
        }
    }
}
