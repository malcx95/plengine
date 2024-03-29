use libplen::constants;
use libplen::gamestate::GameState;
use libplen::math::{self, vec2, Vec2};
use macroquad::prelude::*;
use macroquad::texture;

use crate::assets::Assets;

pub struct ClientState {
    // add client side state
}

impl ClientState {
    pub fn new() -> ClientState {
        ClientState {
            // init client stuff
        }
    }

    pub fn update(&mut self, delta_time: f32, game_state: &GameState, my_id: u64) {
        // update client side stuff
    }

    pub fn draw(
        &self,
        my_id: u64,
        game_state: &GameState,
        assets: &mut Assets,
    ) -> Result<(), String> {

        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        for player in &game_state.players {
            let params = texture::DrawTextureParams {
                dest_size: None,
                source: None,
                rotation: player.angle,
                flip_x: false,
                flip_y: false,
                pivot: None,
            };

            let px = player.position.x;
            let py = player.position.y;

            texture::draw_texture_ex(assets.malcolm, px, py, BLUE, params);
        }

        Ok(())
    }
}
