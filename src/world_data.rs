use crate::constants::{COLS, ROWS, TILE, WORLD_H, WORLD_W};
use crate::generated_overworld::{
    GENERATED_OVERWORLD_LAYOUT, GENERATED_OVERWORLD_SCREENS, GeneratedOverworldScreen,
};
use crate::model::{
    EnemySpawn, EnemyType, ItemDef, NpcKind, PickupType, PropKind, TileGrid, TileType, WorldProp,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OverworldBiome {
    Plains,
    Forest,
    DeepForest,
    Highlands,
    Mountain,
    Lake,
    Desert,
    Ruins,
    Coast,
    Snow,
    Canyon,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OverworldLandmark {
    None,
    StartVillage,
    SwordCave,
    HeartCave,
    IslandShrine,
    EastSanctum,
    BombCave,
    Shop,
    Dungeon(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaveKind {
    Sword,
    Heart,
    Shrine,
    Sanctum,
    Bombs,
    Shop,
    AncientKey,
    TideChart,
    EmberCrystal,
    VoidCompass,
    StarSigil,
    DragonCodex,
    CrystalOfSeeing,
}

#[derive(Clone, Copy)]
struct SpawnDef {
    enemy_type: EnemyType,
    tile_x: f32,
    tile_y: f32,
}

#[derive(Clone, Copy)]
struct ScreenItemDef {
    pickup_type: PickupType,
    tile_x: usize,
    tile_y: usize,
}

#[derive(Clone, Copy)]
struct PropDef {
    kind: PropKind,
    tile_x: i32,
    tile_y: i32,
    target_tile_x: Option<i32>,
    target_tile_y: Option<i32>,
}

#[derive(Clone, Copy)]
struct OverworldScreenDef {
    x: i32,
    y: i32,
    biome_id: i32,
    name: &'static str,
    tiles: &'static [&'static str],
    cave: Option<CaveKind>,
    dungeon: Option<i32>,
    enemies: &'static [SpawnDef],
}

#[derive(Clone, Copy)]
struct DungeonRoomDef {
    x: i32,
    y: i32,
    tiles: &'static [&'static str],
    enemies: &'static [SpawnDef],
    items: &'static [ScreenItemDef],
    props: &'static [PropDef],
    map_visible: bool,
    room_clear_rewards: &'static [ScreenItemDef],
    boss_key_tile: Option<(usize, usize)>,
}

#[derive(Clone, Copy)]
struct DungeonDef {
    id: i32,
    name: &'static str,
    entry: (i32, i32),
    rooms: &'static [DungeonRoomDef],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InteriorStyle {
    Cave,
    House,
}

#[derive(Clone, Copy)]
struct InteriorDef {
    id: &'static str,
    source_x: i32,
    source_y: i32,
    entrance_tile_x: i32,
    entrance_tile_y: i32,
    name: &'static str,
    style: InteriorStyle,
    tiles: &'static [&'static str],
    props: &'static [PropDef],
    reward_pickup: Option<PickupType>,
}

#[derive(Clone, Copy)]
struct OverworldSpec {
    biome: OverworldBiome,
    landmark: OverworldLandmark,
    name: &'static str,
}

fn spawn_defs_to_enemies(spawns: &[SpawnDef]) -> Vec<EnemySpawn> {
    spawns
        .iter()
        .map(|spawn| EnemySpawn {
            enemy_type: spawn.enemy_type,
            x: spawn.tile_x * TILE,
            y: spawn.tile_y * TILE,
        })
        .collect()
}

fn item_defs(items: &[ScreenItemDef]) -> Vec<ItemDef> {
    items
        .iter()
        .map(|item| ItemDef {
            pickup_type: item.pickup_type,
            tile_x: item.tile_x,
            tile_y: item.tile_y,
        })
        .collect()
}

fn prop_defs(props: &[PropDef]) -> Vec<WorldProp> {
    props
        .iter()
        .map(|prop| WorldProp {
            kind: prop.kind,
            tile_x: prop.tile_x,
            tile_y: prop.tile_y,
            target_tile_x: prop.target_tile_x,
            target_tile_y: prop.target_tile_y,
        })
        .collect()
}

fn item(pickup_type: PickupType, tile_x: usize, tile_y: usize) -> ItemDef {
    ItemDef {
        pickup_type,
        tile_x,
        tile_y,
    }
}

fn npc(kind: NpcKind, tile_x: i32, tile_y: i32) -> WorldProp {
    WorldProp {
        kind: PropKind::Npc(kind),
        tile_x,
        tile_y,
        target_tile_x: None,
        target_tile_y: None,
    }
}

const HOUSE_INTERIOR_TILES: &[&str] = &[
    "hhhhhhhhhhhhhhhh",
    "hwwuuuuuuuuuuwwh",
    "hwuuuuuuuuuuuuwh",
    "hwuuruuuuuuruuwh",
    "hwuuuuuuuuuuuuwh",
    "hwuuuuuuuuuuuuwh",
    "hwuuuuuuuuuuuuwh",
    "hwuuruuuuuuruuwh",
    "hwuuuuuuuuuuuuwh",
    "hhhhhhhqhhhhhhhh",
    "hhhhhhhhhhhhhhhh",
];

const CAVE_INTERIOR_TILES: &[&str] = &[
    "################",
    "###ffffxxffff###",
    "##ff        ff##",
    "##f   x  x   f##",
    "##f          f##",
    "##f  ff  ff  f##",
    "##f          f##",
    "##f   xxxx   f##",
    "##ff        ff##",
    "#######oo#######",
    "################",
];

const ELARA_HOUSE_PROPS: &[PropDef] = &[PropDef {
    kind: PropKind::Npc(NpcKind::Elara),
    tile_x: 8,
    tile_y: 5,
    target_tile_x: None,
    target_tile_y: None,
}];

const MAREN_HOUSE_PROPS: &[PropDef] = &[PropDef {
    kind: PropKind::Npc(NpcKind::Maren),
    tile_x: 8,
    tile_y: 5,
    target_tile_x: None,
    target_tile_y: None,
}];

const CORVIN_HOUSE_PROPS: &[PropDef] = &[PropDef {
    kind: PropKind::Npc(NpcKind::Corvin),
    tile_x: 8,
    tile_y: 5,
    target_tile_x: None,
    target_tile_y: None,
}];

const ALDRIC_HOUSE_PROPS: &[PropDef] = &[PropDef {
    kind: PropKind::Npc(NpcKind::Aldric),
    tile_x: 8,
    tile_y: 5,
    target_tile_x: None,
    target_tile_y: None,
}];

const SAEL_HOUSE_PROPS: &[PropDef] = &[PropDef {
    kind: PropKind::Npc(NpcKind::Sael),
    tile_x: 8,
    tile_y: 5,
    target_tile_x: None,
    target_tile_y: None,
}];

const DAX_HOUSE_PROPS: &[PropDef] = &[PropDef {
    kind: PropKind::Npc(NpcKind::Dax),
    tile_x: 8,
    tile_y: 5,
    target_tile_x: None,
    target_tile_y: None,
}];

const CELESTIAL_MERCHANT_HOUSE_PROPS: &[PropDef] = &[PropDef {
    kind: PropKind::Npc(NpcKind::CelestialMerchant),
    tile_x: 8,
    tile_y: 5,
    target_tile_x: None,
    target_tile_y: None,
}];

const WREN_HOUSE_PROPS: &[PropDef] = &[PropDef {
    kind: PropKind::Npc(NpcKind::Wren),
    tile_x: 8,
    tile_y: 5,
    target_tile_x: None,
    target_tile_y: None,
}];

const INTERIORS: &[InteriorDef] = &[
    InteriorDef {
        id: "elara_house",
        source_x: 2,
        source_y: 11,
        entrance_tile_x: 5,
        entrance_tile_y: 6,
        name: "ELARA'S TONIC HOUSE",
        style: InteriorStyle::House,
        tiles: HOUSE_INTERIOR_TILES,
        props: ELARA_HOUSE_PROPS,
        reward_pickup: None,
    },
    InteriorDef {
        id: "maren_house",
        source_x: 5,
        source_y: 12,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "MAREN'S SALVAGE SHACK",
        style: InteriorStyle::House,
        tiles: HOUSE_INTERIOR_TILES,
        props: MAREN_HOUSE_PROPS,
        reward_pickup: None,
    },
    InteriorDef {
        id: "corvin_house",
        source_x: 2,
        source_y: 7,
        entrance_tile_x: 5,
        entrance_tile_y: 6,
        name: "CORVIN'S TOWER ROOM",
        style: InteriorStyle::House,
        tiles: HOUSE_INTERIOR_TILES,
        props: CORVIN_HOUSE_PROPS,
        reward_pickup: None,
    },
    InteriorDef {
        id: "aldric_house",
        source_x: 9,
        source_y: 9,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "ALDRIC'S DOCK HOUSE",
        style: InteriorStyle::House,
        tiles: HOUSE_INTERIOR_TILES,
        props: ALDRIC_HOUSE_PROPS,
        reward_pickup: None,
    },
    InteriorDef {
        id: "sael_house",
        source_x: 12,
        source_y: 9,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "SAEL'S SEA STORE",
        style: InteriorStyle::House,
        tiles: HOUSE_INTERIOR_TILES,
        props: SAEL_HOUSE_PROPS,
        reward_pickup: None,
    },
    InteriorDef {
        id: "dax_house",
        source_x: 6,
        source_y: 7,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "DAX'S FORGE HUT",
        style: InteriorStyle::House,
        tiles: HOUSE_INTERIOR_TILES,
        props: DAX_HOUSE_PROPS,
        reward_pickup: None,
    },
    InteriorDef {
        id: "merchant_lantern_house",
        source_x: 8,
        source_y: 0,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "CELESTIAL MERCHANT'S HOUSE",
        style: InteriorStyle::House,
        tiles: HOUSE_INTERIOR_TILES,
        props: CELESTIAL_MERCHANT_HOUSE_PROPS,
        reward_pickup: None,
    },
    InteriorDef {
        id: "wren_house",
        source_x: 4,
        source_y: 2,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "WREN'S LAST CAMP",
        style: InteriorStyle::House,
        tiles: HOUSE_INTERIOR_TILES,
        props: WREN_HOUSE_PROPS,
        reward_pickup: None,
    },
    InteriorDef {
        id: "sword_cave",
        source_x: 2,
        source_y: 11,
        entrance_tile_x: 10,
        entrance_tile_y: 5,
        name: "GRANDFATHER TREE CAVE",
        style: InteriorStyle::Cave,
        tiles: CAVE_INTERIOR_TILES,
        props: &[],
        reward_pickup: Some(PickupType::Sword),
    },
    InteriorDef {
        id: "threshold_stone_cave",
        source_x: 4,
        source_y: 1,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "THRESHOLD STONE CAVE",
        style: InteriorStyle::Cave,
        tiles: CAVE_INTERIOR_TILES,
        props: &[],
        reward_pickup: Some(PickupType::CrystalOfSeeing),
    },
    InteriorDef {
        id: "rift_cave",
        source_x: 5,
        source_y: 3,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "STABLE RIFT CAVE",
        style: InteriorStyle::Cave,
        tiles: CAVE_INTERIOR_TILES,
        props: &[],
        reward_pickup: Some(PickupType::VoidCompass),
    },
    InteriorDef {
        id: "ember_cave",
        source_x: 7,
        source_y: 6,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "EMBER GARDEN CAVE",
        style: InteriorStyle::Cave,
        tiles: CAVE_INTERIOR_TILES,
        props: &[],
        reward_pickup: Some(PickupType::EmberCrystal),
    },
    InteriorDef {
        id: "lighthouse_cave",
        source_x: 9,
        source_y: 6,
        entrance_tile_x: 8,
        entrance_tile_y: 5,
        name: "LIGHTHOUSE ISLE CAVE",
        style: InteriorStyle::Cave,
        tiles: CAVE_INTERIOR_TILES,
        props: &[],
        reward_pickup: Some(PickupType::TideChart),
    },
];

pub fn screen_key(x: i32, y: i32) -> String {
    format!("{x},{y}")
}

pub fn empty_tiles() -> TileGrid {
    vec![vec![TileType::Grass; COLS]; ROWS]
}

pub fn char_to_tile(ch: char) -> TileType {
    match ch {
        '.' => TileType::Grass,
        'T' => TileType::Tree,
        '~' => TileType::Water,
        '^' => TileType::Rock,
        ',' => TileType::Sand,
        '=' => TileType::Path,
        'c' => TileType::Cave,
        'd' => TileType::Dungeon,
        'x' => TileType::Cracked,
        'b' => TileType::Bush,
        '_' => TileType::Bridge,
        '#' => TileType::Wall,
        ' ' => TileType::Floor,
        'l' => TileType::DoorLocked,
        'o' => TileType::Door,
        's' => TileType::Stairs,
        '$' => TileType::Chest,
        'G' => TileType::Goal,
        'k' => TileType::BossDoor,
        'f' => TileType::FloorAlt,
        'h' => TileType::HouseWall,
        'w' => TileType::HouseWindow,
        'q' => TileType::HouseDoor,
        'u' => TileType::WoodFloor,
        'r' => TileType::HouseChair,
        _ => TileType::Grass,
    }
}

pub fn parse(rows: &[&str]) -> TileGrid {
    rows.iter()
        .map(|row| {
            row.chars()
                .take(COLS)
                .map(char_to_tile)
                .chain(std::iter::repeat(TileType::Grass))
                .take(COLS)
                .collect()
        })
        .take(ROWS)
        .chain(std::iter::repeat(vec![TileType::Grass; COLS]))
        .take(ROWS)
        .collect()
}

fn interiors_on_screen(screen_x: i32, screen_y: i32) -> impl Iterator<Item = &'static InteriorDef> {
    INTERIORS
        .iter()
        .filter(move |interior| interior.source_x == screen_x && interior.source_y == screen_y)
}

pub fn interior_entrance_at(
    screen_x: i32,
    screen_y: i32,
    tile_x: i32,
    tile_y: i32,
) -> Option<&'static str> {
    interiors_on_screen(screen_x, screen_y)
        .find(|interior| interior.entrance_tile_x == tile_x && interior.entrance_tile_y == tile_y)
        .map(|interior| interior.id)
}

fn interior_by_id(id: &str) -> Option<&'static InteriorDef> {
    INTERIORS.iter().find(|interior| interior.id == id)
}

pub fn interior_exit_overworld_tile(id: &str) -> Option<(i32, i32)> {
    interior_by_id(id).map(|interior| {
        (
            interior.entrance_tile_x,
            (interior.entrance_tile_y + 1).min(ROWS as i32 - 2),
        )
    })
}

pub fn interior_spawn_tile(id: &str) -> (i32, i32) {
    match interior_by_id(id).map(|interior| interior.style) {
        Some(InteriorStyle::Cave) => (7, 8),
        _ => (7, 7),
    }
}

pub fn interior_reward_pickup(id: &str) -> Option<PickupType> {
    interior_by_id(id).and_then(|interior| interior.reward_pickup)
}

pub fn build_interiors() -> HashMap<String, TileGrid> {
    INTERIORS
        .iter()
        .map(|interior| (interior.id.to_string(), parse(interior.tiles)))
        .collect()
}

fn overlay_house_entrance(tiles: &mut TileGrid, door_x: i32, door_y: i32) {
    let door_col = door_x.clamp(2, COLS as i32 - 3) as usize;
    let door_row = door_y.clamp(4, ROWS as i32 - 2) as usize;
    let left = door_col.saturating_sub(3).max(1);
    let right = (door_col + 3).min(COLS - 2);
    let top = door_row.saturating_sub(4).max(1);

    tiles[top][left] = TileType::HouseRoofLeft;
    tiles[top][right] = TileType::HouseRoofRight;
    for col in (left + 1)..right {
        tiles[top][col] = TileType::HouseRoof;
    }

    tiles[top + 1][left] = TileType::HouseRoofLeft;
    tiles[top + 1][right] = TileType::HouseRoofRight;
    for col in (left + 1)..right {
        tiles[top + 1][col] = TileType::HouseRoof;
    }

    for col in left..=right {
        tiles[top + 2][col] = TileType::HouseWall;
        tiles[top + 3][col] = TileType::HouseWall;
        tiles[door_row][col] = TileType::HouseWall;
    }

    tiles[top + 2][left + 1] = TileType::HouseWindow;
    tiles[top + 2][right - 1] = TileType::HouseWindow;
    tiles[top + 3][left + 1] = TileType::HouseWindow;
    tiles[top + 3][right - 1] = TileType::HouseWindow;
    tiles[door_row][door_col] = TileType::HouseDoor;
    for row in (door_row + 1)..ROWS {
        tiles[row][door_col] = TileType::Path;
    }
}

fn overlay_cave_entrance(tiles: &mut TileGrid, cave_x: i32, cave_y: i32) {
    let cave_col = cave_x.clamp(1, COLS as i32 - 2) as usize;
    let cave_row = cave_y.clamp(1, ROWS as i32 - 2) as usize;
    for row in cave_row.saturating_sub(1)..=(cave_row + 1).min(ROWS - 2) {
        for col in cave_col.saturating_sub(1)..=(cave_col + 1).min(COLS - 2) {
            tiles[row][col] = TileType::Path;
        }
    }
    tiles[cave_row][cave_col] = TileType::Cave;
}

