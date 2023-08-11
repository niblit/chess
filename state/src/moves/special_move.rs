use crate::prelude::Square;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SpecialMove {
    EnPassant,
    Castle,
    PawnPromotion(Square),
}
