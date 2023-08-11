use gui::{window_conf, SceneManager};
use macroquad::prelude::*;
use state::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    // Game state manipulation
    let mut game_state = GameState::new();

    // All logic for managing the screen
    let mut scene_manager = SceneManager::new();

    loop {
        // Clear frame
        clear_background(BLACK);

        // Close game
        if is_quit_requested() || is_key_down(KeyCode::Escape) {
            break;
        }

        // Update frame
        scene_manager.update_frame(&mut game_state);

        // Wait for next frame
        next_frame().await
    }
}
