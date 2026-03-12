use crate::constants::{COLS, ROWS, TILE};
use crate::model::{EnemySpawn, ItemDef, TileGrid, TileType, WorldSnapshot};
use crate::world_data::{
    build_dungeons, build_interiors, build_overworld, dungeon_entry, dungeon_map_rooms,
    empty_tiles, enemy_spawns, overworld_start, screen_items, screen_key,
};
use std::collections::{HashMap, HashSet};

pub struct World {
    pub screen_x: i32,
    pub screen_y: i32,
    pub in_dungeon: bool,
    pub dungeon_id: i32,
    pub in_interior: bool,
    pub interior_id: String,
    pub tiles: TileGrid,
    pub visited: HashSet<String>,
    pub cleared_rooms: HashSet<String>,
    pub opened_chests: HashMap<String, Vec<(usize, usize)>>,
    pub destroyed_tiles: HashMap<String, Vec<(usize, usize, TileType)>>,
    pub dev_mode: bool,
    overworld_data: HashMap<String, TileGrid>,
    interior_data: HashMap<String, TileGrid>,
    dungeon_data: HashMap<i32, HashMap<String, TileGrid>>,
}

impl World {
    pub fn new(dev_mode: bool) -> Self {
        let (start_x, start_y) = overworld_start();
        let mut world = Self {
            screen_x: start_x,
            screen_y: start_y,
            in_dungeon: false,
            dungeon_id: 0,
            in_interior: false,
            interior_id: String::new(),
            tiles: empty_tiles(),
            visited: HashSet::new(),
            cleared_rooms: HashSet::new(),
            opened_chests: HashMap::new(),
            destroyed_tiles: HashMap::new(),
            dev_mode,
            overworld_data: build_overworld(),
            interior_data: build_interiors(),
            dungeon_data: build_dungeons(),
        };
        world.load_screen(start_x, start_y);
        world
    }

    pub fn snapshot(&self) -> WorldSnapshot {
        WorldSnapshot {
            screen_x: self.screen_x,
            screen_y: self.screen_y,
            in_dungeon: self.in_dungeon,
            dungeon_id: self.dungeon_id,
            in_interior: self.in_interior,
            interior_id: self.interior_id.clone(),
            tiles: self.tiles.clone(),
            visited: self.visited.clone(),
            cleared_rooms: self.cleared_rooms.clone(),
            opened_chests: self.opened_chests.clone(),
            destroyed_tiles: self.destroyed_tiles.clone(),
            dungeon_rooms: self.dungeon_rooms(),
            dev_mode: self.dev_mode,
        }
    }

