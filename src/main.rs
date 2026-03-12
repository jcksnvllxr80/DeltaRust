mod audio;
mod character;
mod constants;
mod game;
mod generated_overworld;
mod model;
mod render;
mod sprites;
mod world;
mod world_data;

use crate::constants::window_conf;
use crate::game::Game;

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dev_mode = args.iter().any(|a| a == "--dev-mode" || a == "--dev");
    let mut game = Game::new(dev_mode).await;
    loop {
        game.update();
        game.draw();
        macroquad::prelude::next_frame().await;
    }
}
