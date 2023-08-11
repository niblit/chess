use std::collections::HashMap;

use macroquad::prelude::*;

use state::prelude::*;

pub struct GameScene {
    textures: HashMap<Square, Texture2D>,
    texture_params: DrawTextureParams,

    white_squares: Color,
    black_squares: Color,

    check_color: Color,
    move_color: Color,
    selected_color: Color,

    square_size: f32,
    x_padding: f32,
    y_padding: f32,

    first_square_selected: Option<BoardCoordinates>,
    second_square_selected: Option<BoardCoordinates>,
}

impl Default for GameScene {
    fn default() -> Self {
        let textures = Self::load_california_pieces();

        let white_squares = Color::from_rgba(240, 217, 181, 255);
        let black_squares = Color::from_rgba(181, 136, 99, 255);

        let check_color = Color::from_rgba(255, 50, 50, 128);
        let move_color = Color::from_rgba(150, 200, 128, 128);
        let selected_color = Color::from_rgba(160, 180, 160, 180);
        Self::new(
            textures,
            white_squares,
            black_squares,
            check_color,
            move_color,
            selected_color,
        )
    }
}

impl GameScene {
    pub fn new(
        textures: HashMap<Square, Texture2D>,
        white_squares: Color,
        black_squares: Color,
        check_color: Color,
        move_color: Color,
        selected_color: Color,
    ) -> Self {
        let texture_params = DrawTextureParams::default();

        let square_size = 0f32;
        let x_padding = 0f32;
        let y_padding = 0f32;

        let first_square_selected: Option<BoardCoordinates> = None;
        let second_square_selected: Option<BoardCoordinates> = None;

        Self {
            textures,
            texture_params,

            square_size,
            x_padding,
            y_padding,

            white_squares,
            black_squares,

            check_color,
            move_color,
            selected_color,

            first_square_selected,
            second_square_selected,
        }
    }

    fn get_square_size(&self) -> f32 {
        self.square_size
    }

    fn get_board_start(&self) -> (f32, f32) {
        (self.x_padding, self.y_padding)
    }

    fn get_board_end(&self) -> (f32, f32) {
        (
            self.x_padding + self.square_size * 8.0,
            self.y_padding + self.square_size * 8.0,
        )
    }

    pub fn update_frame(&mut self, game_state: &mut State) -> crate::Scene {
        // Undo last move
        if is_key_pressed(KeyCode::Z) {
            game_state.undo_move();
            game_state.generate_valid_moves();
        }

        // Move logic
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_location = mouse_position();

            let board_start = self.get_board_start();
            let board_end = self.get_board_end();

            // Make sure the mouse click is inside the board
            if mouse_location.0 > board_start.0
                && mouse_location.1 > board_start.1
                && mouse_location.0 < board_end.0
                && mouse_location.1 < board_end.1
            {
                let square_clicked = BoardCoordinates::new(
                    ((mouse_location.1 - board_start.1) / self.get_square_size()) as usize,
                    ((mouse_location.0 - board_start.0) / self.get_square_size()) as usize,
                );

                if self.first_square_selected.is_none() {
                    if let Square::Occupied(p, _) = game_state.get_square(square_clicked) {
                        if p == game_state.get_turn() {
                            self.first_square_selected = Some(square_clicked);
                        }
                    }
                } else if square_clicked == self.first_square_selected.unwrap() {
                    self.first_square_selected = None;
                } else if let Square::Occupied(p, _) = game_state.get_square(square_clicked) {
                    if p != game_state.get_turn() {
                        self.second_square_selected = Some(square_clicked);
                    } else {
                        self.first_square_selected = Some(square_clicked);
                    }
                } else {
                    self.second_square_selected = Some(square_clicked);
                }

                if self.first_square_selected.is_some() && self.second_square_selected.is_some() {
                    let mut potential_move = Move::new(
                        self.first_square_selected.unwrap(),
                        self.second_square_selected.unwrap(),
                        None,
                        game_state,
                    );

                    let mut is_move_valid = false;
                    for real_move in game_state.get_valid_moves() {
                        if real_move == &potential_move {
                            // Use the generated move in game state instead of the one generated
                            // with mouse input, to preserve special move's properties
                            potential_move = *real_move;
                            is_move_valid = true;
                        }
                    }
                    if is_move_valid {
                        game_state.make_move(potential_move);
                        game_state.generate_valid_moves();
                    }

                    self.first_square_selected = None;
                    self.second_square_selected = None;
                }
            }
        }

        self.update_square_size();
        self.update_padding();
        self.update_texture_params();

        self.draw_frame(game_state);