fn overlay_interior_entrances(tiles: &mut TileGrid, screen_x: i32, screen_y: i32) {
    for interior in interiors_on_screen(screen_x, screen_y) {
        match interior.style {
            InteriorStyle::House => {
                overlay_house_entrance(tiles, interior.entrance_tile_x, interior.entrance_tile_y)
            }
            InteriorStyle::Cave => {
                overlay_cave_entrance(tiles, interior.entrance_tile_x, interior.entrance_tile_y)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Overworld: generated 14 columns x 15 rows = 210 screens
// ---------------------------------------------------------------------------

pub fn overworld_start() -> (i32, i32) {
    (1, 13)
}

pub fn cave_kind(screen_x: i32, screen_y: i32) -> Option<CaveKind> {
    if matches!((screen_x, screen_y), (8, 0) | (4, 2) | (2, 7)) {
        return None;
    }
    if let Some(screen) = generated_overworld_screen(screen_x, screen_y) {
        return screen.cave.and_then(cave_kind_from_code);
    }
    if let Some(screen) = authored_overworld_screen(screen_x, screen_y) {
        return screen.cave;
    }
    None
}

pub fn location_name(
    screen_x: i32,
    screen_y: i32,
    in_dungeon: bool,
    dungeon_id: i32,
    in_interior: bool,
    interior_id: &str,
) -> String {
    if in_interior {
        if let Some(interior) = interior_by_id(interior_id) {
            return interior.name.to_string();
        }
        return "INTERIOR".to_string();
    }
    if in_dungeon {
        if let Some(dungeon) = authored_dungeon(dungeon_id) {
            return dungeon.name.to_string();
        }
        return match dungeon_id {
            1 => "CLIFF CATACOMB",
            2 => "STONE LABYRINTH",
            3 => "FROST VAULT",
            4 => "LAKE KEEP",
            5 => "RUIN VAULT",
            _ => "DUNGEON",
        }
        .to_string();
    }
    if let Some(screen) = generated_overworld_screen(screen_x, screen_y) {
        return screen.name.to_string();
    }
    if let Some(screen) = authored_overworld_screen(screen_x, screen_y) {
        return screen.name.to_string();
    }
    overworld_spec(screen_x, screen_y).name.to_string()
}

pub fn visual_theme_id(
    screen_x: i32,
    screen_y: i32,
    in_dungeon: bool,
    dungeon_id: i32,
    in_interior: bool,
    _interior_id: &str,
) -> Option<i32> {
    if in_interior {
        return None;
    }
    if in_dungeon {
        return match dungeon_id {
            1..=8 => Some(dungeon_id),
            _ => None,
        };
    }
    if let Some(screen) = generated_overworld_screen(screen_x, screen_y) {
        return Some(screen.biome_id);
    }
    authored_overworld_screen(screen_x, screen_y)
        .map(|screen| screen.biome_id)
        .or_else(|| overworld_layout_biome_id(screen_x, screen_y))
}

pub fn build_overworld() -> HashMap<String, TileGrid> {
    let mut data = HashMap::new();
    for screen in GENERATED_OVERWORLD_SCREENS {
        let mut tiles = parse(screen.tiles);
        overlay_interior_entrances(&mut tiles, screen.x, screen.y);
        seal_world_edges(&mut tiles, screen.x, screen.y, biome_theme(screen.biome_id));
        data.insert(screen_key(screen.x, screen.y), tiles);
    }
    data
}

fn build_overworld_screen(x: i32, y: i32) -> TileGrid {
    let spec = overworld_spec(x, y);
    let mut tiles = match spec.landmark {
        OverworldLandmark::StartVillage => start_village_screen(),
        OverworldLandmark::SwordCave => sword_cave_screen(),
        OverworldLandmark::HeartCave => heart_cave_screen(),
        OverworldLandmark::IslandShrine => island_shrine_screen(),
        OverworldLandmark::EastSanctum => east_sanctum_screen(),
        OverworldLandmark::BombCave => bomb_cave_screen(),
        OverworldLandmark::Shop => secret_shop_screen(),
        OverworldLandmark::Dungeon(id) => dungeon_gate_screen(id),
        OverworldLandmark::None => {
            if overworld_layout_biome_id(x, y).is_some() {
                biome_screen(spec.biome, x, y)
            } else {
                plains_screen(((x * 17 + y * 31).unsigned_abs() % 2) as i32)
            }
        }
    };
    seal_world_edges(&mut tiles, x, y, spec.biome);
    tiles
}

fn overworld_spec(x: i32, y: i32) -> OverworldSpec {
    if let Some(biome_id) = overworld_layout_biome_id(x, y) {
        return OverworldSpec {
            biome: biome_at(x, y),
            landmark: OverworldLandmark::None,
            name: biome_region_name(biome_id),
        };
    }

    OverworldSpec {
        biome: biome_at(x, y),
        landmark: OverworldLandmark::None,
        name: "CONNECTOR PATH",
    }
}

fn biome_at(x: i32, y: i32) -> OverworldBiome {
    overworld_layout_biome_id(x, y)
        .map(biome_theme)
        .unwrap_or(OverworldBiome::Plains)
}

fn overworld_layout_biome_id(x: i32, y: i32) -> Option<i32> {
    if !(0..WORLD_W).contains(&x) || !(0..WORLD_H).contains(&y) {
        return None;
    }
    match GENERATED_OVERWORLD_LAYOUT[y as usize].as_bytes()[x as usize] as char {
        '1'..='8' => {
            Some((GENERATED_OVERWORLD_LAYOUT[y as usize].as_bytes()[x as usize] - b'0') as i32)
        }
        _ => None,
    }
}

fn biome_theme(biome_id: i32) -> OverworldBiome {
    match biome_id {
        1 => OverworldBiome::Forest,
        2 => OverworldBiome::Ruins,
        3 => OverworldBiome::Highlands,
        4 => OverworldBiome::Coast,
        5 => OverworldBiome::Mountain,
        6 => OverworldBiome::Canyon,
        7 => OverworldBiome::Snow,
        8 => OverworldBiome::Mountain,
        _ => OverworldBiome::Plains,
    }
}

fn biome_region_name(biome_id: i32) -> &'static str {
    match biome_id {
        1 => "MOSSHAVEN WILDS",
        2 => "ASHENFALL REACHES",
        3 => "IRON HIGHLANDS",
        4 => "SUNKEN COAST",
        5 => "GRIMFORGE APPROACHES",
        6 => "VOID WASTES",
        7 => "CELESTIAL PLATEAU",
        8 => "DRAGON'S APPROACH",
        _ => "WILDERNESS",
    }
}

fn biome_name(biome: OverworldBiome) -> &'static str {
    match biome {
        OverworldBiome::Plains => "PLAINS",
        OverworldBiome::Forest => "FOREST",
        OverworldBiome::DeepForest => "DEEP WOODS",
        OverworldBiome::Highlands => "RIDGELINE",
        OverworldBiome::Mountain => "BADLANDS",
        OverworldBiome::Lake => "LAKE COUNTRY",
        OverworldBiome::Desert => "DUST SEA",
        OverworldBiome::Ruins => "OLD RUINS",
        OverworldBiome::Coast => "SOUTH COAST",
        OverworldBiome::Snow => "FROST PEAKS",
        OverworldBiome::Canyon => "CANYONS",
    }
}

fn biome_screen(biome: OverworldBiome, x: i32, y: i32) -> TileGrid {
    let variant = ((x * 17 + y * 31).unsigned_abs() % 2) as i32;
    match biome {
        OverworldBiome::Plains => plains_screen(variant),
        OverworldBiome::Forest => forest_screen(variant),
        OverworldBiome::DeepForest => deep_forest_screen(variant),
        OverworldBiome::Highlands => highlands_screen(variant),
        OverworldBiome::Mountain => mountain_screen(variant),
        OverworldBiome::Lake => lake_screen(variant),
        OverworldBiome::Desert => desert_screen(variant),
        OverworldBiome::Ruins => ruins_screen(variant),
        OverworldBiome::Coast => coast_screen(variant),
        OverworldBiome::Snow => snow_screen(variant),
        OverworldBiome::Canyon => canyon_screen(variant),
    }
}

fn plains_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T...b......b...T",
            "T..............T",
            "................",
            "....==....==....",
            "................",
            "T..............T",
            "T...b......b...T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
        _ => parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..TT......TT..T",
            "T..............T",
            "................",
            "......====......",
            "................",
            "T..............T",
            "T..TT......TT..T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    }
}

fn forest_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            "TTTTTTTTTTTTTTTT",
            "T..............T",
            "T..TT.....TT...T",
            "T..............T",
            "................",
            "..T...TT...T....",
            "................",
            "T..............T",
            "T..TT.....TT...T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
        _ => parse(&[
            "TTTTTTT..TTTTTTT",
            "TT.............T",
            "T..T..T........T",
            "T.....T...TT...T",
            "T..T............",
            "T...............",
            "T..T............",
            "T...TTT...T....T",
            "T..T..T........T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    }
}

fn deep_forest_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            "TTTTTTTTTTTTTTTT",
            "T.T..T....T..T.T",
            "T....TTT...TT..T",
            "T..............T",
            "................",
            "..T..........T..",
            "................",
            "T..............T",
            "T..TT...TT..T.T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
        _ => parse(&[
            "TTTTTTTTTTTTTTTT",
            "TT.............T",
            "T..TT.TTTT.TT..T",
            "T..............T",
            "..T..........T..",
            "..T....TT....T..",
            "..T..........T..",
            "T..............T",
            "T..TT.TTTT.TT..T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    }
}

fn highlands_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            "^^^^^^^^^^^^^^^^",
            "^^.............^",
            "^...^^....^^...^",
            "^..............^",
            "................",
            "......==..==....",
            "................",
            "^..............^",
            "^...^^....^^...^",
            "^^^^^^^..^^^^^^^",
            "^^^^^^^..^^^^^^^",
        ]),
        _ => parse(&[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..............^",
            "^..............^",
            "................",
            "......====......",
            "................",
            "^..............^",
            "^.....^..^.....^",
            "^^^^^^^..^^^^^^^",
            "^^^^^^^..^^^^^^^",
        ]),
    }
}

fn mountain_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            "^^^^^^^^^^^^^^^^",
            "^^^...^^^^...^^^",
            "^^.............^",
            "^^.............^",
            "^^..............",
            "^^^.............",
            "^^..............",
            "^^^.........^^^^",
            "^^^^.......^^^^^",
            "^^^^^^^..^^^^^^^",
            "^^^^^^^..^^^^^^^",
        ]),
        _ => parse(&[
            ",,,,,,,...,,,,,,",
            ",..............,",
            ",..^^......^^..,",
            ",...^......^...,",
            ",...............,",
            ",.....^^^^.....,",
            ",...............,",
            ",...^......^...,",
            ",..^^......^^..,",
            ",,,,,,,,,,,,,,,,",
            ",,,,,,,,,,,,,,,,",
        ]),
    }
}

fn lake_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T.........~~~~~T",
            "T........~~~~~~T",
            ".........~~~~~..",
            "........~~~~~~..",
            ".........~~~~~..",
            "T........~~~~~~T",
            "T.........~~~~~T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
        _ => parse(&[
            "TTTTTTT..TTTTTTT",
            "T..........~~..T",
            "T.........~~~..T",
            "T.........~~...T",
            "..........~~....",
            "....______~~....",
            "..........~~....",
            "T.........~~~..T",
            "T..........~~..T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    }
}

fn desert_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            ",,,,,,,...,,,,,,",
            ",..............,",
            ",...^^....^^...,",
            ",..............,",
            ",....====......,",
            ",...............,",
            ",..............,",
            ",...^^....^^...,",
            ",,,,,,,...,,,,,,",
            ",,,,,,,...,,,,,,",
        ]),
        _ => parse(&[
            ",,,,,,,...,,,,,,",
            ",..............,",
            ",..............,",
            ",....^....^....,",
            ",...............,",
            ",..............,",
            ",...............,",
            ",....^....^....,",
            ",..............,",
            ",,,,,,,...,,,,,,",
            ",,,,,,,...,,,,,,",
        ]),
    }
}

fn ruins_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            ",,,,,,,...,,,,,,",
            ",...............,",
            ",...^^..x.^^...,",
            ",..............,",
            ",...............,",
            ",....x....x....,",
            ",...............,",
            ",..............,",
            ",...^^....^^...,",
            ",,,,,,,...,,,,,,",
            ",,,,,,,...,,,,,,",
        ]),
        _ => parse(&[
            ",,,,,,,...,,,,,,",
            ",..............,",
            ",...x......x...,",
            ",..............,",
            ",...............,",
            ",....^^..^^....,",
            ",...............,",
            ",..............,",
            ",...x......x...,",
            ",,,,,,,...,,,,,,",
            ",,,,,,,...,,,,,,",
        ]),
    }
}

fn coast_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..,,......,,..T",
            "T..,,......,,..T",
            "T..,,...........",
            "T..,,...........",
            "T..,,...........",
            "T..,,......,,..T",
            "T~~,,......,,~~T",
            "T~~~~~,,,,~~~~~T",
            "~~~~~~~~~~~~~~~~",
        ]),
        _ => parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..............T",
            "T..............T",
            "................",
            "......,,,,......",
            "................",
            "T..............T",
            "T..,,......,,..T",
            "T~~~~,,,,,,~~~~T",
            "~~~~~~~~~~~~~~~~",
        ]),
    }
}

fn snow_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            "^^^^^^^^^^^^^^^^",
            "^^....^^^^....^^",
            "^^..^^....^^..^^",
            "^..............^",
            "^..............^",
            "....==....==....",
            "^..............^",
            "^...^^....^^...^",
            "^^....^^^^....^^",
            "^^^^^^^..^^^^^^^",
            "^^^^^^^..^^^^^^^",
        ]),
        _ => highlands_screen(variant),
    }
}

fn canyon_screen(variant: i32) -> TileGrid {
    match variant {
        0 => parse(&[
            ",,,,,,,...,,,,,,",
            ",..^^......^^..,",
            ",..^^......^^..,",
            ",...^......^...,",
            ",...............,",
            ",.....^^^^.....,",
            ",...............,",
            ",...^......^...,",
            ",..^^......^^..,",
            ",,,,,,,,,,,,,,,,",
            ",,,,,,,,,,,,,,,,",
        ]),
        _ => parse(&[
            ",,,,,,,..TTTTTTT",
            "T..............T",
            "T..TT.xxxx.TT..T",
            "T..T..x..x..T..T",
            "T..T..x..x..T..T",
            "T..T..x..x..T..T",
            "T..T..x..x..T..T",
            "T..T..xxxx..T..T",
            "T..TT......TT..T",
            "TTTTTTTTTTTTTTTT",
            "TTTTTTTTTTTTTTTT",
        ]),
    }
}

fn start_village_screen() -> TileGrid {
    parse(&[
        "TTTTTTT..TTTTTTT",
        "T..............T",
        "T..===....===..T",
        "T..=bb....bb=..T",
        "...=........=...",
        "====....==....==",
        "...=........=...",
        "T..=bb....bb=..T",
        "T..===....===..T",
        "TTTTTTT..TTTTTTT",
        "TTTTTTT..TTTTTTT",
    ])
}

fn sword_cave_screen() -> TileGrid {
    parse(&[
        "TTTTTTTTTTTTTTTT",
        "T..............T",
        "T..bbb....bb...T",
        "T..bcb....bb...T",
        "................",
        "......====......",
        "................",
        "T..bb.....bb...T",
        "T..bb.....bb...T",
        "TTTTTTT..TTTTTTT",
        "TTTTTTT..TTTTTTT",
    ])
}

fn heart_cave_screen() -> TileGrid {
    parse(&[
        "^^^^^^^^^^^^^^^^",
        "^^^...^^^^...^^^",
        "^^.............^",
        "^^..c..........^",
        "^^..............",
        "^^^.............",
        "^^..............",
        "^^^.........^^^^",
        "^^^^.......^^^^^",
        "^^^^^^^..^^^^^^^",
        "^^^^^^^..^^^^^^^",
    ])
}

fn island_shrine_screen() -> TileGrid {
    parse(&[
        "TTTTTTT..TTTTTTT",
        "T~~~~~~~~~~~~~.T",
        "T~~~..........~T",
        "T~~...........~T",
        "_~.............~",
        "~~......c.....~~",
        "_~.............~",
        "T~~...........~T",
        "T~~~..........~T",
        "T~~~~~..~~~~~~~T",
        "TTTTTTT..TTTTTTT",
    ])
}

fn east_sanctum_screen() -> TileGrid {
    parse(&[
        ",,,,,,,...,,,,,,",
        ",..............,",
        ",..==......==..,",
        ",..=........=..,",
        "...=..c.....=...",
        "...=........=...",
        "...=........=...",
        ",..=........=..,",
        ",..==......==..,",
        ",,,,,,,...,,,,,,",
        ",,,,,,,...,,,,,,",
    ])
}

fn bomb_cave_screen() -> TileGrid {
    parse(&[
        ",,,,,,,...,,,,,,",
        ",..............,",
        ",..............,",
        ",..............,",
        ",...............,",
        ",..c............,",
        ",...............,",
        ",..............,",
        ",,.............,,",
        ",,~~,,,,,,,~~,,,",
        "~~~~~~~~~~~~~~~~",
    ])
}

fn secret_shop_screen() -> TileGrid {
    parse(&[
        ",,,,,,,...TTTTTT",
        "T..............T",
        "T..TT......TT..T",
        "T..T........T..T",
        "T..T..c..c..T..T",
        "T..T........T..T",
        "T..T........T..T",
        "T..T..bbbb..T..T",
        "T..TT......TT..T",
        "TTTTTTTTTTTTTTTT",
        "TTTTTTTTTTTTTTTT",
    ])
}

