use macroquad::prelude::*;

pub struct EndScene {
    font_color: Color,
}

impl Default for EndScene {
    fn default() -> Self {
        Self::new()
    }
}

impl EndScene {
    pub fn new() -> Self {
        Self {
            font_color: assets::colors::FONT,
        }
    }
    pub fn update_frame(
        &mut self,
        game_scene: &crate::GameScene,
        game_state: &state::prelude::GameState,
    ) -> crate::Scene {
        let (start, end) = (game_scene.get_board_start(), game_scene.get_board_end());
        let font_size = game_scene.get_square_size() / 2.0;

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
            "Press [Space] To Play Again",
            start.0,
            (end.1 + start.1) / 2.0 + font_size / 2.0,
            font_size,
            self.font_color,
        );
        if is_key_pressed(KeyCode::Space) {
            return crate::Scene::Game;
        }
        crate::Scene::End
    }
}
