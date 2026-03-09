use crate::constants::{GAME_H, GAME_W, HUD_H, PIXEL_SCALE, TILE, WORLD_H, WORLD_W};
use crate::model::{
    Bomb, Dir, Enemy, EnemyType, Pickup, PickupType, Player, PlayerState, Projectile, TileGrid,
    TileType, Transition, WorldSnapshot,
};
use crate::sprites::{HeartState, Sprites};
use macroquad::prelude::*;

fn px(v: f32) -> f32 {
    v * PIXEL_SCALE
}

pub fn draw_game(
    sprites: &Sprites,
    world: &WorldSnapshot,
    player: &Player,
    enemies: &[Enemy],
    pickups: &[Pickup],
    bombs: &[Bomb],
    projectiles: &[Projectile],
) {
    clear_background(color_u8!(17, 17, 17, 255));
    draw_tiles(sprites, &world.tiles, 0.0, 0.0);
    draw_pickups(sprites, pickups);
    draw_bombs(sprites, bombs);
    draw_projectiles(sprites, projectiles);
    for enemy in enemies {
        if enemy.active {
            draw_enemy(sprites, enemy);
        }
    }
    draw_player(sprites, player);
    draw_hud(sprites, player, world);
}

pub fn draw_transition(
    sprites: &Sprites,
    world: &WorldSnapshot,
    transition: &Transition,
    next_tiles: Option<&TileGrid>,
    player: &Player,
) {
    clear_background(color_u8!(17, 17, 17, 255));
    let (mut old_ox, mut old_oy, mut new_ox, mut new_oy) = (0.0, 0.0, 0.0, 0.0);
    match transition.dir.unwrap() {
        Dir::Left => {
            old_ox = transition.progress;
            new_ox = -GAME_W + transition.progress;
        }
        Dir::Right => {
            old_ox = -transition.progress;
            new_ox = GAME_W - transition.progress;
        }
        Dir::Up => {
            old_oy = transition.progress;
            new_oy = -GAME_H + transition.progress;
        }
        Dir::Down => {
            old_oy = -transition.progress;
            new_oy = GAME_H - transition.progress;
        }
    }
    draw_tiles(sprites, &transition.old_tiles, old_ox, old_oy);
    if let Some(next_tiles) = next_tiles {
        draw_tiles(sprites, next_tiles, new_ox, new_oy);
    }
    draw_hud(sprites, player, world);
}

pub fn draw_fade_overlay(alpha: f32) {
    draw_rectangle(0.0, HUD_H, GAME_W, GAME_H, Color::new(0.0, 0.0, 0.0, alpha));
}

pub fn draw_title(frame: i32) {
    clear_background(color_u8!(17, 17, 17, 255));
    let cx = GAME_W / 2.0;
    let cy = px(70.0);
    draw_triangle(
        vec2(cx, cy - px(30.0)),
        vec2(cx + px(35.0), cy + px(25.0)),
        vec2(cx - px(35.0), cy + px(25.0)),
        YELLOW,
    );
    draw_text("DELTA", cx - px(52.0), cy + px(48.0), px(36.0), YELLOW);
    draw_text(
        "A World of Secrets",
        cx - px(88.0),
        cy + px(68.0),
        px(20.0),
        GRAY,
    );
    if (frame / 30) % 2 == 0 {
        draw_text(
            "Press ENTER",
            cx - px(64.0),
            cy + px(104.0),
            px(24.0),
            WHITE,
        );
    }
}

pub fn draw_game_over(frame: i32) {
    clear_background(BLACK);
    draw_text(
        "GAME OVER",
        GAME_W / 2.0 - px(92.0),
        px(110.0),
        px(42.0),
        RED,
    );
    if (frame / 30) % 2 == 0 {
        draw_text(
            "Press ENTER",
            GAME_W / 2.0 - px(72.0),
            px(170.0),
            px(24.0),
            WHITE,
        );
    }
}

