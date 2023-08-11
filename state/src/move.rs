use crate::*;

#[derive(Copy, Clone)]
pub struct Move {
    pub start: BoardCoordinates,
    pub end: BoardCoordinates,
    pub piece_moved: Square,
    pub piece_captured: Square,
    pub special_move: Option<SpecialMove>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SpecialMove {
    EnPassant,
    Castle,
    PawnPromotion(Square),
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
        let piece_moved = game_state
            .board
            .get_square(start.row as usize, start.col as usize);
        let piece_captured = game_state
            .board
            .get_square(end.row as usize, end.col as usize);
        Self {
            start,
            end,
            piece_moved,
            piece_captured,
            special_move,
        }
    }
}