        if game_state.get_is_checkmate() || game_state.get_is_stalemate() {
            return crate::Scene::End;
        }
        crate::Scene::Game
    }

    fn draw_frame(&self, game_state: &State) {
        self.draw_board();
        self.draw_highlights(game_state);
        self.draw_pieces(game_state);
    }

    fn draw_board(&self) {
        for row in 0u8..8u8 {
            let y = f32::from(row);
            for col in 0u8..8u8 {
                let x = f32::from(col);
                let color = if (row + col) % 2 == 0 {
                    self.white_squares
                } else {
                    self.black_squares
                };
                draw_rectangle(
                    x * self.square_size + self.x_padding,
                    y * self.square_size + self.y_padding,
                    self.square_size,
                    self.square_size,
                    color,
                );
            }
        }
    }

    fn draw_highlights(&self, game_state: &State) {
        self.draw_last_move(game_state);
        self.draw_checks(game_state);
        self.draw_selected_square();
        self.draw_valid_moves(game_state);
    }

    fn draw_last_move(&self, game_state: &State) {
        if let Some(last_move) = game_state.get_last_move() {
            draw_rectangle(
                last_move.start.col() as f32 * self.square_size + self.x_padding,
                last_move.start.row() as f32 * self.square_size + self.y_padding,
                self.square_size,
                self.square_size,
                self.move_color,
            );
            draw_rectangle(
                last_move.end.col() as f32 * self.square_size + self.x_padding,
                last_move.end.row() as f32 * self.square_size + self.y_padding,
                self.square_size,
                self.square_size,
                self.move_color,
            );
        }
    }

    fn draw_checks(&self, game_state: &State) {
        if game_state.get_is_check() {
            let king_location = match game_state.get_turn() {
                Player::White => game_state.get_white_king_location(),
                Player::Black => game_state.get_black_king_location(),
            };
            draw_rectangle(
                king_location.col() as f32 * self.square_size + self.x_padding,
                king_location.row() as f32 * self.square_size + self.y_padding,
                self.square_size,
                self.square_size,
                self.check_color,
            );
        }
    }

    fn draw_selected_square(&self) {
        if let Some(sq) = self.first_square_selected {
            draw_rectangle(
                f32::from(sq.col() as u8) * self.square_size + self.x_padding,
                f32::from(sq.row() as u8) * self.square_size + self.y_padding,
                self.square_size,
                self.square_size,
                self.selected_color,
            );
        }
    }

    fn draw_valid_moves(&self, game_state: &State) {
        if let Some(sq) = self.first_square_selected {
            let selected_piece = game_state.get_square(sq);
            for valid_move in game_state.get_valid_moves() {
                if valid_move.piece_moved == selected_piece && valid_move.start == sq {
                    if valid_move.piece_captured == Square::Empty
                        && valid_move.special_move != Some(SpecialMove::EnPassant)
                    {
                        draw_circle(
                            self.x_padding
                                + self.square_size * valid_move.end.col() as f32
                                + self.square_size / 2.0,
                            self.y_padding
                                + self.square_size * valid_move.end.row() as f32
                                + self.square_size / 2.0,
                            self.square_size / 5.0,
                            self.selected_color,
                        );
                    } else {
                        draw_rectangle_lines(
                            self.x_padding + self.square_size * valid_move.end.col() as f32,
                            self.y_padding + self.square_size * valid_move.end.row() as f32,
                            self.square_size,
                            self.square_size,
                            self.square_size / 6.0,
                            self.check_color,
                        );
                    }
                }
            }
        }
    }

    fn draw_pieces(&self, game_state: &State) {
        for row in 0u8..8u8 {
            let y = f32::from(row) * self.square_size + self.y_padding;
            for col in 0u8..8u8 {
                let x = f32::from(col) * self.square_size + self.x_padding;

                let coordinates = BoardCoordinates::new(row as usize, col as usize);
                if let Some(texture) = self.textures.get(&game_state.get_square(coordinates)) {
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

    fn update_square_size(&mut self) {
        self.square_size = (screen_width().min(screen_height()) / 8.5).floor();
    }

    fn update_padding(&mut self) {
        self.x_padding = ((screen_width() - (self.square_size * 8.0)) / 2.0).floor();
        self.y_padding = ((screen_height() - (self.square_size * 8.0)) / 2.0).floor();
    }

    fn update_texture_params(&mut self) {
        self.texture_params = DrawTextureParams {
            dest_size: Some(Vec2::new(self.square_size, self.square_size)),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
    }

    fn load_california_pieces() -> HashMap<Square, Texture2D> {
        use Piece::*;
        use Player::*;

        let mut textures: HashMap<Square, Texture2D> = HashMap::new();
        textures.insert(
            Square::Occupied(Black, Pawn),
            Texture2D::from_file_with_format(
                assets::pieces::california::BLACK_PAWN,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(Black, Knight),
            Texture2D::from_file_with_format(
                assets::pieces::california::BLACK_KNIGHT,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(Black, Bishop),
            Texture2D::from_file_with_format(
                assets::pieces::california::BLACK_BISHOP,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(Black, Rook),
            Texture2D::from_file_with_format(
                assets::pieces::california::BLACK_ROOK,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(Black, Queen),
            Texture2D::from_file_with_format(
                assets::pieces::california::BLACK_QUEEN,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(Black, King),
            Texture2D::from_file_with_format(
                assets::pieces::california::BLACK_KING,
                Some(ImageFormat::Png),
            ),
        );

        textures.insert(
            Square::Occupied(White, Pawn),
            Texture2D::from_file_with_format(
                assets::pieces::california::WHITE_PAWN,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(White, Knight),
            Texture2D::from_file_with_format(
                assets::pieces::california::WHITE_KNIGHT,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(White, Bishop),
            Texture2D::from_file_with_format(
                assets::pieces::california::WHITE_BISHOP,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(White, Rook),
            Texture2D::from_file_with_format(
                assets::pieces::california::WHITE_ROOK,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(White, Queen),
            Texture2D::from_file_with_format(
                assets::pieces::california::WHITE_QUEEN,
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            Square::Occupied(White, King),
            Texture2D::from_file_with_format(
                assets::pieces::california::WHITE_KING,
                Some(ImageFormat::Png),
            ),
        );

        textures
    }
}