pub fn draw_victory(frame: i32) {
    clear_background(BLACK);
    draw_text(
        "VICTORY!",
        GAME_W / 2.0 - px(84.0),
        px(120.0),
        px(40.0),
        YELLOW,
    );
    draw_text(
        "You discovered the secret of Delta!",
        px(80.0),
        px(160.0),
        px(24.0),
        LIGHTGRAY,
    );
    if (frame / 30) % 2 == 0 {
        draw_text(
            "Press ENTER",
            GAME_W / 2.0 - px(72.0),
            px(210.0),
            px(24.0),
            WHITE,
        );
    }
}

pub fn draw_message_box(text: &str) {
    let lines: Vec<&str> = text.lines().collect();
    let height = px(24.0) + lines.len() as f32 * px(20.0);
    let width = px(240.0);
    let x = (GAME_W - width) / 2.0;
    let y = ((GAME_H + HUD_H) - height) / 2.0;
    draw_rectangle(x, y, width, height, BLACK);
    draw_rectangle_lines(x, y, width, height, px(2.0), WHITE);
    for (index, line) in lines.iter().enumerate() {
        draw_text(
            line,
            x + px(10.0),
            y + px(24.0) + index as f32 * px(18.0),
            px(20.0),
            WHITE,
        );
    }
}

fn draw_tiles(sprites: &Sprites, tiles: &TileGrid, ox: f32, oy: f32) {
    for (row, line) in tiles.iter().enumerate() {
        for (col, tile) in line.iter().enumerate() {
            let x = ox + col as f32 * TILE;
            let y = oy + row as f32 * TILE + HUD_H;
            let underlay = tile_base(*tile).unwrap_or(*tile);
            if !sprites.draw_tile(underlay, x, y) {
                draw_rectangle(x, y, TILE, TILE, tile_color(underlay));
            }
            if !sprites.draw_tile(*tile, x, y) {
                draw_rectangle(x, y, TILE, TILE, tile_color(*tile));
            }
        }
    }
}

fn tile_base(tile: TileType) -> Option<TileType> {
    match tile {
        TileType::Tree | TileType::Bush | TileType::Rock | TileType::Cave | TileType::Dungeon => {
            Some(TileType::Grass)
        }
        TileType::Bridge => Some(TileType::Water),
        _ => None,
    }
}

fn tile_color(tile: TileType) -> Color {
    match tile {
        TileType::Grass => color_u8!(68, 170, 68, 255),
        TileType::Tree => color_u8!(34, 102, 51, 255),
        TileType::Water => color_u8!(34, 102, 204, 255),
        TileType::Rock => color_u8!(128, 128, 128, 255),
        TileType::Sand => color_u8!(204, 170, 102, 255),
        TileType::Path => color_u8!(170, 136, 85, 255),
        TileType::Cave => color_u8!(34, 34, 34, 255),
        TileType::Dungeon => color_u8!(85, 51, 68, 255),
        TileType::Cracked => color_u8!(140, 110, 80, 255),
        TileType::Bush => color_u8!(51, 170, 68, 255),
        TileType::Bridge => color_u8!(136, 102, 51, 255),
        TileType::Wall => color_u8!(51, 51, 85, 255),
        TileType::Floor => color_u8!(85, 85, 119, 255),
        TileType::DoorLocked => color_u8!(170, 119, 34, 255),
        TileType::Door => color_u8!(85, 85, 119, 255),
        TileType::Stairs => color_u8!(119, 119, 153, 255),
        TileType::Chest => color_u8!(221, 170, 34, 255),
        TileType::Goal => color_u8!(255, 221, 34, 255),
        TileType::BossDoor => color_u8!(170, 34, 51, 255),
        TileType::FloorAlt => color_u8!(102, 102, 136, 255),
    }
}

