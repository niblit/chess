use crate::prelude::*;

pub struct State {
    board: [[Square; 8]; 8],

    turn: Player,

    available_castles: CastlingRights,
    en_passant_square: Option<BoardCoordinates>,

    halfmove_clock: u8,
    fullmove_clock: usize,

    move_log: Vec<Move>,
    valid_moves: Vec<Move>,

    white_king_location: BoardCoordinates,
    black_king_location: BoardCoordinates,

    is_white_in_check: bool,
    is_black_in_check: bool,

    is_checkmate: bool,
    is_stalemate: bool,
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
        let valid_moves = Vec::new();

        let black_king_location = BoardCoordinates::new(0, 4);
        let white_king_location = BoardCoordinates::new(7, 4);

        let is_white_in_check = false;
        let is_black_in_check = false;

        let is_checkmate = false;
        let is_stalemate = false;

        let mut new_state = Self {
            board,

            turn,

            available_castles,

            en_passant_square,

            halfmove_clock,
            fullmove_clock,

            move_log,
            valid_moves,

            white_king_location,
            black_king_location,

            is_white_in_check,
            is_black_in_check,

            is_checkmate,
            is_stalemate,
        };

        new_state.generate_valid_moves();

        new_state
    }

    pub fn get_square(&self, coordinates: BoardCoordinates) -> Square {
        self.board[coordinates.row() as usize][coordinates.col() as usize]
    }
    pub fn set_square(&mut self, coordinates: BoardCoordinates, square: Square) {
        self.board[coordinates.row() as usize][coordinates.col() as usize] = square;
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn get_white_king_location(&self) -> BoardCoordinates {
        self.white_king_location
    }

    pub fn get_black_king_location(&self) -> BoardCoordinates {
        self.black_king_location
    }

    pub fn get_is_white_in_check(&self) -> bool {
        self.is_white_in_check
    }
    pub fn get_is_black_in_check(&self) -> bool {
        self.is_black_in_check
    }

    pub fn get_en_passant_square(&self) -> Option<BoardCoordinates> {
        self.en_passant_square
    }

    pub fn get_last_move(&self) -> Option<&Move> {
        self.move_log.last()
    }

    pub fn get_valid_moves(&self) -> &Vec<Move> {
        &self.valid_moves
    }

    pub fn make_move(&mut self, to_move: Move) {
        self.move_log.push(to_move);

        self.set_square(to_move.start, Square::Empty);
        self.set_square(to_move.end, to_move.piece_moved);

        if let Some(special_move) = to_move.special_move {
            if let SpecialMove::PawnPromotion(square) = special_move {
                self.set_square(to_move.end, square);
            }
            if special_move == SpecialMove::EnPassant {
                let eaten_pawn_location =
                    BoardCoordinates::new(to_move.start.row() as usize, to_move.end.col() as usize);
                self.set_square(eaten_pawn_location, Square::Empty)
            }
        }

        self.en_passant_square = None;
        if to_move.piece_moved == Square::Occupied(Player::White, Piece::Pawn) {
            let jump_distance = to_move.start.row().abs_diff(to_move.end.row());
            if jump_distance == 2 {
                self.en_passant_square = Some(BoardCoordinates::new(
                    to_move.start.row() as usize - 1,
                    to_move.start.col() as usize,
                ))
            }
        } else if to_move.piece_moved == Square::Occupied(Player::Black, Piece::Pawn) {
            let jump_distance = to_move.start.row().abs_diff(to_move.end.row());
            if jump_distance == 2 {
                self.en_passant_square = Some(BoardCoordinates::new(
                    to_move.start.row() as usize + 1,
                    to_move.start.col() as usize,
                ))
            }
        }

        if to_move.piece_moved == Square::Occupied(Player::White, Piece::King) {
            self.white_king_location = to_move.end;
        } else if to_move.piece_moved == Square::Occupied(Player::Black, Piece::King) {
            self.black_king_location = to_move.end;
        }

        self.change_turn();
        self.generate_valid_moves();
    }
    pub fn undo_move(&mut self) {
        self.undo_change_turn();
        self.generate_valid_moves();
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

    fn generate_valid_moves(&mut self) {
        self.valid_moves.clear();
        self.generate_all_moves();
    }

    fn generate_all_moves(&mut self) {
        for row in 0..8usize {
            for col in 0..8usize {
                let coordinates = BoardCoordinates::new(row, col);
                if let Square::Occupied(player, piece) = self.get_square(coordinates) {
                    if player == self.turn {
                        match piece {
                            Piece::Pawn => self.generate_pawn_moves(coordinates),
                            Piece::Knight => self.generate_knight_moves(coordinates),
                            Piece::Bishop => self.generate_bishop_moves(coordinates),
                            Piece::Rook => self.generate_rook_moves(coordinates),
                            Piece::Queen => self.generate_queen_moves(coordinates),
                            Piece::King => self.generate_king_moves(coordinates),
                        };
                    }
                }
            }
        }
    }
    fn generate_pawn_moves(&mut self, coordinates: BoardCoordinates) {
        let row = coordinates.row() as usize;
        let col = coordinates.col() as usize;

        let special_move = if self.turn == Player::White && row == 1 {
            Some(SpecialMove::PawnPromotion(Square::Occupied(
                Player::White,
                Piece::Queen,
            )))
        } else if self.turn == Player::Black && row == 6 {
            Some(SpecialMove::PawnPromotion(Square::Occupied(
                Player::Black,
                Piece::Queen,
            )))
        } else {
            None
        };

        if self.turn == Player::White {
            let end = BoardCoordinates::new(row - 1, col);
            if self.get_square(end) == Square::Empty {
                self.valid_moves
                    .push(Move::new(coordinates, end, special_move, self));
            }

            if row >= 2 {
                let end = BoardCoordinates::new(row - 2, col);
                let middle = BoardCoordinates::new(row - 1, col);
                if row == 6
                    && self.get_square(end) == Square::Empty
                    && self.get_square(middle) == Square::Empty
                {
                    self.valid_moves
                        .push(Move::new(coordinates, end, None, self));
                }
            }
            if col > 0 {
                let end = BoardCoordinates::new(row - 1, col - 1);
                if let Square::Occupied(player, _) = self.get_square(end) {
                    if player == Player::Black {
                        self.valid_moves
                            .push(Move::new(coordinates, end, special_move, self));
                    }
                } else if self.en_passant_square.is_some()
                    && end == self.en_passant_square.unwrap()
                    && end.row() == 2
                {
                    self.valid_moves.push(Move::new(
                        coordinates,
                        end,
                        Some(SpecialMove::EnPassant),
                        self,
                    ));
                }
            }
            if col < 7 {
                let end = BoardCoordinates::new(row - 1, col + 1);
                if let Square::Occupied(player, _) = self.get_square(end) {
                    if player == Player::Black {
                        self.valid_moves
                            .push(Move::new(coordinates, end, special_move, self));
                    }
                } else if self.en_passant_square.is_some()
                    && end == self.en_passant_square.unwrap()
                    && end.row() == 2
                {
                    self.valid_moves.push(Move::new(
                        coordinates,
                        end,
                        Some(SpecialMove::EnPassant),
                        self,
                    ));
                }
            }
        } else {
            let end = BoardCoordinates::new(row + 1, col);
            if self.get_square(end) == Square::Empty {
                self.valid_moves
                    .push(Move::new(coordinates, end, special_move, self));
            }

            if row <= 5 {
                let end = BoardCoordinates::new(row + 2, col);
                let middle = BoardCoordinates::new(row + 1, col);
                if row == 1
                    && self.get_square(end) == Square::Empty
                    && self.get_square(middle) == Square::Empty
                {
                    self.valid_moves
                        .push(Move::new(coordinates, end, None, self));
                }
            }
            if col > 0 {
                let end = BoardCoordinates::new(row + 1, col - 1);
                if let Square::Occupied(player, _) = self.get_square(end) {
                    if player == Player::White {
                        self.valid_moves
                            .push(Move::new(coordinates, end, special_move, self));
                    }
                } else if self.en_passant_square.is_some()
                    && end == self.en_passant_square.unwrap()
                    && end.row() == 5
                {
                    self.valid_moves.push(Move::new(
                        coordinates,
                        end,
                        Some(SpecialMove::EnPassant),
                        self,
                    ));
                }
            }
            if col < 7 {
                let end = BoardCoordinates::new(row + 1, col + 1);
                if let Square::Occupied(player, _) = self.get_square(end) {
                    if player == Player::White {
                        self.valid_moves
                            .push(Move::new(coordinates, end, special_move, self));
                    }
                } else if self.en_passant_square.is_some()
                    && end == self.en_passant_square.unwrap()
                    && end.row() == 5
                {
                    self.valid_moves.push(Move::new(
                        coordinates,
                        end,
                        Some(SpecialMove::EnPassant),
                        self,
                    ));
                }
            }
        }
    }

    fn generate_knight_moves(&self, coordinates: BoardCoordinates) {}

    fn generate_bishop_moves(&self, coordinates: BoardCoordinates) {}

    fn generate_rook_moves(&self, coordinates: BoardCoordinates) {}

    fn generate_queen_moves(&self, coordinates: BoardCoordinates) {
        self.generate_bishop_moves(coordinates);
        self.generate_rook_moves(coordinates);
    }

    fn generate_king_moves(&self, coordinates: BoardCoordinates) {}

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
