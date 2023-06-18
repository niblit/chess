mod scenes;
mod window_configuration;

pub use scenes::prelude::SceneManager;
pub use window_configuration::window_configuration;

use macroquad::prelude::*;

pub fn is_inside(location: Vec2, rectangle: Rect) -> bool {
    rectangle.x < location.x
        && rectangle.y < location.y
        && location.x < rectangle.x + rectangle.w
        && location.y < rectangle.y + rectangle.h
}
