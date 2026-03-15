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

// --- Config-driven accessors (fall back to defaults if config not yet loaded) ---

pub fn player_speed() -> f32 {
    crate::config::get().player.speed * PIXEL_SCALE
}
pub fn player_max_hp() -> i32 {
    crate::config::get().player.max_hp
}
pub fn player_full_hearts_hp() -> i32 {
    crate::config::get().player.full_hearts_hp
}
pub fn attack_duration() -> i32 {
    crate::config::get().player.attack_frames
}
pub fn knockback_speed() -> f32 {
    crate::config::get().player.knockback_speed * PIXEL_SCALE
}
pub fn knockback_frames() -> i32 {
    crate::config::get().player.knockback_frames
}
pub fn trans_speed() -> f32 {
    crate::config::get().player.transition_speed * PIXEL_SCALE
}

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