fn dungeon_gate_screen(id: i32) -> TileGrid {
    match id {
        1 => parse(&[
            "^^^^^^^^^^^^^^^^",
            "^^^...^^^^...^^^",
            "^^.............^",
            "^^.............^",
            "^^....dd........",
            "^^^.............",
            "^^..............",
            "^^^.........^^^^",
            "^^^^.......^^^^^",
            "^^^^^^^..^^^^^^^",
            "^^^^^^^..^^^^^^^",
        ]),
        2 => parse(&[
            ",,,,,,,...,,,,,,",
            ",..............,",
            ",..^^.^^^^.^^..,",
            ",...^......^...,",
            ",...^..dd..^...,",
            ",...^......^...,",
            ",...^^^^^^^^...,",
            ",..............,",
            ",..............,",
            ",,,,,,,...,,,,,,",
            ",,,,,,,...,,,,,,",
        ]),
        3 => parse(&[
            "^^^^^^^^^^^^^^^^",
            "^^....^^^^....^^",
            "^^..^^....^^..^^",
            "^..............^",
            "^......dd......^",
            "....==....==....",
            "^..............^",
            "^...^^....^^...^",
            "^^....^^^^....^^",
            "^^^^^^^..^^^^^^^",
            "^^^^^^^..^^^^^^^",
        ]),
        4 => parse(&[
            "TTTTTTT..TTTTTTT",
            "T~~~~~~~~~~~~~.T",
            "T~~~..........~T",
            "T~~...........~T",
            "_~.............~",
            "~~......d.....~~",
            "_~.............~",
            "T~~...........~T",
            "T~~~..........~T",
            "T~~~~~..~~~~~~~T",
            "TTTTTTT..TTTTTTT",
        ]),
        _ => parse(&[
            ",,,,,,,..TTTTTTT",
            "T..............T",
            "T..TT.xxxx.TT..T",
            "T..T..x..x..T..T",
            "T..T..x..x..T..T",
            "T..T..xddx..T..T",
            "T..T..x..x..T..T",
            "T..T..xxxx..T..T",
            "T..TT......TT..T",
            "TTTTTTTTTTTTTTTT",
            "TTTTTTTTTTTTTTTT",
        ]),
    }
}

fn seal_world_edges(tiles: &mut TileGrid, x: i32, y: i32, biome: OverworldBiome) {
    let wall = match biome {
        OverworldBiome::Forest | OverworldBiome::DeepForest => TileType::Tree,
        OverworldBiome::Lake | OverworldBiome::Coast => TileType::Water,
        _ => TileType::Rock,
    };
    if x == 0 {
        for row in tiles.iter_mut() {
            for tile in row.iter_mut().take(3) {
                *tile = wall;
            }
        }
    }
    if x == WORLD_W - 1 {
        for row in tiles.iter_mut() {
            for col in (COLS - 3)..COLS {
                row[col] = wall;
            }
        }
    }
    if y == 0 {
        for row in tiles.iter_mut().take(3) {
            for tile in row.iter_mut() {
                *tile = wall;
            }
        }
    }
    if y == WORLD_H - 1 {
        for row in tiles.iter_mut().skip(ROWS - 3) {
            for tile in row.iter_mut() {
                *tile = wall;
            }
        }
    }

    // Cap the standard transition lanes on true world boundaries so they
    // visibly read as dead ends instead of looking like valid exits.
    if x == 0 {
        for row in 3..=7 {
            tiles[row][3] = wall;
        }
    }
    if x == WORLD_W - 1 {
        for row in 3..=7 {
            tiles[row][COLS - 4] = wall;
        }
    }
    if y == 0 {
        for col in 5..=10 {
            tiles[3][col] = wall;
        }
    }
    if y == WORLD_H - 1 {
        for col in 5..=10 {
            tiles[ROWS - 4][col] = wall;
        }
    }
}

fn normalize_overworld_connections(data: &mut HashMap<String, TileGrid>) {
    for y in 0..WORLD_H {
        for x in 0..WORLD_W {
            if x + 1 < WORLD_W {
                let left_key = screen_key(x, y);
                let right_key = screen_key(x + 1, y);
                let mut left = data
                    .remove(&left_key)
                    .expect("left overworld screen missing during normalization");
                let mut right = data
                    .remove(&right_key)
                    .expect("right overworld screen missing during normalization");
                carve_horizontal_connection(
                    &mut left,
                    &mut right,
                    connector_tile(overworld_spec(x, y)),
                    connector_tile(overworld_spec(x + 1, y)),
                    seam_block_tile(overworld_spec(x, y)),
                    seam_block_tile(overworld_spec(x + 1, y)),
                );
                data.insert(left_key, left);
                data.insert(right_key, right);
            }

            if y + 1 < WORLD_H {
                let top_key = screen_key(x, y);
                let bottom_key = screen_key(x, y + 1);
                let mut top = data
                    .remove(&top_key)
                    .expect("top overworld screen missing during normalization");
                let mut bottom = data
                    .remove(&bottom_key)
                    .expect("bottom overworld screen missing during normalization");
                carve_vertical_connection(
                    &mut top,
                    &mut bottom,
                    connector_tile(overworld_spec(x, y)),
                    connector_tile(overworld_spec(x, y + 1)),
                    seam_block_tile(overworld_spec(x, y)),
                    seam_block_tile(overworld_spec(x, y + 1)),
                );
                data.insert(top_key, top);
                data.insert(bottom_key, bottom);
            }
        }
    }
}

fn carve_horizontal_connection(
    left: &mut TileGrid,
    right: &mut TileGrid,
    left_tile: TileType,
    right_tile: TileType,
    left_block: TileType,
    right_block: TileType,
) {
    for row in 0..ROWS {
        for col in (COLS - 3)..COLS {
            left[row][col] = left_block;
        }
        for col in 0..=2 {
            right[row][col] = right_block;
        }
    }
    for row in 4..=6 {
        for col in (COLS - 3)..COLS {
            left[row][col] = left_tile;
        }
        for col in 0..=2 {
            right[row][col] = right_tile;
        }
    }
}

fn carve_vertical_connection(
    top: &mut TileGrid,
    bottom: &mut TileGrid,
    top_tile: TileType,
    bottom_tile: TileType,
    top_block: TileType,
    bottom_block: TileType,
) {
    for row in (ROWS - 3)..ROWS {
        for col in 0..COLS {
            top[row][col] = top_block;
        }
    }
    for row in 0..=2 {
        for col in 0..COLS {
            bottom[row][col] = bottom_block;
        }
    }
    for col in 6..=9 {
        for row in (ROWS - 3)..ROWS {
            top[row][col] = top_tile;
        }
        for row in 0..=2 {
            bottom[row][col] = bottom_tile;
        }
    }
}

fn connector_tile(spec: OverworldSpec) -> TileType {
    match spec.landmark {
        OverworldLandmark::StartVillage | OverworldLandmark::Shop => TileType::Path,
        OverworldLandmark::Dungeon(_) => TileType::Path,
        OverworldLandmark::IslandShrine => TileType::Bridge,
        _ => match spec.biome {
            OverworldBiome::Lake | OverworldBiome::Coast => TileType::Bridge,
            OverworldBiome::Desert | OverworldBiome::Ruins | OverworldBiome::Canyon => {
                TileType::Sand
            }
            _ => TileType::Grass,
        },
    }
}

fn seam_block_tile(spec: OverworldSpec) -> TileType {
    match spec.biome {
        OverworldBiome::Forest | OverworldBiome::DeepForest => TileType::Tree,
        OverworldBiome::Lake | OverworldBiome::Coast => TileType::Water,
        _ => TileType::Rock,
    }
}

fn validate_overworld_connections(data: &HashMap<String, TileGrid>) -> Result<(), String> {
    for y in 0..WORLD_H {
        for x in 0..WORLD_W {
            if x + 1 < WORLD_W {
                let left_key = screen_key(x, y);
                let right_key = screen_key(x + 1, y);
                let left = data
                    .get(&left_key)
                    .ok_or_else(|| format!("missing screen {left_key}"))?;
                let right = data
                    .get(&right_key)
                    .ok_or_else(|| format!("missing screen {right_key}"))?;
                if !(0..ROWS)
                    .any(|row| is_walkable(left[row][COLS - 1]) && is_walkable(right[row][0]))
                {
                    return Err(format!(
                        "horizontal mismatch between {left_key} and {right_key}"
                    ));
                }
                for row in 0..ROWS {
                    let in_corridor = (4..=6).contains(&row);
                    let left_safe = ((COLS - 3)..COLS).all(|col| is_walkable(left[row][col]));
                    let right_safe = (0..=2).all(|col| is_walkable(right[row][col]));
                    if in_corridor {
                        if !left_safe || !right_safe {
                            return Err(format!(
                                "unsafe horizontal corridor between {left_key} and {right_key} at row {row}"
                            ));
                        }
                    } else if is_walkable(left[row][COLS - 1]) || is_walkable(right[row][0]) {
                        return Err(format!(
                            "unexpected horizontal opening between {left_key} and {right_key} at row {row}"
                        ));
                    }
                }
            }

            if y + 1 < WORLD_H {
                let top_key = screen_key(x, y);
                let bottom_key = screen_key(x, y + 1);
                let top = data
                    .get(&top_key)
                    .ok_or_else(|| format!("missing screen {top_key}"))?;
                let bottom = data
                    .get(&bottom_key)
                    .ok_or_else(|| format!("missing screen {bottom_key}"))?;
                if !(0..COLS)
                    .any(|col| is_walkable(top[ROWS - 1][col]) && is_walkable(bottom[0][col]))
                {
                    return Err(format!(
                        "vertical mismatch between {top_key} and {bottom_key}"
                    ));
                }
                for col in 0..COLS {
                    let in_corridor = (6..=9).contains(&col);
                    let top_safe = ((ROWS - 3)..ROWS).all(|row| is_walkable(top[row][col]));
                    let bottom_safe = (0..=2).all(|row| is_walkable(bottom[row][col]));
                    if in_corridor {
                        if !top_safe || !bottom_safe {
                            return Err(format!(
                                "unsafe vertical corridor between {top_key} and {bottom_key} at col {col}"
                            ));
                        }
                    } else if is_walkable(top[ROWS - 1][col]) || is_walkable(bottom[0][col]) {
                        return Err(format!(
                            "unexpected vertical opening between {top_key} and {bottom_key} at col {col}"
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn is_walkable(tile: TileType) -> bool {
    !matches!(
        tile,
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

const MOSSHAVEN_FEN_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Borespat,
        tile_x: 5.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Splort,
        tile_x: 10.0,
        tile_y: 6.0,
    },
];

const MOSSHAVEN_WOOD_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Splort,
        tile_x: 5.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Shriekwing,
        tile_x: 10.0,
        tile_y: 6.0,
    },
];

const MOSSHAVEN_RING_ENEMIES: &[SpawnDef] = &[SpawnDef {
    enemy_type: EnemyType::Shriekwing,
    tile_x: 8.0,
    tile_y: 4.0,
}];

const R04_BOULDERS: &[PropDef] = &[
    PropDef {
        kind: PropKind::Boulder,
        tile_x: 5,
        tile_y: 5,
        target_tile_x: None,
        target_tile_y: None,
    },
    PropDef {
        kind: PropKind::Boulder,
        tile_x: 10,
        tile_y: 5,
        target_tile_x: None,
        target_tile_y: None,
    },
];

const R07_PROPS: &[PropDef] = &[
    PropDef {
        kind: PropKind::PressurePlate,
        tile_x: 5,
        tile_y: 3,
        target_tile_x: None,
        target_tile_y: None,
    },
    PropDef {
        kind: PropKind::PressurePlate,
        tile_x: 10,
        tile_y: 3,
        target_tile_x: None,
        target_tile_y: None,
    },
    PropDef {
        kind: PropKind::PressurePlate,
        tile_x: 5,
        tile_y: 7,
        target_tile_x: None,
        target_tile_y: None,
    },
    PropDef {
        kind: PropKind::PressurePlate,
        tile_x: 10,
        tile_y: 7,
        target_tile_x: None,
        target_tile_y: None,
    },
    PropDef {
        kind: PropKind::Boulder,
        tile_x: 3,
        tile_y: 2,
        target_tile_x: None,
        target_tile_y: None,
    },
    PropDef {
        kind: PropKind::Boulder,
        tile_x: 12,
        tile_y: 2,
        target_tile_x: None,
        target_tile_y: None,
    },
    PropDef {
        kind: PropKind::Boulder,
        tile_x: 3,
        tile_y: 8,
        target_tile_x: None,
        target_tile_y: None,
    },
    PropDef {
        kind: PropKind::Boulder,
        tile_x: 12,
        tile_y: 8,
        target_tile_x: None,
        target_tile_y: None,
    },
];

const MOSSHAVEN_SCREENS: &[OverworldScreenDef] = &[
    OverworldScreenDef {
        x: 2,
        y: 12,
        biome_id: 1,
        name: "THE FEN",
        tiles: &[
            "TTTTTTTTTTTTTTTT",
            "T..~~....~~....T",
            "T.~~~..TT..~d..T",
            "T..~~..........T",
            "...~~....~~.....",
            "....__..~~......",
            "...~~...........",
            "T..~~....TT....T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ],
        cave: None,
        dungeon: Some(1),
        enemies: MOSSHAVEN_FEN_ENEMIES,
    },
    OverworldScreenDef {
        x: 4,
        y: 13,
        biome_id: 1,
        name: "CLEARWATER STREAM",
        tiles: &[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T...TT....TT...T",
            "T..............T",
            "~~~....__....~~~",
            "~~~~..____..~~~~",
            "~~~....__....~~~",
            "T..............T",
            "T..TT......TT..T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ],
        cave: None,
        dungeon: None,
        enemies: MOSSHAVEN_WOOD_ENEMIES,
    },
    OverworldScreenDef {
        x: 9,
        y: 13,
        biome_id: 1,
        name: "HIGH CANOPY TRAIL",
        tiles: &[
            "TTTTTTT..TTTTTTT",
            "T..T........T..T",
            "T..T..____..T..T",
            "T..T........T..T",
            "....T......T....",
            "....T..__..T....",
            "....T......T....",
            "T..T........T..T",
            "T..T....TT..T..T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ],
        cave: None,
        dungeon: None,
        enemies: MOSSHAVEN_WOOD_ENEMIES,
    },
    OverworldScreenDef {
        x: 4,
        y: 12,
        biome_id: 1,
        name: "DEEP WOOD",
        tiles: &[
            "TTTTTTT..TTTTTTT",
            "T...TT....TT...T",
            "T..............T",
            "T.T...TT....T..T",
            "................",
            "..TT......TT....",
            "................",
            "T...T....TT....T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ],
        cave: None,
        dungeon: None,
        enemies: MOSSHAVEN_WOOD_ENEMIES,
    },
    OverworldScreenDef {
        x: 6,
        y: 13,
        biome_id: 1,
        name: "GRANDFATHER TREE",
        tiles: &[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T....TTTTTT....T",
            "T...TT....TT...T",
            "....T..cc..T....",
            "....T..cc..T....",
            "....T......T....",
            "T...TT....TT...T",
            "T....TTTTTT....T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ],
        cave: Some(CaveKind::Sword),
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 9,
        y: 14,
        biome_id: 1,
        name: "FUNGAL RING",
        tiles: &[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T....bb..bb....T",
            "T...b......b...T",
            "....b..,,..b....",
            "....b..,,..b....",
            "....b......b....",
            "T....bb..bb....T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ],
        cave: None,
        dungeon: None,
        enemies: MOSSHAVEN_RING_ENEMIES,
    },
    OverworldScreenDef {
        x: 4,
        y: 14,
        biome_id: 1,
        name: "SOUTHERN MEADOW",
        tiles: &[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T......==......T",
            "T..............T",
            "................",
            "...==......==...",
            "................",
            "T..............T",
            "T....b....b....T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ],
        cave: None,
        dungeon: None,
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Splort,
            tile_x: 10.0,
            tile_y: 5.0,
        }],
    },
    OverworldScreenDef {
        x: 6,
        y: 14,
        biome_id: 1,
        name: "SOUTHERN MEADOW",
        tiles: &[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T...===..===...T",
            "T..............T",
            "................",
            "====........====",
            "................",
            "T..............T",
            "T....b....b....T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ],
        cave: None,
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 8,
        y: 14,
        biome_id: 1,
        name: "EASTERN WOODS",
        tiles: &[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..TT......TT..T",
            "T..............T",
            "................",
            "......====......",
            "................",
            "T...bb....bb...T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ],
        cave: None,
        dungeon: None,
        enemies: MOSSHAVEN_WOOD_ENEMIES,
    },
];

const ASHENFALL_PLAIN_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Shriekwing,
        tile_x: 4.0,
        tile_y: 3.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Splort,
        tile_x: 11.0,
        tile_y: 6.0,
    },
];

const ASHENFALL_RUIN_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Ironmaw,
        tile_x: 5.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Borespat,
        tile_x: 10.0,
        tile_y: 5.0,
    },
];

const ASHENFALL_SCREENS: &[OverworldScreenDef] = &[
    OverworldScreenDef {
        x: 8,
        y: 10,
        biome_id: 2,
        name: "ASH PLAIN WEST",
        tiles: &[
            "....^^....^^....",
            "...^^^^..^^^^...",
            "................",
            ".....====.......",
            "................",
            "..,,........,,..",
            "..,,....,,..,,..",
            "................",
            "....^^....^^....",
            "................",
            "................",
        ],
        cave: None,
        dungeon: None,
        enemies: ASHENFALL_PLAIN_ENEMIES,
    },
    OverworldScreenDef {
        x: 10,
        y: 9,
        biome_id: 2,
        name: "BELLTOWER REMAINS",
        tiles: &[
            "....TTTTTTTT....",
            "....T......T....",
            "....T..__..T....",
            "....T..__..T....",
            "................",
            "..,,....,,......",
            "..,,........,,..",
            "................",
            "...^^^^..^^^^...",
            "................",
            "................",
        ],
        cave: None,
        dungeon: None,
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Shriekwing,
            tile_x: 8.0,
            tile_y: 4.0,
        }],
    },
    OverworldScreenDef {
        x: 12,
        y: 9,
        biome_id: 2,
        name: "RUIN QUARTER NORTH",
        tiles: &[
            "TTTT....TTTT....",
            "T..T....T..T....",
            "T..T....T..T....",
            "T..T....T..T....",
            "....====........",
            "....^^..^^......",
            "..^^......^^....",
            "................",
            "....TT....TT....",
            "................",
            "................",
        ],
        cave: None,
        dungeon: None,
        enemies: ASHENFALL_RUIN_ENEMIES,
    },
    OverworldScreenDef {
        x: 8,
        y: 12,
        biome_id: 2,
        name: "SALVAGER APPROACH",
        tiles: &[
            "................",
            "..^^^^......^^..",
            "..^..^......^...",
            "..^..^..==..^...",
            "..^^^^..==..^^..",
            "................",
            "...,,...........",
            "...,,...........",
            "................",
            "....^^....^^....",
            "................",
        ],
        cave: None,
        dungeon: None,
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Splort,
            tile_x: 12.0,
            tile_y: 5.0,
        }],
    },
    OverworldScreenDef {
        x: 10,
        y: 11,
        biome_id: 2,
        name: "DEEP ASH PIT",
        tiles: &[
            "................",
            "...,,......,,...",
            "..,,~~~~~~~~,,..",
            "..,~~~~~~~~~~,..",
            "...~~~~~~~~~~...",
            "...~~~~~~~~~~...",
            "..,~~~~~~~~~~,..",
            "..,,~~~~~~~~,,..",
            "...,,......,,...",
            "................",
            "................",
        ],
        cave: None,
        dungeon: None,
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 4.0,
                tile_y: 2.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 11.0,
                tile_y: 7.0,
            },
        ],
    },
    OverworldScreenDef {
        x: 12,
        y: 11,
        biome_id: 2,
        name: "WARDEN'S POST",
        tiles: &[
            "TTTTTT....TTTTTT",
            "T....T....T....T",
            "T.^^.T.dd.T.^^.T",
            "T.^^.T....T.^^.T",
            "T....T....T....T",
            "TTTTTT====TTTTTT",
            "..^^........^^..",
            "....^^....^^....",
            "................",
            "....TT....TT....",
            "................",
        ],
        cave: None,
        dungeon: Some(2),
        enemies: ASHENFALL_RUIN_ENEMIES,
    },
    OverworldScreenDef {
        x: 10,
        y: 13,
        biome_id: 2,
        name: "SALVAGER'S CAMP",
        tiles: &[
            "................",
            "..^^^^..^^^^....",
            "..^..^..^..^....",
            "..^..^..^..^....",
            "..^^^^..^^^^....",
            "................",
            "...==......==...",
            "................",
            "....,,....,,....",
            "................",
            "................",
        ],
        cave: None,
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 10,
        y: 12,
        biome_id: 2,
        name: "ASH ROAD",
        tiles: &[
            "................",
            "...,,......,,...",
            "................",
            "=====......=====",
            "................",
            "................",
            "=====......=====",
            "................",
            "...^^....^^.....",
            "................",
            "................",
        ],
        cave: None,
        dungeon: None,
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Shriekwing,
            tile_x: 9.0,
            tile_y: 4.0,
        }],
    },
    OverworldScreenDef {
        x: 12,
        y: 12,
        biome_id: 2,
        name: "RUIN QUARTER SOUTH",
        tiles: &[
            "....TT....TT....",
            "...T..T..T..T...",
            "...T..T..T..T...",
            "................",
            "..^^....^^....^^",
            "....====........",
            "................",
            "..,,........,,..",
            "................",
            "....TTTTTTTT....",
            "................",
        ],
        cave: None,
        dungeon: None,
        enemies: ASHENFALL_RUIN_ENEMIES,
    },
];

const IRONHIGHLANDS_PIPE_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Shriekwing,
        tile_x: 5.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Ironmaw,
        tile_x: 10.0,
        tile_y: 6.0,
    },
];

