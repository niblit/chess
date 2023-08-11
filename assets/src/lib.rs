pub mod pieces {
    pub const WHITE_KING: &[u8] = include_bytes!("../pieces/WhiteKing.png");
    pub const WHITE_QUEEN: &[u8] = include_bytes!("../pieces/WhiteQueen.png");
    pub const WHITE_ROOK: &[u8] = include_bytes!("../pieces/WhiteRook.png");
    pub const WHITE_BISHOP: &[u8] = include_bytes!("../pieces/WhiteBishop.png");
    pub const WHITE_KNIGHT: &[u8] = include_bytes!("../pieces/WhiteKnight.png");
    pub const WHITE_PAWN: &[u8] = include_bytes!("../pieces/WhitePawn.png");

    pub const BLACK_KING: &[u8] = include_bytes!("../pieces/BlackKing.png");
    pub const BLACK_QUEEN: &[u8] = include_bytes!("../pieces/BlackQueen.png");
    pub const BLACK_ROOK: &[u8] = include_bytes!("../pieces/BlackRook.png");
    pub const BLACK_BISHOP: &[u8] = include_bytes!("../pieces/BlackBishop.png");
    pub const BLACK_KNIGHT: &[u8] = include_bytes!("../pieces/BlackKnight.png");
    pub const BLACK_PAWN: &[u8] = include_bytes!("../pieces/BlackPawn.png");
}
