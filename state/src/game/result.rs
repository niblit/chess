#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameResult {
    Checkmate,
    Stalemate,
    DeadPosition,
    ThreefoldRepetition,
    FiftyMoveRule,
}

impl GameResult {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Checkmate => "Checkmate",
            Self::Stalemate => "Stalemate",
            Self::DeadPosition => "Dead Position",
            Self::ThreefoldRepetition => "Threefold Repetition",
            Self::FiftyMoveRule => "Fifty-move rule",
        }
    }
}
