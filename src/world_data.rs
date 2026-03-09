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

pub fn build_overworld() -> HashMap<String, TileGrid> {
    let mut data = HashMap::new();
    data.insert(
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
    data.insert(
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
    data.insert(
        "2,0".into(),
        parse(&[
            "TTTTTTTTTTTTTTTT",
            "TT...TTTTT...TTT",
            "T..............T",
            "T..TT......TT..T",
            "..............x.",
            "..T...T..T....x.",
            "..............x.",
            "T..TT......TT..T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
            "TTTTTTT..TTTTTTT",
        ]),
    );
    data.insert(
        "3,0".into(),
        parse(&[
            "TTTTTTTTTTTTTTTT",
            "TT....TTTT....TT",
            "T..............T",
            "T.....TTTT.....T",
            "......T..T......",
            "......T.GT......",
            "......T..T......",
            "T.....TTTT.....T",
            "T..............T",
            "TTTTTTTTTTTTTTTT",
            "TTTTTTTTTTTTTTTT",
        ]),
    );
    data.insert(
        "0,1".into(),
        parse(&[
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
    );
    data.insert(
        "1,1".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..bbb....bb...T",
            "T..bcb....bb...T",
            "................",
            "......====......",
            "................",
            "T..bb.....bb...T",
            "T..bb.....bb...T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
        ]),
    );
    data.insert(
        "2,1".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T..............T",
            "T....b....b....T",
            "................",
            "........b.......",
            "................",
            "T....b....b....T",
            "T..............T",
            "T..............T",
            "TTTTTTT..TTTTTTT",
        ]),
    );
    data.insert(
        "3,1".into(),
        parse(&[
            "TTTTTTTTTTTTT~~T",
            "T..........T~~.T",
            "T..........~~~.T",
            "T..........~~..T",
            "...........~~..T",
            "..........~~~..T",
            "...........~~..T",
            "T..........~~..T",
            "T.........~~~..T",
            "T..TTTT..TT~~.TT",
            "TTTTTTT..TTT~~TT",
        ]),
    );
    data.insert(
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
            "TT....TTTTTT..TT",
            "TTTTTTTTTTTTTTTT",
        ]),
    );
    data.insert(
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
            "TT............TT",
            "TTTTTTTTTTTTTTTT",
        ]),
    );
    data.insert(
        "2,2".into(),
        parse(&[
            "TTTTTTT..TTTTTTT",
            "T..............T",
            "T...^^....^^...T",
            "T...^^....^^...T",
            "................",
            "....^^....^^....",
            "................",
            "T...^^.........T",
            "T..............T",
            "TT............TT",
            "TTTTTTTTTTTTTTTT",
        ]),
    );
    data.insert(
        "3,2".into(),
        parse(&[
            "TTTTTTT..TTT~~TT",
            "T..........~~~.T",
            "T..........~~..T",
            "T.........~~~..T",
            "..........~~..TT",
            "..........~~..TT",
            "..........~~~..T",
            "T..........~~..T",
            "T..........~~~.T",
            "TT.........~~TTT",
            "TTTTTTTTTTTTTTTT",
        ]),
    );
    data
}

