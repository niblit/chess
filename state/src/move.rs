use crate::prelude::*;

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
        let compare = self.start == other.start
            && self.end == other.end
            && self.piece_moved == other.piece_moved
            && self.piece_captured == other.piece_captured;

        if let Some(SpecialMove::PawnPromotion(_)) = other.special_move {
            return compare && self.special_move == other.special_move;
        }

        compare
    }
}

impl Move {
    pub fn new(
        start: BoardCoordinates,
        end: BoardCoordinates,

        special_move: Option<SpecialMove>,
        game_state: &GameState,
    ) -> Self {
        let piece_moved = game_state.get_square(start);
        let piece_captured = game_state.get_square(end);
        Self {
            start,
            end,

            piece_moved,
            piece_captured,

            special_move,
        }
    }
}
