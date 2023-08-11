use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

use crate::scenes::prelude::Scene;

pub struct Settings;

impl Default for Settings {
    fn default() -> Self {
        Self
    }
}

impl Settings {
    pub async fn update_frame(&mut self) -> Option<Scene> {
        let start = vec2(0.0, 0.0);
        let size = vec2(screen_width(), screen_height());

        let mut next = Some(Scene::Settings);

        root_ui().window(hash!(), start, size, |ui| {
            if ui.button(None, "Play") {
                next = Some(Scene::Game);
            }

            ui.separator();

            if ui.button(None, "Quit") {
                next = None;
            }
        });
        next
    }
}
