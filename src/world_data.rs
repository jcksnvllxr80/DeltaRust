use crate::constants::{COLS, ROWS, TILE, WORLD_H, WORLD_W};
use crate::model::{EnemySpawn, EnemyType, ItemDef, PickupType, PropKind, TileGrid, TileType, WorldProp};
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
    items.iter()
        .map(|item| ItemDef {
            pickup_type: item.pickup_type,
            tile_x: item.tile_x,
            tile_y: item.tile_y,
        })
        .collect()
}

fn prop_defs(props: &[PropDef]) -> Vec<WorldProp> {
    props.iter()
        .map(|prop| WorldProp {
            kind: prop.kind,
            tile_x: prop.tile_x,
            tile_y: prop.tile_y,
            target_tile_x: prop.target_tile_x,
            target_tile_y: prop.target_tile_y,
        })
        .collect()
}

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

// ---------------------------------------------------------------------------
// Overworld: generated 14 columns x 15 rows = 210 screens
// ---------------------------------------------------------------------------

pub fn overworld_start() -> (i32, i32) {
    (3, 7)
}

pub fn cave_kind(screen_x: i32, screen_y: i32) -> Option<CaveKind> {
    if let Some(screen) = authored_overworld_screen(screen_x, screen_y) {
        return screen.cave;
    }
    match (screen_x, screen_y) {
        (3, 5) => Some(CaveKind::Sword),
        (1, 1) => Some(CaveKind::Heart),
        (8, 8) => Some(CaveKind::Shrine),
        (12, 6) => Some(CaveKind::Sanctum),
        (1, 12) => Some(CaveKind::Bombs),
        (10, 13) => Some(CaveKind::Shop),
        _ => None,
    }
}

pub fn location_name(screen_x: i32, screen_y: i32, in_dungeon: bool, dungeon_id: i32) -> String {
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
    if let Some(screen) = authored_overworld_screen(screen_x, screen_y) {
        return screen.name.to_string();
    }
    overworld_spec(screen_x, screen_y).name.to_string()
}

pub fn visual_theme_id(screen_x: i32, screen_y: i32, in_dungeon: bool, dungeon_id: i32) -> Option<i32> {
    if in_dungeon {
        return match dungeon_id {
            1 | 2 => Some(dungeon_id),
            _ => None,
        };
    }
    authored_overworld_screen(screen_x, screen_y).map(|screen| screen.biome_id)
}

pub fn build_overworld() -> HashMap<String, TileGrid> {
    let mut data = HashMap::new();
    for y in 0..WORLD_H {
        for x in 0..WORLD_W {
            data.insert(screen_key(x, y), build_overworld_screen(x, y));
        }
    }
    for screen in MOSSHAVEN_SCREENS.iter().chain(ASHENFALL_SCREENS.iter()) {
        data.insert(screen_key(screen.x, screen.y), parse(screen.tiles));
    }
    normalize_overworld_connections(&mut data);
    debug_assert!(validate_overworld_connections(&data).is_ok());
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
        OverworldLandmark::None => biome_screen(spec.biome, x, y),
    };
    seal_world_edges(&mut tiles, x, y, spec.biome);
    tiles
}

