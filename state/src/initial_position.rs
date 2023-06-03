use crate::prelude::*;
use Piece::*;
use Player::*;
use Square::*;
pub const INITIAL_POSITION: [[Square; 8]; 8] = [
    [
        Occupied(Black, Rook),
        Occupied(Black, Knight),
        Occupied(Black, Bishop),
        Occupied(Black, Queen),
        Occupied(Black, King),
        Occupied(Black, Bishop),
        Occupied(Black, Knight),
        Occupied(Black, Rook),
    ],
    [Occupied(Black, Pawn); 8],
    [Empty; 8],
    [Empty; 8],
    [Empty; 8],
    [Empty; 8],
    [Occupied(White, Pawn); 8],
    [
        Occupied(White, Rook),
        Occupied(White, Knight),
        Occupied(White, Bishop),
        Occupied(White, Queen),
        Occupied(White, King),
        Occupied(White, Bishop),
        Occupied(White, Knight),
        Occupied(White, Rook),
    ],
];
