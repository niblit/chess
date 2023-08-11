use std::collections::HashMap;

use crate::game_result::GameResult;
use crate::prelude::*;

pub struct GameState {
    board: [[Square; 8]; 8],

    turn: Player,

    en_passant_square: Option<BoardCoordinates>,

    halfmove_clock: usize,
    fullmove_clock: usize,

    move_log: Vec<Move>,
    valid_moves: Vec<Move>,

    white_king_location: BoardCoordinates,
    black_king_location: BoardCoordinates,

    castling_rights_log: Vec<CastlingRights>,

    is_check: bool,

    game_result: Option<GameResult>,
    position_repetitions: HashMap<[[Square; 8]; 8], usize>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        let board = Self::initial_position();

        let turn = Player::White;

        let en_passant_square = None;

        let halfmove_clock = 0usize;
        let fullmove_clock = 1usize;
        let move_log = Vec::new();
        let valid_moves = Vec::new();

        let black_king_location = BoardCoordinates::new(0, 4);
        let white_king_location = BoardCoordinates::new(7, 4);
        let castling_rights_log = vec![CastlingRights::default()];

        let is_check = false;

        let game_result = None;

        let position_repetitions = HashMap::new();

        let mut new_state = Self {
            board,

            turn,

            en_passant_square,

            halfmove_clock,
            fullmove_clock,

            move_log,
            valid_moves,

            white_king_location,
            black_king_location,
            castling_rights_log,

            is_check,

            game_result,
            position_repetitions,
        };

        new_state.generate_valid_moves();

