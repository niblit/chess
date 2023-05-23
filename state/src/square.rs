use crate::{piece::Piece, Player};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Square {
    Empty,
    Occupied(Player, Piece),
}

impl Square {
    pub fn get_color(&self) -> Option<Player> {
        match self {
            Square::Empty => None,
            Square::Occupied(player, _) => Some(*player),
        }
    }
}