pub fn build_dungeon() -> HashMap<String, TileGrid> {
    let mut data = HashMap::new();
    data.insert(
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
    data.insert(
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
    data.insert(
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
    data.insert(
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
    data.insert(
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
    data
}

pub fn enemy_spawns(
    screen_x: i32,
    screen_y: i32,
    in_dungeon: bool,
    cleared: bool,
) -> Vec<EnemySpawn> {
    if in_dungeon {
        if cleared {
            return vec![];
        }
        return match screen_key(screen_x, screen_y).as_str() {
            "0,1" => vec![
                EnemySpawn {
                    enemy_type: EnemyType::Darknut,
                    x: 5.0 * TILE,
                    y: 4.0 * TILE,
                },
                EnemySpawn {
                    enemy_type: EnemyType::Darknut,
                    x: 10.0 * TILE,
                    y: 6.0 * TILE,
                },
                EnemySpawn {
                    enemy_type: EnemyType::Bat,
                    x: 8.0 * TILE,
                    y: 3.0 * TILE,
                },
            ],
            "1,0" => vec![EnemySpawn {
                enemy_type: EnemyType::Boss,
                x: 7.0 * TILE,
                y: 4.0 * TILE,
            }],
            _ => vec![],
        };
    }
    match screen_key(screen_x, screen_y).as_str() {
        "1,0" => vec![
            EnemySpawn {
                enemy_type: EnemyType::Slime,
                x: 4.0 * TILE,
                y: 3.0 * TILE,
            },
            EnemySpawn {
                enemy_type: EnemyType::Slime,
                x: 11.0 * TILE,
                y: 7.0 * TILE,
            },
        ],
        "2,0" => vec![
            EnemySpawn {
                enemy_type: EnemyType::Octorok,
                x: 5.0 * TILE,
                y: 4.0 * TILE,
            },
            EnemySpawn {
                enemy_type: EnemyType::Octorok,
                x: 10.0 * TILE,
                y: 6.0 * TILE,
            },
        ],
        "0,1" => vec![
            EnemySpawn {
                enemy_type: EnemyType::Slime,
                x: 6.0 * TILE,
                y: 3.0 * TILE,
            },
            EnemySpawn {
                enemy_type: EnemyType::Bat,
                x: 10.0 * TILE,
                y: 5.0 * TILE,
            },
        ],
        "2,1" => vec![
            EnemySpawn {
                enemy_type: EnemyType::Slime,
                x: 5.0 * TILE,
                y: 4.0 * TILE,
            },
            EnemySpawn {
                enemy_type: EnemyType::Slime,
                x: 10.0 * TILE,
                y: 7.0 * TILE,
            },
            EnemySpawn {
                enemy_type: EnemyType::Octorok,
                x: 8.0 * TILE,
                y: 5.0 * TILE,
            },
        ],
        "3,1" => vec![EnemySpawn {
            enemy_type: EnemyType::Octorok,
            x: 4.0 * TILE,
            y: 5.0 * TILE,
        }],
        "0,2" => vec![
            EnemySpawn {
                enemy_type: EnemyType::Slime,
                x: 10.0 * TILE,
                y: 3.0 * TILE,
            },
            EnemySpawn {
                enemy_type: EnemyType::Bat,
                x: 8.0 * TILE,
                y: 7.0 * TILE,
            },
        ],
        "1,2" => vec![
            EnemySpawn {
                enemy_type: EnemyType::Slime,
                x: 5.0 * TILE,
                y: 5.0 * TILE,
            },
            EnemySpawn {
                enemy_type: EnemyType::Slime,
                x: 11.0 * TILE,
                y: 5.0 * TILE,
            },
        ],
        "2,2" => vec![
            EnemySpawn {
                enemy_type: EnemyType::Octorok,
                x: 7.0 * TILE,
                y: 4.0 * TILE,
            },
            EnemySpawn {
                enemy_type: EnemyType::Darknut,
                x: 10.0 * TILE,
                y: 6.0 * TILE,
            },
        ],
        "3,2" => vec![EnemySpawn {
            enemy_type: EnemyType::Octorok,
            x: 4.0 * TILE,
            y: 4.0 * TILE,
        }],
        _ => vec![],
    }
}

pub fn screen_items(screen_x: i32, screen_y: i32, in_dungeon: bool) -> Vec<ItemDef> {
    if in_dungeon {
        return match screen_key(screen_x, screen_y).as_str() {
            "2,1" => vec![ItemDef {
                pickup_type: PickupType::Bombs,
                tile_x: 7,
                tile_y: 5,
            }],
            _ => vec![],
        };
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
