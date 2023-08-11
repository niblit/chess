use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = state::State::new();
    let mut drawing = gui::Drawing::default();

    let mut square_selected: Option<state::BoardCoordinates> = None;
    let mut player_clicks: Vec<state::BoardCoordinates> = Vec::new();
    loop {
        // Inputs
        if is_quit_requested() || is_key_down(KeyCode::Escape) {
            break;
        }

        // State Logic
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_location = mouse_position();
            let board_start = drawing.get_board_start();
            let board_end = drawing.get_board_end();

            if mouse_location.0 > board_start.0
                && mouse_location.1 > board_start.1
                && mouse_location.0 < board_end.0
                && mouse_location.1 < board_end.1
            {
                let coord = state::BoardCoordinates {
                    row: ((mouse_location.1 - board_start.1) / drawing.square_size) as u8,
                    col: ((mouse_location.0 - board_start.0) / drawing.square_size) as u8,
                };

                if square_selected.is_none() {
                    square_selected = Some(coord);
                    player_clicks.push(coord);
                } else if square_selected.unwrap() == coord {
                    square_selected = None;
                    player_clicks.clear();
                } else {
                    square_selected = Some(coord);
                    player_clicks.push(coord);
                }

                if player_clicks.len() == 2 {
                    let to_move =
                        state::Move::new(player_clicks[0], player_clicks[1], None, &game_state);

                    if let Some(color) = to_move.piece_moved.get_color() {
                        if color == game_state.turn {
                            game_state.make_move(to_move);
                        }
                    }
                    square_selected = None;
                    player_clicks.clear();
                }
            }
        }

        // Drawing
        clear_background(BLACK);
        drawing.update_frame(&game_state, square_selected);

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
