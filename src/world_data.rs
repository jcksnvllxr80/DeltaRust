use crate::constants::{COLS, ROWS, TILE};
use crate::model::{EnemySpawn, EnemyType, ItemDef, PickupType, TileGrid, TileType};
use std::collections::HashMap;

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
// Overworld: 7 columns x 5 rows = 35 screens
// ---------------------------------------------------------------------------

pub fn build_overworld() -> HashMap<String, TileGrid> {
    let mut d = HashMap::new();

    // ===== ROW 0 (y=0) : Mountains / Highlands / Castle / Forest / Town =====

    // (0,0) MOUNTAIN PEAK - cave for Dungeon 1
    d.insert(
        "0,0".into(),
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
        ]),
    );

    // (1,0) HIGHLANDS
    d.insert(
        "1,0".into(),
        parse(&[
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
    );

    // (2,0) MOUNTAIN PASS
    d.insert(
        "2,0".into(),
        parse(&[
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
    );

    // (3,0) CASTLE GATE - Dungeon 4 entrance
    d.insert(
        "3,0".into(),
        parse(&[
            "^^^^^^^^^^^^^^^^",
            "^..............^",
            "^..^^.####.^^..^",
            "^..^..#..#..^..^",
            "....=.#dd#.=....",
            "....=.#..#.=....",
            "....=.####.=....",
            "^..^........^..^",
            "^..............^",
            "^^^^^^^..^^^^^^^",
            "^^^^^^^..^^^^^^^",
        ]),
    );

    // (4,0) NORTH FOREST
    d.insert(
        "4,0".into(),
        parse(&[
            "TTTTTTTTTTTTTTTT",
            "T..............T",
            "T..TT.....TT..T",
            "T..............T",
            "................",
            "..T...TT...T....",
            "................",
            "T..............T",
            "T..TT.....TT..T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    );

    // (5,0) DEEP FOREST
    d.insert(
        "5,0".into(),
        parse(&[
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
    );

    // (6,0) FOREST TOWN - cave (shop)
    d.insert(
        "6,0".into(),
        parse(&[
            "TTTTTTTTTTTTTTTT",
            "T..............T",
            "T..===....===..T",
            "T..=bb....bb=..T",
            "...=...c....=..T",
            "...=........=..T",
            "...=........=..T",
            "T..=bb....bb=..T",
            "T..===....===..T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    );

    // ===== ROW 1 (y=1) : Forest / Village / Lake / Settlement =====

    // (0,1) WESTERN FOREST
    d.insert(
        "0,1".into(),
        parse(&[
            "^^^^^^^..^^^^^^^",
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
    );

    // (1,1) VILLAGE - sword cave
    d.insert(
        "1,1".into(),
        parse(&[
            "^^^^^^^..^^^^^^^",
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
        ]),
    );

    // (2,1) LAKE WEST SHORE
    d.insert(
        "2,1".into(),
        parse(&[
            "^^^^^^^..^^^^^^^",
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
    );

    // (3,1) LAKE ISLAND - water with bridge to island
    d.insert(
        "3,1".into(),
        parse(&[
            "^^^^^^^..^^^^^^^",
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
        ]),
    );

    // (4,1) LAKE EAST SHORE
    d.insert(
        "4,1".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T~~~~~.........T",
            "T~~~~~~........T",
            "~~~~~~..........",
            "~~~~~...........",
            "~~~~~~..........",
            "T~~~~~~........T",
            "T~~~~~.........T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    );

    // (5,1) EASTERN WOOD
    d.insert(
        "5,1".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T...b......b...T",
            "T..............T",
            "................",
            "....b....b......",
            "................",
            "T..............T",
            "T...b......b...T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    );

    // (6,1) EASTERN SETTLEMENT
    d.insert(
        "6,1".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..==......==..T",
            "T..=........=..T",
            "...=..c.....=..T",
            "...=........=..T",
            "...=........=..T",
            "T..=........=..T",
            "T..==......==..T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    );

    // ===== ROW 2 (y=2) : Dungeon Gate / Crossroads / River / Desert Edge / Ruins =====

    // (0,2) DUNGEON GATE - Dungeon 2 entrance
    d.insert(
        "0,2".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "TT.............T",
            "T..............T",
            "T..............T",
            "T.....d.........",
            "T...............",
            "T...............",
            "T..............T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    );

    // (1,2) SOUTH CROSSROAD
    d.insert(
        "1,2".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..............T",
            "T..............T",
            "................",
            "......====......",
            "................",
            "T..............T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    );

    // (2,2) RIVER FORD
    d.insert(
        "2,2".into(),
        parse(&[
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
    );

    // (3,2) DESERT EDGE
    d.insert(
        "3,2".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T...............",
            "T.........,,,...",
            "T........,,,,...",
            ".........,,,,...",
            ".........,,,,...",
            ".........,,,,...",
            "T........,,,,...",
            "T.........,,,...",
            ",,,,,,,...,,,,,,",
            ",,,,,,,...,,,,,,",
        ]),
    );

    // (4,2) DESERT PATH
    d.insert(
        "4,2".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            ",..............,",
            ",...^^....^^...,",
            ",..............,",
            ",...............,",
            ",....====......,",
            ",...............,",
            ",..............,",
            ",...^^....^^...,",
            ",,,,,,,...,,,,,,",
            ",,,,,,,...,,,,,,",
        ]),
    );

    // (5,2) OLD RUINS - cracked walls
    d.insert(
        "5,2".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
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
    );

    // (6,2) RUIN DEPTHS
    d.insert(
        "6,2".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            ",..............T",
            ",...^^....^^...T",
            ",..............T",
            "...............T",
            "...x....x......T",
            "...............T",
            ",..............T",
            ",...^^....^^...T",
            ",,,,,,,...TTTTTT",
            ",,,,,,,...TTTTTT",
        ]),
    );

    // ===== ROW 3 (y=3) : Southern Forest / Desert / Pyramid =====

    // (0,3) SOUTHERN FOREST
    d.insert(
        "0,3".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..TT......TT..T",
            "T..............T",
            "T...............",
            "T...............",
            "T...............",
            "T..TT......TT..T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    );

    // (1,3) SOUTHERN PATH
    d.insert(
        "1,3".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..............T",
            "T..............T",
            "................",
            "......====......",
            "................",
            "T..............T",
            "T..............T",
            ",,,,,,,..TTTTTTT",
            ",,,,,,,..TTTTTTT",
        ]),
    );

    // (2,3) SAND DRIFT
    d.insert(
        "2,3".into(),
        parse(&[
            ",,,,,,,...,,,,,,",
            ",..............,",
            ",...^^....^^...,",
            ",..............,",
            ",...............,",
            ",...............,",
            ",...............,",
            ",..............,",
            ",..............,",
            ",,,,,,,...,,,,,,",
            ",,,,,,,...,,,,,,",
        ]),
    );

    // (3,3) PYRAMID - Dungeon 3 entrance
    d.insert(
        "3,3".into(),
        parse(&[
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
    );

    // (4,3) DESERT EXPANSE
    d.insert(
        "4,3".into(),
        parse(&[
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
    );

    // (5,3) DESERT RUINS
    d.insert(
        "5,3".into(),
        parse(&[
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
    );

    // (6,3) WASTELAND
    d.insert(
        "6,3".into(),
        parse(&[
            ",,,,,,,...TTTTTT",
            ",...............T",
            ",...^^....^^...T",
            ",.............T",
            "..............T",
            "..............T",
            "..............T",
            ",.............T",
            ",...^^....^^...T",
            ",,,,,,,..TTTTTTT",
            ",,,,,,,..TTTTTTT",
        ]),
    );

    // ===== ROW 4 (y=4) : Beach / Coast / Canyon / Secret =====

    // (0,4) BEACH
    d.insert(
        "0,4".into(),
        parse(&[
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
    );

    // (1,4) COASTLINE
    d.insert(
        "1,4".into(),
        parse(&[
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
    );

    // (2,4) SOUTH SHORE
    d.insert(
        "2,4".into(),
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
        ]),
    );

    // (3,4) CANYON
    d.insert(
        "3,4".into(),
        parse(&[
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
    );

    // (4,4) DEEP DESERT
    d.insert(
        "4,4".into(),
        parse(&[
            ",,,,,,,...,,,,,,",
            ",..............,",
            ",..............,",
            ",....^....^....,",
            ",...............,",
            ",..............,",
            ",...............,",
            ",....^....^....,",
            ",..............,",
            ",,,,,,,,,,,,,,,,",
            ",,,,,,,,,,,,,,,,",
        ]),
    );

    // (5,4) BADLANDS
    d.insert(
        "5,4".into(),
        parse(&[
            ",,,,,,,...,,,,,,",
            ",...............,",
            ",..^^..x.^^...,",
            ",..............,",
            ",...............,",
            ",...x......x...,",
            ",...............,",
            ",..............,",
            ",..^^....^^...,",
            ",,,,,,,,,,,,,,,,",
            ",,,,,,,,,,,,,,,,",
        ]),
    );

    // (6,4) SECRET GROVE - Secret Dungeon 5 entrance
    d.insert(
        "6,4".into(),
        parse(&[
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
    );

    d
}

// ---------------------------------------------------------------------------
// Dungeons
// ---------------------------------------------------------------------------

pub fn build_dungeons() -> HashMap<i32, HashMap<String, TileGrid>> {
    let mut all = HashMap::new();

    // ===== DUNGEON 1: Mountain Cave (entrance at overworld 0,0) =====
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

    // ===== DUNGEON 2: Forest Shrine (entrance at overworld 0,2) =====
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

    // ===== DUNGEON 3: Desert Pyramid (entrance at overworld 3,3) =====
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

    // ===== DUNGEON 4: Castle Depths (entrance at overworld 3,0) =====
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

    // ===== DUNGEON 5: Secret Ancient Ruins (entrance at overworld 6,4) =====
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
        (0, 0) => 1,
        (0, 2) => 2,
        (3, 3) => 3,
        (3, 0) => 4,
        (6, 4) => 5,
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
    match screen_key(sx, sy).as_str() {
        // Row 0
        "1,0" => vec![
            es(EnemyType::Slime, 4.0, 3.0),
            es(EnemyType::Slime, 11.0, 7.0),
        ],
        "2,0" => vec![
            es(EnemyType::Octorok, 5.0, 4.0),
            es(EnemyType::Octorok, 10.0, 6.0),
        ],
        "4,0" => vec![
            es(EnemyType::Slime, 5.0, 3.0),
            es(EnemyType::Slime, 10.0, 7.0),
        ],
        "5,0" => vec![
            es(EnemyType::Bat, 6.0, 4.0),
            es(EnemyType::Slime, 10.0, 6.0),
            es(EnemyType::Bat, 4.0, 7.0),
        ],
        // Row 1
        "0,1" => vec![
            es(EnemyType::Slime, 6.0, 3.0),
            es(EnemyType::Bat, 10.0, 5.0),
        ],
        "2,1" => vec![es(EnemyType::Octorok, 4.0, 5.0)],
        "4,1" => vec![
            es(EnemyType::Octorok, 10.0, 4.0),
            es(EnemyType::Bat, 12.0, 6.0),
        ],
        "5,1" => vec![
            es(EnemyType::Slime, 5.0, 4.0),
            es(EnemyType::Slime, 10.0, 7.0),
            es(EnemyType::Octorok, 8.0, 5.0),
        ],
        // Row 2
        "1,2" => vec![
            es(EnemyType::Slime, 5.0, 5.0),
            es(EnemyType::Slime, 11.0, 5.0),
        ],
        "2,2" => vec![es(EnemyType::Octorok, 4.0, 4.0)],
        "3,2" => vec![
            es(EnemyType::Octorok, 10.0, 4.0),
            es(EnemyType::Slime, 12.0, 6.0),
        ],
        "4,2" => vec![
            es(EnemyType::Octorok, 7.0, 4.0),
            es(EnemyType::Darknut, 10.0, 6.0),
        ],
        "5,2" => vec![
            es(EnemyType::Darknut, 5.0, 4.0),
            es(EnemyType::Octorok, 10.0, 7.0),
        ],
        "6,2" => vec![
            es(EnemyType::Darknut, 6.0, 5.0),
            es(EnemyType::Darknut, 10.0, 5.0),
        ],
        // Row 3
        "0,3" => vec![
            es(EnemyType::Slime, 8.0, 4.0),
            es(EnemyType::Bat, 10.0, 6.0),
        ],
        "1,3" => vec![
            es(EnemyType::Octorok, 5.0, 5.0),
            es(EnemyType::Slime, 11.0, 5.0),
        ],
        "2,3" => vec![
            es(EnemyType::Octorok, 7.0, 4.0),
            es(EnemyType::Octorok, 10.0, 7.0),
        ],
        "4,3" => vec![
            es(EnemyType::Darknut, 5.0, 4.0),
            es(EnemyType::Octorok, 10.0, 6.0),
        ],
        "5,3" => vec![
            es(EnemyType::Darknut, 6.0, 4.0),
            es(EnemyType::Darknut, 10.0, 6.0),
            es(EnemyType::Octorok, 8.0, 8.0),
        ],
        "6,3" => vec![
            es(EnemyType::Darknut, 5.0, 5.0),
            es(EnemyType::Bat, 10.0, 3.0),
            es(EnemyType::Darknut, 8.0, 7.0),
        ],
        // Row 4
        "0,4" => vec![es(EnemyType::Slime, 8.0, 4.0)],
        "1,4" => vec![
            es(EnemyType::Octorok, 5.0, 4.0),
            es(EnemyType::Slime, 10.0, 6.0),
        ],
        "2,4" => vec![es(EnemyType::Octorok, 8.0, 3.0)],
        "3,4" => vec![
            es(EnemyType::Darknut, 5.0, 4.0),
            es(EnemyType::Darknut, 10.0, 6.0),
        ],
        "4,4" => vec![
            es(EnemyType::Darknut, 7.0, 4.0),
            es(EnemyType::Octorok, 10.0, 7.0),
            es(EnemyType::Bat, 5.0, 6.0),
        ],
        "5,4" => vec![
            es(EnemyType::Darknut, 6.0, 4.0),
            es(EnemyType::Darknut, 10.0, 6.0),
            es(EnemyType::Bat, 4.0, 7.0),
            es(EnemyType::Bat, 12.0, 3.0),
        ],
        _ => vec![],
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
    match screen_key(screen_x, screen_y).as_str() {
        "0,0" => vec![ItemDef {
            pickup_type: PickupType::HeartContainer,
            tile_x: 4,
            tile_y: 3,
        }],
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
