use crate::scenes::prelude::{Game, Scene};
use macroquad::prelude::*;

pub struct GameOver {
    font_color: Color,
}

impl Default for GameOver {
    fn default() -> Self {
        Self::new()
    }
}

impl GameOver {
    pub fn new() -> Self {
        Self {
            font_color: assets::colors::FONT,
        }
    }
    pub async fn update_frame(
        &mut self,
        game_scene: &mut Game,
        game_state: &mut state::prelude::GameState,
    ) -> Option<Scene> {
        let (start, end) = (game_scene.get_board_start(), game_scene.get_board_end());
        let font_size = game_scene.get_square_size() / 2.0;
        game_scene.update_sizes();
        game_scene.update_frame(game_state).await;

        draw_rectangle(
            start.0,
            start.1,
            end.0 - start.0,
            end.1 - start.1,
            assets::colors::BACKGROUND_DIMMING,
        );

        draw_text(
            game_state.get_game_result().unwrap().to_str(),
            start.0,
            (end.1 + start.1) / 2.0 - font_size / 2.0,
            font_size,
            self.font_color,
        );
        draw_text(
            "Click Anywhere To Play Again",
            start.0,
            (end.1 + start.1) / 2.0 + font_size / 2.0,
            font_size,
            self.font_color,
        );
        if is_mouse_button_pressed(MouseButton::Left) {
            return Some(Scene::Game);
        }
        Some(Scene::GameOver)
    }
}
