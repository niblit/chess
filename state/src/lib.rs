mod board_coordinates;
mod castling_rights;
mod game_state;
mod r#move;
mod piece;
mod player;
mod square;

pub mod prelude {
    pub use crate::board_coordinates::BoardCoordinates;
    pub use crate::castling_rights::CastlingRights;
    pub use crate::game_state::State;
    pub use crate::piece::Piece;
    pub use crate::player::Player;
    pub use crate::r#move::{Move, SpecialMove};
    pub use crate::square::Square;
}