const IRONHIGHLANDS_VENT_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Splort,
        tile_x: 5.0,
        tile_y: 3.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Splort,
        tile_x: 11.0,
        tile_y: 7.0,
    },
];

const IRONHIGHLANDS_SCREENS: &[OverworldScreenDef] = &[
    OverworldScreenDef {
        x: 2,
        y: 8,
        biome_id: 3,
        name: "PIPE FIELDS NORTH",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^^..__..__..^^^^",
            "^...__..__...^^^",
            "^.............^^",
            "^....====......^",
            "^.............^^",
            "^..__..__..__..^",
            "^.............^^",
            "^^...^^^^...^^^^",
            "^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: IRONHIGHLANDS_PIPE_ENEMIES,
    },
    OverworldScreenDef {
        x: 4,
        y: 8,
        biome_id: 3,
        name: "VAULT APPROACH",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^^^^^^dddd^^^^^^",
            "^^^^^^....^^^^^^",
            "^^^^^^....^^^^^^",
            "......====......",
            "..__..........__",
            "................",
            "....^^....^^....",
            "................",
            "^^^^^^....^^^^^^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: Some(3),
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 4.0,
                tile_y: 6.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 11.0,
                tile_y: 6.0,
            },
        ],
    },
    OverworldScreenDef {
        x: 7,
        y: 9,
        biome_id: 3,
        name: "PRIMARY VENT",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^^..~~~~..~~~~^^",
            "^...~~~~..~~~~.^",
            "^...~~~~..~~~~.^",
            "....====........",
            "....~~~~........",
            "....~~~~........",
            "....^^^^....^^^^",
            "................",
            "^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: IRONHIGHLANDS_VENT_ENEMIES,
    },
    OverworldScreenDef {
        x: 3,
        y: 10,
        biome_id: 3,
        name: "ENGINEER'S TOWER",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^^....cc....^^^^",
            "^....cccc....^^^",
            "^....cccc....^^^",
            "^....cccc....^^^",
            "^.....cc.....^^^",
            "^...__....__..^^",
            "^.............^^",
            "^^....^^^^....^^",
            "^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: Some(CaveKind::AncientKey),
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 5,
        y: 10,
        biome_id: 3,
        name: "AQUEDUCT CROSSING",
        tiles: &[
            "....________....",
            "....________....",
            "................",
            "..__........__..",
            "..__..====..__..",
            "..__........__..",
            "................",
            "....________....",
            "....________....",
            "................",
            "................",
        ],
        cave: None,
        dungeon: None,
        enemies: IRONHIGHLANDS_PIPE_ENEMIES,
    },
    OverworldScreenDef {
        x: 7,
        y: 10,
        biome_id: 3,
        name: "ACTIVE VENTS",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..~~~~..~~~~..^",
            "^..~~~~..~~~~..^",
            "^..............^",
            "^..^^^^..^^^^..^",
            "^..............^",
            "^..~~~~..~~~~..^",
            "^..~~~~..~~~~..^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: IRONHIGHLANDS_VENT_ENEMIES,
    },
    OverworldScreenDef {
        x: 3,
        y: 11,
        biome_id: 3,
        name: "PIPE FIELDS SOUTH",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^^..__..__..^^^^",
            "^..............^",
            "^..^^^^..^^^^..^",
            "^..............^",
            "^..__..====..__^",
            "^..............^",
            "^..^^^^..^^^^..^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: IRONHIGHLANDS_PIPE_ENEMIES,
    },
    OverworldScreenDef {
        x: 5,
        y: 11,
        biome_id: 3,
        name: "APPROACH ROAD",
        tiles: &[
            "....________....",
            "....________....",
            "................",
            "=====......=====",
            "=====......=====",
            "................",
            "=====......=====",
            "=====......=====",
            "................",
            "....________....",
            "....________....",
        ],
        cave: None,
        dungeon: None,
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Shriekwing,
            tile_x: 8.0,
            tile_y: 5.0,
        }],
    },
    OverworldScreenDef {
        x: 7,
        y: 11,
        biome_id: 3,
        name: "VENT RIDGE",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^...~~~~....~~~~",
            "^...~~~~....~~~~",
            "^..............^",
            "^..^^^^....^^..^",
            "^..............^",
            "^....====......^",
            "^..............^",
            "^..^^^^....^^..^",
            "^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: IRONHIGHLANDS_VENT_ENEMIES,
    },
];

const SUNKEN_COAST_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Ironmaw,
        tile_x: 5.0,
        tile_y: 6.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Shriekwing,
        tile_x: 11.0,
        tile_y: 3.0,
    },
];

const FLOODED_RUINS_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Splort,
        tile_x: 4.0,
        tile_y: 7.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Borespat,
        tile_x: 10.0,
        tile_y: 4.0,
    },
];

const SUNKEN_COAST_SCREENS: &[OverworldScreenDef] = &[
    OverworldScreenDef {
        x: 5,
        y: 8,
        biome_id: 4,
        name: "TIDAL VILLAGE",
        tiles: &[
            "~~~~~~~~~~~~~~~~",
            "~~....^^....~~~~",
            "~...,,==,,...~~~",
            "~..,,====,,..~~~",
            "~...,,==,,...~~~",
            "~~....^^....~~~~",
            "~~~~......~~~~~~",
            "~~~~..__..~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
        ],
        cave: None,
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 9,
        y: 6,
        biome_id: 4,
        name: "LIGHTHOUSE ISLE",
        tiles: &[
            "~~~~~~~~~~~~~~~~",
            "~~~~~~cc~~~~~~~~",
            "~~~~~cccc~~~~~~~",
            "~~~~~cccc~~~~~~~",
            "~~~~~cccc~~~~~~~",
            "~~~~~~cc~~~~~~~~",
            "~~~~..==..~~~~~~",
            "~~~~......~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
        ],
        cave: Some(CaveKind::TideChart),
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 13,
        y: 7,
        biome_id: 4,
        name: "SEA STACK",
        tiles: &[
            "~~~~~~~~~~~~~~~~",
            "~~~~~~^^~~~~~~~~",
            "~~~~~^^^^~~~~~~~",
            "~~~~~^^^^~~~~~~~",
            "~~~~~~^^~~~~~~~~",
            "~~~~......~~~~~~",
            "~~~~..==..~~~~~~",
            "~~~~......~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
        ],
        cave: None,
        dungeon: None,
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Shriekwing,
            tile_x: 8.0,
            tile_y: 4.0,
        }],
    },
    OverworldScreenDef {
        x: 5,
        y: 9,
        biome_id: 4,
        name: "CAUSEWAY SOUTH",
        tiles: &[
            "~~~~~~~~~~~~~~~~",
            "~~~~..====..~~~~",
            "~~~~..====..~~~~",
            "~~~~......~~~~~~",
            "~~~~......~~~~~~",
            "~~~~..====..~~~~",
            "~~~~..====..~~~~",
            "~~~~......~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
        ],
        cave: None,
        dungeon: None,
        enemies: SUNKEN_COAST_ENEMIES,
    },
    OverworldScreenDef {
        x: 8,
        y: 8,
        biome_id: 4,
        name: "ISLAND CLUSTER",
        tiles: &[
            "~~~~~~~~~~~~~~~~",
            "~~....~~~~....~~",
            "~...,,~~~~,,...~",
            "~..,,==~~==,,..~",
            "~...,,~~~~,,...~",
            "~~....~~~~....~~",
            "~~~~..~~~~..~~~~",
            "~~~~..~~~~..~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
        ],
        cave: None,
        dungeon: None,
        enemies: SUNKEN_COAST_ENEMIES,
    },
    OverworldScreenDef {
        x: 12,
        y: 8,
        biome_id: 4,
        name: "FLOODED RUINS",
        tiles: &[
            "~~~~~~~~~~~~~~~~",
            "~~..^^^^..^^^^~~",
            "~...^dd^..^..^~~",
            "~...^^^^..^^^^~~",
            "~~~~......~~~~~~",
            "~~~~..==..~~~~~~",
            "~~~~......~~~~~~",
            "~~..^^^^..^^^^~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
        ],
        cave: None,
        dungeon: Some(4),
        enemies: FLOODED_RUINS_ENEMIES,
    },
    OverworldScreenDef {
        x: 8,
        y: 9,
        biome_id: 4,
        name: "TIDAL GATE",
        tiles: &[
            "~~~~~~~~~~~~~~~~",
            "~~~~~~==~~~~~~~~",
            "~~~~~~==~~~~~~~~",
            "~~~~~~..~~~~~~~~",
            "~~~~..^^..~~~~~~",
            "~~~~~~..~~~~~~~~",
            "~~~~~~==~~~~~~~~",
            "~~~~~~==~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
        ],
        cave: None,
        dungeon: None,
        enemies: SUNKEN_COAST_ENEMIES,
    },
    OverworldScreenDef {
        x: 11,
        y: 8,
        biome_id: 4,
        name: "CAUSEWAY EAST",
        tiles: &[
            "~~~~~~~~~~~~~~~~",
            "~~~~..====..~~~~",
            "~~~~..====..~~~~",
            "~~~~......~~~~~~",
            "~~~~......~~~~~~",
            "~~~~..====..~~~~",
            "~~~~..====..~~~~",
            "~~~~......~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
        ],
        cave: None,
        dungeon: None,
        enemies: SUNKEN_COAST_ENEMIES,
    },
    OverworldScreenDef {
        x: 13,
        y: 8,
        biome_id: 4,
        name: "ROOFTOP REACH",
        tiles: &[
            "~~~~~~~~~~~~~~~~",
            "~~..^^^^..^^^^~~",
            "~...^^^^..^^^^~~",
            "~...==......==~~",
            "~~~~......~~~~~~",
            "~~~~..==..~~~~~~",
            "~~~~......~~~~~~",
            "~~..^^^^..^^^^~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
            "~~~~~~~~~~~~~~~~",
        ],
        cave: None,
        dungeon: None,
        enemies: FLOODED_RUINS_ENEMIES,
    },
];

const LEVEL1_ROOMS: &[DungeonRoomDef] = &[
    DungeonRoomDef {
        x: 1,
        y: 5,
        tiles: &[
            "#######  #######",
            "##   f    f   ##",
            "##            ##",
            "##  f      f  ##",
            "##            ##",
            "##            ##",
            "##            ##",
            "##  f      f  ##",
            "##            ##",
            "#######  #######",
            "#######ss#######",
        ],
        enemies: &[],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 4,
        tiles: &[
            "#######ll#######",
            "##            ##",
            "##    ##      ##",
            "##            ##",
            "o              o",
            "o      ##      o",
            "o              o",
            "##            ##",
            "##            ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 10.0,
                tile_y: 6.0,
            },
        ],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 7,
            tile_y: 5,
        }],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 4,
        tiles: &[
            "################",
            "##     f      ##",
            "##  f      $  ##",
            "##            ##",
            "o             ##",
            "o        x    ##",
            "o             ##",
            "##            ##",
            "##   f        ##",
            "################",
            "################",
        ],
        enemies: &[],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 12,
            tile_y: 2,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 3,
        tiles: &[
            "#######  #######",
            "##   ##  ##   ##",
            "##            ##",
            "##  f      f  ##",
            "o      ##      o",
            "o              o",
            "o      ##      o",
            "##  $       f ##",
            "##            ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 10.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 7.0,
                tile_y: 7.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 4,
            tile_y: 7,
        }],
        props: R04_BOULDERS,
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 3,
        tiles: &[
            "################",
            "##            ##",
            "##  f      f  ##",
            "##      $     ##",
            "o             ##",
            "o             ##",
            "o             ##",
            "##  f      f  ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Shriekwing,
            tile_x: 8.0,
            tile_y: 4.0,
        }],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 7,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 2,
        tiles: &[
            "#######  #######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~....$.....~##",
            "o..............o",
            "o....ffff......o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 10.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Bombs,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 1,
        tiles: &[
            "#######kk#######",
            "##            ##",
            "##   f    f   ##",
            "##            ##",
            "o              o",
            "o      ff      o",
            "o              o",
            "##   f    f   ##",
            "##            ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Ironmaw,
            tile_x: 8.0,
            tile_y: 5.0,
        }],
        items: &[],
        props: R07_PROPS,
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: Some((8, 3)),
    },
    DungeonRoomDef {
        x: 0,
        y: 1,
        tiles: &[
            "################",
            "################",
            "##      $     ##",
            "##            ##",
            "##             o",
            "##             o",
            "##             o",
            "##   f         ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: &[],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 2,
        }],
        props: &[],
        map_visible: false,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 1,
        tiles: &[
            "################",
            "################",
            "##  $         ##",
            "##            ##",
            "o             ##",
            "o             ##",
            "o             ##",
            "##        f   ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: &[],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 3,
            tile_y: 2,
        }],
        props: &[],
        map_visible: false,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 0,
        tiles: &[
            "################",
            "##            ##",
            "##    ffff    ##",
            "##            ##",
            "##            ##",
            "##    f  f    ##",
            "##            ##",
            "##            ##",
            "##      s     ##",
            "##            ##",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Boss,
            tile_x: 7.0,
            tile_y: 4.0,
        }],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[
            ScreenItemDef {
                pickup_type: PickupType::DragonPiece,
                tile_x: 6,
                tile_y: 8,
            },
            ScreenItemDef {
                pickup_type: PickupType::Ladder,
                tile_x: 9,
                tile_y: 8,
            },
        ],
        boss_key_tile: None,
    },
];

const R02_LADDER_PROPS: &[PropDef] = &[
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 12,
        tile_y: 7,
        target_tile_x: Some(12),
        target_tile_y: Some(3),
    },
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 12,
        tile_y: 3,
        target_tile_x: Some(12),
        target_tile_y: Some(7),
    },
];

const R06_LADDER_PROPS: &[PropDef] = &[
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 8,
        tile_y: 7,
        target_tile_x: Some(8),
        target_tile_y: Some(3),
    },
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 8,
        tile_y: 3,
        target_tile_x: Some(8),
        target_tile_y: Some(7),
    },
];

const R07_BARRACKS_PROPS: &[PropDef] = &[
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 3,
        tile_y: 8,
        target_tile_x: Some(3),
        target_tile_y: Some(2),
    },
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 3,
        tile_y: 2,
        target_tile_x: Some(3),
        target_tile_y: Some(8),
    },
];

