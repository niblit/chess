mod board;
mod components;
mod game_state;
mod moves;

pub mod prelude {
    pub use crate::board::{BoardCoordinates, INITIAL_POSITION};
    pub use crate::components::{CastlingRights, GameResult, Piece, Player, Square};
    pub use crate::game_state::GameState;
    pub use crate::moves::{Move, MoveCounter, SpecialMove};
}
