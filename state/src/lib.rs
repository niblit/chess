mod r#move;
pub use r#move::{Move, SpecialMove};

mod piece;
mod player;
mod square;

pub mod prelude {
    pub use crate::piece::Piece;
    pub use crate::player::Player;
    pub use crate::square::Square;
}

use prelude::*;

pub struct State {
    pub board: [[Square; 8]; 8],

    pub turn: Player,

    pub available_castles: CastlingRights,

    pub en_passant_square: Option<BoardCoordinates>,

    pub halfmove_clock: u8,
    pub fullmove_clock: usize,

    pub white_king_location: BoardCoordinates,
    pub black_king_location: BoardCoordinates,

    pub is_white_in_check: bool,
    pub is_black_in_check: bool,

    pub is_checkmate: bool,
    pub is_stalemate: bool,

    pub move_log: Vec<Move>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let turn = Player::White;

        let available_castle = CastlingRights {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        };
        let en_passant_square = None;

        let halfmove_clock = 0u8;
        let fullmove_clock = 1usize;

        let black_king_location = BoardCoordinates { row: 0, col: 4 };
        let white_king_location = BoardCoordinates { row: 7, col: 4 };

        let is_white_in_check = false;
        let is_black_in_check = false;

        let is_checkmate = false;
        let is_stalemate = false;

        let move_log = Vec::new();
        let board = Self::initial_position();

        Self {
            board,

            turn,

            available_castles: available_castle,

            en_passant_square,

            halfmove_clock,
            fullmove_clock,

            white_king_location,
            black_king_location,

            is_white_in_check,
            is_black_in_check,
            is_checkmate,
            is_stalemate,

            move_log,
        }
    }

    fn change_turn(&mut self) {
        self.turn = match self.turn {
            Player::White => Player::Black,
            Player::Black => {
                self.fullmove_clock += 1;
                Player::White
            }
        };
        self.halfmove_clock += 1;
    }
    fn undo_change_turn(&mut self) {
        if self.halfmove_clock == 0 {
            return;
        }
        self.turn = match self.turn {
            Player::White => {
                self.fullmove_clock -= 1;
                Player::Black
            }
            Player::Black => Player::White,
        };
        self.halfmove_clock -= 1;
    }

    pub fn make_move(&mut self, to_move: Move) {
        self.move_log.push(to_move);

        self.set_square(
            to_move.start.row as usize,
            to_move.start.col as usize,
            Square::Empty,
        );
        self.set_square(
            to_move.end.row as usize,
            to_move.end.col as usize,
            to_move.piece_moved,
        );

        if to_move.piece_moved == Square::Occupied(Player::White, Piece::King) {
            self.white_king_location = to_move.end;
        } else if to_move.piece_moved == Square::Occupied(Player::Black, Piece::King) {
            self.black_king_location = to_move.end;
        }

        self.change_turn();
    }

    fn undo_move(&mut self) {}

    fn initial_position() -> [[Square; 8]; 8] {
        use Piece::*;
        use Player::*;
        use Square::*;
        [
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
        ]
    }

    pub fn set_square(&mut self, row: usize, col: usize, square: Square) {
        assert!(row <= 7 && col <= 7);

        self.board[row][col] = square;
    }

    pub fn get_square(&self, row: usize, col: usize) -> Square {
        assert!(row <= 7 && col <= 7);

        self.board[row][col]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BoardCoordinates {
    pub row: u8,
    pub col: u8,
}