fn overworld_spec(x: i32, y: i32) -> OverworldSpec {
    let biome = biome_at(x, y);
    match (x, y) {
        (2, 6) => OverworldSpec {
            biome: OverworldBiome::Plains,
            landmark: OverworldLandmark::StartVillage,
            name: "TRADING POST",
        },
        (3, 5) => OverworldSpec {
            biome: OverworldBiome::Forest,
            landmark: OverworldLandmark::SwordCave,
            name: "ELDER WOODS",
        },
        (1, 1) => OverworldSpec {
            biome: OverworldBiome::Mountain,
            landmark: OverworldLandmark::HeartCave,
            name: "CLIFF HOLLOW",
        },
        (8, 8) => OverworldSpec {
            biome: OverworldBiome::Lake,
            landmark: OverworldLandmark::IslandShrine,
            name: "ISLAND SHRINE",
        },
        (12, 6) => OverworldSpec {
            biome: OverworldBiome::Ruins,
            landmark: OverworldLandmark::EastSanctum,
            name: "SUNKEN SANCTUM",
        },
        (1, 12) => OverworldSpec {
            biome: OverworldBiome::Coast,
            landmark: OverworldLandmark::BombCave,
            name: "SALT CAVERN",
        },
        (10, 13) => OverworldSpec {
            biome: OverworldBiome::Canyon,
            landmark: OverworldLandmark::Shop,
            name: "SECRET SHOP",
        },
        (1, 0) => OverworldSpec {
            biome: OverworldBiome::Mountain,
            landmark: OverworldLandmark::Dungeon(1),
            name: "LEVEL 1 GATE",
        },
        (6, 10) => OverworldSpec {
            biome: OverworldBiome::Mountain,
            landmark: OverworldLandmark::Dungeon(2),
            name: "LEVEL 2 GATE",
        },
        (11, 1) => OverworldSpec {
            biome: OverworldBiome::Snow,
            landmark: OverworldLandmark::Dungeon(3),
            name: "LEVEL 3 GATE",
        },
        (6, 6) => OverworldSpec {
            biome: OverworldBiome::Lake,
            landmark: OverworldLandmark::Dungeon(4),
            name: "LEVEL 4 GATE",
        },
        (10, 4) => OverworldSpec {
            biome: OverworldBiome::Ruins,
            landmark: OverworldLandmark::Dungeon(5),
            name: "LEVEL 5 GATE",
        },
        _ => OverworldSpec {
            biome,
            landmark: OverworldLandmark::None,
            name: biome_name(biome),
        },
    }
}

fn biome_at(x: i32, y: i32) -> OverworldBiome {
    const BIOME_MAP: [&str; 15] = [
        "MMHHHFFHMMMSSS",
        "MMHFFFFHMMSSSS",
        "MFFFHFHMMRSSSS",
        "FFFPGGFHMRRREE",
        "FFFGWWGFMRREEE",
        "FGGGWWWGGRREEE",
        "FGGWWWWGGREEEE",
        "GGRWWWWGGRNEEE",
        "GGRRGWGGGNNNEE",
        "GGGGMMGGGNNNEE",
        "GGGMMMMGGNNEEE",
        "CGGMMMMMMNNCEE",
        "CCGMMMMMMNNCCE",
        "CCCGMMMSNNNCCW",
        "CCCCEEEENNCCWW",
    ];
    match BIOME_MAP[y as usize].as_bytes()[x as usize] as char {
        'P' | 'G' => OverworldBiome::Plains,
        'F' => OverworldBiome::Forest,
        'H' => OverworldBiome::Highlands,
        'M' => OverworldBiome::Mountain,
        'W' => OverworldBiome::Lake,
        'E' => OverworldBiome::Desert,
        'R' => OverworldBiome::Ruins,
        'C' => OverworldBiome::Coast,
        'S' => OverworldBiome::Snow,
        'N' => OverworldBiome::Canyon,
        _ => OverworldBiome::DeepForest,
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
            row[0] = wall;
        }
    }
    if x == WORLD_W - 1 {
        for row in tiles.iter_mut() {
            row[COLS - 1] = wall;
        }
    }
    if y == 0 {
        for col in 0..COLS {
            tiles[0][col] = wall;
        }
    }
    if y == WORLD_H - 1 {
        for col in 0..COLS {
            tiles[ROWS - 1][col] = wall;
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
) {
    for row in 4..=6 {
        left[row][COLS - 2] = left_tile;
        left[row][COLS - 1] = left_tile;
        right[row][0] = right_tile;
        right[row][1] = right_tile;
    }
}

fn carve_vertical_connection(
    top: &mut TileGrid,
    bottom: &mut TileGrid,
    top_tile: TileType,
    bottom_tile: TileType,
) {
    for col in 6..=9 {
        top[ROWS - 2][col] = top_tile;
        top[ROWS - 1][col] = top_tile;
        bottom[0][col] = bottom_tile;
        bottom[1][col] = bottom_tile;
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
                if !(0..ROWS).any(|row| {
                    is_walkable(left[row][COLS - 1]) && is_walkable(right[row][0])
                }) {
                    return Err(format!(
                        "horizontal mismatch between {left_key} and {right_key}"
                    ));
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
                if !(0..COLS).any(|col| {
                    is_walkable(top[ROWS - 1][col]) && is_walkable(bottom[0][col])
                }) {
                    return Err(format!(
                        "vertical mismatch between {top_key} and {bottom_key}"
                    ));
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
        enemy_type: EnemyType::Octorok,
        tile_x: 5.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Slime,
        tile_x: 10.0,
        tile_y: 6.0,
    },
];

const MOSSHAVEN_WOOD_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Slime,
        tile_x: 5.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Bat,
        tile_x: 10.0,
        tile_y: 6.0,
    },
];

const MOSSHAVEN_RING_ENEMIES: &[SpawnDef] = &[SpawnDef {
    enemy_type: EnemyType::Bat,
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
        y: 5,
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
        x: 3,
        y: 5,
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
        x: 4,
        y: 5,
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
        x: 2,
        y: 6,
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
        x: 3,
        y: 6,
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
        x: 4,
        y: 6,
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
        x: 2,
        y: 7,
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
            enemy_type: EnemyType::Slime,
            tile_x: 10.0,
            tile_y: 5.0,
        }],
    },
    OverworldScreenDef {
        x: 3,
        y: 7,
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
        x: 4,
        y: 7,
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
        enemy_type: EnemyType::Bat,
        tile_x: 4.0,
        tile_y: 3.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Slime,
        tile_x: 11.0,
        tile_y: 6.0,
    },
];

