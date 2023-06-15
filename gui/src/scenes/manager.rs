pub use crate::scenes::prelude::*;
use macroquad::prelude::*;
use state::prelude::GameState;

pub struct SceneManager {
    current_scene: Scene,
    settings: Settings,
    game: Game,
    game_over: GameOver,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    pub fn new() -> Self {
        let current_scene = Scene::Settings;

        let settings = Settings::default();
        let game = Game::default();
        let game_over = GameOver::default();

        Self {
            current_scene,
            settings,
            game,
            game_over,
        }
    }

    pub async fn update_frame(&mut self, game_state: &mut GameState) -> Option<()> {
        let mut next_scene = match self.current_scene {
            Scene::Settings => self.settings.update_frame().await,
            Scene::Game => self.game.update_frame(game_state).await,
            Scene::GameOver => {
                self.game_over
                    .update_frame(&mut self.game, game_state)
                    .await
            }
        };

        if is_key_pressed(KeyCode::Escape) {
            next_scene = Some(Scene::Settings);
        }

        match next_scene {
            None => None,
            Some(s) => {
                if next_scene == Some(Scene::Game) && self.current_scene == Scene::GameOver {
                    game_state.restart();
                }
                self.current_scene = s;
                Some(())
            }
        }
    }
}
