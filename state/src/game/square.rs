use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Square {
    Empty,
    Occupied(Player, Piece),
}