fn draw_player(sprites: &Sprites, player: &Player) {
    let x = player.x.round();
    let y = player.y.round() + HUD_H;
    if player.invuln_timer > 0 && (player.invuln_timer / 3) % 2 == 0 {
        return;
    }
    if sprites.draw_player(player, x, y) {
        if player.attack_timer > 0 {
            draw_player_sword(player.dir, x, y, true);
        }
        return;
    }
    if player.attack_timer > 0 {
        draw_player_sword(player.dir, x, y, false);
    }
    let outline = color_u8!(20, 24, 20, 255);
    let tunic = color_u8!(50, 148, 66, 255);
    let tunic_shadow = color_u8!(34, 104, 46, 255);
    let skin = color_u8!(238, 206, 184, 255);
    let hair = color_u8!(122, 82, 46, 255);
    let boots = color_u8!(124, 82, 48, 255);

    draw_rectangle(x + px(7.0), y + px(7.0), px(18.0), px(19.0), outline);
    draw_rectangle(x + px(8.0), y + px(8.0), px(16.0), px(17.0), tunic);
    draw_rectangle(x + px(8.0), y + px(17.0), px(16.0), px(8.0), tunic_shadow);

    match player.dir {
        Dir::Down => {
            draw_rectangle(x + px(9.0), y + px(2.0), px(14.0), px(11.0), skin);
            draw_rectangle(x + px(9.0), y, px(14.0), px(4.0), hair);
            draw_rectangle(x + px(11.0), y + px(6.0), px(2.0), px(2.0), outline);
            draw_rectangle(x + px(19.0), y + px(6.0), px(2.0), px(2.0), outline);
        }
        Dir::Up => {
            draw_rectangle(x + px(9.0), y + px(2.0), px(14.0), px(11.0), skin);
            draw_rectangle(x + px(9.0), y + px(1.0), px(14.0), px(8.0), hair);
            draw_rectangle(x + px(11.0), y + px(11.0), px(10.0), px(2.0), tunic);
        }
        Dir::Left => {
            draw_rectangle(x + px(8.0), y + px(2.0), px(12.0), px(11.0), skin);
            draw_rectangle(x + px(8.0), y + px(1.0), px(12.0), px(4.0), hair);
            draw_rectangle(x + px(10.0), y + px(6.0), px(2.0), px(2.0), outline);
            draw_rectangle(x + px(5.0), y + px(12.0), px(4.0), px(8.0), tunic_shadow);
        }
        Dir::Right => {
            draw_rectangle(x + px(12.0), y + px(2.0), px(12.0), px(11.0), skin);
            draw_rectangle(x + px(12.0), y + px(1.0), px(12.0), px(4.0), hair);
            draw_rectangle(x + px(20.0), y + px(6.0), px(2.0), px(2.0), outline);
            draw_rectangle(x + px(23.0), y + px(12.0), px(4.0), px(8.0), tunic_shadow);
        }
    }
    let (left_leg, right_leg) = if player.state == PlayerState::Walking && player.walk_frame == 0 {
        (26.0, 24.0)
    } else if player.state == PlayerState::Walking {
        (24.0, 26.0)
    } else {
        (26.0, 26.0)
    };
    draw_rectangle(x + px(8.0), y + px(left_leg), px(6.0), px(6.0), boots);
    draw_rectangle(x + px(18.0), y + px(right_leg), px(6.0), px(6.0), boots);
    draw_rectangle(x + px(9.0), y + px(12.0), px(3.0), px(6.0), skin);
    draw_rectangle(x + px(20.0), y + px(12.0), px(3.0), px(6.0), skin);
}

