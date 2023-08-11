use macroquad::prelude::*;

use end_scene::EndScene;
use game_scene::GameScene;
use state::prelude::*;

mod game_scene;

mod end_scene;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Chess"),
        window_width: 600,
        window_height: 600,
        high_dpi: true,
        fullscreen: false,
        sample_count: 8,
        window_resizable: true,
        ..Default::default()
    }
}

#[derive(Eq, PartialEq)]
pub enum Scene {
    Game,
    End,
}

pub struct SceneManager {
    game_scene: GameScene,
    end_scene: EndScene,
    scene: Scene,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    pub fn new() -> Self {
        let game_scene = GameScene::default();
        let end_scene = EndScene::default();
        let scene = Scene::Game;
        Self {
            game_scene,
            end_scene,
            scene,
        }
    }

    pub fn update_frame(&mut self, game_state: &mut State) {
        let new_scene = match self.scene {
            Scene::Game => self.game_scene.update_frame(game_state),
            Scene::End => {
                self.game_scene.update_sizes();
                self.game_scene.draw_frame(game_state);
                self.end_scene.update_frame(&self.game_scene, game_state)
            }
        };

        if self.scene == Scene::End && new_scene == Scene::Game {
            game_state.restart();
        }
        self.scene = new_scene;
    }
}
