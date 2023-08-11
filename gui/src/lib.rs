use macroquad::prelude::*;
use state::prelude::*;

mod game_scene;
use game_scene::GameScene;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Chess"),
        window_width: 600,
        window_height: 600,
        high_dpi: true,
        fullscreen: true,
        sample_count: 8,
        window_resizable: true,
        ..Default::default()
    }
}

pub struct SceneManager {
    game_scene: GameScene,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    pub fn new() -> Self {
        let game_scene = GameScene::default();
        Self { game_scene }
    }

    pub fn update_frame(&mut self, game_state: &mut State) {
        self.game_scene.update_frame(game_state);
    }
}
