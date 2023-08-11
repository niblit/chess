mod game;
mod game_over;
mod manager;
mod settings;

pub mod prelude {
    pub use crate::scenes::game::Game;
    pub use crate::scenes::game_over::GameOver;
    pub use crate::scenes::manager::SceneManager;
    pub use crate::scenes::settings::Settings;

    #[derive(Eq, PartialEq, Clone, Copy)]
    pub enum Scene {
        Settings,
        Game,
        GameOver,
    }
}
