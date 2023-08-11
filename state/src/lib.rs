mod r#move;
pub use r#move::{Move, SpecialMove};

mod square;
pub use square::Square;

pub struct State {
    pub board: [[Square; 8]; 8],

    pub turn: Turn,

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
        let turn = Turn::White;

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
            Turn::White => Turn::Black,
            Turn::Black => {
                self.fullmove_clock += 1;
                Turn::White
            }
        };
        self.halfmove_clock += 1;
    }
    fn undo_change_turn(&mut self) {
        if self.halfmove_clock == 0 {
            return;
        }
        self.turn = match self.turn {
            Turn::White => {
                self.fullmove_clock -= 1;
                Turn::Black
            }
            Turn::Black => Turn::White,
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

        if to_move.piece_moved == Square::WhiteKing {
            self.white_king_location = to_move.end;
        } else if to_move.piece_moved == Square::BlackKing {
            self.black_king_location = to_move.end;
        }

        self.change_turn();
    }

    fn undo_move(&mut self) {}

    fn initial_position() -> [[Square; 8]; 8] {
        use Square::*;
        [
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
pub enum Turn {
    White,
    Black,
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
