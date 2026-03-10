use crate::constants::{COLS, ROWS, TILE, WORLD_H, WORLD_W};
use crate::model::{EnemySpawn, EnemyType, ItemDef, PickupType, TileGrid, TileType};
use std::collections::HashMap;

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
struct OverworldSpec {
    biome: OverworldBiome,
    landmark: OverworldLandmark,
    name: &'static str,
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
    (2, 6)
}

pub fn cave_kind(screen_x: i32, screen_y: i32) -> Option<CaveKind> {
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
    overworld_spec(screen_x, screen_y).name.to_string()
}

pub fn build_overworld() -> HashMap<String, TileGrid> {
    let mut data = HashMap::new();
    for y in 0..WORLD_H {
        for x in 0..WORLD_W {
            data.insert(screen_key(x, y), build_overworld_screen(x, y));
        }
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
}

// ---------------------------------------------------------------------------
// Dungeons
// ---------------------------------------------------------------------------

pub fn build_dungeons() -> HashMap<i32, HashMap<String, TileGrid>> {
    let mut all = HashMap::new();

    // ===== DUNGEON 1: Mountain Cave (entrance at overworld 1,0) =====
    // Layout:      [1,0] boss
    //               |
    //        [0,1]-[1,1]
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
                "o              o",
                "o              o",
                "o              o",
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
                "##      $      o",
                "##             o",
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
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "##            ##",
                "#######  #######",
            ]),
        );
        all.insert(1, d);
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
    match id {
        1 => (1, 2),
        2 => (1, 2),
        3 => (1, 2),
        4 => (2, 2),
        5 => (0, 2),
        _ => (0, 0),
    }
}

/// Map overworld position to dungeon ID (0 = no dungeon here).
pub fn dungeon_at(screen_x: i32, screen_y: i32) -> i32 {
    match (screen_x, screen_y) {
        (1, 0) => 1,
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
        return dungeon_enemy_spawns(dungeon_id, screen_x, screen_y);
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
