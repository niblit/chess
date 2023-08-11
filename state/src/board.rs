use crate::Square::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BoardCoordinates {
    pub row: u8,
    pub col: u8,
}

#[derive(Copy, Clone)]
pub struct Board {
    board: [[crate::Square; 8]; 8],
}

impl Board {
    pub fn initial_position() -> Self {
        Self {
            board: [
                [
                    BlackRook,
                    BlackKnight,
                    BlackBishop,
                    BlackQueen,
                    BlackKing,
                    BlackBishop,
                    BlackKnight,
                    BlackRook,
                ],
                [BlackPawn; 8],
                [Empty; 8],
                [Empty; 8],
                [Empty; 8],
                [Empty; 8],
                [WhitePawn; 8],
                [
                    WhiteRook,
                    WhiteKnight,
                    WhiteBishop,
                    WhiteQueen,
                    WhiteKing,
                    WhiteBishop,
                    WhiteKnight,
                    WhiteRook,
                ],
            ],
        }
    }

    pub fn set_square(&mut self, row: usize, col: usize, square: crate::Square) {
        assert!(row <= 7 && col <= 7);

        self.board[row][col] = square;
    }

    pub fn get_square(&self, row: usize, col: usize) -> crate::Square {
        assert!(row <= 7 && col <= 7);

        self.board[row][col]
    }
}
