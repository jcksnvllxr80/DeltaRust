use crate::constants::{COLS, ROWS, TILE};
use crate::model::{EnemySpawn, ItemDef, TileGrid, TileType, WorldSnapshot};
use crate::world_data::{
    build_dungeon, build_overworld, empty_tiles, enemy_spawns, screen_items, screen_key,
};
use std::collections::{HashMap, HashSet};

pub struct World {
    pub screen_x: i32,
    pub screen_y: i32,
    pub in_dungeon: bool,
    pub tiles: TileGrid,
    pub visited: HashSet<String>,
    pub cleared_rooms: HashSet<String>,
    pub opened_chests: HashMap<String, Vec<(usize, usize)>>,
    pub destroyed_tiles: HashMap<String, Vec<(usize, usize, TileType)>>,
    overworld_data: HashMap<String, TileGrid>,
    dungeon_data: HashMap<String, TileGrid>,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            screen_x: 1,
            screen_y: 1,
            in_dungeon: false,
            tiles: empty_tiles(),
            visited: HashSet::new(),
            cleared_rooms: HashSet::new(),
            opened_chests: HashMap::new(),
            destroyed_tiles: HashMap::new(),
            overworld_data: build_overworld(),
            dungeon_data: build_dungeon(),
        };
        world.load_screen(1, 1);
        world
    }

    pub fn snapshot(&self) -> WorldSnapshot {
        WorldSnapshot {
            screen_x: self.screen_x,
            screen_y: self.screen_y,
            in_dungeon: self.in_dungeon,
            tiles: self.tiles.clone(),
            visited: self.visited.clone(),
            cleared_rooms: self.cleared_rooms.clone(),
            opened_chests: self.opened_chests.clone(),
            destroyed_tiles: self.destroyed_tiles.clone(),
        }
    }

    pub fn load_screen(&mut self, sx: i32, sy: i32) {
        self.screen_x = sx;
        self.screen_y = sy;
        let key = screen_key(sx, sy);
        self.visited.insert(key.clone());
        self.tiles = if self.in_dungeon {
            self.dungeon_data
                .get(&key)
                .cloned()
                .unwrap_or_else(empty_tiles)
        } else {
            self.overworld_data
                .get(&key)
                .cloned()
                .unwrap_or_else(empty_tiles)
        };
        let persisted_key = format!("{}:{key}", if self.in_dungeon { "d" } else { "o" });
        if let Some(changes) = self.destroyed_tiles.get(&persisted_key) {
            for (row, col, tile) in changes {
                self.tiles[*row][*col] = *tile;
            }
        }
        if let Some(chests) = self.opened_chests.get(&persisted_key) {
            for (row, col) in chests {
                self.tiles[*row][*col] = if self.in_dungeon {
                    TileType::Floor
                } else {
                    TileType::Grass
                };
            }
        }
    }

    pub fn get_tile(&self, col: i32, row: i32) -> TileType {
        if col < 0 || row < 0 || col >= COLS as i32 || row >= ROWS as i32 {
            TileType::Rock
        } else {
            self.tiles[row as usize][col as usize]
        }
    }

    pub fn is_solid(&self, col: i32, row: i32) -> bool {
        matches!(
            self.get_tile(col, row),
            TileType::Tree
                | TileType::Water
                | TileType::Rock
                | TileType::Cracked
                | TileType::Wall
                | TileType::DoorLocked
                | TileType::BossDoor
                | TileType::Chest
        )
    }

    pub fn collides(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let l = (x / TILE).floor() as i32;
        let r = ((x + w - 1.0) / TILE).floor() as i32;
        let t = (y / TILE).floor() as i32;
        let b = ((y + h - 1.0) / TILE).floor() as i32;
        for row in t..=b {
            for col in l..=r {
                if col >= 0
                    && row >= 0
                    && col < COLS as i32
                    && row < ROWS as i32
                    && self.is_solid(col, row)
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn destroy_tile(&mut self, col: usize, row: usize, new_tile: TileType) {
        self.tiles[row][col] = new_tile;
        self.destroyed_tiles
            .entry(format!(
                "{}:{}",
                if self.in_dungeon { "d" } else { "o" },
                screen_key(self.screen_x, self.screen_y)
            ))
            .or_default()
            .push((row, col, new_tile));
    }

    pub fn mark_chest_opened(&mut self, col: usize, row: usize) {
        self.opened_chests
            .entry(format!(
                "{}:{}",
                if self.in_dungeon { "d" } else { "o" },
                screen_key(self.screen_x, self.screen_y)
            ))
            .or_default()
            .push((row, col));
        self.tiles[row][col] = if self.in_dungeon {
            TileType::Floor
        } else {
            TileType::Grass
        };
    }

    pub fn enter_dungeon(&mut self) {
        self.in_dungeon = true;
        self.load_screen(1, 2);
    }

    pub fn exit_dungeon(&mut self, x: i32, y: i32) {
        self.in_dungeon = false;
        self.load_screen(x, y);
    }

    pub fn get_enemy_spawns(&self) -> Vec<EnemySpawn> {
        enemy_spawns(
            self.screen_x,
            self.screen_y,
            self.in_dungeon,
            self.cleared_rooms
                .contains(&format!("d:{}", screen_key(self.screen_x, self.screen_y))),
        )
    }

    pub fn get_screen_items(&self) -> Vec<ItemDef> {
        screen_items(self.screen_x, self.screen_y, self.in_dungeon)
    }

    pub fn screen_exists(&self, x: i32, y: i32) -> bool {
        let key = screen_key(x, y);
        if self.in_dungeon {
            self.dungeon_data.contains_key(&key)
        } else {
            self.overworld_data.contains_key(&key)
        }
    }

    pub fn screen_tiles(&self, x: i32, y: i32) -> Option<&TileGrid> {
        let key = screen_key(x, y);
        if self.in_dungeon {
            self.dungeon_data.get(&key)
        } else {
            self.overworld_data.get(&key)
        }
    }
}
