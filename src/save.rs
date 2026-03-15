use crate::character::CharacterAppearance;
use crate::model::{Player, TileType};
use crate::world_data;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

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

#[derive(Clone, Debug)]
pub struct SaveSlotSummary {
    pub slot: usize,
    pub exists: bool,
    pub location: String,
    pub stats: String,
}

pub const SAVE_SLOT_COUNT: usize = 3;

fn save_root() -> std::path::PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.to_path_buf();
        }
    }
    std::path::PathBuf::from(".")
}

fn legacy_save_path() -> std::path::PathBuf {
    save_root().join("save.toml")
}

fn save_path(slot: usize) -> std::path::PathBuf {
    save_root().join(format!("save_slot_{}.toml", slot + 1))
}

fn slot_file_path(slot: usize) -> std::path::PathBuf {
    let path = save_path(slot);
    if slot == 0 && !path.exists() && legacy_save_path().exists() {
        legacy_save_path()
    } else {
        path
    }
}

fn read_save(path: &std::path::Path) -> Result<SaveData, String> {
    let contents = std::fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;
    toml::from_str(&contents).map_err(|e| format!("deserialize error: {e}"))
}

fn slot_summary(slot: usize, data: &SaveData) -> SaveSlotSummary {
    let location = world_data::location_name(
        data.screen_x,
        data.screen_y,
        data.in_dungeon,
        data.dungeon_id,
        data.in_interior,
        &data.interior_id,
    );
    SaveSlotSummary {
        slot,
        exists: true,
        location,
        stats: format!(
            "HP {}/{}   Gems {}   Pieces {}",
            data.player.hp, data.player.max_hp, data.player.gems, data.player.dragon_pieces
        ),
    }
}

pub fn save_game(slot: usize, data: &SaveData) -> Result<(), String> {
    let toml_str = toml::to_string(data).map_err(|e| format!("serialize error: {e}"))?;
    std::fs::write(save_path(slot), toml_str).map_err(|e| format!("write error: {e}"))
}

pub fn load_game(slot: usize) -> Result<SaveData, String> {
    read_save(&slot_file_path(slot))
}

pub fn slot_count() -> usize {
    SAVE_SLOT_COUNT
}

pub fn has_save(slot: usize) -> bool {
    slot_file_path(slot).exists()
}

pub fn has_any_save() -> bool {
    (0..SAVE_SLOT_COUNT).any(has_save)
}

pub fn list_saves() -> Vec<SaveSlotSummary> {
    (0..SAVE_SLOT_COUNT)
        .map(|slot| {
            let path = slot_file_path(slot);
            if path.exists() {
                match read_save(&path) {
                    Ok(data) => slot_summary(slot, &data),
                    Err(_) => SaveSlotSummary {
                        slot,
                        exists: true,
                        location: "Unreadable Save".to_string(),
                        stats: "Save data could not be parsed".to_string(),
                    },
                }
            } else {
                SaveSlotSummary {
                    slot,
                    exists: false,
                    location: "Empty Slot".to_string(),
                    stats: "No save data".to_string(),
                }
            }
        })
        .collect()
}

pub fn latest_save_slot() -> Option<usize> {
    let mut best: Option<(usize, SystemTime)> = None;
    for slot in 0..SAVE_SLOT_COUNT {
        let path = slot_file_path(slot);
        if !path.exists() {
            continue;
        }
        let modified = std::fs::metadata(&path)
            .and_then(|meta| meta.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        match best {
            Some((_, best_time)) if modified <= best_time => {}
            _ => best = Some((slot, modified)),
        }
    }
    best.map(|(slot, _)| slot)
}
