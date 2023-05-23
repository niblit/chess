use crate::prelude::*;

pub struct State {
    pub board: [[Square; 8]; 8],

    pub turn: Player,

    pub available_castles: CastlingRights,
    pub en_passant_square: Option<BoardCoordinates>,

    pub halfmove_clock: u8,
    pub fullmove_clock: usize,

    pub move_log: Vec<Move>,

    pub white_king_location: BoardCoordinates,
    pub black_king_location: BoardCoordinates,

    pub is_white_in_check: bool,
    pub is_black_in_check: bool,

    pub is_checkmate: bool,
    pub is_stalemate: bool,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let board = Self::initial_position();

        let turn = Player::White;

        let available_castles = CastlingRights::default();
        let en_passant_square = None;

        let halfmove_clock = 0u8;
        let fullmove_clock = 1usize;
        let move_log = Vec::new();

        let black_king_location = BoardCoordinates::new(0, 4);
        let white_king_location = BoardCoordinates::new(7, 4);

        let is_white_in_check = false;
        let is_black_in_check = false;

        let is_checkmate = false;
        let is_stalemate = false;

        Self {
            board,

            turn,

            available_castles,

            en_passant_square,

            halfmove_clock,
            fullmove_clock,
            move_log,

            white_king_location,
            black_king_location,

            is_white_in_check,
            is_black_in_check,

            is_checkmate,
            is_stalemate,
        }
    }

    pub fn get_square(&self, coordinates: BoardCoordinates) -> Square {
        self.board[coordinates.row() as usize][coordinates.col() as usize]
    }
    pub fn set_square(&mut self, coordinates: BoardCoordinates, square: Square) {
        self.board[coordinates.row() as usize][coordinates.col() as usize] = square;
    }

    pub fn make_move(&mut self, to_move: Move) {
        self.move_log.push(to_move);

        self.set_square(to_move.start, Square::Empty);
        self.set_square(to_move.end, to_move.piece_moved);

        if to_move.piece_moved == Square::Occupied(Player::White, Piece::King) {
            self.white_king_location = to_move.end;
        } else if to_move.piece_moved == Square::Occupied(Player::Black, Piece::King) {
            self.black_king_location = to_move.end;
        }

        self.change_turn();
    }
    pub fn undo_move(&mut self) {
        self.undo_change_turn();
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
}