const ASHENFALL_RUIN_ENEMIES: &[SpawnDef] = &[
    SpawnDef {
        enemy_type: EnemyType::Darknut,
        tile_x: 5.0,
        tile_y: 4.0,
    },
    SpawnDef {
        enemy_type: EnemyType::Octorok,
        tile_x: 10.0,
        tile_y: 5.0,
    },
];

const ASHENFALL_SCREENS: &[OverworldScreenDef] = &[
    OverworldScreenDef {
        x: 5,
        y: 5,
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
        x: 6,
        y: 5,
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
            enemy_type: EnemyType::Bat,
            tile_x: 8.0,
            tile_y: 4.0,
        }],
    },
    OverworldScreenDef {
        x: 7,
        y: 5,
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
        x: 5,
        y: 6,
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
            enemy_type: EnemyType::Slime,
            tile_x: 12.0,
            tile_y: 5.0,
        }],
    },
    OverworldScreenDef {
        x: 6,
        y: 6,
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
                enemy_type: EnemyType::Slime,
                tile_x: 4.0,
                tile_y: 2.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Bat,
                tile_x: 11.0,
                tile_y: 7.0,
            },
        ],
    },
    OverworldScreenDef {
        x: 7,
        y: 6,
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
        x: 5,
        y: 7,
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
        x: 6,
        y: 7,
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
            enemy_type: EnemyType::Bat,
            tile_x: 9.0,
            tile_y: 4.0,
        }],
    },
    OverworldScreenDef {
        x: 7,
        y: 7,
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

const LEVEL1_ROOMS: &[DungeonRoomDef] = &[
    DungeonRoomDef {
        x: 1,
        y: 5,
        tiles: &[
            "################",
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
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Slime,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Slime,
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
                enemy_type: EnemyType::Slime,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Slime,
                tile_x: 10.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Slime,
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
            enemy_type: EnemyType::Bat,
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
                enemy_type: EnemyType::Octorok,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Octorok,
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
            enemy_type: EnemyType::Darknut,
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
            "################",
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
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Darknut,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Darknut,
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
                enemy_type: EnemyType::Darknut,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Darknut,
                tile_x: 10.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Darknut,
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
            "##      $     ##",
            "##            ##",
            "##            ##",
            "##   ffff     ##",
            "##            ##",
            "################",
            "################",
        ],
        enemies: &[
            SpawnDef {
                enemy_type: EnemyType::Octorok,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Octorok,
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
                enemy_type: EnemyType::Darknut,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Darknut,
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
            enemy_type: EnemyType::Darknut,
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
                enemy_type: EnemyType::Darknut,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Darknut,
                tile_x: 10.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Darknut,
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
                enemy_type: EnemyType::Bat,
                tile_x: 5.0,
                tile_y: 3.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Bat,
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
                enemy_type: EnemyType::Octorok,
                tile_x: 5.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Octorok,
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
                enemy_type: EnemyType::Darknut,
                tile_x: 4.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Darknut,
                tile_x: 11.0,
                tile_y: 4.0,
            },
            SpawnDef {
                enemy_type: EnemyType::Bat,
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
            "#######kk#######",
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
];

fn authored_overworld_screen(x: i32, y: i32) -> Option<&'static OverworldScreenDef> {
    MOSSHAVEN_SCREENS
        .iter()
        .chain(ASHENFALL_SCREENS.iter())
        .find(|screen| screen.x == x && screen.y == y)
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

pub fn boss_key_spawn_tile(dungeon_id: i32, screen_x: i32, screen_y: i32) -> Option<(usize, usize)> {
    authored_dungeon_room(dungeon_id, screen_x, screen_y)?.boss_key_tile
}

pub fn screen_props(screen_x: i32, screen_y: i32, in_dungeon: bool, dungeon_id: i32) -> Vec<WorldProp> {
    if in_dungeon {
        if let Some(room) = authored_dungeon_room(dungeon_id, screen_x, screen_y) {
            return prop_defs(room.props);
        }
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
        assert!(
            validate_overworld_connections(&world).is_ok(),
            "generated overworld has mismatched borders"
        );
    }

    #[test]
    fn mosshaven_overrides_and_dungeon_metadata_exist() {
        assert_eq!(overworld_start(), (3, 7));
        assert_eq!(location_name(2, 5, false, 0), "THE FEN");
        assert_eq!(dungeon_at(2, 5), 1);
        assert_eq!(location_name(0, 0, true, 1), "MOSSHAVEN CAVE");

        let rooms = dungeon_map_rooms(1);
        assert!(rooms.contains("1,5"));
        assert!(rooms.contains("1,1"));
        assert!(!rooms.contains("0,1"));
        assert!(!rooms.contains("2,1"));
    }

    #[test]
    fn mosshaven_room_props_and_rewards_are_authored() {
        let r04_props = screen_props(1, 3, true, 1);
        assert_eq!(r04_props.len(), 2);

        let r07_props = screen_props(1, 1, true, 1);
        assert_eq!(r07_props.len(), 8);
        assert_eq!(boss_key_spawn_tile(1, 1, 1), Some((8, 3)));

        let boss_rewards = room_clear_rewards(1, 1, 0);
        assert_eq!(boss_rewards.len(), 2);
        assert!(boss_rewards
            .iter()
            .any(|item| item.pickup_type == PickupType::DragonPiece));
        assert!(boss_rewards
            .iter()
            .any(|item| item.pickup_type == PickupType::Ladder));
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

    all
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
    cleared: bool,
) -> Vec<EnemySpawn> {
    if in_dungeon {
        if cleared {
            return vec![];
        }
        if let Some(room) = authored_dungeon_room(dungeon_id, screen_x, screen_y) {
            return spawn_defs_to_enemies(room.enemies);
        }
        return dungeon_enemy_spawns(dungeon_id, screen_x, screen_y);
    }
    if let Some(screen) = authored_overworld_screen(screen_x, screen_y) {
        return spawn_defs_to_enemies(screen.enemies);
    }
    overworld_enemy_spawns(screen_x, screen_y)
}

fn overworld_enemy_spawns(sx: i32, sy: i32) -> Vec<EnemySpawn> {
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
            0 => if roll == 0 { EnemyType::Bat } else { EnemyType::Slime },
            1 => if roll >= 2 { EnemyType::Bat } else { EnemyType::Octorok },
            2 => if roll == 0 { EnemyType::Darknut } else { EnemyType::Octorok },
            _ => if roll % 2 == 0 { EnemyType::Darknut } else { EnemyType::Bat },
        },
        OverworldBiome::Lake | OverworldBiome::Coast => match difficulty {
            0 => EnemyType::Octorok,
            1 => if roll == 0 { EnemyType::Bat } else { EnemyType::Octorok },
            2 => if roll >= 2 { EnemyType::Darknut } else { EnemyType::Octorok },
            _ => if roll == 0 { EnemyType::Bat } else { EnemyType::Darknut },
        },
        OverworldBiome::Desert
        | OverworldBiome::Ruins
        | OverworldBiome::Mountain
        | OverworldBiome::Snow
        | OverworldBiome::Canyon => match difficulty {
            0 => EnemyType::Octorok,
            1 => if roll == 0 { EnemyType::Bat } else { EnemyType::Octorok },
            2 => if roll >= 2 { EnemyType::Darknut } else { EnemyType::Octorok },
            _ => if roll % 2 == 0 { EnemyType::Darknut } else { EnemyType::Bat },
        },
        OverworldBiome::Plains | OverworldBiome::Highlands => match difficulty {
            0 => EnemyType::Slime,
            1 => if roll == 0 { EnemyType::Bat } else { EnemyType::Slime },
            2 => if roll >= 2 { EnemyType::Octorok } else { EnemyType::Bat },
            _ => if roll % 2 == 0 { EnemyType::Darknut } else { EnemyType::Octorok },
        },
    }
}

fn dungeon_enemy_spawns(dungeon_id: i32, sx: i32, sy: i32) -> Vec<EnemySpawn> {
    let key = screen_key(sx, sy);
    match dungeon_id {
        // Dungeon 1: Mountain Cave
        1 => match key.as_str() {
            "0,1" => vec![
                es(EnemyType::Slime, 5.0, 4.0),
                es(EnemyType::Slime, 10.0, 6.0),
            ],
            "1,1" => vec![
                es(EnemyType::Bat, 5.0, 4.0),
                es(EnemyType::Bat, 10.0, 6.0),
                es(EnemyType::Slime, 7.0, 5.0),
            ],
            "1,0" => vec![es(EnemyType::Boss, 7.0, 4.0)],
            _ => vec![],
        },
        // Dungeon 2: Forest Shrine
        2 => match key.as_str() {
            "0,1" => vec![
                es(EnemyType::Darknut, 5.0, 4.0),
                es(EnemyType::Darknut, 10.0, 6.0),
                es(EnemyType::Bat, 8.0, 3.0),
            ],
            "1,0" => vec![es(EnemyType::Boss, 7.0, 4.0)],
            _ => vec![],
        },
        // Dungeon 3: Desert Pyramid
        3 => match key.as_str() {
            "0,2" => vec![
                es(EnemyType::Octorok, 5.0, 4.0),
                es(EnemyType::Octorok, 10.0, 6.0),
            ],
            "0,1" => vec![
                es(EnemyType::Darknut, 5.0, 4.0),
                es(EnemyType::Darknut, 10.0, 6.0),
            ],
            "1,1" => vec![
                es(EnemyType::Octorok, 5.0, 4.0),
                es(EnemyType::Octorok, 10.0, 6.0),
                es(EnemyType::Bat, 8.0, 3.0),
            ],
            "1,0" => vec![
                es(EnemyType::Darknut, 5.0, 4.0),
                es(EnemyType::Bat, 10.0, 6.0),
            ],
            "0,0" => vec![es(EnemyType::Boss, 7.0, 4.0)],
            _ => vec![],
        },
        // Dungeon 4: Castle Depths
        4 => match key.as_str() {
            "2,1" => vec![
                es(EnemyType::Darknut, 5.0, 4.0),
                es(EnemyType::Darknut, 10.0, 6.0),
            ],
            "1,1" => vec![
                es(EnemyType::Bat, 5.0, 4.0),
                es(EnemyType::Bat, 10.0, 6.0),
                es(EnemyType::Darknut, 8.0, 5.0),
                es(EnemyType::Darknut, 7.0, 7.0),
            ],
            "2,0" => vec![
                es(EnemyType::Darknut, 5.0, 4.0),
                es(EnemyType::Octorok, 10.0, 6.0),
                es(EnemyType::Darknut, 8.0, 3.0),
            ],
            "1,0" => vec![
                es(EnemyType::Darknut, 5.0, 4.0),
                es(EnemyType::Darknut, 10.0, 6.0),
                es(EnemyType::Darknut, 8.0, 7.0),
            ],
            "0,0" => vec![es(EnemyType::Boss, 7.0, 4.0)],
            _ => vec![],
        },
        // Dungeon 5: Secret Ruins
        5 => match key.as_str() {
            "0,2" => vec![
                es(EnemyType::Darknut, 5.0, 4.0),
                es(EnemyType::Darknut, 10.0, 6.0),
            ],
            "0,1" => vec![
                es(EnemyType::Darknut, 5.0, 3.0),
                es(EnemyType::Darknut, 10.0, 3.0),
                es(EnemyType::Bat, 5.0, 7.0),
                es(EnemyType::Bat, 10.0, 7.0),
                es(EnemyType::Darknut, 7.0, 5.0),
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
) -> Vec<ItemDef> {
    if in_dungeon {
        if let Some(room) = authored_dungeon_room(dungeon_id, screen_x, screen_y) {
            return item_defs(room.items);
        }
        return dungeon_screen_items(dungeon_id, screen_x, screen_y);
    }
    vec![]
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