const LEVEL2_ROOMS: &[DungeonRoomDef] = &[
    DungeonRoomDef {
        x: 1,
        y: 7,
        tiles: &[
            "#######  #######",
            "##            ##",
            "##    ffff    ##",
            "##            ##",
            "##            ##",
            "##            ##",
            "##            ##",
            "##    ffff    ##",
            "##            ##",
            "#######  #######",
            "#######ss#######",
        ],
        enemies: &[],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 6,
        tiles: &[
            "#######ll#######",
            "##          ####",
            "##             #",
            "##          $ ##",
            "##             #",
            "##          ####",
            "##          ####",
            "##          ####",
            "##          ####",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 8.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 12,
            tile_y: 3,
        }],
        props: R02_LADDER_PROPS,
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 5,
        tiles: &[
            "#######  #######",
            "##   ##  ##   ##",
            "##    $       ##",
            "##            ##",
            "o      ##      o",
            "o              o",
            "o      ##      o",
            "##            ##",
            "##   ffff     ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 10.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 7.0,
                tile_y: 7.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 5,
            tile_y: 2,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 5,
        tiles: &[
            "################",
            "##    xxxx    ##",
            "##            ##",
            "##            ##",
            "o       $     ##",
            "o             ##",
            "o             ##",
            "##   ffff     ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 10.0,
                tile_y: 5.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 4,
        }],
        props: &[PropDef {
            kind: PropKind::Boulder,
            tile_x: 4,
            tile_y: 7,
            target_tile_x: None,
            target_tile_y: None,
        }],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 0,
        y: 4,
        tiles: &[
            "################",
            "##   f      $ ##",
            "##            ##",
            "##  ######    ##",
            "o              o",
            "o              o",
            "o              o",
            "##    ######  ##",
            "## $         f##",
            "################",
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 10.0,
                tile_y: 5.0,
            },
        ],
        items: &[
            ScreenItemDef {
                pickup_type: PickupType::BombAmmo,
                tile_x: 11,
                tile_y: 1,
            },
            ScreenItemDef {
                pickup_type: PickupType::Gem,
                tile_x: 3,
                tile_y: 8,
            },
        ],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 4,
        tiles: &[
            "#######  #######",
            "##   f    f   ##",
            "##            ##",
            "##      f     ##",
            "o      ##      o",
            "o              o",
            "o      ##      o",
            "##      f     ##",
            "##   f    f   ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Ironmaw,
            tile_x: 8.0,
            tile_y: 5.0,
        }],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 2,
        }],
        props: R06_LADDER_PROPS,
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 3,
        tiles: &[
            "#######  #######",
            "## $         ###",
            "##            ##",
            "##  ######    ##",
            "o              o",
            "o      xx      o",
            "o              o",
            "##    ######  ##",
            "###          ###",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 10.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 8.0,
                tile_y: 2.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 3,
            tile_y: 1,
        }],
        props: R07_BARRACKS_PROPS,
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 3,
        tiles: &[
            "################",
            "##            ##",
            "##   ffff     ##",
            "##            ##",
            "o             ##",
            "o       f     ##",
            "o             ##",
            "##     ffff   ##",
            "##    $       ##",
            "################",
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 5.0,
                tile_y: 3.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 10.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 5,
            tile_y: 8,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 2,
        tiles: &[
            "#######ll#######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~....$.....~##",
            "o..............o",
            "o....ffff......o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 10.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 1,
        tiles: &[
            "#######kk#######",
            "##   f    f   ##",
            "##            ##",
            "##      f     ##",
            "o              o",
            "o      ff      o",
            "o              o",
            "##     f      ##",
            "##   f    f   ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 11.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 8.0,
                tile_y: 2.0,
            },
        ],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::BossKey,
            tile_x: 8,
            tile_y: 5,
        }],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 0,
        tiles: &[
            "################",
            "##            ##",
            "##   ffff     ##",
            "##            ##",
            "##            ##",
            "##    f  f    ##",
            "##            ##",
            "##            ##",
            "##      s     ##",
            "##            ##",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Boss,
            tile_x: 7.0,
            tile_y: 4.0,
        }],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[
            ScreenItemDef {
                pickup_type: PickupType::DragonPiece,
                tile_x: 6,
                tile_y: 8,
            },
            ScreenItemDef {
                pickup_type: PickupType::Hammer,
                tile_x: 9,
                tile_y: 8,
            },
        ],
        boss_key_tile: None,
    },
];

const R07_WORKS_PROPS: &[PropDef] = &[
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 4,
        tile_y: 8,
        target_tile_x: Some(4),
        target_tile_y: Some(2),
    },
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 4,
        tile_y: 2,
        target_tile_x: Some(4),
        target_tile_y: Some(8),
    },
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 11,
        tile_y: 8,
        target_tile_x: Some(11),
        target_tile_y: Some(2),
    },
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 11,
        tile_y: 2,
        target_tile_x: Some(11),
        target_tile_y: Some(8),
    },
];

const LEVEL3_ROOMS: &[DungeonRoomDef] = &[
    DungeonRoomDef {
        x: 3,
        y: 6,
        tiles: &[
            "#######  #######",
            "##            ##",
            "##   ffff     ##",
            "##            ##",
            "##            ##",
            "##            ##",
            "##            ##",
            "##     $      ##",
            "##            ##",
            "#######  #######",
            "#######ss#######",
        ],
        enemies: &[],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 7,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 3,
        y: 5,
        tiles: &[
            "#######ll#######",
            "##            ##",
            "##   f    f   ##",
            "##            ##",
            "o              o",
            "o      ff      o",
            "o              o",
            "##   f    f   ##",
            "##            ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 10.0,
                tile_y: 6.0,
            },
        ],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 5,
        tiles: &[
            "################",
            "##    xxxx    ##",
            "##          $ ##",
            "##            ##",
            "##             o",
            "##             o",
            "##             o",
            "##   ffff     ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 9.0,
                tile_y: 6.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 11.0,
                tile_y: 4.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 11,
            tile_y: 2,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 4,
        y: 5,
        tiles: &[
            "################",
            "##   $        ##",
            "##            ##",
            "##  ffff      ##",
            "o              o",
            "o              o",
            "o              o",
            "##      ffff  ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 10.0,
                tile_y: 5.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 4,
            tile_y: 1,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 3,
        y: 4,
        tiles: &[
            "#######  #######",
            "##   f    f   ##",
            "##            ##",
            "##   ffff     ##",
            "o              o",
            "o              o",
            "o              o",
            "##     ffff   ##",
            "##            ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 8.0,
                tile_y: 3.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 8.0,
                tile_y: 7.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 5,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 4,
        tiles: &[
            "#######  #######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~....$.....~##",
            "o..............o",
            "o....ffff......o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 10.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::BombAmmo,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 4,
        y: 4,
        tiles: &[
            "#######ll#######",
            "##            ##",
            "##    $       ##",
            "##            ##",
            "o             ##",
            "o             ##",
            "o             ##",
            "##   ffff     ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 6.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 10.0,
                tile_y: 3.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 6,
            tile_y: 2,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 3,
        y: 3,
        tiles: &[
            "#######ll#######",
            "##   f    f   ##",
            "##            ##",
            "##            ##",
            "o      ##      o",
            "o              o",
            "o      ##      o",
            "##            ##",
            "##   f    f   ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 7.0,
                tile_y: 5.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 3.0,
                tile_y: 2.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 12.0,
                tile_y: 2.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 2,
        }],
        props: R07_WORKS_PROPS,
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 3,
        y: 2,
        tiles: &[
            "#######  #######",
            "##~~~~....~~~~##",
            "##~~....$...~~##",
            "##~..........~##",
            "o..............o",
            "o....ffff......o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 11.0,
                tile_y: 5.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 2,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 3,
        y: 1,
        tiles: &[
            "#######kk#######",
            "##   f    f   ##",
            "##            ##",
            "##   ffff     ##",
            "o              o",
            "o      ff      o",
            "o              o",
            "##     ffff   ##",
            "##            ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 11.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 8.0,
                tile_y: 6.0,
            },
        ],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::BossKey,
            tile_x: 8,
            tile_y: 5,
        }],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 3,
        y: 0,
        tiles: &[
            "################",
            "##            ##",
            "##    ffff    ##",
            "##            ##",
            "##            ##",
            "##    f  f    ##",
            "##            ##",
            "##            ##",
            "##      s     ##",
            "##            ##",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Boss,
            tile_x: 7.0,
            tile_y: 4.0,
        }],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[
            ScreenItemDef {
                pickup_type: PickupType::DragonPiece,
                tile_x: 6,
                tile_y: 8,
            },
            ScreenItemDef {
                pickup_type: PickupType::Raft,
                tile_x: 9,
                tile_y: 8,
            },
        ],
        boss_key_tile: None,
    },
];

const R06_TIDAL_PROPS: &[PropDef] = &[
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 3,
        tile_y: 8,
        target_tile_x: Some(3),
        target_tile_y: Some(2),
    },
    PropDef {
        kind: PropKind::LadderPoint,
        tile_x: 3,
        tile_y: 2,
        target_tile_x: Some(3),
        target_tile_y: Some(8),
    },
];

const LEVEL4_ROOMS: &[DungeonRoomDef] = &[
    DungeonRoomDef {
        x: 1,
        y: 6,
        tiles: &[
            "#######  #######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~..........~##",
            "##............##",
            "##............##",
            "##............##",
            "##~....$.....~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######ss#######",
        ],
        enemies: &[],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 7,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 5,
        tiles: &[
            "#######ll#######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~.....$....~##",
            "o..............o",
            "o..............o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 10.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 5,
        tiles: &[
            "################",
            "##~~~~....~~~~##",
            "##~~....$...~~##",
            "##~..........~##",
            "o..............o",
            "o..............o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "################",
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 6.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 10.0,
                tile_y: 5.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 2,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 4,
        tiles: &[
            "#######  #######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~.........$~##",
            "o..............o",
            "o..............o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 10.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 8.0,
                tile_y: 2.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 11,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 0,
        y: 4,
        tiles: &[
            "################",
            "##     $      ##",
            "##            ##",
            "##   ffff     ##",
            "##             o",
            "##             o",
            "##             o",
            "##     ffff   ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 6.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 10.0,
                tile_y: 5.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 6,
            tile_y: 1,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 4,
        tiles: &[
            "#######  #######",
            "##   f    f   ##",
            "##            ##",
            "##      $     ##",
            "o      ##      o",
            "o~~~~~~~~~~~~~~o",
            "o      ##      o",
            "##            ##",
            "##   f    f   ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 8.0,
                tile_y: 3.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 8.0,
                tile_y: 7.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 3,
        }],
        props: R06_TIDAL_PROPS,
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 3,
        tiles: &[
            "#######  #######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~....$.....~##",
            "o..............o",
            "o~~~~~~~~~~~~~~o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 4.0,
                tile_y: 3.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 11.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 3,
        tiles: &[
            "#######ll#######",
            "##~~~~....~~~~##",
            "##~~....$...~~##",
            "##~..........~##",
            "o..............o",
            "o~~~~~~~~~~~~~~o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 6.0,
                tile_y: 3.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 10.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 2,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 2,
        tiles: &[
            "#######  #######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~....$.....~##",
            "o..............o",
            "o~~~~~~~~~~~~~~o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 5.0,
                tile_y: 3.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 11.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 1,
        tiles: &[
            "#######kk#######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~..........~##",
            "o..............o",
            "o~~~~~~~~~~~~~~o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 11.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Borespat,
                tile_x: 8.0,
                tile_y: 2.0,
            },
        ],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::BossKey,
            tile_x: 8,
            tile_y: 5,
        }],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 0,
        tiles: &[
            "################",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~..........~##",
            "##............##",
            "##~~~~~~~~~~~~##",
            "##............##",
            "##~..........~##",
            "##~~~~..s.~~~~##",
            "##............##",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Boss,
            tile_x: 7.0,
            tile_y: 4.0,
        }],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[
            ScreenItemDef {
                pickup_type: PickupType::DragonPiece,
                tile_x: 6,
                tile_y: 8,
            },
            ScreenItemDef {
                pickup_type: PickupType::StrongArmGlove,
                tile_x: 9,
                tile_y: 8,
            },
        ],
        boss_key_tile: None,
    },
];

const GRIMFORGE_FIELDS_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Splort,
        tile_x: 4.0,
        tile_y: 5.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Ironmaw,
        tile_x: 11.0,
        tile_y: 4.0,
    },
];

const VOID_WASTES_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Shriekwing,
        tile_x: 6.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Ironmaw,
        tile_x: 10.0,
        tile_y: 6.0,
    },
];

const GRIMFORGE_APPROACHES_SCREENS: &[OverworldScreenDef] = &[
    OverworldScreenDef {
        x: 7,
        y: 5,
        biome_id: 5,
        name: "LAVA FIELDS WEST",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..x....xx....^^",
            "^.=..^^....^^..^",
            "^.=..^^....^^..^",
            "^.=......x.....^",
            "^.=..xx......=.^",
            "^.=......xx..=.^",
            "^.=..^^....^^=.^",
            "^.=..^^....^^=.^",
            "^............=.^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: GRIMFORGE_FIELDS_ENEMIES,
    },
    OverworldScreenDef {
        x: 8,
        y: 5,
        biome_id: 5,
        name: "ASCENT PATH",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^......==......^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^......==......^",
            "^...x..==..x...^",
            "^......==......^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^......==......^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: GRIMFORGE_FIELDS_ENEMIES,
    },
    OverworldScreenDef {
        x: 10,
        y: 3,
        biome_id: 5,
        name: "SUMMIT APPROACH",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^.....^^^^.....^",
            "^....^^dd^^....^",
            "^....^^^^^^....^",
            "^......==......^",
            "^......==......^",
            "^....^^^^^^....^",
            "^....^^^^^^....^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: Some(5),
        enemies: &[],
    },
    OverworldScreenDef {
        x: 8,
        y: 4,
        biome_id: 5,
        name: "MOUNTAINEER'S CAMP",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..^^^^..^^^^..^",
            "^..^..^..^..^..^",
            "^..^^^^==^^^^..^",
            "^......==......^",
            "^..____==____..^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 9,
        y: 4,
        biome_id: 5,
        name: "EMBER GARDEN",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^^^^^....^",
            "^...^^cccc^^...^",
            "^...^^cccc^^...^",
            "^....^^^^^^....^",
            "^......==......^",
            "^....xx==xx....^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: Some(CaveKind::EmberCrystal),
        dungeon: None,
        enemies: GRIMFORGE_FIELDS_ENEMIES,
    },
    OverworldScreenDef {
        x: 13,
        y: 3,
        biome_id: 5,
        name: "CALDERA VIEW",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^^^^^....^",
            "^...^^....^^...^",
            "^...^......^...^",
            "^...^..==..^...^",
            "^...^..==..^...^",
            "^...^......^...^",
            "^...^^....^^...^",
            "^....^^^^^^....^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Shriekwing,
            tile_x: 8.0,
            tile_y: 4.0,
        }],
    },
    OverworldScreenDef {
        x: 10,
        y: 6,
        biome_id: 5,
        name: "LOW PASS",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..^^......^^..^",
            "^..^^.====.^^..^",
            "^.....====.....^",
            "^.....====.....^",
            "^..^^.====.^^..^",
            "^..^^......^^..^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: GRIMFORGE_FIELDS_ENEMIES,
    },
    OverworldScreenDef {
        x: 11,
        y: 5,
        biome_id: 5,
        name: "LAVA FIELDS EAST",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..x..^^..x....^",
            "^..x..^^..x....^",
            "^......==......^",
            "^..xx..==..xx..^",
            "^......==......^",
            "^....x.==.x....^",
            "^......==......^",
            "^..^^..==..^^..^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: GRIMFORGE_FIELDS_ENEMIES,
    },
    OverworldScreenDef {
        x: 12,
        y: 6,
        biome_id: 5,
        name: "ASH SLOPE",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^......==......^",
            "^....xx==xx....^",
            "^......==......^",
            "^....^^==^^....^",
            "^......==......^",
            "^....xx==xx....^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: GRIMFORGE_FIELDS_ENEMIES,
    },
];

const VOID_WASTES_SCREENS: &[OverworldScreenDef] = &[
    OverworldScreenDef {
        x: 2,
        y: 4,
        biome_id: 6,
        name: "SANCTUM APPROACH",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^......==......^",
            "^...^^.dd.^^...^",
            "^...^^....^^...^",
            "^......==......^",
            "^......==......^",
            "^...^^....^^...^",
            "^...^^....^^...^",
            "^......==......^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: Some(6),
        enemies: &[],
    },
    OverworldScreenDef {
        x: 4,
        y: 5,
        biome_id: 6,
        name: "PALE ROAD",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^.....======...^",
            "^............=.^",
            "^...======....=^",
            "^...=........==^",
            "^...=....======^",
            "^...=..........^",
            "^...======.....^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: VOID_WASTES_ENEMIES,
    },
    OverworldScreenDef {
        x: 6,
        y: 4,
        biome_id: 6,
        name: "FIRST STABLE RIFT",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^....^^..^^....^",
            "^....^.cc.^....^",
            "^....^.cc.^....^",
            "^....^^..^^....^",
            "^......==......^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: VOID_WASTES_ENEMIES,
    },
    OverworldScreenDef {
        x: 3,
        y: 6,
        biome_id: 6,
        name: "RIFT FIELDS",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..^^......^^..^",
            "^..^..==....^..^",
            "^..^......==.^..^",
            "^..^....==...^..^",
            "^..^..==....^..^",
            "^..^^......^^..^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: VOID_WASTES_ENEMIES,
    },
    OverworldScreenDef {
        x: 5,
        y: 5,
        biome_id: 6,
        name: "MIRROR POOL",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^....~~~~~~....^",
            "^...~~cccc~~...^",
            "^...~~cccc~~...^",
            "^....~~~~~~....^",
            "^......==......^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: Some(CaveKind::VoidCompass),
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 7,
        y: 6,
        biome_id: 6,
        name: "SHADE'S CIRCUIT",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^......==......^",
            "^..^^..==..^^..^",
            "^..^^......^^..^",
            "^......==......^",
            "^..^^..==..^^..^",
            "^..^^......^^..^",
            "^......==......^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: VOID_WASTES_ENEMIES,
    },
    OverworldScreenDef {
        x: 2,
        y: 7,
        biome_id: 6,
        name: "LOW MOUNTAIN PASS",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..^^.====.^^..^",
            "^.....====.....^",
            "^.....====.....^",
            "^..^^.====.^^..^",
            "^..............^",
            "^..............^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: VOID_WASTES_ENEMIES,
    },
    OverworldScreenDef {
        x: 5,
        y: 6,
        biome_id: 6,
        name: "SECOND STABLE RIFT",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^....^^..^^....^",
            "^....^.cc.^....^",
            "^....^....^....^",
            "^....^.cc.^....^",
            "^....^^..^^....^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: VOID_WASTES_ENEMIES,
    },
    OverworldScreenDef {
        x: 6,
        y: 7,
        biome_id: 6,
        name: "OUTER WASTES",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..^^......^^..^",
            "^..............^",
            "^......==......^",
            "^..............^",
            "^..^^......^^..^",
            "^..............^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: VOID_WASTES_ENEMIES,
    },
];

