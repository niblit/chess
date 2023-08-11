#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self::new(true, true, true, true)
    }
}

impl CastlingRights {
    pub fn new(
        white_king_side: bool,
        white_queen_side: bool,
        black_king_side: bool,
        black_queen_side: bool,
    ) -> Self {
        Self {
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side,
        }
    }
    pub fn get_white_king_side(&self) -> bool {
        self.white_king_side
    }
    pub fn get_white_queen_side(&self) -> bool {
        self.white_queen_side
    }
    pub fn get_black_king_side(&self) -> bool {
        self.black_king_side
    }
    pub fn get_black_queen_side(&self) -> bool {
        self.black_queen_side
    }
    pub fn ban_white_king_side(&mut self) {
        self.white_king_side = false;
    }
    pub fn ban_white_queen_side(&mut self) {
        self.white_queen_side = false;
    }
    pub fn ban_black_king_side(&mut self) {
        self.black_king_side = false;
    }
    pub fn ban_black_queen_side(&mut self) {
        self.black_queen_side = false;
    }
}
