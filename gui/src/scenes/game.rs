use crate::assets;
use crate::scenes::prelude::Scene;
use macroquad::{
    audio::{play_sound_once, Sound},
    prelude::*,
};
use std::collections::HashMap;

use state::prelude::*;

pub struct Game {
    piece_textures: HashMap<Square, Texture2D>,
    piece_texture_params: DrawTextureParams,

    board_texture: Texture2D,
    board_texture_params: DrawTextureParams,

    check_color: Color,
    move_color: Color,
    selected_color: Color,

    square_size: f32,
    x_padding: f32,
    y_padding: f32,

    first_square_selected: Option<BoardCoordinates>,
    second_square_selected: Option<BoardCoordinates>,

    move_sound: Sound,
    capture_sound: Sound,

    engine_turn: Option<Player>,
}

impl Game {
    pub fn default(move_sound: Sound, capture_sound: Sound) -> Self {
        let piece_textures = Self::load_california_pieces();
        let board_texture =
            Texture2D::from_file_with_format(assets::board::BOARD, Some(ImageFormat::Png));
        board_texture.set_filter(FilterMode::Nearest);

        let check_color = assets::colors::CHECK;
        let move_color = assets::colors::LAST_MOVE;
        let selected_color = assets::colors::SQUARE_SELECTED;
        let engine_turn = None;
        Self::new(
            piece_textures,
            board_texture,
            check_color,
            move_color,
            selected_color,
            move_sound,
            capture_sound,
            engine_turn,
        )
    }
}

impl Game {
    pub fn new(
        piece_textures: HashMap<Square, Texture2D>,
        board_texture: Texture2D,
        check_color: Color,
        move_color: Color,
        selected_color: Color,
        move_sound: Sound,
        capture_sound: Sound,
        engine_turn: Option<Player>,
    ) -> Self {
        let piece_textures_params = DrawTextureParams::default();
        let board_texture_params = DrawTextureParams::default();

        let square_size = 0f32;
        let x_padding = 0f32;
        let y_padding = 0f32;

        let first_square_selected: Option<BoardCoordinates> = None;
        let second_square_selected: Option<BoardCoordinates> = None;
        Self {
            piece_textures,
            piece_texture_params: piece_textures_params,

            board_texture,
            board_texture_params,

            square_size,
            x_padding,
            y_padding,

            check_color,
            move_color,
            selected_color,

            first_square_selected,
            second_square_selected,

            move_sound,
            capture_sound,
            engine_turn,
        }
    }

    pub fn set_engine_turn(&mut self, turn: Option<Player>) {
        self.engine_turn = turn;
    }

