use crate::constants::{PIXEL_SCALE, TILE, attack_duration, player_full_hearts_hp, player_max_hp};
use macroquad::prelude::Rect;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type TileGrid = Vec<Vec<TileType>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Title,
    CharacterCreate,
    Playing,
    Inventory,
    Transition,
    DungeonEnter,
    DungeonExit,
    Message,
    GameOver,
    Victory,
    PauseMenu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    HouseRoof,
    HouseRoofLeft,
    HouseRoofRight,
    HouseWall,
    HouseWindow,
    HouseDoor,
    WoodFloor,
    HouseChair,
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
    Gem,
    Ladder,
    Hammer,
    Raft,
    StrongArmGlove,
    PortalTool,
    DragonPiece,
    Sword,
    TideChart,
    EmberCrystal,
    VoidCompass,
    CrystalOfSeeing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NpcKind {
    Elara,
    Barnett,
    Maren,
    Oswin,
    Corvin,
    Petra,
    Aldric,
    Sael,
    Dax,
    Vel,
    CelestialMerchant,
    Senna,
    Wren,
    GnomeHealer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropKind {
    Boulder,
    PressurePlate,
    LadderPoint,
    Npc(NpcKind),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerState {
    Idle,
    Walking,
    Attacking,
    Hurt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquippedItem {
    None,
    Sword,
    Bombs,
    Hammer,
}

impl EquippedItem {
    pub fn label(self) -> &'static str {
        match self {
            EquippedItem::None => "None",
            EquippedItem::Sword => "Sword",
            EquippedItem::Bombs => "Bombs",
            EquippedItem::Hammer => "Hammer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemSlot {
    Main,
    Side,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryItem {
    Sword,
    Bombs,
    Keys,
    BossKey(i32), // dungeon_id
    Ladder,
    Hammer,
    Raft,
    Lantern,
    StrongArmGlove,
    PortalTool,
    AncientKey,
    TideChart,
    EmberCrystal,
    VoidCompass,
    StarSigil,
    DragonCodex,
    CrystalOfSeeing,
    Gems,
    DragonPieces,
}

impl InventoryItem {
    pub fn equipped_item(self) -> Option<EquippedItem> {
        match self {
            InventoryItem::Sword => Some(EquippedItem::Sword),
            InventoryItem::Bombs => Some(EquippedItem::Bombs),
            InventoryItem::Hammer => Some(EquippedItem::Hammer),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InventoryEntry {
    pub item: InventoryItem,
    pub label: &'static str,
    pub description: &'static str,
    pub owned: bool,
    pub count: Option<i32>,
    pub equipable: bool,
}

// BossKey is excluded — entries are appended dynamically per dungeon in inventory_entries().
pub const INVENTORY_ITEMS: [InventoryItem; 18] = [
    InventoryItem::Sword,
    InventoryItem::Bombs,
    InventoryItem::Keys,
    InventoryItem::Ladder,
    InventoryItem::Hammer,
    InventoryItem::Raft,
    InventoryItem::Lantern,
    InventoryItem::StrongArmGlove,
    InventoryItem::PortalTool,
    InventoryItem::AncientKey,
    InventoryItem::TideChart,
    InventoryItem::EmberCrystal,
    InventoryItem::VoidCompass,
    InventoryItem::StarSigil,
    InventoryItem::DragonCodex,
    InventoryItem::CrystalOfSeeing,
    InventoryItem::Gems,
    InventoryItem::DragonPieces,
];

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

#[derive(Clone, Serialize, Deserialize)]
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
    #[serde(default)]
    pub boss_keys: HashSet<i32>,
    pub keys: i32,
    pub gems: i32,
    pub dragon_pieces: i32,
    pub bomb_count: i32,
    pub max_bombs: i32,
    pub has_ancient_key: bool,
    pub has_tide_chart: bool,
    pub has_ember_crystal: bool,
    pub has_void_compass: bool,
    pub has_star_sigil: bool,
    pub has_dragon_codex: bool,
    pub has_crystal_of_seeing: bool,
    pub has_ladder: bool,
    pub has_hammer: bool,
    pub has_raft: bool,
    #[serde(default)]
    pub has_lantern: bool,
    pub has_strong_arm_glove: bool,
    pub has_portal_tool: bool,
    pub main_item: EquippedItem,
    pub side_item: EquippedItem,
    pub walk_frame: i32,
    pub walk_timer: i32,
    pub last_axis: Option<char>,
}

fn boss_key_label(id: i32) -> &'static str {
    match id {
        1 => "Boss Key 1",
        2 => "Boss Key 2",
        3 => "Boss Key 3",
        4 => "Boss Key 4",
        5 => "Boss Key 5",
        6 => "Boss Key 6",
        7 => "Boss Key 7",
        8 => "Boss Key 8",
        _ => "Boss Key ?",
    }
}

fn boss_key_description(id: i32) -> &'static str {
    match id {
        1 => "Boss Key for Dungeon 1. Unlocks the boss door in Mosshaven Cave.",
        2 => "Boss Key for Dungeon 2. Unlocks the boss door in Ruins of Ashenfall.",
        3 => "Boss Key for Dungeon 3. Unlocks the boss door in Ironclad Vault.",
        4 => "Boss Key for Dungeon 4. Unlocks the boss door in Sunken Citadel.",
        5 => "Boss Key for Dungeon 5. Unlocks the boss door in Grimforge Depths.",
        6 => "Boss Key for Dungeon 6. Unlocks the boss door in Fractured Sanctum.",
        7 => "Boss Key for Dungeon 7. Unlocks the boss door in Aetherian Spire.",
        8 => "Boss Key for Dungeon 8. Unlocks the boss door in Dragon's Eternal Throne.",
        _ => "Boss Key for an unknown dungeon.",
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 7.0 * TILE,
            y: 5.0 * TILE,
            hp: player_max_hp(),
            max_hp: player_max_hp(),
            dir: Dir::Down,
            state: PlayerState::Idle,
            attack_timer: 0,
            invuln_timer: 0,
            hurt_timer: 0,
            knock_dx: 0.0,
            knock_dy: 0.0,
            has_sword: false,
            has_bombs: false,
            boss_keys: HashSet::new(),
            keys: 0,
            gems: 0,
            dragon_pieces: 0,
            bomb_count: 0,
            max_bombs: 8,
            has_ancient_key: false,
            has_tide_chart: false,
            has_ember_crystal: false,
            has_void_compass: false,
            has_star_sigil: false,
            has_dragon_codex: false,
            has_crystal_of_seeing: false,
            has_ladder: false,
            has_hammer: false,
            has_raft: false,
            has_lantern: false,
            has_strong_arm_glove: false,
            has_portal_tool: false,
            main_item: EquippedItem::None,
            side_item: EquippedItem::None,
            walk_frame: 0,
            walk_timer: 0,
            last_axis: None,
        }
    }

    pub fn grant_all_items(&mut self) {
        self.has_sword = true;
        self.has_bombs = true;
        self.boss_keys = (1..=10).collect();
        self.keys = 9999;
        self.gems = 9999;
        self.dragon_pieces = 7;
        self.bomb_count = self.max_bombs;
        self.has_ancient_key = true;
        self.has_tide_chart = true;
        self.has_ember_crystal = true;
        self.has_void_compass = true;
        self.has_star_sigil = true;
        self.has_dragon_codex = true;
        self.has_crystal_of_seeing = true;
        self.has_ladder = true;
        self.has_hammer = true;
        self.has_raft = true;
        self.has_lantern = true;
        self.has_strong_arm_glove = true;
        self.has_portal_tool = true;
        self.main_item = EquippedItem::Sword;
        self.side_item = EquippedItem::Bombs;
    }

    pub fn grant_full_hearts(&mut self) {
        self.max_hp = player_full_hearts_hp();
        self.hp = self.max_hp;
    }

    pub fn has_boss_key_for(&self, dungeon_id: i32) -> bool {
        self.boss_keys.contains(&dungeon_id)
    }

    pub fn grant_boss_key_for(&mut self, dungeon_id: i32) {
        self.boss_keys.insert(dungeon_id);
    }

    pub fn inventory_entries(&self, dungeon_ids: &[i32]) -> Vec<InventoryEntry> {
        let mut entries: Vec<InventoryEntry> = INVENTORY_ITEMS
            .iter()
            .copied()
            .map(|item| self.inventory_entry(item))
            .collect();
        // Append one boss key entry per dungeon, in order.
        for &id in dungeon_ids {
            entries.push(self.inventory_entry(InventoryItem::BossKey(id)));
        }
        entries
    }

    pub fn inventory_entry(&self, item: InventoryItem) -> InventoryEntry {
        match item {
            InventoryItem::Sword => InventoryEntry {
                item,
                label: "Sword",
                description: "Assign to MAIN or SIDE. Swings instantly and interrupts movement.",
                owned: self.has_sword,
                count: None,
                equipable: true,
            },
            InventoryItem::Bombs => InventoryEntry {
                item,
                label: "Bombs",
                description: "Assign to MAIN or SIDE. Explodes cracked walls and damages clustered foes.",
                owned: self.has_bombs,
                count: Some(self.bomb_count),
                equipable: true,
            },
            InventoryItem::Keys => InventoryEntry {
                item,
                label: "Keys",
                description: "Dungeon keys for locked doors on the current floor.",
                owned: self.keys > 0,
                count: Some(self.keys),
                equipable: false,
            },
            InventoryItem::BossKey(id) => InventoryEntry {
                item,
                label: boss_key_label(id),
                description: boss_key_description(id),
                owned: self.has_boss_key_for(id),
                count: None,
                equipable: false,
            },
            InventoryItem::Ladder => InventoryEntry {
                item,
                label: "Ladder",
                description: "Use at ladder markers to climb between authored height changes.",
                owned: self.has_ladder,
                count: None,
                equipable: false,
            },
            InventoryItem::Hammer => InventoryEntry {
                item,
                label: "Hammer",
                description: "Assign to MAIN or SIDE. Smashes cracked tiles directly in front of you.",
                owned: self.has_hammer,
                count: None,
                equipable: true,
            },
            InventoryItem::Raft => InventoryEntry {
                item,
                label: "Raft",
                description: "Lets you travel safely across water tiles.",
                owned: self.has_raft,
                count: None,
                equipable: false,
            },
            InventoryItem::Lantern => InventoryEntry {
                item,
                label: "Lantern",
                description: "Casts a warm circle of light around you at night.",
                owned: self.has_lantern,
                count: None,
                equipable: false,
            },
            InventoryItem::StrongArmGlove => InventoryEntry {
                item,
                label: "Strong Glove",
                description: "Required for the heaviest forge and summit mechanisms.",
                owned: self.has_strong_arm_glove,
                count: None,
                equipable: false,
            },
            InventoryItem::PortalTool => InventoryEntry {
                item,
                label: "Portal Tool",
                description: "An attunement focus used to access later rift structures.",
                owned: self.has_portal_tool,
                count: None,
                equipable: false,
            },
            InventoryItem::AncientKey => InventoryEntry {
                item,
                label: "Ancient Key",
                description: "Opens the Iron Highlands vault approach.",
                owned: self.has_ancient_key,
                count: None,
                equipable: false,
            },
            InventoryItem::TideChart => InventoryEntry {
                item,
                label: "Tide Chart",
                description: "Marks the safe timing for the Sunken Coast routes.",
                owned: self.has_tide_chart,
                count: None,
                equipable: false,
            },
            InventoryItem::EmberCrystal => InventoryEntry {
                item,
                label: "Ember Crystal",
                description: "A heat ward needed to enter the Grimforge depths.",
                owned: self.has_ember_crystal,
                count: None,
                equipable: false,
            },
            InventoryItem::VoidCompass => InventoryEntry {
                item,
                label: "Void Compass",
                description: "Stabilizes your route through the fractured sanctum.",
                owned: self.has_void_compass,
                count: None,
                equipable: false,
            },
            InventoryItem::StarSigil => InventoryEntry {
                item,
                label: "Star Sigil",
                description: "The merchant's seal required for the Aetherian ascent.",
                owned: self.has_star_sigil,
                count: None,
                equipable: false,
            },
            InventoryItem::DragonCodex => InventoryEntry {
                item,
                label: "Dragon Codex",
                description: "Ancient lore needed to face the final approach.",
                owned: self.has_dragon_codex,
                count: None,
                equipable: false,
            },
            InventoryItem::CrystalOfSeeing => InventoryEntry {
                item,
                label: "Crystal of Seeing",
                description: "Reveals the path hidden in the last ascent.",
                owned: self.has_crystal_of_seeing,
                count: None,
                equipable: false,
            },
            InventoryItem::Gems => InventoryEntry {
                item,
                label: "Gems",
                description: "Currency used by merchants across the overworld.",
                owned: true,
                count: Some(self.gems),
                equipable: false,
            },
            InventoryItem::DragonPieces => InventoryEntry {
                item,
                label: "Dragon Pieces",
                description: "Collect all seven to unlock the final confrontation.",
                owned: self.dragon_pieces > 0,
                count: Some(self.dragon_pieces),
                equipable: false,
            },
        }
    }

    pub fn begin_attack(&mut self) {
        self.state = PlayerState::Attacking;
        self.attack_timer = attack_duration();
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
    pub key: Option<String>,
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
pub struct WorldProp {
    pub kind: PropKind,
    pub tile_x: i32,
    pub tile_y: i32,
    pub target_tile_x: Option<i32>,
    pub target_tile_y: Option<i32>,
}

#[derive(Clone)]
pub struct DeathAnimation {
    pub x: f32,
    pub y: f32,
    pub timer: i32,
}

#[derive(Clone)]
pub struct Gnome {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub move_timer: i32,
    pub caught: bool,
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
    pub in_interior: bool,
    pub interior_id: String,
    pub tiles: TileGrid,
    pub visited: HashSet<String>,
    pub cleared_rooms: HashSet<String>,
    pub opened_chests: HashMap<String, Vec<(usize, usize)>>,
    pub destroyed_tiles: HashMap<String, Vec<(usize, usize, TileType)>>,
    pub dungeon_rooms: HashSet<String>,
    pub dev_mode: bool,
    pub time_minutes: i32,
    pub day_number: i32,
}