fn draw_player_sword(dir: Dir, x: f32, y: f32, sprite_mode: bool) {
    let blade = LIGHTGRAY;
    let hilt = color_u8!(196, 160, 74, 255);
    if sprite_mode {
        // Account for centered frame offset (scale 3 offsets y by -16)
        let y_offset = y - px(16.0);
        match dir {
            Dir::Up => {
                draw_rectangle(x + px(13.0), y_offset - px(12.0), px(6.0), px(18.0), blade);
                draw_rectangle(x + px(11.0), y_offset + px(4.0), px(10.0), px(3.0), hilt);
            }
            Dir::Down => {
                draw_rectangle(x + px(13.0), y_offset + px(26.0), px(6.0), px(18.0), blade);
                draw_rectangle(x + px(11.0), y_offset + px(25.0), px(10.0), px(3.0), hilt);
            }
            Dir::Left => {
                draw_rectangle(x - px(12.0), y_offset + px(13.0), px(18.0), px(6.0), blade);
                draw_rectangle(x + px(4.0), y_offset + px(11.0), px(3.0), px(10.0), hilt);
            }
            Dir::Right => {
                draw_rectangle(x + px(26.0), y_offset + px(13.0), px(18.0), px(6.0), blade);
                draw_rectangle(x + px(25.0), y_offset + px(11.0), px(3.0), px(10.0), hilt);
            }
        }
        return;
    }
    match dir {
        Dir::Up => {
            draw_rectangle(x + px(14.0), y - px(10.0), px(4.0), px(12.0), blade);
            draw_rectangle(x + px(12.0), y + px(1.0), px(8.0), px(2.0), hilt);
        }
        Dir::Down => {
            draw_rectangle(x + px(14.0), y + px(28.0), px(4.0), px(12.0), blade);
            draw_rectangle(x + px(12.0), y + px(27.0), px(8.0), px(2.0), hilt);
        }
        Dir::Left => {
            draw_rectangle(x - px(10.0), y + px(14.0), px(12.0), px(4.0), blade);
            draw_rectangle(x + px(1.0), y + px(12.0), px(2.0), px(8.0), hilt);
        }
        Dir::Right => {
            draw_rectangle(x + px(28.0), y + px(14.0), px(12.0), px(4.0), blade);
            draw_rectangle(x + px(27.0), y + px(12.0), px(2.0), px(8.0), hilt);
        }
    }
}

