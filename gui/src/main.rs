use gui::{window_configuration, SceneManager};
use macroquad::prelude::*;
use state::prelude::*;

#[macroquad::main(window_configuration)]
async fn main() {
    // Game state manipulation
    let mut game_state = GameState::default();

    // All logic for managing the screen
    let mut scene_manager = SceneManager::default();

    loop {
        // Clear frame
        clear_background(BLACK);

        // Close game
        if is_quit_requested() {
            break;
        }

        // Update frame
        scene_manager.update_frame(&mut game_state).await;

        // Wait for next frame
        next_frame().await
    }
}
