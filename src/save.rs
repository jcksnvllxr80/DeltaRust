use crate::character::CharacterAppearance;
use crate::model::{Player, TileType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub player: Player,
    pub screen_x: i32,
    pub screen_y: i32,
    pub in_dungeon: bool,
    pub dungeon_id: i32,
    pub in_interior: bool,
    pub interior_id: String,
    pub visited: HashSet<String>,
    pub cleared_rooms: HashSet<String>,
    pub opened_chests: HashMap<String, Vec<(usize, usize)>>,
    pub destroyed_tiles: HashMap<String, Vec<(usize, usize, TileType)>>,
    pub appearance: CharacterAppearance,
    pub dungeon_overworld_x: i32,
    pub dungeon_overworld_y: i32,
}

fn save_path() -> std::path::PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.join("save.toml");
        }
    }
    std::path::PathBuf::from("save.toml")
}

pub fn save_game(data: &SaveData) -> Result<(), String> {
    let toml_str = toml::to_string(data).map_err(|e| format!("serialize error: {e}"))?;
    std::fs::write(save_path(), toml_str).map_err(|e| format!("write error: {e}"))
}

pub fn load_game() -> Result<SaveData, String> {
    let contents =
        std::fs::read_to_string(save_path()).map_err(|e| format!("read error: {e}"))?;
    toml::from_str(&contents).map_err(|e| format!("deserialize error: {e}"))
}

pub fn has_save() -> bool {
    save_path().exists()
}
