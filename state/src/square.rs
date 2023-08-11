#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Square {
    Empty,
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Square {
    pub fn get_color(&self) -> Option<crate::Turn> {
        use Square::*;
        match self {
            WhitePawn | WhiteKnight | WhiteBishop | WhiteRook | WhiteQueen | WhiteKing => {
                Some(crate::Turn::White)
            }
            BlackPawn | BlackKnight | BlackBishop | BlackRook | BlackQueen | BlackKing => {
                Some(crate::Turn::Black)
            }
            _ => None,
        }
    }
}
