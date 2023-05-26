use macroquad::prelude::*;

#[derive(Default)]
pub struct EndScene {}

impl EndScene {
    pub fn update_frame(&mut self) -> crate::Scene {
        clear_background(Color::from_rgba(50, 50, 50, 255));

        crate::Scene::End
    }
}
