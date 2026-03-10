use crate::constants::{ATTACK_DURATION, PIXEL_SCALE, PLAYER_MAX_HP, TILE};
use macroquad::prelude::Rect;
use std::collections::{HashMap, HashSet};

pub type TileGrid = Vec<Vec<TileType>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Title,
    Playing,
    Inventory,
    Transition,
    DungeonEnter,
    DungeonExit,
    Message,
    GameOver,
    Victory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileType {
    Grass,
    Tree,
    Water,
    Rock,
    Sand,
    Path,
    Cave,
    Dungeon,
    Cracked,
    Bush,
    Bridge,
    Wall,
    Floor,
    DoorLocked,
    Door,
    Stairs,
    Chest,
    Goal,
    BossDoor,
    FloorAlt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyType {
    Slime,
    Octorok,
    Bat,
    Darknut,
    Boss,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickupType {
    Heart,
    HeartContainer,
    Key,
    BossKey,
    BombAmmo,
    Bombs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Walking,
    Attacking,
    Hurt,
}

#[derive(Clone)]
pub struct Transition {
    pub dir: Option<Dir>,
    pub progress: f32,
    pub old_tiles: TileGrid,
    pub new_screen_x: i32,
    pub new_screen_y: i32,
    pub player_new_x: f32,
    pub player_new_y: f32,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            dir: None,
            progress: 0.0,
            old_tiles: vec![],
            new_screen_x: 0,
            new_screen_y: 0,
            player_new_x: 0.0,
            player_new_y: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub dir: Dir,
    pub state: PlayerState,
    pub attack_timer: i32,
    pub invuln_timer: i32,
    pub hurt_timer: i32,
    pub knock_dx: f32,
    pub knock_dy: f32,
    pub has_sword: bool,
    pub has_bombs: bool,
    pub has_boss_key: bool,
    pub keys: i32,
    pub bomb_count: i32,
    pub max_bombs: i32,
    pub walk_frame: i32,
    pub walk_timer: i32,
    pub last_axis: Option<char>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 7.0 * TILE,
            y: 5.0 * TILE,
            hp: PLAYER_MAX_HP,
            max_hp: PLAYER_MAX_HP,
            dir: Dir::Down,
            state: PlayerState::Idle,
            attack_timer: 0,
            invuln_timer: 0,
            hurt_timer: 0,
            knock_dx: 0.0,
            knock_dy: 0.0,
            has_sword: false,
            has_bombs: false,
            has_boss_key: false,
            keys: 0,
            bomb_count: 0,
            max_bombs: 8,
            walk_frame: 0,
            walk_timer: 0,
            last_axis: None,
        }
    }

    pub fn begin_attack(&mut self) {
        self.state = PlayerState::Attacking;
        self.attack_timer = ATTACK_DURATION;
    }

    pub fn hitbox(&self) -> Rect {
        Rect::new(
            self.x + 2.0 * PIXEL_SCALE,
            self.y + 4.0 * PIXEL_SCALE,
            12.0 * PIXEL_SCALE,
            12.0 * PIXEL_SCALE,
        )
    }
}

#[derive(Clone)]
pub struct EnemySpawn {
    pub enemy_type: EnemyType,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub speed: f32,
    pub dir: Dir,
    pub move_timer: i32,
    pub hurt_timer: i32,
    pub knock_x: f32,
    pub knock_y: f32,
    pub flash_timer: i32,
    pub shoot_cooldown: i32,
    pub active: bool,
    pub timer: i32,
    pub vx: f32,
    pub vy: f32,
}

#[derive(Clone)]
pub struct Pickup {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub pickup_type: PickupType,
    pub timer: i32,
    pub collected: bool,
}

#[derive(Clone)]
pub struct Bomb {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub timer: i32,
    pub exploded: bool,
    pub explosion_timer: i32,
}

#[derive(Clone)]
pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub dx: f32,
    pub dy: f32,
    pub from_enemy: bool,
    pub active: bool,
    pub timer: i32,
}

#[derive(Clone)]
pub struct DeathAnimation {
    pub x: f32,
    pub y: f32,
    pub timer: i32,
}

#[derive(Clone)]
pub struct ItemDef {
    pub pickup_type: PickupType,
    pub tile_x: usize,
    pub tile_y: usize,
}

#[derive(Clone)]
pub struct WorldSnapshot {
    pub screen_x: i32,
    pub screen_y: i32,
    pub in_dungeon: bool,
    pub dungeon_id: i32,
    pub tiles: TileGrid,
    pub visited: HashSet<String>,
    pub cleared_rooms: HashSet<String>,
    pub opened_chests: HashMap<String, Vec<(usize, usize)>>,
    pub destroyed_tiles: HashMap<String, Vec<(usize, usize, TileType)>>,
    pub dungeon_rooms: HashSet<String>,
    pub dev_mode: bool,
}