    pub fn get_square_size(&self) -> f32 {
        self.square_size
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

    pub async fn update_frame(&mut self, game_state: &mut GameState) -> Option<Scene> {
        self.update_logic(game_state).await;

        self.update_sizes();

        self.draw_frame(game_state);

        if game_state.is_game_over() {
            return Some(Scene::GameOver);
        }

        Some(Scene::Game)
    }

    pub fn update_sizes(&mut self) {
        self.update_square_size();
        self.update_padding();
        self.update_texture_params();
    }

    async fn update_logic(&mut self, game_state: &mut GameState) {
        if Some(game_state.get_turn()) == self.engine_turn {
            if let Some(to_move) = game_state.best_move() {
                game_state.make_new_move(to_move);
                if to_move.piece_captured == Square::Empty {
                    play_sound_once(&self.move_sound);
                } else {
                    play_sound_once(&self.capture_sound);
                }
            }
            return;
        }
        // Undo last move
        if is_key_pressed(KeyCode::Z) {
            game_state.undo_last_move();
            if self.engine_turn.is_some() {
                game_state.undo_last_move();
            }
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

                    if game_state.get_valid_moves().contains(&potential_move) {
                        if potential_move.piece_moved
                            == Square::Occupied(Player::White, Piece::Pawn)
                            && potential_move.end.row() == 0
                        {
                            let promotion_piece =
                                self.get_promotion_piece(Player::White, game_state).await;
                            potential_move = Move::new(
                                potential_move.start,
                                potential_move.end,
                                Some(SpecialMove::PawnPromotion(promotion_piece)),
                                game_state,
                            );
                        } else if potential_move.piece_moved
                            == Square::Occupied(Player::Black, Piece::Pawn)
                            && potential_move.end.row() == 7
                        {
                            let promotion_piece =
                                self.get_promotion_piece(Player::Black, game_state).await;
                            potential_move = Move::new(
                                potential_move.start,
                                potential_move.end,
                                Some(SpecialMove::PawnPromotion(promotion_piece)),
                                game_state,
                            );
                        }
                    }

                    let mut is_move_valid = false;
                    for real_move in game_state.get_valid_moves() {
                        if &potential_move == real_move {
                            // Use the generated move in game state instead of the one generated
                            // with mouse input, to preserve special move's properties
                            potential_move = *real_move;
                            is_move_valid = true;
                        }
                    }
                    if is_move_valid {
                        game_state.make_new_move(potential_move);
                        if potential_move.piece_captured == Square::Empty {
                            play_sound_once(&self.move_sound);
                        } else {
                            play_sound_once(&self.capture_sound);
                        }
                    }

                    self.first_square_selected = None;
                    self.second_square_selected = None;
                }
            }
        }
    }

    async fn get_promotion_piece(&mut self, color: Player, game_state: &GameState) -> Square {
        let pieces = [
            [
                Square::Occupied(color, Piece::Queen),
                Square::Occupied(color, Piece::Rook),
            ],
            [
                Square::Occupied(color, Piece::Bishop),
                Square::Occupied(color, Piece::Knight),
            ],
        ];
        loop {
            self.update_sizes();
            self.draw_frame(game_state);
            let pieces_start = (
                (screen_width() - self.square_size * 2.0) / 2.0,
                (screen_height() - self.square_size * 2.0) / 2.0,
            );
            let pieces_end = (
                pieces_start.0 + self.square_size * 2.0,
                pieces_start.1 + self.square_size * 2.0,
            );
            draw_rectangle(
                self.get_board_start().0,
                self.get_board_start().1,
                self.get_board_end().0 - self.get_board_start().0,
                self.get_board_end().1 - self.get_board_start().1,
                assets::colors::BACKGROUND_DIMMING,
            );

            for (i, pieces_line) in pieces.iter().enumerate() {
                for (j, piece) in pieces_line.iter().enumerate() {
                    let texture = self.piece_textures.get(piece).unwrap();
                    draw_texture_ex(
                        texture,
                        pieces_start.0 + self.square_size * j as f32,
                        pieces_start.1 + self.square_size * i as f32,
                        assets::colors::TEXTURE,
                        self.piece_texture_params.clone(),
                    );
                }
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_location = mouse_position();
                if mouse_location.0 >= pieces_start.0
                    && mouse_location.1 >= pieces_start.1
                    && mouse_location.0 <= pieces_end.0
                    && mouse_location.1 <= pieces_end.1
                {
                    let click_square = (
                        ((mouse_location.0 - pieces_start.0) / self.square_size) as usize,
                        ((mouse_location.1 - pieces_start.1) / self.square_size) as usize,
                    );
                    return pieces[click_square.1][click_square.0];
                }
            }
            next_frame().await
        }
    }

    pub fn draw_frame(&self, game_state: &GameState) {
        self.draw_board();
        self.draw_highlights(game_state);
        self.draw_pieces(game_state);
    }

    fn draw_board(&self) {
        draw_texture_ex(
            &self.board_texture,
            self.get_board_start().0,
            self.get_board_start().1,
            assets::colors::TEXTURE,
            self.board_texture_params.clone(),
        );
    }

    fn draw_highlights(&self, game_state: &GameState) {
        self.draw_last_move(game_state);
        self.draw_checks(game_state);
        self.draw_selected_square();
        self.draw_valid_moves(game_state);
    }

    fn draw_last_move(&self, game_state: &GameState) {
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

    fn draw_checks(&self, game_state: &GameState) {
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

    fn draw_valid_moves(&self, game_state: &GameState) {
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

    fn draw_pieces(&self, game_state: &GameState) {
        for row in 0u8..8u8 {
            let y = f32::from(row) * self.square_size + self.y_padding;
            for col in 0u8..8u8 {
                let x = f32::from(col) * self.square_size + self.x_padding;

                let coordinates = BoardCoordinates::new(row as usize, col as usize);
                if let Some(texture) = self.piece_textures.get(&game_state.get_square(coordinates))
                {
                    draw_texture_ex(
                        texture,
                        x,
                        y,
                        assets::colors::TEXTURE,
                        self.piece_texture_params.clone(),
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
        self.piece_texture_params = DrawTextureParams {
            dest_size: Some(Vec2::new(self.square_size, self.square_size)),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        self.board_texture_params = DrawTextureParams {
            dest_size: Some(Vec2::new(
                self.get_board_end().0 - self.get_board_start().0,
                self.get_board_end().1 - self.get_board_start().1,
            )),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        }
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
