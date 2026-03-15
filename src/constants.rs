use macroquad::prelude::Conf;

pub const BASE_TILE: f32 = 32.0;
pub const TILE: f32 = 64.0;
pub const PIXEL_SCALE: f32 = TILE / BASE_TILE;
pub const COLS: usize = 16;
pub const ROWS: usize = 11;
pub const HUD_H: f32 = 128.0;
pub const GAME_W: f32 = TILE * COLS as f32;
pub const GAME_H: f32 = TILE * ROWS as f32;
pub const WORLD_W: i32 = 14;
pub const WORLD_H: i32 = 15;
pub const PLAYER_SPEED: f32 = 0.8 * PIXEL_SCALE;
pub const PLAYER_MAX_HP: i32 = 6;
pub const ATTACK_DURATION: i32 = 12;
pub const KNOCKBACK_SPEED: f32 = 2.5 * PIXEL_SCALE;
pub const KNOCKBACK_FRAMES: i32 = 10;
pub const TRANS_SPEED: f32 = 14.0 * PIXEL_SCALE;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "DeltaRust".to_string(),
        window_width: GAME_W as i32,
        window_height: (GAME_H + HUD_H) as i32,
        high_dpi: false,
        sample_count: 1,
        window_resizable: false,
        ..Default::default()
    }
}
