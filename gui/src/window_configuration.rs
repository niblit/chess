use macroquad::prelude::Conf;

pub fn window_configuration() -> Conf {
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
