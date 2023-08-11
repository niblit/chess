use crate::assets;
use macroquad::prelude::*;
use state::prelude::Player;

use crate::{is_inside, scenes::prelude::Scene};

use super::prelude::Game;

pub struct Settings {
    pvp: Texture2D,
    pvp_params: DrawTextureParams,
    pve: Texture2D,
    pve_params: DrawTextureParams,
    evp: Texture2D,
    evp_params: DrawTextureParams,
    exit: Texture2D,
    exit_params: DrawTextureParams,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Self {
        let pvp = Texture2D::from_file_with_format(assets::buttons::PVP, Some(ImageFormat::Png));
        let pvp_params = DrawTextureParams::default();

        let pve = Texture2D::from_file_with_format(assets::buttons::PVE, Some(ImageFormat::Png));
        let pve_params = DrawTextureParams::default();

        let evp = Texture2D::from_file_with_format(assets::buttons::EVP, Some(ImageFormat::Png));
        let evp_params = DrawTextureParams::default();

        let exit = Texture2D::from_file_with_format(assets::buttons::EXIT, Some(ImageFormat::Png));
        let exit_params = DrawTextureParams::default();

        Self {
            pvp,
            pvp_params,

            pve,
            pve_params,

            evp,
            evp_params,

            exit,
            exit_params,
        }
    }
    pub async fn update_frame(&mut self, game_scene: &mut Game) -> Option<Scene> {
        let mut next = Some(Scene::Settings);
        let individual_size = screen_width().min(screen_height()) / 4.0;

        let section_size = Vec2::new(individual_size * 3.0, individual_size * 2.0);
        let x_padding = (screen_width() - section_size.x) / 2.0;
        let y_padding = (screen_height() - section_size.y) / 2.0;

        let mouse_pos = mouse_position();
        let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);

        let inside_evp = is_inside(
            mouse_pos,
            Rect::new(x_padding, y_padding, individual_size, individual_size),
        );
        let evp_source = if inside_evp {
            Some(Rect::new(0.0, 64.0, 64.0, 64.0))
        } else {
            Some(Rect::new(0.0, 0.0, 64.0, 64.0))
        };

        let inside_pvp = is_inside(
            mouse_pos,
            Rect::new(
                x_padding + individual_size,
                y_padding,
                individual_size,
                individual_size,
            ),
        );
        let pvp_source = if inside_pvp {
            Some(Rect::new(0.0, 64.0, 64.0, 64.0))
        } else {
            Some(Rect::new(0.0, 0.0, 64.0, 64.0))
        };

        let inside_pve = is_inside(
            mouse_pos,
            Rect::new(
                x_padding + individual_size + individual_size,
                y_padding,
                individual_size,
                individual_size,
            ),
        );
        let pve_source = if inside_pve {
            Some(Rect::new(0.0, 64.0, 64.0, 64.0))
        } else {
            Some(Rect::new(0.0, 0.0, 64.0, 64.0))
        };

        let inside_exit = is_inside(
            mouse_pos,
            Rect::new(
                x_padding,
                y_padding + individual_size,
                section_size.x,
                section_size.y / 2.0,
            ),
        );
        let exit_source = if inside_exit {
            Some(Rect::new(0.0, 64.0, 192.0, 64.0))
        } else {
            Some(Rect::new(0.0, 0.0, 192.0, 64.0))
        };

        self.evp_params = DrawTextureParams {
            dest_size: Some(Vec2::new(individual_size, individual_size)),
            source: evp_source,
            ..Default::default()
        };

        self.pvp_params = DrawTextureParams {
            dest_size: Some(Vec2::new(individual_size, individual_size)),
            source: pvp_source,
            ..Default::default()
        };

        self.pve_params = DrawTextureParams {
            dest_size: Some(Vec2::new(individual_size, individual_size)),
            source: pve_source,
            ..Default::default()
        };

        self.exit_params = DrawTextureParams {
            dest_size: Some(Vec2::new(individual_size * 3.0, individual_size)),
            source: exit_source,
            ..Default::default()
        };

        draw_texture_ex(
            self.evp,
            x_padding,
            y_padding,
            assets::colors::TEXTURE,
            self.evp_params.clone(),
        );
        draw_texture_ex(
            self.pvp,
            x_padding + individual_size,
            y_padding,
            assets::colors::TEXTURE,
            self.pvp_params.clone(),
        );
        draw_texture_ex(
            self.pve,
            x_padding + individual_size + individual_size,
            y_padding,
            assets::colors::TEXTURE,
            self.pve_params.clone(),
        );
        draw_texture_ex(
            self.exit,
            x_padding,
            y_padding + individual_size,
            assets::colors::TEXTURE,
            self.exit_params.clone(),
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            if inside_exit {
                next = None;
            } else if inside_evp {
                game_scene.set_engine_turn(Some(Player::White));
                next = Some(Scene::Game);
            } else if inside_pvp {
                game_scene.set_engine_turn(None);
                next = Some(Scene::Game);
            } else if inside_pve {
                game_scene.set_engine_turn(Some(Player::Black));
                next = Some(Scene::Game);
            }
        }

        next
    }
}