fn draw_enemy(sprites: &Sprites, enemy: &Enemy) {
    let x = enemy.x.round();
    let y = enemy.y.round() + HUD_H;
    if enemy.flash_timer > 0 && (enemy.flash_timer / 2) % 2 == 0 {
        draw_rectangle(x, y, enemy.w, enemy.h, WHITE);
        return;
    }
    if sprites.draw_enemy(enemy, x, y) {
        return;
    }
    let outline = color_u8!(18, 18, 24, 255);
    match enemy.enemy_type {
        EnemyType::Slime => {
            draw_rectangle(x + px(1.0), y + px(6.0), px(10.0), px(5.0), outline);
            draw_rectangle(
                x + px(2.0),
                y + px(7.0),
                px(8.0),
                px(4.0),
                color_u8!(86, 219, 96, 255),
            );
            draw_rectangle(
                x + px(3.0),
                y + px(4.0),
                px(6.0),
                px(4.0),
                color_u8!(120, 245, 128, 255),
            );
            draw_rectangle(x + px(4.0), y + px(7.0), px(1.0), px(1.0), outline);
            draw_rectangle(x + px(7.0), y + px(7.0), px(1.0), px(1.0), outline);
        }
        EnemyType::Octorok => {
            draw_rectangle(x + px(2.0), y + px(3.0), px(10.0), px(9.0), outline);
            draw_rectangle(
                x + px(3.0),
                y + px(4.0),
                px(8.0),
                px(7.0),
                color_u8!(206, 76, 66, 255),
            );
            draw_rectangle(
                x + px(4.0),
                y + px(10.0),
                px(1.0),
                px(2.0),
                color_u8!(170, 40, 34, 255),
            );
            draw_rectangle(
                x + px(6.0),
                y + px(10.0),
                px(1.0),
                px(2.0),
                color_u8!(170, 40, 34, 255),
            );
            draw_rectangle(
                x + px(8.0),
                y + px(10.0),
                px(1.0),
                px(2.0),
                color_u8!(170, 40, 34, 255),
            );
            draw_rectangle(x + px(4.0), y + px(6.0), px(2.0), px(2.0), WHITE);
            draw_rectangle(x + px(8.0), y + px(6.0), px(2.0), px(2.0), WHITE);
            draw_rectangle(x + px(5.0), y + px(7.0), px(1.0), px(1.0), outline);
            draw_rectangle(x + px(8.0), y + px(7.0), px(1.0), px(1.0), outline);
        }
        EnemyType::Bat => {
            let flap = if (enemy.timer / 8) % 2 == 0 {
                px(1.0)
            } else {
                px(3.0)
            };
            draw_rectangle(
                x + px(3.0),
                y + px(4.0),
                px(4.0),
                px(4.0),
                color_u8!(132, 92, 184, 255),
            );
            draw_triangle(
                vec2(x + px(3.0), y + flap + px(4.0)),
                vec2(x - px(2.0), y + flap + px(1.0)),
                vec2(x + px(1.0), y + flap + px(7.0)),
                color_u8!(120, 76, 170, 255),
            );
            draw_triangle(
                vec2(x + px(7.0), y + flap + px(4.0)),
                vec2(x + px(12.0), y + flap + px(1.0)),
                vec2(x + px(9.0), y + flap + px(7.0)),
                color_u8!(120, 76, 170, 255),
            );
            draw_rectangle(x + px(4.0), y + px(5.0), px(1.0), px(1.0), RED);
            draw_rectangle(x + px(6.0), y + px(5.0), px(1.0), px(1.0), RED);
        }
        EnemyType::Darknut => {
            draw_rectangle(x + px(2.0), y + px(2.0), px(10.0), px(12.0), outline);
            draw_rectangle(
                x + px(3.0),
                y + px(3.0),
                px(8.0),
                px(10.0),
                color_u8!(72, 84, 150, 255),
            );
            draw_rectangle(
                x + px(4.0),
                y + px(2.0),
                px(6.0),
                px(4.0),
                color_u8!(110, 126, 188, 255),
            );
            draw_rectangle(x + px(5.0), y + px(5.0), px(4.0), px(2.0), outline);
            match enemy.dir {
                Dir::Left => draw_rectangle(
                    x + px(1.0),
                    y + px(5.0),
                    px(2.0),
                    px(6.0),
                    color_u8!(88, 98, 160, 255),
                ),
                Dir::Right => draw_rectangle(
                    x + px(11.0),
                    y + px(5.0),
                    px(2.0),
                    px(6.0),
                    color_u8!(88, 98, 160, 255),
                ),
                _ => {}
            }
        }
        EnemyType::Boss => {
            draw_rectangle(x + px(2.0), y + px(3.0), px(20.0), px(19.0), outline);
            draw_rectangle(
                x + px(3.0),
                y + px(4.0),
                px(18.0),
                px(17.0),
                color_u8!(168, 56, 48, 255),
            );
            draw_rectangle(
                x + px(5.0),
                y + px(1.0),
                px(14.0),
                px(7.0),
                color_u8!(214, 80, 68, 255),
            );
            draw_triangle(
                vec2(x + px(5.0), y + px(2.0)),
                vec2(x + px(3.0), y - px(2.0)),
                vec2(x + px(7.0), y + px(2.0)),
                color_u8!(226, 206, 120, 255),
            );
            draw_triangle(
                vec2(x + px(19.0), y + px(2.0)),
                vec2(x + px(21.0), y - px(2.0)),
                vec2(x + px(17.0), y + px(2.0)),
                color_u8!(226, 206, 120, 255),
            );
            draw_rectangle(x + px(8.0), y + px(5.0), px(3.0), px(2.0), YELLOW);
            draw_rectangle(x + px(14.0), y + px(5.0), px(3.0), px(2.0), YELLOW);
            draw_rectangle(x + px(9.0), y + px(6.0), px(1.0), px(1.0), outline);
            draw_rectangle(x + px(15.0), y + px(6.0), px(1.0), px(1.0), outline);
        }
    }
}

fn draw_pickups(sprites: &Sprites, pickups: &[Pickup]) {
    for pickup in pickups {
        let x = pickup.x.round();
        let y = pickup.y.round() + HUD_H + (pickup.timer as f32 * 0.1).sin() * px(1.5);
        if !sprites.draw_pickup(pickup.pickup_type, x, y, pickup.w, pickup.h) {
            let color = match pickup.pickup_type {
                PickupType::Heart | PickupType::HeartContainer => RED,
                PickupType::Key => YELLOW,
                PickupType::BossKey => ORANGE,
                PickupType::BombAmmo | PickupType::Bombs => DARKGRAY,
            };
            draw_rectangle(x, y, pickup.w, pickup.h, color);
        }
    }
}

