pub mod pieces {
    pub mod california {
        pub const WHITE_KING: &[u8] = include_bytes!("pieces/california/WhiteKing.png");
        pub const WHITE_QUEEN: &[u8] = include_bytes!("pieces/california/WhiteQueen.png");
        pub const WHITE_ROOK: &[u8] = include_bytes!("pieces/california/WhiteRook.png");
        pub const WHITE_BISHOP: &[u8] = include_bytes!("pieces/california/WhiteBishop.png");
        pub const WHITE_KNIGHT: &[u8] = include_bytes!("pieces/california/WhiteKnight.png");
        pub const WHITE_PAWN: &[u8] = include_bytes!("pieces/california/WhitePawn.png");

        pub const BLACK_KING: &[u8] = include_bytes!("pieces/california/BlackKing.png");
        pub const BLACK_QUEEN: &[u8] = include_bytes!("pieces/california/BlackQueen.png");
        pub const BLACK_ROOK: &[u8] = include_bytes!("pieces/california/BlackRook.png");
        pub const BLACK_BISHOP: &[u8] = include_bytes!("pieces/california/BlackBishop.png");
        pub const BLACK_KNIGHT: &[u8] = include_bytes!("pieces/california/BlackKnight.png");
        pub const BLACK_PAWN: &[u8] = include_bytes!("pieces/california/BlackPawn.png");
    }
}