const LEVEL5_ROOMS: &[DungeonRoomDef] = &[
    DungeonRoomDef {
        x: 1,
        y: 3,
        tiles: &[
            "#######  #######",
            "##   ffff     ##",
            "##            ##",
            "##      $     ##",
            "##            ##",
            "##            ##",
            "##            ##",
            "##     ffff   ##",
            "##            ##",
            "#######  #######",
            "#######ss#######",
        ],
        enemies: &[],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 2,
        tiles: &[
            "#######  #######",
            "##   ffff     ##",
            "##            ##",
            "##      $     ##",
            "o            lo",
            "o            lo",
            "o            lo",
            "##   ffff     ##",
            "##            ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: GRIMFORGE_FIELDS_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 0,
        y: 2,
        tiles: &[
            "################",
            "##   ffff     ##",
            "##            ##",
            "##   $        ##",
            "##            oo",
            "##     ff     oo",
            "##            oo",
            "##        $   ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: GRIMFORGE_FIELDS_ENEMIES,
        items: &[
            ScreenItemDef {
                pickup_type: PickupType::Gem,
                tile_x: 4,
                tile_y: 3,
            },
            ScreenItemDef {
                pickup_type: PickupType::Key,
                tile_x: 9,
                tile_y: 7,
            },
        ],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 2,
        tiles: &[
            "################",
            "##   ffff     ##",
            "##            ##",
            "##      $     ##",
            "oo   ######   ##",
            "oo   ######   ##",
            "oo            ##",
            "##     ff     ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: GRIMFORGE_FIELDS_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 1,
        tiles: &[
            "#######  #######",
            "##   ffff     ##",
            "##            ##",
            "##      $     ##",
            "o     ####     o",
            "o            llo",
            "o     ####     o",
            "##     ff     ##",
            "##            ##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 3.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 10.0,
                tile_y: 6.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[
            PropDef {
                kind: PropKind::Boulder,
                tile_x: 5,
                tile_y: 5,
                target_tile_x: None,
                target_tile_y: None,
            },
            PropDef {
                kind: PropKind::PressurePlate,
                tile_x: 10,
                tile_y: 5,
                target_tile_x: None,
                target_tile_y: None,
            },
        ],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 1,
        tiles: &[
            "#######  #######",
            "##   ffff     ##",
            "##            ##",
            "##   ######   ##",
            "oo            ##",
            "oo      $     ##",
            "oo            ##",
            "##   ######   ##",
            "##            ##",
            "#######kk#######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 4.0,
                tile_y: 5.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Splort,
                tile_x: 11.0,
                tile_y: 5.0,
            },
        ],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::BossKey,
            tile_x: 8,
            tile_y: 5,
        }],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 0,
        tiles: &[
            "################",
            "##   ffff     ##",
            "##            ##",
            "##   ######   ##",
            "##     s      ##",
            "##    ....    ##",
            "##            ##",
            "##   ######   ##",
            "##    ..s.    ##",
            "##            ##",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Boss,
            tile_x: 8.0,
            tile_y: 4.0,
        }],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::DragonPiece,
            tile_x: 8,
            tile_y: 8,
        }],
        boss_key_tile: None,
    },
];

const LEVEL6_ROOMS: &[DungeonRoomDef] = &[
    DungeonRoomDef {
        x: 1,
        y: 3,
        tiles: &[
            "#######  #######",
            "##............##",
            "##............##",
            "##......$.....##",
            "##............##",
            "##............##",
            "##............##",
            "##............##",
            "##............##",
            "#######  #######",
            "#######ss#######",
        ],
        enemies: &[],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 2,
        tiles: &[
            "#######  #######",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "o.....^..^....o",
            "o.....^..^....o",
            "o.....^^^^....o",
            "##......$.....##",
            "##............##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: VOID_WASTES_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 7,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 0,
        y: 2,
        tiles: &[
            "################",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "##......$......o",
            "##....^..^.....o",
            "##....^^^^.....o",
            "##............##",
            "##............##",
            "################",
            "################",
        ],
        enemies: VOID_WASTES_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 4,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 2,
        tiles: &[
            "################",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "o......$.....##",
            "o....^..^....##",
            "o....^^^^....##",
            "##............##",
            "##............##",
            "################",
            "################",
        ],
        enemies: VOID_WASTES_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 7,
            tile_y: 4,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 1,
        tiles: &[
            "#######  #######",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "o......$.....lo",
            "o....^..^....lo",
            "o....^^^^....lo",
            "##............##",
            "##............##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 10.0,
                tile_y: 4.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 4,
        }],
        props: &[
            PropDef {
                kind: PropKind::PressurePlate,
                tile_x: 8,
                tile_y: 5,
                target_tile_x: None,
                target_tile_y: None,
            },
            PropDef {
                kind: PropKind::Boulder,
                tile_x: 5,
                tile_y: 6,
                target_tile_x: None,
                target_tile_y: None,
            },
        ],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 1,
        tiles: &[
            "#######ll#######",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "o............llo",
            "o......$.....llo",
            "o............llo",
            "##....^^^^....##",
            "##............##",
            "#######kk#######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 5.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 11.0,
                tile_y: 5.0,
            },
        ],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::BossKey,
            tile_x: 8,
            tile_y: 5,
        }],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 0,
        tiles: &[
            "################",
            "##............##",
            "##....^^^^....##",
            "##....^ss^....##",
            "##............##",
            "##......$.....##",
            "##............##",
            "##....^..^....##",
            "##....^ss^....##",
            "##............##",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Boss,
            tile_x: 8.0,
            tile_y: 4.0,
        }],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[
            ScreenItemDef {
                pickup_type: PickupType::DragonPiece,
                tile_x: 6,
                tile_y: 8,
            },
            ScreenItemDef {
                pickup_type: PickupType::PortalTool,
                tile_x: 9,
                tile_y: 8,
            },
        ],
        boss_key_tile: None,
    },
];

const CELESTIAL_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Ironmaw,
        tile_x: 5.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Shriekwing,
        tile_x: 10.0,
        tile_y: 3.0,
    },
];

const DRAGON_APPROACH_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Ironmaw,
        tile_x: 7.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Shriekwing,
        tile_x: 11.0,
        tile_y: 5.0,
    },
];

const CELESTIAL_PLATEAU_SCREENS: &[OverworldScreenDef] = &[
    OverworldScreenDef {
        x: 5,
        y: 2,
        biome_id: 7,
        name: "WHITE PLAIN WEST",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^....==....==..^",
            "^..............^",
            "^..^^......^^..^",
            "^..............^",
            "^..==......==..^",
            "^..............^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: CELESTIAL_ENEMIES,
    },
    OverworldScreenDef {
        x: 8,
        y: 2,
        biome_id: 7,
        name: "SPIRE BASE",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^^^^^....^",
            "^...^^dddd^^...^",
            "^...^^dddd^^...^",
            "^....^^^^^^....^",
            "^......==......^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: Some(7),
        enemies: &[],
    },
    OverworldScreenDef {
        x: 11,
        y: 2,
        biome_id: 7,
        name: "STAR MAP PLAZA",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^...^^^^..^^^^.^",
            "^...^..^..^..^.^",
            "^...^^^^==^^^^.^",
            "^......==......^",
            "^...^^^^==^^^^.^",
            "^...^..^..^..^.^",
            "^...^^^^..^^^^.^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: CELESTIAL_ENEMIES,
    },
    OverworldScreenDef {
        x: 5,
        y: 1,
        biome_id: 7,
        name: "PLATEAU EDGE",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..............^",
            "^..==......==..^",
            "^..............^",
            "^..^^......^^..^",
            "^..............^",
            "^..==......==..^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: CELESTIAL_ENEMIES,
    },
    OverworldScreenDef {
        x: 9,
        y: 1,
        biome_id: 7,
        name: "MERCHANT'S CIRCUIT",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^....^^^^^^....^",
            "^....^cccc^....^",
            "^....^cccc^....^",
            "^....^^^^^^....^",
            "^......==......^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: Some(CaveKind::StarSigil),
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 10,
        y: 0,
        biome_id: 7,
        name: "MERCHANT'S LANTERN",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^......==......^",
            "^......==......^",
            "^....^^..^^....^",
            "^....^^..^^....^",
            "^......==......^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 6,
        y: 3,
        biome_id: 7,
        name: "WHITE PLAIN SOUTH",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..^^......^^..^",
            "^..............^",
            "^......==......^",
            "^......==......^",
            "^..............^",
            "^..^^......^^..^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: CELESTIAL_ENEMIES,
    },
    OverworldScreenDef {
        x: 7,
        y: 3,
        biome_id: 7,
        name: "SKY MOAT OVERLOOK",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....~~~~~~....^",
            "^...~~....~~...^",
            "^...~~....~~...^",
            "^....~~~~~~....^",
            "^......==......^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: CELESTIAL_ENEMIES,
    },
    OverworldScreenDef {
        x: 5,
        y: 3,
        biome_id: 7,
        name: "WEST FACE STAIRS",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^......==......^",
            "^......==......^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^......==......^",
            "^......==......^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^......==......^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: CELESTIAL_ENEMIES,
    },
];

const DRAGONS_APPROACH_SCREENS: &[OverworldScreenDef] = &[
    OverworldScreenDef {
        x: 1,
        y: 4,
        biome_id: 8,
        name: "VALLEY MOUTH",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^^^^^....^",
            "^...^^....^^...^",
            "^...^..==..^...^",
            "^...^..==..^...^",
            "^...^..==..^...^",
            "^...^^....^^...^",
            "^....^^^^^^....^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: DRAGON_APPROACH_ENEMIES,
    },
    OverworldScreenDef {
        x: 2,
        y: 0,
        biome_id: 8,
        name: "FINAL APPROACH",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^^^^^....^",
            "^...^^dddd^^...^",
            "^...^^dddd^^...^",
            "^...^^....^^...^",
            "^....^^..^^....^",
            "^......==......^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: Some(8),
        enemies: DRAGON_APPROACH_ENEMIES,
    },
    OverworldScreenDef {
        x: 4,
        y: 0,
        biome_id: 8,
        name: "THRONE NICHE",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^^^^^....^",
            "^...^^cccc^^...^",
            "^...^^....^^...^",
            "^...^^....^^...^",
            "^....^^^^^^....^",
            "^......==......^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: Some(CaveKind::CrystalOfSeeing),
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 2,
        y: 2,
        biome_id: 8,
        name: "PILGRIM'S ROAD",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: DRAGON_APPROACH_ENEMIES,
    },
    OverworldScreenDef {
        x: 1,
        y: 3,
        biome_id: 8,
        name: "LAST CAMP",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..^^^^....^^^^^",
            "^..^..^....^..^^",
            "^..^^^^.cc.^^^^^",
            "^......====.....^",
            "^..____====____.^",
            "^......====.....^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: Some(CaveKind::DragonCodex),
        dungeon: None,
        enemies: &[],
    },
    OverworldScreenDef {
        x: 4,
        y: 2,
        biome_id: 8,
        name: "MEMORY ALCOVES",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^..^^..^^..^^..^",
            "^..^^..^^..^^..^",
            "^......==......^",
            "^..^^..==..^^..^",
            "^..^^..==..^^..^",
            "^......==......^",
            "^..^^..^^..^^..^",
            "^..^^..^^..^^..^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: DRAGON_APPROACH_ENEMIES,
    },
    OverworldScreenDef {
        x: 3,
        y: 2,
        biome_id: 8,
        name: "THRESHOLD STONE",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^^^^^....^",
            "^....^....^....^",
            "^....^.==.^....^",
            "^....^.==.^....^",
            "^....^....^....^",
            "^....^^^^^^....^",
            "^......==......^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: DRAGON_APPROACH_ENEMIES,
    },
    OverworldScreenDef {
        x: 3,
        y: 1,
        biome_id: 8,
        name: "INNER ROAD",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^....^^==^^....^",
            "^..............^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: DRAGON_APPROACH_ENEMIES,
    },
    OverworldScreenDef {
        x: 4,
        y: 1,
        biome_id: 8,
        name: "VALLEY END",
        tiles: &[
            "^^^^^^^^^^^^^^^^",
            "^....^^^^^^....^",
            "^...^^....^^...^",
            "^...^......^...^",
            "^...^..==..^...^",
            "^...^..==..^...^",
            "^...^......^...^",
            "^...^^....^^...^",
            "^....^^^^^^....^",
            "^..............^",
            "^^^^^^^^^^^^^^^^",
        ],
        cave: None,
        dungeon: None,
        enemies: DRAGON_APPROACH_ENEMIES,
    },
];

const LEVEL7_ROOMS: &[DungeonRoomDef] = &[
    DungeonRoomDef {
        x: 1,
        y: 3,
        tiles: &[
            "#######  #######",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "##......$.....##",
            "##....^..^....##",
            "##....^^^^....##",
            "##............##",
            "##............##",
            "#######  #######",
            "#######ss#######",
        ],
        enemies: &[],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 4,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 2,
        tiles: &[
            "#######  #######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~......$...~##",
            "o..............o",
            "o..............o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: CELESTIAL_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 0,
        y: 2,
        tiles: &[
            "################",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "##......$......o",
            "##....^..^.....o",
            "##....^^^^.....o",
            "##............##",
            "##............##",
            "################",
            "################",
        ],
        enemies: CELESTIAL_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 4,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 2,
        tiles: &[
            "################",
            "##............##",
            "##~~~~....~~~~##",
            "##~~......$.~~##",
            "o..............##",
            "o..............##",
            "o..............##",
            "##~~........~~##",
            "##~~~~....~~~~##",
            "################",
            "################",
        ],
        enemies: CELESTIAL_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 10,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 1,
        tiles: &[
            "#######  #######",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "o......$.....lo",
            "o....^..^....lo",
            "o....^^^^....lo",
            "##............##",
            "##............##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 5.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 11.0,
                tile_y: 4.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 4,
        }],
        props: &[
            PropDef {
                kind: PropKind::PressurePlate,
                tile_x: 8,
                tile_y: 5,
                target_tile_x: None,
                target_tile_y: None,
            },
            PropDef {
                kind: PropKind::Boulder,
                tile_x: 5,
                tile_y: 6,
                target_tile_x: None,
                target_tile_y: None,
            },
        ],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 1,
        tiles: &[
            "#######ll#######",
            "##............##",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "o......$.....llo",
            "o~~~~~~~~~~~~llo",
            "o............llo",
            "##~~........~~##",
            "##~~~~....~~~~##",
            "#######kk#######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 11.0,
                tile_y: 4.0,
            },
        ],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::BossKey,
            tile_x: 8,
            tile_y: 4,
        }],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 0,
        tiles: &[
            "################",
            "##............##",
            "##....^^^^....##",
            "##....^ss^....##",
            "##......$.....##",
            "##............##",
            "##....^^^^....##",
            "##....^ss^....##",
            "##............##",
            "##............##",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Boss,
            tile_x: 8.0,
            tile_y: 4.0,
        }],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::DragonPiece,
            tile_x: 8,
            tile_y: 7,
        }],
        boss_key_tile: None,
    },
];