        new_state
    }

    pub fn restart(&mut self) {
        self.board = Self::initial_position();
        self.turn = Player::White;
        self.en_passant_square = None;
        self.halfmove_clock = 0usize;
        self.fullmove_clock = 1usize;
        self.move_log.clear();
        self.valid_moves.clear();
        self.black_king_location = BoardCoordinates::new(0, 4);
        self.white_king_location = BoardCoordinates::new(7, 4);
        self.castling_rights_log = vec![CastlingRights::default()];
        self.is_check = false;
        self.game_result = None;
        self.generate_valid_moves();
    }

    pub fn get_square(&self, coordinates: BoardCoordinates) -> Square {
        self.board[coordinates.row()][coordinates.col()]
    }

    pub fn set_square(&mut self, coordinates: BoardCoordinates, square: Square) {
        self.board[coordinates.row()][coordinates.col()] = square;
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

    pub fn get_is_check(&self) -> bool {
        self.is_check
    }

    pub fn is_game_over(&self) -> bool {
        self.game_result.is_some()
    }

    pub fn get_game_result(&self) -> Option<GameResult> {
        self.game_result
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

    pub fn make_new_move(&mut self, new_move: Move) {
        self.make_move(new_move);
        self.generate_valid_moves();

        if self.position_repetitions.get(&self.board).is_none() {
            self.position_repetitions.insert(self.board, 0);
        }
        if let Some(v) = self.position_repetitions.get_mut(&self.board) {
            *v += 1;
        }
        if self.position_repetitions.values().any(|v| *v >= 3) && self.game_result.is_none() {
            self.game_result = Some(GameResult::ThreefoldRepetition);
        }
        if self.board.iter().flatten().all(|sq| {
            sq == &Square::Empty
                || sq == &Square::Occupied(Player::Black, Piece::King)
                || sq == &Square::Occupied(Player::White, Piece::King)
        }) {
            self.game_result = Some(GameResult::DeadPosition);
        }
        if let Some(SpecialMove::PawnPromotion(_)) = new_move.special_move {
            self.halfmove_clock = 0;
        } else if new_move.piece_moved == Square::Occupied(Player::White, Piece::Pawn)
            || new_move.piece_moved == Square::Occupied(Player::Black, Piece::Pawn)
            || new_move.piece_captured != Square::Empty
            || new_move.special_move == Some(SpecialMove::EnPassant)
        {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
        if self.halfmove_clock >= 50 && self.game_result.is_none() {
            self.game_result = Some(GameResult::FiftyMoveRule);
        }
    }

    fn make_move(&mut self, to_move: Move) {
        self.move_log.push(to_move);

        self.set_square(to_move.start, Square::Empty);
        self.set_square(to_move.end, to_move.piece_moved);

        if let Some(special_move) = to_move.special_move {
            if let SpecialMove::PawnPromotion(square) = special_move {
                self.set_square(to_move.end, square);
            } else if special_move == SpecialMove::EnPassant {
                let captured_pawn = BoardCoordinates::new(to_move.start.row(), to_move.end.col());
                self.set_square(captured_pawn, Square::Empty)
            } else if special_move == SpecialMove::Castle {
                let (rook_start, rook_end) =
                    if (to_move.start.col() as i8 - to_move.end.col() as i8) < 0 {
                        (
                            BoardCoordinates::new(to_move.end.row(), to_move.end.col() + 1),
                            BoardCoordinates::new(to_move.end.row(), to_move.end.col() - 1),
                        )
                    } else {
                        (
                            BoardCoordinates::new(to_move.end.row(), to_move.end.col() - 2),
                            BoardCoordinates::new(to_move.end.row(), to_move.end.col() + 1),
                        )
                    };

                self.set_square(rook_end, self.get_square(rook_start));
                self.set_square(rook_start, Square::Empty);
            }
        }

        self.en_passant_square = None;
        if let Square::Occupied(player, piece) = to_move.piece_moved {
            if piece == Piece::Pawn {
                let jump_distance = to_move.start.row().abs_diff(to_move.end.row());
                if jump_distance == 2 {
                    self.en_passant_square = Some(BoardCoordinates::new(
                        if player == Player::White {
                            to_move.start.row() - 1
                        } else {
                            to_move.start.row() + 1
                        },
                        to_move.start.col(),
                    ));
                }
            }
        }

        self.update_castle_rights(to_move);

        self.change_turn();
        self.is_check = self.in_check();
    }

    fn update_castle_rights(&mut self, to_move: Move) {
        let mut new_castling_rights = *self.castling_rights_log.last().unwrap();

        if to_move.piece_moved == Square::Occupied(Player::White, Piece::King) {
            self.white_king_location = to_move.end;
            new_castling_rights.ban_white_king_side();
            new_castling_rights.ban_white_queen_side();
        } else if to_move.piece_moved == Square::Occupied(Player::Black, Piece::King) {
            self.black_king_location = to_move.end;
            new_castling_rights.ban_black_king_side();
            new_castling_rights.ban_black_queen_side();
        }

        let mut check_then_ban = |x, y| {
            match (x, y) {
                (0, 0) => new_castling_rights.ban_black_queen_side(),
                (0, 7) => new_castling_rights.ban_black_king_side(),
                (7, 0) => new_castling_rights.ban_white_queen_side(),
                (7, 7) => new_castling_rights.ban_white_king_side(),
                _ => {}
            };
        };

        if let Square::Occupied(_, piece_moved) = to_move.piece_moved {
            if piece_moved == Piece::Rook {
                check_then_ban(to_move.start.row(), to_move.start.col());
            }
        }
        if let Square::Occupied(_, piece_captured) = to_move.piece_captured {
            if piece_captured == Piece::Rook {
                check_then_ban(to_move.end.row(), to_move.end.col());
            }
        }
        self.castling_rights_log.push(new_castling_rights);
    }

    pub fn undo_last_move(&mut self) {
        self.undo_move();
        self.generate_valid_moves();
        if let Some(v) = self.position_repetitions.get_mut(&self.board) {
            *v -= 1;
        }
    }
    fn undo_move(&mut self) {
        if let Some(last_move) = self.move_log.pop() {
            self.set_square(last_move.start, last_move.piece_moved);
            self.set_square(last_move.end, last_move.piece_captured);

            if let Some(special_move) = last_move.special_move {
                if let SpecialMove::PawnPromotion(_) = special_move {
                    // There is nothing to do
                } else if special_move == SpecialMove::EnPassant {
                    let captured_pawn =
                        BoardCoordinates::new(last_move.start.row(), last_move.end.col());
                    self.set_square(captured_pawn, Square::Occupied(self.turn, Piece::Pawn))
                } else if special_move == SpecialMove::Castle {
                    let (rook_start, rook_end) =
                        if (last_move.start.col() as i8 - last_move.end.col() as i8) < 0 {
                            (
                                BoardCoordinates::new(last_move.end.row(), last_move.end.col() - 1),
                                BoardCoordinates::new(last_move.end.row(), last_move.end.col() + 1),
                            )
                        } else {
                            (
                                BoardCoordinates::new(last_move.end.row(), last_move.end.col() + 1),
                                BoardCoordinates::new(last_move.end.row(), last_move.end.col() - 2),
                            )
                        };

                    self.set_square(rook_end, self.get_square(rook_start));
                    self.set_square(rook_start, Square::Empty);
                }
            }

            if let Some(possible_pawn_move) = self.get_last_move() {
                if let Square::Occupied(player, piece) = possible_pawn_move.piece_moved {
                    if piece == Piece::Pawn {
                        let jump_distance = possible_pawn_move
                            .start
                            .row()
                            .abs_diff(possible_pawn_move.end.row());
                        if jump_distance == 2 {
                            self.en_passant_square = Some(BoardCoordinates::new(
                                match player {
                                    Player::White => possible_pawn_move.start.row() - 1,
                                    Player::Black => possible_pawn_move.start.row() + 1,
                                },
                                possible_pawn_move.start.col(),
                            ));
                        }
                    }
                }
            }

            if last_move.piece_moved == Square::Occupied(Player::White, Piece::King) {
                self.white_king_location = last_move.start;
            } else if last_move.piece_moved == Square::Occupied(Player::Black, Piece::King) {
                self.black_king_location = last_move.start;
            }

            self.castling_rights_log.pop();
            self.undo_change_turn();
            self.is_check = self.in_check();
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
    }

    fn in_check(&mut self) -> bool {
        self.under_attack(match self.turn {
            Player::White => self.white_king_location,
            Player::Black => self.black_king_location,
        })
    }

    fn under_attack(&mut self, coordinates: BoardCoordinates) -> bool {
        self.change_turn();
        let enemy_moves = self.generate_all_moves();
        self.undo_change_turn();

        for enemy_move in enemy_moves {
            if enemy_move.end == coordinates {
                return true;
            }
        }
        false
    }

    fn generate_valid_moves(&mut self) {
        let mut all_moves = self.generate_all_moves();
        self.generate_castling_moves(&mut all_moves);
        for move_index in (0..all_moves.len()).rev() {
            let current_move = all_moves[move_index];

            self.make_move(current_move);
            let enemy_moves = self.generate_all_moves();
            'enemy_move_search: for enemy_move in enemy_moves {
                if let Square::Occupied(_, piece) = enemy_move.piece_captured {
                    if piece == Piece::King {
                        all_moves.remove(move_index);
                        break 'enemy_move_search;
                    }
                }
            }
            self.undo_move();
        }
        self.valid_moves = all_moves;
        if self.valid_moves.is_empty() {
            if self.is_check {
                self.game_result = Some(GameResult::Checkmate);
            } else {
                self.game_result = Some(GameResult::Stalemate);
            }
        } else {
            self.game_result = None;
        }
    }

    fn generate_all_moves(&mut self) -> Vec<Move> {
        let mut all_moves = Vec::new();

        for row in 0..8usize {
            for col in 0..8usize {
                let coordinates = BoardCoordinates::new(row, col);
                if let Square::Occupied(player, piece) = self.get_square(coordinates) {
                    if player == self.turn {
                        match piece {
                            Piece::Pawn => self.generate_pawn_moves(coordinates, &mut all_moves),
                            Piece::Knight => {
                                self.generate_knight_moves(coordinates, &mut all_moves)
                            }
                            Piece::Bishop => {
                                self.generate_bishop_moves(coordinates, &mut all_moves)
                            }
                            Piece::Rook => self.generate_rook_moves(coordinates, &mut all_moves),
                            Piece::Queen => self.generate_queen_moves(coordinates, &mut all_moves),
                            Piece::King => self.generate_king_moves(coordinates, &mut all_moves),
                        };
                    }
                }
            }
        }
        all_moves
    }

    fn generate_pawn_moves(&mut self, coordinates: BoardCoordinates, moves: &mut Vec<Move>) {
        let row = coordinates.row();
        let col = coordinates.col();

        let special_moves = if self.turn == Player::White && row == 1 {
            vec![
                Some(SpecialMove::PawnPromotion(Square::Occupied(
                    Player::White,
                    Piece::Queen,
                ))),
                Some(SpecialMove::PawnPromotion(Square::Occupied(
                    Player::White,
                    Piece::Rook,
                ))),
                Some(SpecialMove::PawnPromotion(Square::Occupied(
                    Player::White,
                    Piece::Bishop,
                ))),
                Some(SpecialMove::PawnPromotion(Square::Occupied(
                    Player::White,
                    Piece::Knight,
                ))),
            ]
        } else if self.turn == Player::Black && row == 6 {
            vec![
                Some(SpecialMove::PawnPromotion(Square::Occupied(
                    Player::Black,
                    Piece::Queen,
                ))),
                Some(SpecialMove::PawnPromotion(Square::Occupied(
                    Player::Black,
                    Piece::Rook,
                ))),
                Some(SpecialMove::PawnPromotion(Square::Occupied(
                    Player::Black,
                    Piece::Bishop,
                ))),
                Some(SpecialMove::PawnPromotion(Square::Occupied(
                    Player::Black,
                    Piece::Knight,
                ))),
            ]
        } else {
            vec![None]
        };

        if self.turn == Player::White {
            let end = BoardCoordinates::new(row - 1, col);
            if self.get_square(end) == Square::Empty {
                for special_move in special_moves.clone() {
                    moves.push(Move::new(coordinates, end, special_move, self));
                }
            }

            if row >= 2 {
                let end = BoardCoordinates::new(row - 2, col);
                let middle = BoardCoordinates::new(row - 1, col);
                if row == 6
                    && self.get_square(end) == Square::Empty
                    && self.get_square(middle) == Square::Empty
                {
                    moves.push(Move::new(coordinates, end, None, self));
                }
            }
            if col > 0 {
                let end = BoardCoordinates::new(row - 1, col - 1);
                if let Square::Occupied(player, _) = self.get_square(end) {
                    if player == Player::Black {
                        for special_move in special_moves.clone() {
                            moves.push(Move::new(coordinates, end, special_move, self));
                        }
                    }
                } else if self.en_passant_square.is_some()
                    && end == self.en_passant_square.unwrap()
                    && end.row() == 2
                {
                    moves.push(Move::new(
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
                        for special_move in special_moves {
                            moves.push(Move::new(coordinates, end, special_move, self));
                        }
                    }
                } else if self.en_passant_square.is_some()
                    && end == self.en_passant_square.unwrap()
                    && end.row() == 2
                {
                    moves.push(Move::new(
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
                for special_move in special_moves.clone() {
                    moves.push(Move::new(coordinates, end, special_move, self));
                }
            }

            if row <= 5 {
                let end = BoardCoordinates::new(row + 2, col);
                let middle = BoardCoordinates::new(row + 1, col);
                if row == 1
                    && self.get_square(end) == Square::Empty
                    && self.get_square(middle) == Square::Empty
                {
                    moves.push(Move::new(coordinates, end, None, self));
                }
            }
            if col > 0 {
                let end = BoardCoordinates::new(row + 1, col - 1);
                if let Square::Occupied(player, _) = self.get_square(end) {
                    if player == Player::White {
                        for special_move in special_moves.clone() {
                            moves.push(Move::new(coordinates, end, special_move, self));
                        }
                    }
                } else if self.en_passant_square.is_some()
                    && end == self.en_passant_square.unwrap()
                    && end.row() == 5
                {
                    moves.push(Move::new(
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
                        for special_move in special_moves {
                            moves.push(Move::new(coordinates, end, special_move, self));
                        }
                    }
                } else if self.en_passant_square.is_some()
                    && end == self.en_passant_square.unwrap()
                    && end.row() == 5
                {
                    moves.push(Move::new(
                        coordinates,
                        end,
                        Some(SpecialMove::EnPassant),
                        self,
                    ));
                }
            }
        }
    }

    fn generate_knight_moves(&mut self, coordinates: BoardCoordinates, moves: &mut Vec<Move>) {
        let directions: [[isize; 2]; 8] = [
            [1, 2],
            [-1, 2],
            [1, -2],
            [-1, -2],
            [2, 1],
            [-2, 1],
            [2, -1],
            [-2, -1],
        ];

        self.generate_sliding_piece_moves(coordinates, 1, &directions, moves);
    }

    fn generate_bishop_moves(&mut self, coordinates: BoardCoordinates, moves: &mut Vec<Move>) {
        let directions: [[isize; 2]; 4] = [[1, 1], [-1, 1], [1, -1], [-1, -1]];
        self.generate_sliding_piece_moves(coordinates, 8, &directions, moves);
    }

    fn generate_rook_moves(&mut self, coordinates: BoardCoordinates, moves: &mut Vec<Move>) {
        let directions: [[isize; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
        self.generate_sliding_piece_moves(coordinates, 8, &directions, moves);
    }

    fn generate_queen_moves(&mut self, coordinates: BoardCoordinates, moves: &mut Vec<Move>) {
        self.generate_bishop_moves(coordinates, moves);
        self.generate_rook_moves(coordinates, moves);
    }

    fn generate_king_moves(&mut self, coordinates: BoardCoordinates, moves: &mut Vec<Move>) {
        let directions: [[isize; 2]; 8] = [
            [0, 1],
            [0, -1],
            [1, 0],
            [-1, 0],
            [1, 1],
            [1, -1],
            [-1, 1],
            [-1, -1],
        ];

        self.generate_sliding_piece_moves(coordinates, 1, &directions, moves);
    }

    fn generate_castling_moves(&mut self, moves: &mut Vec<Move>) {
        if !self.in_check() {
            let current_castling_rights = *self.castling_rights_log.last().unwrap();
            match self.turn {
                Player::White => {
                    if current_castling_rights.get_white_king_side() {
                        let square1 = BoardCoordinates::new(7, 5);
                        let square2 = BoardCoordinates::new(7, 6);
                        if self.get_square(square1) == Square::Empty
                            && self.get_square(square2) == Square::Empty
                            && !self.under_attack(square1)
                            && !self.under_attack(square2)
                        {
                            moves.push(Move::new(
                                self.white_king_location,
                                square2,
                                Some(SpecialMove::Castle),
                                self,
                            ))
                        }
                    }
                    if current_castling_rights.get_white_queen_side() {
                        let square1 = BoardCoordinates::new(7, 3);
                        let square2 = BoardCoordinates::new(7, 2);
                        let square3 = BoardCoordinates::new(7, 1);
                        if self.get_square(square1) == Square::Empty
                            && self.get_square(square2) == Square::Empty
                            && self.get_square(square3) == Square::Empty
                            && !self.under_attack(square1)
                            && !self.under_attack(square2)
                        {
                            moves.push(Move::new(
                                self.white_king_location,
                                square2,
                                Some(SpecialMove::Castle),
                                self,
                            ))
                        }
                    }
                }
                Player::Black => {
                    if current_castling_rights.get_black_king_side() {
                        let square1 = BoardCoordinates::new(0, 5);
                        let square2 = BoardCoordinates::new(0, 6);
                        if self.get_square(square1) == Square::Empty
                            && self.get_square(square2) == Square::Empty
                            && !self.under_attack(square1)
                            && !self.under_attack(square2)
                        {
                            moves.push(Move::new(
                                self.black_king_location,
                                square2,
                                Some(SpecialMove::Castle),
                                self,
                            ))
                        }
                    }
                    if current_castling_rights.get_black_queen_side() {
                        let square1 = BoardCoordinates::new(0, 3);
                        let square2 = BoardCoordinates::new(0, 2);
                        let square3 = BoardCoordinates::new(0, 1);
                        if self.get_square(square1) == Square::Empty
                            && self.get_square(square2) == Square::Empty
                            && self.get_square(square3) == Square::Empty
                            && !self.under_attack(square1)
                            && !self.under_attack(square2)
                        {
                            moves.push(Move::new(
                                self.black_king_location,
                                square2,
                                Some(SpecialMove::Castle),
                                self,
                            ))
                        }
                    }
                }
            }
        }
    }

    fn generate_sliding_piece_moves(
        &mut self,
        coordinates: BoardCoordinates,
        distance: isize,
        directions: &[[isize; 2]],
        moves: &mut Vec<Move>,
    ) {
        let row = coordinates.row() as isize;
        let col = coordinates.col() as isize;

        for direction in directions {
            for dis in 1..=distance {
                let end_row = row + direction[0] * dis;
                let end_col = col + direction[1] * dis;

                if (0..=7).contains(&end_row) && (0..=7).contains(&end_col) {
                    let end = BoardCoordinates::new(end_row as usize, end_col as usize);
                    let end_piece = self.get_square(end);
                    let potential_move = Move::new(coordinates, end, None, self);

                    if let Square::Occupied(player, _) = end_piece {
                        if player != self.turn {
                            moves.push(potential_move);
                        }
                        break;
                    } else {
                        moves.push(potential_move);
                    }
                } else {
                    break;
                }
            }
        }
    }

    const fn initial_position() -> [[Square; 8]; 8] {
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