fn draw_bombs(sprites: &Sprites, bombs: &[Bomb]) {
    for bomb in bombs {
        let x = bomb.x.round();
        let y = bomb.y.round() + HUD_H;
        if bomb.exploded {
            draw_circle(
                x + px(6.0),
                y + px(6.0),
                px(20.0) - bomb.explosion_timer as f32 * PIXEL_SCALE,
                Color::new(1.0, 0.6, 0.0, bomb.explosion_timer as f32 / 20.0),
            );
        } else {
            if !sprites.draw_bomb(x, y, bomb.w, bomb.h) {
                draw_rectangle(x + px(2.0), y + px(3.0), px(8.0), px(9.0), DARKGRAY);
            }
        }
    }
}

fn draw_projectiles(sprites: &Sprites, projectiles: &[Projectile]) {
    for projectile in projectiles {
        let x = projectile.x.round();
        let y = projectile.y.round() + HUD_H;
        if !sprites.draw_projectile(projectile, x, y) {
            draw_rectangle(
                x,
                y,
                projectile.w,
                projectile.h,
                if projectile.from_enemy {
                    ORANGE
                } else {
                    SKYBLUE
                },
            );
        }
    }
}

fn draw_hud(sprites: &Sprites, player: &Player, world: &WorldSnapshot) {
    draw_rectangle(0.0, 0.0, GAME_W, HUD_H, color_u8!(17, 17, 17, 255));
    draw_rectangle(0.0, HUD_H - px(2.0), GAME_W, px(2.0), color_u8!(100, 100, 100, 255));
    for i in 0..(player.max_hp / 2) {
        let x = GAME_W - px(20.0) - i as f32 * px(14.0);
        let state = if player.hp >= (i + 1) * 2 {
            HeartState::Full
        } else if player.hp >= i * 2 + 1 {
            HeartState::Half
        } else {
            HeartState::Empty
        };
        if !sprites.draw_hud_heart(state, x, px(8.0), px(12.0)) {
            let color = match state {
                HeartState::Full => RED,
                HeartState::Half => PINK,
                HeartState::Empty => DARKGRAY,
            };
            draw_rectangle(x, px(8.0), px(10.0), px(10.0), color);
        }
    }
    if player.has_bombs {
        if !sprites.draw_hud_bomb(px(16.0), px(6.0), px(16.0)) {
            draw_rectangle(px(18.0), px(8.0), px(12.0), px(12.0), DARKGRAY);
        }
        draw_text(
            &format!("x{}", player.bomb_count),
            px(36.0),
            px(20.0),
            px(20.0),
            WHITE,
        );
    }
    if player.keys > 0 || world.in_dungeon {
        if !sprites.draw_hud_key(px(16.0), px(28.0), px(16.0)) {
            draw_rectangle(px(18.0), px(30.0), px(12.0), px(12.0), YELLOW);
        }
        draw_text(
            &format!("x{}", player.keys),
            px(36.0),
            px(40.0),
            px(16.0),
            YELLOW,
        );
    }
    if player.has_boss_key {
        if !sprites.draw_hud_boss_key(px(80.0), px(28.0), px(16.0)) {
            draw_rectangle(px(80.0), px(30.0), px(12.0), px(12.0), ORANGE);
        }
        draw_text("BOSS", px(100.0), px(40.0), px(16.0), ORANGE);
    }
    draw_text(
        location_name(world),
        px(50.0),
        px(20.0),
        px(20.0),
        LIGHTGRAY,
    );
    draw_minimap(world);
}

fn draw_minimap(world: &WorldSnapshot) {
    if world.in_dungeon {
        draw_dungeon_minimap(world);
        return;
    }
    // Overworld minimap - scaled for 7x5, centered in HUD
    let cell_w = px(7.0);
    let cell_h = px(5.0);
    let gap_x = px(8.0);
    let gap_y = px(6.0);
    let minimap_w = WORLD_W as f32 * gap_x;
    let base_x = (GAME_W - minimap_w) / 2.0; // Center horizontally
    for sy in 0..WORLD_H {
        for sx in 0..WORLD_W {
            let key = format!("{sx},{sy}");
            let x = base_x + sx as f32 * gap_x;
            let y = px(6.0) + sy as f32 * gap_y;
            let current = world.screen_x == sx && world.screen_y == sy;
            let visited = world.visited.contains(&key);
            draw_rectangle(
                x,
                y,
                cell_w,
                cell_h,
                if current {
                    GREEN
                } else if visited {
                    color_u8!(68, 68, 102, 255)
                } else {
                    color_u8!(34, 34, 34, 255)
                },
            );
        }
    }
}

