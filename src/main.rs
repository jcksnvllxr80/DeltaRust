mod audio;
mod constants;
mod game;
mod model;
mod render;
mod sprites;
mod world;
mod world_data;

use crate::constants::window_conf;
use crate::game::Game;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    loop {
        game.update();
        game.draw();
        macroquad::prelude::next_frame().await;
    }
}
