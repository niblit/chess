use macroquad::prelude::*;

use state::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = State::new();
    let mut drawing = gui::Drawing::default();

    let mut first_square_selected: Option<BoardCoordinates> = None;
    let mut second_square_selected: Option<BoardCoordinates> = None;

    loop {
        // Close game
        if is_quit_requested() || is_key_down(KeyCode::Escape) {
            break;
        }

        // Undo last move
        if is_key_pressed(KeyCode::Z) {
            game_state.undo_move();
        }

        // Move logic
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_location = mouse_position();

            let board_start = drawing.get_board_start();
            let board_end = drawing.get_board_end();

            // Make sure the mouse click is inside the board
            if mouse_location.0 > board_start.0
                && mouse_location.1 > board_start.1
                && mouse_location.0 < board_end.0
                && mouse_location.1 < board_end.1
            {
                let square_clicked = BoardCoordinates::new(
                    ((mouse_location.1 - board_start.1) / drawing.get_square_size()) as usize,
                    ((mouse_location.0 - board_start.0) / drawing.get_square_size()) as usize,
                );

                if first_square_selected.is_none() {
                    if let Square::Occupied(p, _) = game_state.get_square(square_clicked) {
                        if p == game_state.get_turn() {
                            first_square_selected = Some(square_clicked);
                        }
                    }
                } else if square_clicked == first_square_selected.unwrap() {
                    first_square_selected = None;
                } else if let Square::Occupied(p, _) = game_state.get_square(square_clicked) {
                    if p != game_state.get_turn() {
                        second_square_selected = Some(square_clicked);
                    } else {
                        first_square_selected = Some(square_clicked);
                    }
                } else {
                    second_square_selected = Some(square_clicked);
                }

                if first_square_selected.is_some() && second_square_selected.is_some() {
                    let mut potential_move = Move::new(
                        first_square_selected.unwrap(),
                        second_square_selected.unwrap(),
                        None,
                        &game_state,
                    );

                    let mut is_move_valid = false;
                    for real_move in game_state.get_valid_moves() {
                        if real_move == &potential_move {
                            potential_move = *real_move;
                            is_move_valid = true;
                        }
                    }
                    if is_move_valid {
                        game_state.make_move(potential_move);
                    }

                    first_square_selected = None;
                    second_square_selected = None;
                }
            }
        }

        // Draw
        clear_background(BLACK);
        drawing.update_frame(&game_state, first_square_selected);

        // Wait for next frame
        next_frame().await
    }
}

fn window_conf() -> Conf {
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