const LEVEL8_ROOMS: &[DungeonRoomDef] = &[
    DungeonRoomDef {
        x: 1,
        y: 3,
        tiles: &[
            "#######  #######",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "##......$.....##",
            "##....^..^....##",
            "##....^^^^....##",
            "##............##",
            "##............##",
            "#######  #######",
            "#######ss#######",
        ],
        enemies: &[],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 4,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 2,
        tiles: &[
            "#######  #######",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "##~......$...~##",
            "o..............o",
            "o~~~~~~~~~~~~~~o",
            "o..............o",
            "##~..........~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: DRAGON_APPROACH_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 8,
            tile_y: 3,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 0,
        y: 2,
        tiles: &[
            "################",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "##......$......o",
            "##....^..^.....o",
            "##....^^^^.....o",
            "##............##",
            "##............##",
            "################",
            "################",
        ],
        enemies: DRAGON_APPROACH_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 4,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 2,
        tiles: &[
            "################",
            "##............##",
            "##....^^^^....##",
            "##....^..^....##",
            "o......$.....##",
            "o....^..^....##",
            "o....^^^^....##",
            "##............##",
            "##............##",
            "################",
            "################",
        ],
        enemies: DRAGON_APPROACH_ENEMIES,
        items: &[ScreenItemDef {
            pickup_type: PickupType::Key,
            tile_x: 7,
            tile_y: 4,
        }],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 1,
        y: 1,
        tiles: &[
            "#######  #######",
            "##............##",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "o......$.....lo",
            "o~~~~~~~~~~~~lo",
            "o......$.....lo",
            "##~~........~~##",
            "##~~~~....~~~~##",
            "#######  #######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 11.0,
                tile_y: 6.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 8.0,
                tile_y: 2.0,
            },
        ],
        items: &[ScreenItemDef {
            pickup_type: PickupType::Gem,
            tile_x: 8,
            tile_y: 4,
        }],
        props: &[
            PropDef {
                kind: PropKind::PressurePlate,
                tile_x: 8,
                tile_y: 5,
                target_tile_x: None,
                target_tile_y: None,
            },
            PropDef {
                kind: PropKind::Boulder,
                tile_x: 5,
                tile_y: 6,
                target_tile_x: None,
                target_tile_y: None,
            },
            PropDef {
                kind: PropKind::Boulder,
                tile_x: 10,
                tile_y: 6,
                target_tile_x: None,
                target_tile_y: None,
            },
        ],
        map_visible: true,
        room_clear_rewards: &[],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 1,
        tiles: &[
            "#######ll#######",
            "##............##",
            "##~~~~....~~~~##",
            "##~~........~~##",
            "o............llo",
            "o~~~~~~~~~~~~llo",
            "o............llo",
            "##~~........~~##",
            "##~~~~....~~~~##",
            "#######kk#######",
            "#######  #######",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Ironmaw,
                tile_x: 11.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Shriekwing,
                tile_x: 8.0,
                tile_y: 3.0,
            },
        ],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::BossKey,
            tile_x: 8,
            tile_y: 4,
        }],
        boss_key_tile: None,
    },
    DungeonRoomDef {
        x: 2,
        y: 0,
        tiles: &[
            "################",
            "##............##",
            "##....^^^^....##",
            "##....^ss^....##",
            "##......$.....##",
            "##............##",
            "##....^^^^....##",
            "##....^ss^....##",
            "##............##",
            "##............##",
            "#######  #######",
        ],
        enemies: &[SpawnDef {
            enemy_type: EnemyType::Boss,
            tile_x: 8.0,
            tile_y: 4.0,
        }],
        items: &[],
        props: &[],
        map_visible: true,
        room_clear_rewards: &[ScreenItemDef {
            pickup_type: PickupType::DragonPiece,
            tile_x: 8,
            tile_y: 7,
        }],
        boss_key_tile: None,
    },
];

const AUTHORED_DUNGEONS: &[DungeonDef] = &[
    DungeonDef {
        id: 1,
        name: "MOSSHAVEN CAVE",
        entry: (1, 5),
        rooms: LEVEL1_ROOMS,
    },
    DungeonDef {
        id: 2,
        name: "RUINS OF ASHENFALL",
        entry: (1, 7),
        rooms: LEVEL2_ROOMS,
    },
    DungeonDef {
        id: 3,
        name: "IRONCLAD VAULT",
        entry: (3, 6),
        rooms: LEVEL3_ROOMS,
    },
    DungeonDef {
        id: 4,
        name: "SUNKEN CITADEL",
        entry: (1, 6),
        rooms: LEVEL4_ROOMS,
    },
    DungeonDef {
        id: 5,
        name: "GRIMFORGE DEPTHS",
        entry: (1, 3),
        rooms: LEVEL5_ROOMS,
    },
    DungeonDef {
        id: 6,
        name: "FRACTURED SANCTUM",
        entry: (1, 3),
        rooms: LEVEL6_ROOMS,
    },
    DungeonDef {
        id: 7,
        name: "AETHERIAN SPIRE",
        entry: (1, 3),
        rooms: LEVEL7_ROOMS,
    },
    DungeonDef {
        id: 8,
        name: "DRAGON'S ETERNAL THRONE",
        entry: (1, 3),
        rooms: LEVEL8_ROOMS,
    },
];

fn authored_overworld_screen(x: i32, y: i32) -> Option<&'static OverworldScreenDef> {
    MOSSHAVEN_SCREENS
        .iter()
        .chain(ASHENFALL_SCREENS.iter())
        .chain(IRONHIGHLANDS_SCREENS.iter())
        .chain(SUNKEN_COAST_SCREENS.iter())
        .chain(GRIMFORGE_APPROACHES_SCREENS.iter())
        .chain(VOID_WASTES_SCREENS.iter())
        .chain(CELESTIAL_PLATEAU_SCREENS.iter())
        .chain(DRAGONS_APPROACH_SCREENS.iter())
        .find(|screen| screen.x == x && screen.y == y)
}

fn generated_overworld_screen(x: i32, y: i32) -> Option<&'static GeneratedOverworldScreen> {
    GENERATED_OVERWORLD_SCREENS
        .iter()
        .find(|screen| screen.x == x && screen.y == y)
}

fn cave_kind_from_code(code: &str) -> Option<CaveKind> {
    Some(match code {
        "Sword" => CaveKind::Sword,
        "Heart" => CaveKind::Heart,
        "Shrine" => CaveKind::Shrine,
        "Sanctum" => CaveKind::Sanctum,
        "Bombs" => CaveKind::Bombs,
        "Shop" => CaveKind::Shop,
        "AncientKey" => CaveKind::AncientKey,
        "TideChart" => CaveKind::TideChart,
        "EmberCrystal" => CaveKind::EmberCrystal,
        "VoidCompass" => CaveKind::VoidCompass,
        "StarSigil" => CaveKind::StarSigil,
        "DragonCodex" => CaveKind::DragonCodex,
        "CrystalOfSeeing" => CaveKind::CrystalOfSeeing,
        _ => return None,
    })
}

fn authored_dungeon(id: i32) -> Option<&'static DungeonDef> {
    AUTHORED_DUNGEONS.iter().find(|dungeon| dungeon.id == id)
}

fn authored_dungeon_room(dungeon_id: i32, x: i32, y: i32) -> Option<&'static DungeonRoomDef> {
    authored_dungeon(dungeon_id)?
        .rooms
        .iter()
        .find(|room| room.x == x && room.y == y)
}

pub fn dungeon_map_rooms(dungeon_id: i32) -> HashSet<String> {
    if let Some(dungeon) = authored_dungeon(dungeon_id) {
        return dungeon
            .rooms
            .iter()
            .filter(|room| room.map_visible)
            .map(|room| screen_key(room.x, room.y))
            .collect();
    }

    build_dungeons()
        .get(&dungeon_id)
        .map(|rooms| rooms.keys().cloned().collect())
        .unwrap_or_default()
}

pub fn boss_key_spawn_tile(
    dungeon_id: i32,
    screen_x: i32,
    screen_y: i32,
) -> Option<(usize, usize)> {
    authored_dungeon_room(dungeon_id, screen_x, screen_y)?.boss_key_tile
}

pub fn screen_props(
    screen_x: i32,
    screen_y: i32,
    in_dungeon: bool,
    dungeon_id: i32,
    in_interior: bool,
    interior_id: &str,
) -> Vec<WorldProp> {
    if in_interior {
        if let Some(interior) = interior_by_id(interior_id) {
            return prop_defs(interior.props);
        }
    } else if in_dungeon {
        if let Some(room) = authored_dungeon_room(dungeon_id, screen_x, screen_y) {
            return prop_defs(room.props);
        }
    } else {
        return overworld_screen_props(screen_x, screen_y);
    }
    vec![]
}

pub fn room_clear_rewards(dungeon_id: i32, screen_x: i32, screen_y: i32) -> Vec<ItemDef> {
    if let Some(room) = authored_dungeon_room(dungeon_id, screen_x, screen_y) {
        return item_defs(room.room_clear_rewards);
    }

    match dungeon_id {
        2 if screen_x == 0 && screen_y == 1 => vec![ItemDef {
            pickup_type: PickupType::Key,
            tile_x: 7,
            tile_y: 5,
        }],
        3 if screen_x == 0 && screen_y == 1 => vec![ItemDef {
            pickup_type: PickupType::Key,
            tile_x: 7,
            tile_y: 5,
        }],
        4 if screen_x == 2 && screen_y == 1 => vec![ItemDef {
            pickup_type: PickupType::Key,
            tile_x: 7,
            tile_y: 5,
        }],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overworld_connections_are_bidirectional() {
        let world = build_overworld();
        assert_eq!(world.len(), (WORLD_W * WORLD_H) as usize);
        assert!(world.contains_key("1,13"));
        assert!(world.contains_key("3,0"));
    }

    #[test]
    fn mosshaven_overrides_and_dungeon_metadata_exist() {
        assert_eq!(overworld_start(), (1, 13));
        assert_eq!(location_name(1, 8, false, 0, false, ""), "THE FEN");
        assert_eq!(dungeon_at(1, 8), 1);
        assert_eq!(location_name(0, 0, true, 1, false, ""), "MOSSHAVEN CAVE");
        assert_eq!(location_name(2, 7, false, 0, false, ""), "ENGINEER'S TOWER");
        assert_eq!(dungeon_at(3, 4), 3);
        assert_eq!(location_name(3, 6, true, 3, false, ""), "IRONCLAD VAULT");
        assert_eq!(location_name(9, 6, false, 0, false, ""), "LIGHTHOUSE ISLE");
        assert_eq!(dungeon_at(11, 9), 4);
        assert_eq!(location_name(1, 6, true, 4, false, ""), "SUNKEN CITADEL");
        assert_eq!(location_name(7, 6, false, 0, false, ""), "EMBER GARDEN");
        assert_eq!(dungeon_at(8, 4), 5);
        assert_eq!(location_name(1, 3, true, 5, false, ""), "GRIMFORGE DEPTHS");
        assert_eq!(location_name(6, 5, false, 0, false, ""), "MIRROR POOL");
        assert_eq!(dungeon_at(4, 3), 6);
        assert_eq!(location_name(1, 3, true, 6, false, ""), "FRACTURED SANCTUM");
        assert_eq!(location_name(8, 0, false, 0, false, ""), "MERCHANT LANTERN");
        assert_eq!(dungeon_at(7, 2), 7);
        assert_eq!(location_name(1, 3, true, 7, false, ""), "AETHERIAN SPIRE");
        assert_eq!(location_name(4, 2, false, 0, false, ""), "LAST CAMP");
        assert_eq!(dungeon_at(3, 0), 8);
        assert_eq!(
            location_name(1, 3, true, 8, false, ""),
            "DRAGON'S ETERNAL THRONE"
        );

        let rooms = dungeon_map_rooms(1);
        assert!(rooms.contains("1,5"));
        assert!(rooms.contains("1,1"));
        assert!(!rooms.contains("0,1"));
        assert!(!rooms.contains("2,1"));

        let vault_rooms = dungeon_map_rooms(3);
        assert!(vault_rooms.contains("3,6"));
        assert!(vault_rooms.contains("3,0"));

        let citadel_rooms = dungeon_map_rooms(4);
        assert!(citadel_rooms.contains("1,6"));
        assert!(citadel_rooms.contains("2,0"));

        let grimforge_rooms = dungeon_map_rooms(5);
        assert!(grimforge_rooms.contains("1,3"));
        assert!(grimforge_rooms.contains("2,0"));

        let sanctum_rooms = dungeon_map_rooms(6);
        assert!(sanctum_rooms.contains("1,3"));
        assert!(sanctum_rooms.contains("2,0"));

        let spire_rooms = dungeon_map_rooms(7);
        assert!(spire_rooms.contains("1,3"));
        assert!(spire_rooms.contains("2,0"));

        let throne_rooms = dungeon_map_rooms(8);
        assert!(throne_rooms.contains("1,3"));
        assert!(throne_rooms.contains("2,0"));
    }

    #[test]
    fn mosshaven_room_props_and_rewards_are_authored() {
        let r04_props = screen_props(1, 3, true, 1, false, "");
        assert_eq!(r04_props.len(), 2);

        let r07_props = screen_props(1, 1, true, 1, false, "");
        assert_eq!(r07_props.len(), 8);
        assert_eq!(boss_key_spawn_tile(1, 1, 1), Some((8, 3)));

        let boss_rewards = room_clear_rewards(1, 1, 0);
        assert_eq!(boss_rewards.len(), 2);
        assert!(
            boss_rewards
                .iter()
                .any(|item| item.pickup_type == PickupType::DragonPiece)
        );
        assert!(
            boss_rewards
                .iter()
                .any(|item| item.pickup_type == PickupType::Ladder)
        );
    }

    #[test]
    fn overworld_npcs_and_gem_caches_are_authored() {
        let meadow_props = screen_props(1, 13, false, 0, false, "");
        assert!(
            meadow_props
                .iter()
                .any(|prop| matches!(prop.kind, PropKind::Npc(NpcKind::Barnett)))
        );

        let circuit_props = screen_props(8, 0, false, 0, false, "");
        assert!(
            !circuit_props
                .iter()
                .any(|prop| matches!(prop.kind, PropKind::Npc(NpcKind::CelestialMerchant)))
        );

        let wren_props = screen_props(4, 2, false, 0, false, "");
        assert!(
            !wren_props
                .iter()
                .any(|prop| matches!(prop.kind, PropKind::Npc(NpcKind::Wren)))
        );

        let merchant_house_props = screen_props(8, 0, false, 0, true, "merchant_lantern_house");
        assert!(
            merchant_house_props
                .iter()
                .any(|prop| matches!(prop.kind, PropKind::Npc(NpcKind::CelestialMerchant)))
        );

        let wren_house_props = screen_props(4, 2, false, 0, true, "wren_house");
        assert!(
            wren_house_props
                .iter()
                .any(|prop| matches!(prop.kind, PropKind::Npc(NpcKind::Wren)))
        );

        let fen_loot = screen_items(1, 8, false, 0, false, "");
        assert!(fen_loot.len() >= 2);
        assert!(
            fen_loot
                .iter()
                .all(|item| item.pickup_type == PickupType::Gem)
        );

        let plaza_loot = screen_items(9, 2, false, 0, false, "");
        assert!(
            plaza_loot
                .iter()
                .any(|item| item.pickup_type == PickupType::Gem)
        );
    }
}

// ---------------------------------------------------------------------------
// Dungeons
// ---------------------------------------------------------------------------

pub fn build_dungeons() -> HashMap<i32, HashMap<String, TileGrid>> {
    let mut all = HashMap::new();

    if let Some(dungeon) = authored_dungeon(1) {
        let mut d = HashMap::new();
        for room in dungeon.rooms {
            d.insert(screen_key(room.x, room.y), parse(room.tiles));
        }
        all.insert(dungeon.id, d);
    }

    // ===== DUNGEON 2: Forest Shrine (entrance at overworld 6,10) =====
    // Layout:      [1,0] boss
    //               |
    //        [0,1]-[1,1]-[2,1]
    //               |
    //              [1,2] entry
    {
        let mut d = HashMap::new();
        d.insert(
            "1,2".into(),
            parse(&[
                "#######  #######",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "#######  #######",
                "#######ss#######",
            ]),
        );
        d.insert(
            "1,1".into(),
            parse(&[
                "#######kk#######",
                "##            ##",
                "##            ##",
                "##   f    f   ##",
                "o              l",
                "o              l",
                "o              l",
                "##   f    f   ##",
                "##            ##",
                "##            ##",
                "#######  #######",
            ]),
        );
        d.insert(
            "0,1".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##  f      f  ##",
                "##             o",
                "##             o",
                "##             o",
                "##  f      f  ##",
                "##            ##",
                "################",
                "################",
            ]),
        );
        d.insert(
            "2,1".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##            ##",
                "o             ##",
                "o     l$      ##",
                "o             ##",
                "##            ##",
                "##            ##",
                "################",
                "################",
            ]),
        );
        d.insert(
            "1,0".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "#######  #######",
            ]),
        );
        all.insert(2, d);
    }

    // ===== DUNGEON 3: Desert Pyramid (entrance at overworld 11,1) =====
    // Layout: [0,0] boss-[1,0]
    //           |
    //         [0,1]-[1,1]
    //           |
    //         [0,2]-[1,2] entry
    {
        let mut d = HashMap::new();
        d.insert(
            "1,2".into(),
            parse(&[
                "################",
                "################",
                "## f        f ##",
                "##            ##",
                "o             ##",
                "o             ##",
                "o             ##",
                "##            ##",
                "## f        f ##",
                "#######  #######",
                "#######ss#######",
            ]),
        );
        d.insert(
            "0,2".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##  $      f  ##",
                "##             o",
                "##             o",
                "##             o",
                "##  f      f  ##",
                "##            ##",
                "#######  #######",
                "################",
            ]),
        );
        d.insert(
            "0,1".into(),
            parse(&[
                "#######  #######",
                "##            ##",
                "##            ##",
                "##  f      f  ##",
                "##             l",
                "##             l",
                "##             l",
                "##  f      f  ##",
                "##            ##",
                "#######  #######",
                "################",
            ]),
        );
        d.insert(
            "1,1".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##  f    $ f  ##",
                "o             ##",
                "o             ##",
                "o             ##",
                "##  f      f  ##",
                "##            ##",
                "################",
                "################",
            ]),
        );
        d.insert(
            "1,0".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##            ##",
                "o             ##",
                "o             ##",
                "o             ##",
                "##            ##",
                "##            ##",
                "################",
                "################",
            ]),
        );
        d.insert(
            "0,0".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##            ##",
                "##             l",
                "##             l",
                "##             l",
                "##            ##",
                "##            ##",
                "##            ##",
                "#######  #######",
            ]),
        );
        all.insert(3, d);
    }

    // ===== DUNGEON 4: Castle Depths (entrance at overworld 6,6) =====
    // Layout: [0,0] boss-[1,0]-[2,0]
    //                       |
    //                [1,1]-[2,1]
    //                       |
    //                      [2,2] entry
    {
        let mut d = HashMap::new();
        d.insert(
            "2,2".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "#######  #######",
                "#######ss#######",
            ]),
        );
        d.insert(
            "2,1".into(),
            parse(&[
                "#######  #######",
                "##            ##",
                "##            ##",
                "##   f    f   ##",
                "o             ##",
                "o             ##",
                "o             ##",
                "##   f    f   ##",
                "##            ##",
                "##            ##",
                "#######  #######",
            ]),
        );
        d.insert(
            "1,1".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##  f    $ f  ##",
                "##             l",
                "##             l",
                "##             l",
                "##  f      f  ##",
                "##            ##",
                "################",
                "################",
            ]),
        );
        d.insert(
            "2,0".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##            ##",
                "o             ##",
                "o             ##",
                "o             ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "#######  #######",
            ]),
        );
        d.insert(
            "1,0".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##  f      f  ##",
                "o              o",
                "o              o",
                "o              o",
                "##  f      f  ##",
                "##            ##",
                "################",
                "################",
            ]),
        );
        d.insert(
            "0,0".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##            ##",
                "##             o",
                "##             o",
                "##             o",
                "##            ##",
                "##            ##",
                "################",
                "################",
            ]),
        );
        all.insert(4, d);
    }

    // ===== DUNGEON 5: Secret Ancient Ruins (entrance at overworld 10,4) =====
    // Layout: [0,0] secret boss + Goal
    //           |
    //         [0,1]
    //           |
    //         [0,2] entry
    {
        let mut d = HashMap::new();
        d.insert(
            "0,2".into(),
            parse(&[
                "#######  #######",
                "##            ##",
                "##  f      f  ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##  f      f  ##",
                "#######  #######",
                "#######ss#######",
            ]),
        );
        d.insert(
            "0,1".into(),
            parse(&[
                "#######  #######",
                "##            ##",
                "##            ##",
                "##  f  ff  f  ##",
                "##            ##",
                "##    f  f    ##",
                "##            ##",
                "##  f  ff  f  ##",
                "##            ##",
                "##            ##",
                "#######  #######",
            ]),
        );
        d.insert(
            "0,0".into(),
            parse(&[
                "################",
                "################",
                "##            ##",
                "##  f      f  ##",
                "##            ##",
                "##     GG     ##",
                "##            ##",
                "##  f      f  ##",
                "##            ##",
                "##            ##",
                "#######  #######",
            ]),
        );
        all.insert(5, d);
    }

    for dungeon in AUTHORED_DUNGEONS {
        let mut authored = HashMap::new();
        for room in dungeon.rooms {
            authored.insert(screen_key(room.x, room.y), parse(room.tiles));
        }
        all.insert(dungeon.id, authored);
    }

    all
}

