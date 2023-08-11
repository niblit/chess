mod r#move;
pub use r#move::{Move, SpecialMove};

mod square;
pub use square::Square;

mod board;
use board::Board;
pub use board::BoardCoordinates;

#[derive(Copy, Clone)]
pub struct State {
    pub board: Board,

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

    pub last_move: Option<Move>,
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

        let last_move = None;
        let board = Board::initial_position();

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

            last_move,
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

    pub fn make_move(&mut self, to_move: Move) {
        self.last_move = Some(to_move);

        self.board.set_square(
            to_move.start.row as usize,
            to_move.start.col as usize,
            Square::Empty,
        );
        self.board.set_square(
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
