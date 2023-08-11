use macroquad::prelude::*;
use state::Square::*;
use std::collections::HashMap;

pub struct Drawing {
    textures: HashMap<state::Square, Texture2D>,
    texture_params: DrawTextureParams,

    white_squares: Color,
    black_squares: Color,

    turn_color: Color,
    check_color: Color,
    move_color: Color,

    pub square_size: f32,
    x_padding: f32,
    y_padding: f32,
}

impl Default for Drawing {
    fn default() -> Self {
        let white_squares = Color::from_rgba(240, 217, 181, 255);
        let black_squares = Color::from_rgba(181, 136, 99, 255);

        let turn_color = Color::from_rgba(50, 255, 50, 128);
        let check_color = Color::from_rgba(255, 50, 50, 128);
        let move_color = Color::from_rgba(150, 200, 128, 128);

        let textures = Self::load_textures();

        Self::new(
            textures,
            white_squares,
            black_squares,
            turn_color,
            check_color,
            move_color,
        )
    }
}

impl Drawing {
    pub fn new(
        textures: HashMap<state::Square, Texture2D>,
        white_squares: Color,
        black_squares: Color,
        turn_color: Color,
        check_color: Color,
        move_color: Color,
    ) -> Self {
        let square_size = Self::get_square_size();
        let x_padding = Self::get_x_padding();
        let y_padding = Self::get_y_padding();

        let texture_params = DrawTextureParams {
            dest_size: Some(Vec2::new(square_size, square_size)),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        Self {
            textures,
            texture_params,
            white_squares,
            black_squares,
            turn_color,
            check_color,
            move_color,
            square_size,
            x_padding,
            y_padding,
        }
    }
    pub fn update_frame(&mut self, game_state: &state::State) {
        self.square_size = Self::get_square_size();
        self.x_padding = Self::get_x_padding();
        self.y_padding = Self::get_y_padding();

        self.texture_params = DrawTextureParams {
            dest_size: Some(Vec2::new(self.square_size, self.square_size)),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        self.draw_frame(game_state);
    }
    fn draw_frame(&self, game_state: &state::State) {
        self.draw_board();
        self.draw_highlights(game_state);
        self.draw_pieces(game_state);
        self.draw_turn(game_state);
    }
    fn draw_board(&self) {
        for row in 0u8..8u8 {
            let y = f32::from(row);
            for col in 0u8..8u8 {
                let x = f32::from(col);
                draw_rectangle(
                    x * self.square_size + self.x_padding,
                    y * self.square_size + self.y_padding,
                    self.square_size,
                    self.square_size,
                    if (row + col) % 2 == 0 {
                        self.white_squares
                    } else {
                        self.black_squares
                    },
                );
            }
        }
    }

    fn draw_highlights(&self, game_state: &state::State) {
        // Draw checks
        if game_state.is_white_in_check {
            draw_rectangle(
                game_state.white_king_location.col as f32 * self.square_size + self.x_padding,
                game_state.white_king_location.row as f32 * self.square_size + self.y_padding,
                self.square_size,
                self.square_size,
                self.check_color,
            )
        }
        if game_state.is_black_in_check {
            draw_rectangle(
                game_state.black_king_location.col as f32 * self.square_size + self.x_padding,
                game_state.black_king_location.row as f32 * self.square_size + self.y_padding,
                self.square_size,
                self.square_size,
                self.check_color,
            )
        }

        // Draw last move
        if let Some(last_move) = game_state.last_move {
            draw_rectangle(
                last_move.start.col as f32 * self.square_size + self.x_padding,
                last_move.start.row as f32 * self.square_size + self.y_padding,
                self.square_size,
                self.square_size,
                self.move_color,
            );
            draw_rectangle(
                last_move.end.col as f32 * self.square_size + self.x_padding,
                last_move.end.row as f32 * self.square_size + self.y_padding,
                self.square_size,
                self.square_size,
                self.move_color,
            );
        }
    }

    fn draw_pieces(&self, game_state: &state::State) {
        draw_texture_ex(
            *self.textures.get(&WhiteQueen).unwrap(),
            0.0,
            0.0,
            Color::from_rgba(255, 255, 255, 0),
            self.texture_params.clone(),
        );

        for row in 0u8..8 {
            let y = f32::from(row) * self.square_size + self.y_padding;
            for col in 0u8..8 {
                let x = f32::from(col) * self.square_size + self.x_padding;

                if let Some(texture) = self
                    .textures
                    .get(&game_state.board[row as usize][col as usize])
                {
                    draw_texture_ex(
                        *texture,
                        x,
                        y,
                        Color::from_rgba(255, 255, 255, 255),
                        self.texture_params.clone(),
                    );
                }
            }
        }
    }

    fn draw_turn(&self, game_state: &state::State) {
        let radius = self.square_size / 10.0;

        let (x, y) = match game_state.turn {
            state::Turn::White => (
                self.x_padding + self.square_size * 8.0 + radius * 1.5,
                self.y_padding + self.square_size * 8.0 + radius * 1.5,
            ),
            state::Turn::Black => (
                self.x_padding + self.square_size * 8.0 + radius * 1.5,
                self.y_padding + self.square_size * 0.0 - radius * 1.5,
            ),
        };

        draw_circle(x, y, radius, self.turn_color);
    }

    pub fn get_board_start(&self) -> (f32, f32) {
        (self.x_padding, self.y_padding)
    }

    pub fn get_board_end(&self) -> (f32, f32) {
        (
            self.x_padding + self.square_size * 8.0,
            self.y_padding + self.square_size * 8.0,
        )
    }

    fn get_square_size() -> f32 {
        screen_width().min(screen_height()) / 9.0
    }

    fn get_x_padding() -> f32 {
        (screen_width() - (Self::get_square_size() * 8.0)) / 2.0
    }

    fn get_y_padding() -> f32 {
        (screen_height() - (Self::get_square_size() * 8.0)) / 2.0
    }

    fn load_textures() -> HashMap<state::Square, Texture2D> {
        let mut textures: HashMap<state::Square, Texture2D> = HashMap::new();
        textures.insert(
            BlackPawn,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/BlackPawn.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            BlackKnight,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/BlackKnight.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            BlackBishop,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/BlackBishop.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            BlackRook,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/BlackRook.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            BlackQueen,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/BlackQueen.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            BlackKing,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/BlackKing.png"),
                Some(ImageFormat::Png),
            ),
        );

        textures.insert(
            WhitePawn,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/WhitePawn.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            WhiteKnight,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/WhiteKnight.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            WhiteBishop,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/WhiteBishop.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            WhiteRook,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/WhiteRook.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            WhiteQueen,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/WhiteQueen.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            WhiteKing,
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/pieces/WhiteKing.png"),
                Some(ImageFormat::Png),
            ),
        );

        textures
    }
}
