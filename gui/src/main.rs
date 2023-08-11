use gui::assets;
use gui::{window_configuration, SceneManager};
use macroquad::audio::load_sound_from_bytes;
use macroquad::prelude::*;
use state::prelude::*;

#[macroquad::main(window_configuration)]
async fn main() {
    // Game state manipulation
    let mut game_state = GameState::default();

    let move_sound = load_sound_from_bytes(assets::sounds::MOVE).await.unwrap();
    let capture_sound = load_sound_from_bytes(assets::sounds::CAPTURE)
        .await
        .unwrap();

    // All logic for managing the screen
    let mut scene_manager = SceneManager::new(move_sound, capture_sound);

    loop {
        clear_background(BLACK);

        // Close game
        if is_quit_requested() {
            break;
        }

        // Update frame
        let result = scene_manager.update_frame(&mut game_state).await;
        if result.is_none() {
            break;
        }
        next_frame().await;
    }
}