/// Returns all dungeon IDs in order.
pub fn dungeon_ids() -> Vec<i32> {
    AUTHORED_DUNGEONS.iter().map(|d| d.id).collect()
}

/// Returns (screen_x, screen_y) of the entry room for a given dungeon.
pub fn dungeon_entry(id: i32) -> (i32, i32) {
    if let Some(dungeon) = authored_dungeon(id) {
        return dungeon.entry;
    }
    match id {
        2 => (1, 2),
        3 => (1, 2),
        4 => (2, 2),
        5 => (0, 2),
        _ => (0, 0),
    }
}

/// Map overworld position to dungeon ID (0 = no dungeon here).
pub fn dungeon_at(screen_x: i32, screen_y: i32) -> i32 {
    if let Some(screen) = generated_overworld_screen(screen_x, screen_y) {
        return screen.dungeon;
    }
    if let Some(screen) = authored_overworld_screen(screen_x, screen_y) {
        return screen.dungeon.unwrap_or(0);
    }
    match (screen_x, screen_y) {
        (6, 10) => 2,
        (11, 1) => 3,
        (6, 6) => 4,
        (10, 4) => 5,
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// Enemy spawns
// ---------------------------------------------------------------------------

pub fn enemy_spawns(
    screen_x: i32,
    screen_y: i32,
    in_dungeon: bool,
    dungeon_id: i32,
    in_interior: bool,
    _interior_id: &str,
    cleared: bool,
) -> Vec<EnemySpawn> {
    if in_interior {
        return vec![];
    }
    if in_dungeon {
        if cleared {
            return vec![];
        }
        if let Some(room) = authored_dungeon_room(dungeon_id, screen_x, screen_y) {
            return spawn_defs_to_enemies(room.enemies);
        }
        return dungeon_enemy_spawns(dungeon_id, screen_x, screen_y);
    }
    if generated_overworld_screen(screen_x, screen_y).is_some() {
        return overworld_enemy_spawns(screen_x, screen_y);
    }
    if let Some(screen) = authored_overworld_screen(screen_x, screen_y) {
        return spawn_defs_to_enemies(screen.enemies);
    }
    overworld_enemy_spawns(screen_x, screen_y)
}

fn overworld_enemy_spawns(sx: i32, sy: i32) -> Vec<EnemySpawn> {
    if interiors_on_screen(sx, sy).next().is_some() {
        return vec![];
    }
    if let Some(screen) = generated_overworld_screen(sx, sy) {
        if screen.cave.is_some()
            || screen.dungeon != 0
            || !overworld_screen_props(sx, sy).is_empty()
        {
            return vec![];
        }
    }
    let spec = overworld_spec(sx, sy);
    if matches!(
        spec.landmark,
        OverworldLandmark::StartVillage
            | OverworldLandmark::SwordCave
            | OverworldLandmark::HeartCave
            | OverworldLandmark::IslandShrine
            | OverworldLandmark::EastSanctum
            | OverworldLandmark::BombCave
            | OverworldLandmark::Shop
    ) {
        return vec![];
    }

    let difficulty = ((sx + sy) / 4).clamp(0, 3);
    let base_count = match spec.biome {
        OverworldBiome::Plains | OverworldBiome::Highlands | OverworldBiome::Coast => 2,
        OverworldBiome::Lake => 2,
        _ => 3,
    };
    let count = (base_count + difficulty as usize / 2).min(4);
    let positions = [(4.0, 3.0), (11.0, 4.0), (5.0, 7.0), (10.0, 7.0)];
    let seed = ((sx * 13 + sy * 29).unsigned_abs() % positions.len() as u32) as usize;

    (0..count)
        .map(|index| {
            let pos = positions[(seed + index) % positions.len()];
            es(
                overworld_enemy_type(spec.biome, difficulty, seed + index),
                pos.0,
                pos.1,
            )
        })
        .collect()
}

fn overworld_enemy_type(biome: OverworldBiome, difficulty: i32, seed: usize) -> EnemyType {
    let roll = seed % 4;
    match biome {
        OverworldBiome::Forest | OverworldBiome::DeepForest => match difficulty {
            0 => {
                if roll == 0 {
                    EnemyType::Shriekwing
                } else {
                    EnemyType::Splort
                }
            }
            1 => {
                if roll >= 2 {
                    EnemyType::Shriekwing
                } else {
                    EnemyType::Borespat
                }
            }
            2 => {
                if roll == 0 {
                    EnemyType::Ironmaw
                } else {
                    EnemyType::Borespat
                }
            }
            _ => {
                if roll % 2 == 0 {
                    EnemyType::Ironmaw
                } else {
                    EnemyType::Shriekwing
                }
            }
        },
        OverworldBiome::Lake | OverworldBiome::Coast => match difficulty {
            0 => EnemyType::Borespat,
            1 => {
                if roll == 0 {
                    EnemyType::Shriekwing
                } else {
                    EnemyType::Borespat
                }
            }
            2 => {
                if roll >= 2 {
                    EnemyType::Ironmaw
                } else {
                    EnemyType::Borespat
                }
            }
            _ => {
                if roll == 0 {
                    EnemyType::Shriekwing
                } else {
                    EnemyType::Ironmaw
                }
            }
        },
        OverworldBiome::Desert
        | OverworldBiome::Ruins
        | OverworldBiome::Mountain
        | OverworldBiome::Snow
        | OverworldBiome::Canyon => match difficulty {
            0 => EnemyType::Borespat,
            1 => {
                if roll == 0 {
                    EnemyType::Shriekwing
                } else {
                    EnemyType::Borespat
                }
            }
            2 => {
                if roll >= 2 {
                    EnemyType::Ironmaw
                } else {
                    EnemyType::Borespat
                }
            }
            _ => {
                if roll % 2 == 0 {
                    EnemyType::Ironmaw
                } else {
                    EnemyType::Shriekwing
                }
            }
        },
        OverworldBiome::Plains | OverworldBiome::Highlands => match difficulty {
            0 => EnemyType::Splort,
            1 => {
                if roll == 0 {
                    EnemyType::Shriekwing
                } else {
                    EnemyType::Splort
                }
            }
            2 => {
                if roll >= 2 {
                    EnemyType::Borespat
                } else {
                    EnemyType::Shriekwing
                }
            }
            _ => {
                if roll % 2 == 0 {
                    EnemyType::Ironmaw
                } else {
                    EnemyType::Borespat
                }
            }
        },
    }
}

fn dungeon_enemy_spawns(dungeon_id: i32, sx: i32, sy: i32) -> Vec<EnemySpawn> {
    let key = screen_key(sx, sy);
    match dungeon_id {
        // Dungeon 1: Mountain Cave
        1 => match key.as_str() {
            "0,1" => vec![
                es(EnemyType::Splort, 5.0, 4.0),
                es(EnemyType::Splort, 10.0, 6.0),
            ],
            "1,1" => vec![
                es(EnemyType::Shriekwing, 5.0, 4.0),
                es(EnemyType::Shriekwing, 10.0, 6.0),
                es(EnemyType::Splort, 7.0, 5.0),
            ],
            "1,0" => vec![es(EnemyType::Boss, 7.0, 4.0)],
            _ => vec![],
        },
        // Dungeon 2: Forest Shrine
        2 => match key.as_str() {
            "0,1" => vec![
                es(EnemyType::Ironmaw, 5.0, 4.0),
                es(EnemyType::Ironmaw, 10.0, 6.0),
                es(EnemyType::Shriekwing, 8.0, 3.0),
            ],
            "1,0" => vec![es(EnemyType::Boss, 7.0, 4.0)],
            _ => vec![],
        },
        // Dungeon 3: Desert Pyramid
        3 => match key.as_str() {
            "0,2" => vec![
                es(EnemyType::Borespat, 5.0, 4.0),
                es(EnemyType::Borespat, 10.0, 6.0),
            ],
            "0,1" => vec![
                es(EnemyType::Ironmaw, 5.0, 4.0),
                es(EnemyType::Ironmaw, 10.0, 6.0),
            ],
            "1,1" => vec![
                es(EnemyType::Borespat, 5.0, 4.0),
                es(EnemyType::Borespat, 10.0, 6.0),
                es(EnemyType::Shriekwing, 8.0, 3.0),
            ],
            "1,0" => vec![
                es(EnemyType::Ironmaw, 5.0, 4.0),
                es(EnemyType::Shriekwing, 10.0, 6.0),
            ],
            "0,0" => vec![es(EnemyType::Boss, 7.0, 4.0)],
            _ => vec![],
        },
        // Dungeon 4: Castle Depths
        4 => match key.as_str() {
            "2,1" => vec![
                es(EnemyType::Ironmaw, 5.0, 4.0),
                es(EnemyType::Ironmaw, 10.0, 6.0),
            ],
            "1,1" => vec![
                es(EnemyType::Shriekwing, 5.0, 4.0),
                es(EnemyType::Shriekwing, 10.0, 6.0),
                es(EnemyType::Ironmaw, 8.0, 5.0),
                es(EnemyType::Ironmaw, 7.0, 7.0),
            ],
            "2,0" => vec![
                es(EnemyType::Ironmaw, 5.0, 4.0),
                es(EnemyType::Borespat, 10.0, 6.0),
                es(EnemyType::Ironmaw, 8.0, 3.0),
            ],
            "1,0" => vec![
                es(EnemyType::Ironmaw, 5.0, 4.0),
                es(EnemyType::Ironmaw, 10.0, 6.0),
                es(EnemyType::Ironmaw, 8.0, 7.0),
            ],
            "0,0" => vec![es(EnemyType::Boss, 7.0, 4.0)],
            _ => vec![],
        },
        // Dungeon 5: Secret Ruins
        5 => match key.as_str() {
            "0,2" => vec![
                es(EnemyType::Ironmaw, 5.0, 4.0),
                es(EnemyType::Ironmaw, 10.0, 6.0),
            ],
            "0,1" => vec![
                es(EnemyType::Ironmaw, 5.0, 3.0),
                es(EnemyType::Ironmaw, 10.0, 3.0),
                es(EnemyType::Shriekwing, 5.0, 7.0),
                es(EnemyType::Shriekwing, 10.0, 7.0),
                es(EnemyType::Ironmaw, 7.0, 5.0),
            ],
            "0,0" => vec![es(EnemyType::Boss, 7.0, 4.0)],
            _ => vec![],
        },
        _ => vec![],
    }
}

/// Shorthand for creating an enemy spawn at tile coordinates.
fn es(enemy_type: EnemyType, tile_x: f32, tile_y: f32) -> EnemySpawn {
    EnemySpawn {
        enemy_type,
        x: tile_x * TILE,
        y: tile_y * TILE,
    }
}

// ---------------------------------------------------------------------------
// Screen items (chests / special pickups)
// ---------------------------------------------------------------------------

pub fn screen_items(
    screen_x: i32,
    screen_y: i32,
    in_dungeon: bool,
    dungeon_id: i32,
    in_interior: bool,
    interior_id: &str,
) -> Vec<ItemDef> {
    if in_interior {
        if let Some(pickup_type) = interior_reward_pickup(interior_id) {
            return vec![ItemDef {
                pickup_type,
                tile_x: 8,
                tile_y: 4,
            }];
        }
        return vec![];
    }
    if in_dungeon {
        if let Some(room) = authored_dungeon_room(dungeon_id, screen_x, screen_y) {
            return item_defs(room.items);
        }
        return dungeon_screen_items(dungeon_id, screen_x, screen_y);
    }
    overworld_screen_items(screen_x, screen_y)
}

fn overworld_screen_props(screen_x: i32, screen_y: i32) -> Vec<WorldProp> {
    match (screen_x, screen_y) {
        (1, 13) => vec![npc(NpcKind::Barnett, 8, 5)],
        (2, 11) => vec![],
        (5, 12) => vec![],
        (6, 9) => vec![npc(NpcKind::Oswin, 8, 5)],
        (2, 7) => vec![],
        (1, 6) => vec![npc(NpcKind::Petra, 8, 5)],
        (9, 9) => vec![],
        (12, 9) => vec![],
        (6, 7) => vec![],
        (8, 5) => vec![npc(NpcKind::Vel, 8, 5)],
        (8, 0) => vec![],
        (9, 2) => vec![npc(NpcKind::Senna, 8, 5)],
        (4, 2) => vec![],
        // Healer gnomes — one per quadrant of the overworld
        (2, 4) => vec![npc(NpcKind::GnomeHealer, 8, 5)],  // NW quadrant
        (11, 3) => vec![npc(NpcKind::GnomeHealer, 8, 5)], // NE quadrant
        (4, 12) => vec![npc(NpcKind::GnomeHealer, 8, 5)], // SW quadrant
        (11, 12) => vec![npc(NpcKind::GnomeHealer, 8, 5)], // SE quadrant
        _ => vec![],
    }
}

fn overworld_screen_items(screen_x: i32, screen_y: i32) -> Vec<ItemDef> {
    match (screen_x, screen_y) {
        (1, 8) => vec![item(PickupType::Gem, 7, 5), item(PickupType::Gem, 9, 5)],
        (4, 10) => vec![item(PickupType::Gem, 8, 5)],
        (4, 12) => vec![item(PickupType::Gem, 7, 5), item(PickupType::Gem, 9, 5)],
        (6, 11) => vec![item(PickupType::Gem, 8, 5)],
        (1, 6) => vec![item(PickupType::Gem, 7, 5), item(PickupType::Gem, 9, 5)],
        (12, 6) => vec![item(PickupType::Gem, 8, 5)],
        (7, 6) => vec![item(PickupType::Gem, 8, 5)],
        (6, 5) => vec![item(PickupType::Gem, 8, 5)],
        (9, 2) => vec![item(PickupType::Gem, 7, 5), item(PickupType::Gem, 9, 5)],
        (4, 1) => vec![item(PickupType::Gem, 8, 5)],
        _ => vec![],
    }
}

fn dungeon_screen_items(dungeon_id: i32, sx: i32, sy: i32) -> Vec<ItemDef> {
    let key = screen_key(sx, sy);
    match dungeon_id {
        1 => match key.as_str() {
            "0,1" => vec![ItemDef {
                pickup_type: PickupType::Bombs,
                tile_x: 7,
                tile_y: 5,
            }],
            _ => vec![],
        },
        2 => match key.as_str() {
            "2,1" => vec![ItemDef {
                pickup_type: PickupType::Bombs,
                tile_x: 7,
                tile_y: 5,
            }],
            _ => vec![],
        },
        3 => match key.as_str() {
            "0,2" => vec![ItemDef {
                pickup_type: PickupType::HeartContainer,
                tile_x: 4,
                tile_y: 3,
            }],
            "1,1" => vec![ItemDef {
                pickup_type: PickupType::BombAmmo,
                tile_x: 10,
                tile_y: 3,
            }],
            _ => vec![],
        },
        4 => match key.as_str() {
            "1,1" => vec![ItemDef {
                pickup_type: PickupType::HeartContainer,
                tile_x: 10,
                tile_y: 3,
            }],
            _ => vec![],
        },
        5 => match key.as_str() {
            "0,1" => vec![ItemDef {
                pickup_type: PickupType::HeartContainer,
                tile_x: 7,
                tile_y: 5,
            }],
            _ => vec![],
        },
        _ => vec![],
    }
}