    pub fn load_screen(&mut self, sx: i32, sy: i32) {
        self.screen_x = sx;
        self.screen_y = sy;
        let key = screen_key(sx, sy);
        self.visited.insert(self.visit_key(&key));
        self.tiles = if self.in_interior {
            self.interior_data
                .get(&self.interior_id)
                .cloned()
                .unwrap_or_else(empty_tiles)
        } else if self.in_dungeon {
            self.dungeon_data
                .get(&self.dungeon_id)
                .and_then(|d| d.get(&key))
                .cloned()
                .unwrap_or_else(empty_tiles)
        } else {
            self.overworld_data
                .get(&key)
                .cloned()
                .unwrap_or_else(empty_tiles)
        };
        let persisted_key = self.persist_key(&key);
        if let Some(changes) = self.destroyed_tiles.get(&persisted_key) {
            for (row, col, tile) in changes {
                self.tiles[*row][*col] = *tile;
            }
        }
        if let Some(chests) = self.opened_chests.get(&persisted_key) {
            for (row, col) in chests {
                self.tiles[*row][*col] = if self.in_dungeon || self.in_interior {
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
        let key = screen_key(self.screen_x, self.screen_y);
        self.destroyed_tiles
            .entry(self.persist_key(&key))
            .or_default()
            .push((row, col, new_tile));
    }

    pub fn mark_chest_opened(&mut self, col: usize, row: usize) {
        let key = screen_key(self.screen_x, self.screen_y);
        self.opened_chests
            .entry(self.persist_key(&key))
            .or_default()
            .push((row, col));
        self.tiles[row][col] = if self.in_dungeon || self.in_interior {
            TileType::Floor
        } else {
            TileType::Grass
        };
    }

    pub fn enter_dungeon(&mut self, id: i32) {
        self.in_interior = false;
        self.interior_id.clear();
        self.in_dungeon = true;
        self.dungeon_id = id;
        let (ex, ey) = dungeon_entry(id);
        self.load_screen(ex, ey);
    }

    pub fn exit_dungeon(&mut self, x: i32, y: i32) {
        self.in_dungeon = false;
        self.dungeon_id = 0;
        self.load_screen(x, y);
    }

    pub fn enter_interior(&mut self, id: &str) {
        self.in_dungeon = false;
        self.dungeon_id = 0;
        self.in_interior = true;
        self.interior_id = id.to_string();
        self.load_screen(self.screen_x, self.screen_y);
    }

    pub fn exit_interior(&mut self) {
        self.in_interior = false;
        self.interior_id.clear();
        self.load_screen(self.screen_x, self.screen_y);
    }

    pub fn get_enemy_spawns(&self) -> Vec<EnemySpawn> {
        let cleared_key = self.cleared_room_key();
        enemy_spawns(
            self.screen_x,
            self.screen_y,
            self.in_dungeon,
            self.dungeon_id,
            self.in_interior,
            &self.interior_id,
            self.cleared_rooms.contains(&cleared_key),
        )
    }

    pub fn get_screen_items(&self) -> Vec<ItemDef> {
        screen_items(
            self.screen_x,
            self.screen_y,
            self.in_dungeon,
            self.dungeon_id,
            self.in_interior,
            &self.interior_id,
        )
    }

    pub fn screen_exists(&self, x: i32, y: i32) -> bool {
        let key = screen_key(x, y);
        if self.in_interior {
            x == self.screen_x
                && y == self.screen_y
                && self.interior_data.contains_key(&self.interior_id)
        } else if self.in_dungeon {
            self.dungeon_data
                .get(&self.dungeon_id)
                .is_some_and(|d| d.contains_key(&key))
        } else {
            self.overworld_data.contains_key(&key)
        }
    }

    pub fn screen_tiles(&self, x: i32, y: i32) -> Option<&TileGrid> {
        let key = screen_key(x, y);
        if self.in_interior {
            if x == self.screen_x && y == self.screen_y {
                self.interior_data.get(&self.interior_id)
            } else {
                None
            }
        } else if self.in_dungeon {
            self.dungeon_data
                .get(&self.dungeon_id)
                .and_then(|d| d.get(&key))
        } else {
            self.overworld_data.get(&key)
        }
    }

    /// Returns all room keys for the current dungeon (for minimap).
    pub fn dungeon_rooms(&self) -> HashSet<String> {
        if !self.in_dungeon || self.in_interior {
            return HashSet::new();
        }
        dungeon_map_rooms(self.dungeon_id)
    }

    /// Key used for the cleared-rooms set.
    pub fn cleared_room_key(&self) -> String {
        let key = screen_key(self.screen_x, self.screen_y);
        if self.in_interior {
            format!("i:{}:{key}", self.interior_id)
        } else if self.in_dungeon {
            format!("d{}:{key}", self.dungeon_id)
        } else {
            format!("o:{key}")
        }
    }

    /// Key used for persisted tile changes (destroyed tiles, opened chests).
    fn persist_key(&self, screen: &str) -> String {
        if self.in_interior {
            format!("i:{}:{screen}", self.interior_id)
        } else if self.in_dungeon {
            format!("d{}:{screen}", self.dungeon_id)
        } else {
            format!("o:{screen}")
        }
    }

    /// Key used for the visited set (to distinguish dungeon visits).
    fn visit_key(&self, screen: &str) -> String {
        if self.in_interior {
            format!("i:{}:{screen}", self.interior_id)
        } else if self.in_dungeon {
            format!("d{}:{screen}", self.dungeon_id)
        } else {
            screen.to_string()
        }
    }
}