fn draw_dungeon_minimap(world: &WorldSnapshot) {
    let cell_w = px(10.0);
    let cell_h = px(7.0);
    let gap_x = px(12.0);
    let gap_y = px(9.0);
    // Find bounds of dungeon rooms
    let mut min_x: i32 = 99;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = 99;
    let mut max_y: i32 = 0;
    for key in &world.dungeon_rooms {
        if let Some((sx, sy)) = parse_key(key) {
            min_x = min_x.min(sx);
            max_x = max_x.max(sx);
            min_y = min_y.min(sy);
            max_y = max_y.max(sy);
        }
    }
    let cols = (max_x - min_x + 1) as f32;
    let minimap_w = cols * gap_x;
    let base_x = (GAME_W - minimap_w) / 2.0; // Center horizontally
    for key in &world.dungeon_rooms {
        if let Some((sx, sy)) = parse_key(key) {
            let rx = (sx - min_x) as f32;
            let ry = (sy - min_y) as f32;
            let x = base_x + rx * gap_x;
            let y = px(6.0) + ry * gap_y;
            let current = world.screen_x == sx && world.screen_y == sy;
            let visit_key = format!("d{}:{key}", world.dungeon_id);
            let visited = world.visited.contains(&visit_key);
            draw_rectangle(
                x,
                y,
                cell_w,
                cell_h,
                if current {
                    GREEN
                } else if visited {
                    color_u8!(68, 68, 102, 255)
                } else {
                    color_u8!(34, 34, 34, 255)
                },
            );
        }
    }
}

fn parse_key(key: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = key.split(',').collect();
    if parts.len() == 2 {
        Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
    } else {
        None
    }
}

fn location_name(world: &WorldSnapshot) -> &'static str {
    if world.in_dungeon {
        return match world.dungeon_id {
            1 => "MTN. CAVE",
            2 => "FOREST SHRINE",
            3 => "PYRAMID",
            4 => "CASTLE DEPTHS",
            5 => "ANCIENT RUINS",
            _ => "DUNGEON",
        };
    }
    match format!("{},{}", world.screen_x, world.screen_y).as_str() {
        // Row 0
        "0,0" => "MT. PEAK",
        "1,0" => "HIGHLANDS",
        "2,0" => "MTN. PASS",
        "3,0" => "CASTLE GATE",
        "4,0" => "N. FOREST",
        "5,0" => "DEEP FOREST",
        "6,0" => "FOREST TOWN",
        // Row 1
        "0,1" => "W. FOREST",
        "1,1" => "VILLAGE",
        "2,1" => "LAKE SHORE",
        "3,1" => "LAKE ISLAND",
        "4,1" => "E. LAKE",
        "5,1" => "EASTERN WOOD",
        "6,1" => "E. SETTLEMENT",
        // Row 2
        "0,2" => "DUNGEON GATE",
        "1,2" => "S. CROSSROAD",
        "2,2" => "RIVER FORD",
        "3,2" => "DESERT EDGE",
        "4,2" => "DESERT PATH",
        "5,2" => "OLD RUINS",
        "6,2" => "RUIN DEPTHS",
        // Row 3
        "0,3" => "S. FOREST",
        "1,3" => "S. PATH",
        "2,3" => "SAND DRIFT",
        "3,3" => "PYRAMID",
        "4,3" => "DESERT EXPANSE",
        "5,3" => "DESERT RUINS",
        "6,3" => "WASTELAND",
        // Row 4
        "0,4" => "BEACH",
        "1,4" => "COASTLINE",
        "2,4" => "S. SHORE",
        "3,4" => "CANYON",
        "4,4" => "DEEP DESERT",
        "5,4" => "BADLANDS",
        "6,4" => "SECRET GROVE",
        _ => "UNKNOWN",
    }
}
