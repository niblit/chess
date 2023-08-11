#[derive(Copy, Clone)]
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

    pub last_move: Option<Move>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        use Square::*;
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
        let board = [
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
        ];

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

        self.board[to_move.start.row as usize][to_move.start.col as usize] = Square::Empty;
        self.board[to_move.end.row as usize][to_move.end.col as usize] = to_move.piece_moved;

        if to_move.piece_moved == Square::WhiteKing {
            self.white_king_location = to_move.end;
        } else if to_move.piece_moved == Square::BlackKing {
            self.black_king_location = to_move.end;
        }

        self.change_turn();
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub start: BoardCoordinates,
    pub end: BoardCoordinates,
    pub piece_moved: Square,
    pub piece_captured: Square,
    pub special_move: Option<SpecialMove>,
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            && self.end == other.end
            && self.piece_moved == other.piece_moved
            && self.piece_captured == other.piece_captured
    }
}
impl Eq for Move {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Move {
    pub fn new(
        start: BoardCoordinates,
        end: BoardCoordinates,
        special_move: Option<SpecialMove>,
        game_state: &State,
    ) -> Self {
        let piece_moved = *game_state
            .board
            .get(start.row as usize)
            .unwrap()
            .get(start.col as usize)
            .unwrap();
        let piece_captured = *game_state
            .board
            .get(end.row as usize)
            .unwrap()
            .get(end.col as usize)
            .unwrap();
        Self {
            start,
            end,
            piece_moved,
            piece_captured,
            special_move,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SpecialMove {
    EnPassant,
    Castle,
    PawnPromotion(Square),
}

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BoardCoordinates {
    pub row: u8,
    pub col: u8,
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
