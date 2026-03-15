use crate::character::{CharacterCreator, WeaponStyle};
use crate::constants::{GAME_H, GAME_W, HUD_H, PIXEL_SCALE, TILE, WORLD_H, WORLD_W};
use crate::model::{
    Bomb, DeathAnimation, Dir, Enemy, EnemyType, EquippedItem, InventoryEntry, InventoryItem,
    Pickup, PickupType, Player, PlayerState, Projectile, PropKind, TileGrid, TileType, Transition,
    WorldProp, WorldSnapshot,
};
use crate::sprites::{HeartState, Sprites};
use crate::world_data;
use macroquad::prelude::*;

const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");

fn px(v: f32) -> f32 {
    v * PIXEL_SCALE
}

const PICKUP_RENDER_SCALE: f32 = 2.0;

const MAP_TILE_SIZE: f32 = 18.0;
const MAP_TILE_GAP: f32 = 2.0;

pub fn draw_game(
    sprites: &Sprites,
    frame: i32,
    world: &WorldSnapshot,
    player: &Player,
    enemies: &[Enemy],
    pickups: &[Pickup],
    props: &[WorldProp],
    bombs: &[Bomb],
    projectiles: &[Projectile],
    death_animations: &[DeathAnimation],
) {
    clear_background(color_u8!(17, 17, 17, 255));
    let theme_id = world_data::visual_theme_id(
        world.screen_x,
        world.screen_y,
        world.in_dungeon,
        world.dungeon_id,
        world.in_interior,
        &world.interior_id,
    );
    draw_tiles(sprites, &world.tiles, 0.0, 0.0, theme_id);
    draw_props(sprites, props, frame);
    draw_pickups(sprites, pickups, theme_id);
    draw_bombs(sprites, bombs);
    draw_projectiles(sprites, projectiles);
    for enemy in enemies {
        if enemy.active {
            draw_enemy(sprites, enemy, theme_id);
        }
    }
    draw_death_animations(death_animations);
    draw_player(sprites, player, frame);
    draw_hud(sprites, player, world);
}

pub fn draw_inventory(
    sprites: &Sprites,
    world: &WorldSnapshot,
    player: &Player,
    show_map_tab: bool,
    map_mode: crate::game::MapMode,
    inventory_selection: usize,
    frame: i32,
) {
    clear_background(color_u8!(13, 16, 24, 255));

    let outer_x = px(16.0);
    let outer_y = px(14.0);
    let outer_w = GAME_W - px(32.0);
    let outer_h = GAME_H + HUD_H - px(28.0);
    let content_x = outer_x + px(12.0);
    let content_y = outer_y + px(52.0);
    let content_w = outer_w - px(24.0);
    let content_h = outer_h - px(64.0);

    draw_rectangle(
        outer_x,
        outer_y,
        outer_w,
        outer_h,
        color_u8!(24, 28, 39, 255),
    );
    draw_rectangle_lines(
        outer_x,
        outer_y,
        outer_w,
        outer_h,
        px(2.0),
        color_u8!(197, 170, 119, 255),
    );
    draw_text(
        "PAUSE",
        outer_x + px(12.0),
        outer_y + px(22.0),
        px(22.0),
        WHITE,
    );
    draw_text(
        "I / ESC close   TAB switch tab   Z / Enter main   X side",
        outer_x + outer_w - px(360.0),
        outer_y + px(22.0),
        px(12.0),
        LIGHTGRAY,
    );

    // version (from Cargo.toml)
    let version = format!("v{}", GAME_VERSION);
    let version_font = px(10.0);
    let version_dims = measure_text(&version, None, version_font as u16, 1.0);
    draw_text(
        &version,
        outer_x + outer_w - px(10.0) - version_dims.width,
        outer_y + px(14.0),
        version_font,
        LIGHTGRAY,
    );

    draw_inventory_tab(
        outer_x + px(12.0),
        outer_y + px(30.0),
        px(100.0),
        px(22.0),
        "INVENTORY",
        !show_map_tab,
    );
    draw_inventory_tab(
        outer_x + px(118.0),
        outer_y + px(30.0),
        px(80.0),
        px(22.0),
        "MAP",
        show_map_tab,
    );

    let location = world_data::location_name(
        world.screen_x,
        world.screen_y,
        world.in_dungeon,
        world.dungeon_id,
        world.in_interior,
        &world.interior_id,
    );
    draw_text(
        &location,
        outer_x + px(14.0),
        outer_y + px(58.0),
        px(14.0),
        color_u8!(214, 214, 214, 255),
    );
    if show_map_tab {
        draw_rectangle(
            content_x,
            content_y,
            content_w,
            content_h,
            color_u8!(18, 22, 30, 255),
        );
        draw_rectangle_lines(
            content_x,
            content_y,
            content_w,
            content_h,
            px(1.0),
            color_u8!(90, 103, 124, 255),
        );

        // When in a dungeon, show a small vertical mode switcher on the right.
        // This avoids overlaying the map itself.
        let mut map_x = content_x + px(18.0);
        let mut map_w = content_w - px(36.0);
        let mut map_y = content_y + px(10.0);
        let mut map_h = content_h - px(20.0);

        if world.in_dungeon {
            let btn_w = px(80.0);
            let btn_h = px(18.0);
            let btn_x = content_x + content_w - btn_w - px(12.0);
            let btn_y = content_y + px(18.0);
            draw_inventory_tab(
                btn_x,
                btn_y,
                btn_w,
                btn_h,
                "OVERWORLD",
                map_mode == crate::game::MapMode::Overworld,
            );
            draw_inventory_tab(
                btn_x,
                btn_y + btn_h + px(6.0),
                btn_w,
                btn_h,
                "DUNGEON",
                map_mode == crate::game::MapMode::Dungeon,
            );

            map_w -= btn_w + px(16.0);
        }

        if world.in_dungeon && map_mode == crate::game::MapMode::Dungeon {
            draw_dungeon_map_panel(world, map_x, map_y, map_w, map_h, frame);
        } else {
            draw_overworld_map_panel(world, map_x, map_y, map_w, map_h, frame);
        }
    } else {
        let list_w = px(250.0);
        let details_x = content_x + list_w + px(12.0);
        let details_w = content_w - list_w - px(12.0);
        let entries = player.inventory_entries();
        let selected_index = inventory_selection.min(entries.len().saturating_sub(1));
        let selected = entries
            .get(selected_index)
            .copied()
            .unwrap_or(player.inventory_entry(InventoryItem::Sword));
        let row_h = px(19.0);
        let list_start_y = content_y + px(38.0);
        let visible_rows = (((content_h - px(54.0)) / row_h).floor() as usize).max(1);
        let max_scroll = entries.len().saturating_sub(visible_rows);
        let scroll_offset = selected_index
            .saturating_sub(visible_rows / 2)
            .min(max_scroll);

        draw_rectangle(
            content_x,
            content_y,
            list_w,
            content_h,
            color_u8!(31, 38, 51, 255),
        );
        draw_rectangle_lines(
            content_x,
            content_y,
            list_w,
            content_h,
            px(1.0),
            color_u8!(90, 103, 124, 255),
        );
        draw_rectangle(
            details_x,
            content_y,
            details_w,
            content_h,
            color_u8!(18, 22, 30, 255),
        );
        draw_rectangle_lines(
            details_x,
            content_y,
            details_w,
            content_h,
            px(1.0),
            color_u8!(90, 103, 124, 255),
        );

        draw_text(
            "ITEMS",
            content_x + px(10.0),
            content_y + px(18.0),
            px(14.0),
            color_u8!(197, 170, 119, 255),
        );
        draw_text(
            "M",
            content_x + list_w - px(32.0),
            content_y + px(18.0),
            px(12.0),
            color_u8!(255, 215, 120, 255),
        );
        draw_text(
            "S",
            content_x + list_w - px(18.0),
            content_y + px(18.0),
            px(12.0),
            color_u8!(160, 214, 255, 255),
        );
        for (row_index, (index, entry)) in entries
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(visible_rows)
            .enumerate()
        {
            draw_inventory_entry_row(
                sprites,
                player,
                content_x + px(8.0),
                list_start_y + row_index as f32 * row_h,
                list_w - px(16.0),
                entry,
                index == selected_index,
            );
        }

        if entries.len() > visible_rows {
            let track_x = content_x + list_w - px(8.0);
            let track_y = content_y + px(30.0);
            let track_h = content_h - px(44.0);
            let thumb_h = (track_h * visible_rows as f32 / entries.len() as f32).max(px(18.0));
            let thumb_travel = (track_h - thumb_h).max(0.0);
            let thumb_y = if max_scroll == 0 {
                track_y
            } else {
                track_y + thumb_travel * scroll_offset as f32 / max_scroll as f32
            };
            draw_rectangle(
                track_x,
                track_y,
                px(2.0),
                track_h,
                color_u8!(50, 58, 74, 255),
            );
            draw_rectangle(
                track_x - px(1.0),
                thumb_y,
                px(4.0),
                thumb_h,
                color_u8!(168, 177, 194, 255),
            );
        }

        draw_text(
            "DETAILS",
            details_x + px(12.0),
            content_y + px(18.0),
            px(14.0),
            color_u8!(197, 170, 119, 255),
        );
        draw_inventory_detail_panel(
            sprites,
            player,
            details_x + px(12.0),
            content_y + px(34.0),
            details_w - px(24.0),
            selected,
        );
    }
}

fn draw_inventory_tab(x: f32, y: f32, w: f32, h: f32, label: &str, active: bool) {
    let fill = if active {
        color_u8!(58, 69, 89, 255)
    } else {
        color_u8!(28, 34, 46, 255)
    };
    draw_rectangle(x, y, w, h, fill);
    // draw only the top/left/right edges so the bottom edge blends with the
    // underlying panel (avoids a thin "trash" line under the tabs)
    let edge_color = color_u8!(123, 132, 147, 255);
    draw_line(x, y, x + w, y, px(1.0), edge_color); // top
    draw_line(x, y, x, y + h, px(1.0), edge_color); // left
    draw_line(x + w, y, x + w, y + h, px(1.0), edge_color); // right
    draw_text(
        label,
        x + px(10.0),
        y + px(15.0),
        px(12.0),
        if active { WHITE } else { LIGHTGRAY },
    );
}

fn draw_inventory_entry_row(
    sprites: &Sprites,
    player: &Player,
    x: f32,
    y: f32,
    w: f32,
    entry: &InventoryEntry,
    selected: bool,
) {
    if selected {
        draw_rectangle(x, y - px(13.0), w, px(18.0), color_u8!(61, 73, 96, 255));
    }
    draw_inventory_stat(
        sprites,
        player,
        x + px(4.0),
        y,
        w - px(38.0),
        entry.label,
        entry.owned,
        entry.count,
    );
    if entry.equipable && entry.owned {
        let equipped = entry.item.equipped_item();
        let main_assigned = equipped == Some(player.main_item);
        let side_assigned = equipped == Some(player.side_item);
        draw_text(
            if main_assigned { "M" } else { "-" },
            x + w - px(24.0),
            y,
            px(14.0),
            if main_assigned {
                color_u8!(255, 215, 120, 255)
            } else {
                GRAY
            },
        );
        draw_text(
            if side_assigned { "S" } else { "-" },
            x + w - px(12.0),
            y,
            px(14.0),
            if side_assigned {
                color_u8!(160, 214, 255, 255)
            } else {
                GRAY
            },
        );
    }
}

fn draw_inventory_detail_panel(
    sprites: &Sprites,
    player: &Player,
    x: f32,
    y: f32,
    w: f32,
    entry: InventoryEntry,
) {
    draw_inventory_stat(
        sprites,
        player,
        x,
        y + px(12.0),
        w,
        entry.label,
        entry.owned,
        entry.count,
    );
    let status = if entry.equipable {
        let equipped = entry.item.equipped_item();
        let main_assigned = equipped == Some(player.main_item);
        let side_assigned = equipped == Some(player.side_item);
        if !entry.owned {
            "Locked".to_string()
        } else if main_assigned {
            "Assigned to MAIN (Z / Space)".to_string()
        } else if side_assigned {
            "Assigned to SIDE (X)".to_string()
        } else {
            "Press Enter/Z/Space for MAIN or X for SIDE".to_string()
        }
    } else if entry.owned {
        "Passive / always available".to_string()
    } else {
        "Not acquired yet".to_string()
    };
    draw_text(
        &status,
        x,
        y + px(42.0),
        px(12.0),
        color_u8!(196, 196, 196, 255),
    );

    let wrapped = wrap_text_to_width(entry.description, w, px(12.0));
    for (i, line) in wrapped.iter().enumerate() {
        draw_text(
            line,
            x,
            y + px(72.0) + i as f32 * px(14.0),
            px(12.0),
            LIGHTGRAY,
        );
    }
    let info_y = y + px(72.0) + wrapped.len() as f32 * px(14.0) + px(16.0);

    draw_text(
        &format!("Main: {}", player.main_item.label()),
        x,
        info_y,
        px(12.0),
        color_u8!(255, 215, 120, 255),
    );
    draw_text(
        &format!("Side: {}", player.side_item.label()),
        x,
        info_y + px(24.0),
        px(12.0),
        color_u8!(160, 214, 255, 255),
    );
    draw_text(
        &format!("HP {} / {}", player.hp, player.max_hp),
        x,
        info_y + px(48.0),
        px(12.0),
        WHITE,
    );
    draw_text(
        &format!("Gems {}   Keys {}", player.gems, player.keys),
        x,
        info_y + px(66.0),
        px(12.0),
        WHITE,
    );
}

pub fn wrap_text_to_width(text: &str, max_width: f32, font_size: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let pending = if current.is_empty() {
            word.to_string()
        } else {
            format!("{current} {word}")
        };
        if measure_text(&pending, None, font_size as u16, 1.0).width > max_width
            && !current.is_empty()
        {
            lines.push(current);
            current = word.to_string();
        } else {
            current = pending;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn draw_dragon(x: f32, y: f32, scale: f32) {
    let s = scale * PIXEL_SCALE;
    let outline = color_u8!(37, 22, 18, 255);
    let body = color_u8!(170, 63, 46, 255);
    let body_shadow = color_u8!(118, 36, 34, 255);
    let wing = color_u8!(127, 34, 38, 255);
    let wing_membrane = color_u8!(214, 109, 62, 255);
    let horn = color_u8!(231, 214, 158, 255);
    let flame_core = color_u8!(255, 241, 154, 255);
    let flame_mid = color_u8!(255, 166, 54, 255);
    let flame_outer = color_u8!(220, 69, 29, 255);

    // rear wing
    draw_triangle(
        vec2(x + s * 12.0, y + s * 14.0),
        vec2(x - s * 3.0, y - s * 4.0),
        vec2(x + s * 22.0, y + s * 2.0),
        wing,
    );
    draw_triangle(
        vec2(x + s * 11.0, y + s * 13.0),
        vec2(x + s * 2.0, y + s * 1.0),
        vec2(x + s * 20.0, y + s * 4.0),
        wing_membrane,
    );

    // tail
    draw_triangle(
        vec2(x + s * 4.0, y + s * 24.0),
        vec2(x - s * 15.0, y + s * 20.0),
        vec2(x - s * 6.0, y + s * 11.0),
        body_shadow,
    );
    draw_triangle(
        vec2(x - s * 16.0, y + s * 20.0),
        vec2(x - s * 25.0, y + s * 16.0),
        vec2(x - s * 18.0, y + s * 12.0),
        outline,
    );

    // body and neck
    draw_circle(x + s * 14.0, y + s * 23.0, s * 10.0, outline);
    draw_circle(x + s * 15.0, y + s * 22.0, s * 8.5, body);
    draw_triangle(
        vec2(x + s * 18.0, y + s * 18.0),
        vec2(x + s * 31.0, y + s * 11.0),
        vec2(x + s * 23.0, y + s * 27.0),
        outline,
    );
    draw_triangle(
        vec2(x + s * 19.0, y + s * 18.5),
        vec2(x + s * 29.0, y + s * 12.5),
        vec2(x + s * 23.0, y + s * 25.5),
        body,
    );
    draw_circle(x + s * 17.0, y + s * 25.0, s * 5.5, body_shadow);

    // front wing
    draw_triangle(
        vec2(x + s * 18.0, y + s * 14.0),
        vec2(x + s * 12.0, y - s * 10.0),
        vec2(x + s * 40.0, y + s * 5.0),
        outline,
    );
    draw_triangle(
        vec2(x + s * 18.0, y + s * 14.0),
        vec2(x + s * 15.0, y - s * 5.0),
        vec2(x + s * 36.0, y + s * 6.0),
        wing,
    );
    draw_triangle(
        vec2(x + s * 19.0, y + s * 13.0),
        vec2(x + s * 18.0, y - s * 1.0),
        vec2(x + s * 33.0, y + s * 7.0),
        wing_membrane,
    );

    // legs
    draw_triangle(
        vec2(x + s * 12.0, y + s * 30.0),
        vec2(x + s * 8.0, y + s * 39.0),
        vec2(x + s * 16.0, y + s * 31.0),
        outline,
    );
    draw_triangle(
        vec2(x + s * 22.0, y + s * 29.0),
        vec2(x + s * 19.0, y + s * 38.0),
        vec2(x + s * 28.0, y + s * 31.0),
        outline,
    );

    // head
    draw_triangle(
        vec2(x + s * 27.0, y + s * 10.0),
        vec2(x + s * 43.0, y + s * 10.0),
        vec2(x + s * 34.0, y + s * 22.0),
        outline,
    );
    draw_triangle(
        vec2(x + s * 28.0, y + s * 11.0),
        vec2(x + s * 41.0, y + s * 11.0),
        vec2(x + s * 34.0, y + s * 20.0),
        body,
    );
    draw_triangle(
        vec2(x + s * 30.0, y + s * 10.0),
        vec2(x + s * 32.0, y + s * 3.0),
        vec2(x + s * 35.0, y + s * 10.0),
        horn,
    );
    draw_triangle(
        vec2(x + s * 34.0, y + s * 10.0),
        vec2(x + s * 38.0, y + s * 4.0),
        vec2(x + s * 39.0, y + s * 11.0),
        horn,
    );
    draw_circle(x + s * 36.5, y + s * 13.0, s * 1.2, WHITE);
    draw_circle(x + s * 36.8, y + s * 13.1, s * 0.6, outline);

    // jaw and fire
    draw_triangle(
        vec2(x + s * 35.0, y + s * 16.0),
        vec2(x + s * 44.0, y + s * 17.0),
        vec2(x + s * 35.0, y + s * 21.0),
        body_shadow,
    );
    draw_triangle(
        vec2(x + s * 43.0, y + s * 16.5),
        vec2(x + s * 58.0, y + s * 13.0),
        vec2(x + s * 58.0, y + s * 22.0),
        flame_outer,
    );
    draw_triangle(
        vec2(x + s * 44.0, y + s * 17.0),
        vec2(x + s * 54.0, y + s * 15.0),
        vec2(x + s * 54.0, y + s * 20.0),
        flame_mid,
    );
    draw_triangle(
        vec2(x + s * 45.0, y + s * 17.2),
        vec2(x + s * 50.0, y + s * 16.2),
        vec2(x + s * 50.0, y + s * 19.2),
        flame_core,
    );
}

pub fn draw_transition(
    sprites: &Sprites,
    world: &WorldSnapshot,
    transition: &Transition,
    next_tiles: Option<&TileGrid>,
    player: &Player,
) {
    clear_background(color_u8!(17, 17, 17, 255));
    let theme_id = world_data::visual_theme_id(
        world.screen_x,
        world.screen_y,
        world.in_dungeon,
        world.dungeon_id,
        world.in_interior,
        &world.interior_id,
    );
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
    draw_tiles(sprites, &transition.old_tiles, old_ox, old_oy, theme_id);
    if let Some(next_tiles) = next_tiles {
        draw_tiles(sprites, next_tiles, new_ox, new_oy, theme_id);
    }
    draw_hud(sprites, player, world);
}

pub fn draw_fade_overlay(alpha: f32) {
    draw_rectangle(0.0, HUD_H, GAME_W, GAME_H, Color::new(0.0, 0.0, 0.0, alpha));
}

pub fn draw_title(sprites: &Sprites, frame: i32, selected_menu: usize) {
    clear_background(color_u8!(17, 17, 17, 255));
    let cx = GAME_W / 2.0;
    // enlarge banner by another 50% (now 225% of original)
    let dragon_w = px(128.0) * 2.25;
    let dragon_h = px(96.0) * 2.25;
    let dragon_x = cx - dragon_w / 2.0;
    // ensure dragon doesn't start above window
    let mut dragon_y = px(6.0) - (dragon_h - px(96.0)) / 2.0;
    if dragon_y < 0.0 {
        dragon_y = 0.0;
    }
    if !sprites.draw_title_dragon(dragon_x, dragon_y, dragon_w, dragon_h) {
        draw_dragon(cx - px(36.0), px(12.0), 1.55);
    }

    // compute baseline based on dragon bottom, reducing gap
    let cy = dragon_y + dragon_h + px(4.0);
    let title_width = px(170.0);
    let start_x = cx - title_width / 2.0;
    draw_text("DELTA", start_x + px(38.0), cy + px(8.0), px(36.0), YELLOW);
    draw_text(
        "A World of Secrets",
        start_x + px(4.0),
        cy + px(28.0),
        px(20.0),
        GRAY,
    );
    let menu_y = cy + px(58.0);
    let start_color = if selected_menu == 0 { WHITE } else { GRAY };
    let customize_color = if selected_menu == 1 { WHITE } else { GRAY };
    if selected_menu == 0 && (frame / 30) % 2 == 0 {
        draw_text(">", cx - px(66.0), menu_y, px(24.0), WHITE);
    }
    if selected_menu == 1 && (frame / 30) % 2 == 0 {
        draw_text(">", cx - px(66.0), menu_y + px(22.0), px(24.0), WHITE);
    }
    draw_text("Start Game", cx - px(48.0), menu_y, px(22.0), start_color);
    draw_text(
        "Customize Hero",
        cx - px(74.0),
        menu_y + px(22.0),
        px(20.0),
        customize_color,
    );

    // control notes – formatted clearly
    let notes = [
        "W/S  : menu",
        "ENTER: select",
        "WASD : move around",
        "TAB  : map",
    ];
    // calculate left edge relative to start_x so text is centered under title
    let notes_x = start_x + px(0.0);
    for (i, &note) in notes.iter().enumerate() {
        draw_text(
            note,
            notes_x,
            cy + px(96.0) + i as f32 * px(20.0),
            px(14.0),
            LIGHTGRAY,
        );
    }

    // version (from Cargo.toml)
    let version = format!("v{}", GAME_VERSION);
    let version_font = px(10.0);
    let version_dims = measure_text(&version, None, version_font as u16, 1.0);
    draw_text(
        &version,
        GAME_W - px(10.0) - version_dims.width,
        px(14.0),
        version_font,
        LIGHTGRAY,
    );
}

pub fn draw_character_creator(sprites: &Sprites, creator: &CharacterCreator, frame: i32) {
    clear_background(color_u8!(16, 18, 24, 255));

    let outer_x = px(14.0);
    let outer_y = px(12.0);
    let outer_w = GAME_W - px(28.0);
    let outer_h = GAME_H + HUD_H - px(24.0);
    let preview_w = px(126.0);
    let list_x = outer_x + preview_w + px(18.0);
    let list_w = outer_w - preview_w - px(30.0);

    draw_rectangle(
        outer_x,
        outer_y,
        outer_w,
        outer_h,
        color_u8!(25, 29, 38, 255),
    );
    draw_rectangle_lines(
        outer_x,
        outer_y,
        outer_w,
        outer_h,
        px(2.0),
        color_u8!(193, 170, 118, 255),
    );
    draw_rectangle(
        outer_x + px(8.0),
        outer_y + px(36.0),
        preview_w,
        outer_h - px(44.0),
        color_u8!(33, 38, 50, 255),
    );
    draw_rectangle(
        list_x,
        outer_y + px(36.0),
        list_w,
        outer_h - px(44.0),
        color_u8!(20, 23, 31, 255),
    );

    draw_text(
        "CREATE HERO",
        outer_x + px(12.0),
        outer_y + px(22.0),
        px(22.0),
        WHITE,
    );
    draw_text(
        "R randomize  ENTER save",
        outer_x + outer_w - px(166.0),
        outer_y + px(22.0),
        px(12.0),
        LIGHTGRAY,
    );

    let mut preview = Player::new();
    preview.x = outer_x + px(38.0);
    preview.y = outer_y + px(114.0);
    preview.dir = match (frame / 90) % 4 {
        0 => Dir::Down,
        1 => Dir::Left,
        2 => Dir::Right,
        _ => Dir::Up,
    };
    preview.state = PlayerState::Walking;
    preview.walk_frame = (frame / 18) % 4;
    preview.has_sword = creator.appearance.weapon != WeaponStyle::None;
    draw_text(
        "PREVIEW",
        outer_x + px(18.0),
        outer_y + px(60.0),
        px(16.0),
        color_u8!(197, 170, 119, 255),
    );
    draw_player(sprites, &preview, frame);

    let rows_per_col = creator.field_count().div_ceil(2);
    let col_w = (list_w - px(20.0)) / 2.0;
    for index in 0..creator.field_count() {
        let col = index / rows_per_col;
        let row = index % rows_per_col;
        let line_x = list_x + px(8.0) + col as f32 * col_w;
        let line_y = outer_y + px(58.0) + row as f32 * px(15.0);
        let selected = index == creator.selected_field;
        if selected {
            draw_rectangle(
                line_x - px(2.0),
                line_y - px(14.0),
                col_w - px(4.0),
                px(20.0),
                color_u8!(51, 61, 78, 255),
            );
        }
        if let Some(color) = creator.field_color(index) {
            draw_rectangle(
                line_x + px(4.0),
                line_y - px(10.0),
                px(10.0),
                px(10.0),
                color,
            );
            draw_rectangle_lines(
                line_x + px(4.0),
                line_y - px(10.0),
                px(10.0),
                px(10.0),
                px(1.0),
                BLACK,
            );
        }
        draw_text(
            &creator.field_text(index),
            line_x + px(18.0),
            line_y,
            px(10.0),
            if selected { WHITE } else { LIGHTGRAY },
        );
    }

    draw_text(
        "W/S select  A/D change  ESC back",
        list_x + px(8.0),
        outer_y + outer_h - px(16.0),
        px(12.0),
        color_u8!(160, 167, 178, 255),
    );
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
    let width = px(240.0);
    let text_width = width - px(20.0); // Account for padding on both sides
    let font_size = px(20.0);

    // Wrap text to fit the box width
    let lines = wrap_text_to_width(text, text_width, font_size);

    let height = px(24.0) + lines.len() as f32 * px(20.0);
    let x = (GAME_W - width) / 2.0;
    let y = ((GAME_H + HUD_H) - height) / 2.0;
    draw_rectangle(x, y, width, height, BLACK);
    draw_rectangle_lines(x, y, width, height, px(2.0), WHITE);
    for (index, line) in lines.iter().enumerate() {
        draw_text(
            line,
            x + px(10.0),
            y + px(24.0) + index as f32 * px(18.0),
            font_size,
            WHITE,
        );
    }
}

fn draw_tiles(sprites: &Sprites, tiles: &TileGrid, ox: f32, oy: f32, theme_id: Option<i32>) {
    for (row, line) in tiles.iter().enumerate() {
        for (col, tile) in line.iter().enumerate() {
            let x = ox + col as f32 * TILE;
            let y = oy + row as f32 * TILE + HUD_H;
            let underlay = tile_base(*tile).unwrap_or(*tile);
            if !sprites.draw_tile(theme_id, underlay, x, y, WHITE) {
                draw_rectangle(x, y, TILE, TILE, tile_color(underlay));
            }
            if draw_house_tile(*tile, x, y) {
                continue;
            }
            if !sprites.draw_tile(theme_id, *tile, x, y, tile_tint(*tile)) {
                draw_rectangle(x, y, TILE, TILE, tile_color(*tile));
            }
            match *tile {
                TileType::DoorLocked => draw_keyhole(x, y, color_u8!(200, 170, 50, 255)),
                TileType::BossDoor => draw_keyhole(x, y, color_u8!(220, 40, 40, 255)),
                _ => {}
            }
        }
    }
}

fn draw_house_tile(tile: TileType, x: f32, y: f32) -> bool {
    let roof = color_u8!(145, 62, 48, 255);
    let roof_shadow = color_u8!(102, 39, 32, 255);
    let trim = color_u8!(232, 207, 150, 255);
    let plaster = color_u8!(214, 192, 143, 255);
    let timber = color_u8!(111, 74, 46, 255);
    let window = color_u8!(122, 186, 215, 255);
    let window_glow = color_u8!(214, 235, 242, 255);
    let door_dark = color_u8!(67, 44, 28, 255);
    let door_light = color_u8!(139, 93, 53, 255);
    let floor_dark = color_u8!(106, 73, 44, 255);
    let floor_light = color_u8!(145, 99, 60, 255);
    let chair_wood = color_u8!(120, 78, 46, 255);
    let chair_shadow = color_u8!(74, 47, 29, 255);

    match tile {
        TileType::HouseRoof => {
            draw_rectangle(x, y, TILE, TILE, roof);
            draw_rectangle(x, y + px(2.0), TILE, px(4.0), roof_shadow);
            draw_rectangle(x, y + TILE - px(4.0), TILE, px(3.0), trim);
            true
        }
        TileType::HouseRoofLeft => {
            draw_rectangle(x + px(4.0), y, TILE - px(4.0), TILE, roof);
            draw_triangle(
                vec2(x + px(4.0), y),
                vec2(x + px(4.0), y + TILE),
                vec2(x, y + TILE),
                roof_shadow,
            );
            draw_rectangle(
                x + px(4.0),
                y + TILE - px(4.0),
                TILE - px(4.0),
                px(3.0),
                trim,
            );
            true
        }
        TileType::HouseRoofRight => {
            draw_rectangle(x, y, TILE - px(4.0), TILE, roof);
            draw_triangle(
                vec2(x + TILE - px(4.0), y),
                vec2(x + TILE, y + TILE),
                vec2(x + TILE - px(4.0), y + TILE),
                roof_shadow,
            );
            draw_rectangle(x, y + TILE - px(4.0), TILE - px(4.0), px(3.0), trim);
            true
        }
        TileType::HouseWall => {
            draw_rectangle(x, y, TILE, TILE, plaster);
            draw_rectangle(x, y, TILE, px(3.0), timber);
            draw_rectangle(x, y + TILE - px(3.0), TILE, px(3.0), timber);
            draw_rectangle(x + px(3.0), y, px(3.0), TILE, timber);
            draw_rectangle(x + TILE - px(6.0), y, px(3.0), TILE, timber);
            true
        }
        TileType::HouseWindow => {
            draw_rectangle(x, y, TILE, TILE, plaster);
            draw_rectangle(x, y, TILE, px(3.0), timber);
            draw_rectangle(x, y + TILE - px(3.0), TILE, px(3.0), timber);
            draw_rectangle(x + px(3.0), y, px(3.0), TILE, timber);
            draw_rectangle(x + TILE - px(6.0), y, px(3.0), TILE, timber);
            draw_rectangle(
                x + px(5.0),
                y + px(5.0),
                TILE - px(10.0),
                TILE - px(10.0),
                window,
            );
            draw_line(
                x + TILE / 2.0,
                y + px(5.0),
                x + TILE / 2.0,
                y + TILE - px(5.0),
                px(1.0),
                window_glow,
            );
            draw_line(
                x + px(5.0),
                y + TILE / 2.0,
                x + TILE - px(5.0),
                y + TILE / 2.0,
                px(1.0),
                window_glow,
            );
            true
        }
        TileType::HouseDoor => {
            draw_rectangle(x, y, TILE, TILE, plaster);
            draw_rectangle(x, y, TILE, px(3.0), timber);
            draw_rectangle(
                x + px(2.0),
                y + px(3.0),
                TILE - px(4.0),
                TILE - px(3.0),
                door_dark,
            );
            draw_rectangle(
                x + px(4.0),
                y + px(5.0),
                TILE - px(8.0),
                TILE - px(7.0),
                door_light,
            );
            draw_circle(x + TILE - px(6.0), y + TILE / 2.0, px(1.4), trim);
            true
        }
        TileType::WoodFloor => {
            draw_rectangle(x, y, TILE, TILE, floor_light);
            draw_rectangle(x, y, TILE, px(2.0), floor_dark);
            draw_rectangle(x, y + px(6.0), TILE, px(2.0), floor_dark);
            draw_rectangle(x, y + px(12.0), TILE, px(2.0), floor_dark);
            draw_rectangle(x + TILE / 2.0 - px(1.0), y, px(2.0), TILE, floor_dark);
            true
        }
        TileType::HouseChair => {
            draw_house_tile(TileType::WoodFloor, x, y);
            draw_rectangle(
                x + px(5.0),
                y + px(4.0),
                TILE - px(10.0),
                px(4.0),
                chair_wood,
            );
            draw_rectangle(
                x + px(5.0),
                y + px(8.0),
                px(3.0),
                TILE - px(12.0),
                chair_shadow,
            );
            draw_rectangle(
                x + TILE - px(8.0),
                y + px(8.0),
                px(3.0),
                TILE - px(12.0),
                chair_shadow,
            );
            draw_rectangle(
                x + px(4.0),
                y + TILE - px(8.0),
                TILE - px(8.0),
                px(3.0),
                chair_wood,
            );
            true
        }
        _ => false,
    }
}

fn draw_keyhole(x: f32, y: f32, color: Color) {
    let cx = x + TILE / 2.0;
    let cy = y + TILE / 2.0;
    // Circle top of keyhole
    draw_circle(cx, cy - px(2.0), px(5.0), BLACK);
    draw_circle(cx, cy - px(2.0), px(3.5), color);
    // Slot bottom of keyhole
    draw_rectangle(cx - px(2.0), cy + px(1.0), px(4.0), px(6.0), BLACK);
    draw_rectangle(cx - px(1.0), cy + px(2.0), px(2.0), px(4.0), color);
}

fn draw_props(sprites: &Sprites, props: &[WorldProp], frame: i32) {
    for prop in props {
        let x = prop.tile_x as f32 * TILE;
        let y = prop.tile_y as f32 * TILE + HUD_H;
        match prop.kind {
            PropKind::PressurePlate => {
                draw_rectangle(
                    x + px(6.0),
                    y + px(22.0),
                    TILE - px(12.0),
                    px(10.0),
                    color_u8!(86, 128, 118, 255),
                );
                draw_rectangle_lines(
                    x + px(6.0),
                    y + px(22.0),
                    TILE - px(12.0),
                    px(10.0),
                    px(1.0),
                    color_u8!(44, 66, 60, 255),
                );
            }
            PropKind::Boulder => {
                draw_rectangle(
                    x + px(6.0),
                    y + px(6.0),
                    TILE - px(12.0),
                    TILE - px(12.0),
                    color_u8!(76, 84, 92, 255),
                );
                draw_rectangle(
                    x + px(12.0),
                    y + px(10.0),
                    TILE - px(24.0),
                    TILE - px(24.0),
                    color_u8!(116, 124, 132, 255),
                );
                draw_rectangle_lines(
                    x + px(6.0),
                    y + px(6.0),
                    TILE - px(12.0),
                    TILE - px(12.0),
                    px(2.0),
                    color_u8!(46, 50, 56, 255),
                );
            }
            PropKind::LadderPoint => {
                draw_ladder_icon(x + px(8.0), y + px(4.0), 1.0, color_u8!(174, 138, 88, 255));
                draw_circle(
                    x + TILE - px(8.0),
                    y + px(8.0),
                    px(3.0),
                    color_u8!(230, 214, 164, 255),
                );
            }
            PropKind::Npc(kind) => {
                let _ = sprites.draw_npc(kind, x + TILE * 0.5, y + TILE * 0.72, frame);
            }
        }
    }
}

fn tile_base(tile: TileType) -> Option<TileType> {
    match tile {
        TileType::Tree | TileType::Bush | TileType::Rock | TileType::Cave | TileType::Dungeon => {
            Some(TileType::Grass)
        }
        TileType::HouseRoof
        | TileType::HouseRoofLeft
        | TileType::HouseRoofRight
        | TileType::HouseWall
        | TileType::HouseWindow => Some(TileType::Grass),
        TileType::HouseDoor => Some(TileType::Path),
        TileType::HouseChair => Some(TileType::WoodFloor),
        TileType::Bridge => Some(TileType::Water),
        _ => None,
    }
}

fn tile_tint(tile: TileType) -> Color {
    match tile {
        TileType::Tree => color_u8!(34, 102, 51, 255),
        TileType::Bush => color_u8!(25, 140, 50, 255),
        TileType::Rock => color_u8!(128, 128, 128, 255),
        TileType::Cracked => color_u8!(140, 110, 80, 255),
        TileType::Stairs => color_u8!(119, 119, 153, 255),
        TileType::DoorLocked => color_u8!(200, 170, 50, 255),
        TileType::BossDoor => color_u8!(200, 40, 40, 255),
        TileType::HouseRoof
        | TileType::HouseRoofLeft
        | TileType::HouseRoofRight
        | TileType::HouseWall
        | TileType::HouseWindow
        | TileType::HouseDoor => WHITE,
        TileType::WoodFloor | TileType::HouseChair => WHITE,
        _ => WHITE,
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
        TileType::HouseRoof | TileType::HouseRoofLeft | TileType::HouseRoofRight => {
            color_u8!(145, 62, 48, 255)
        }
        TileType::HouseWall => color_u8!(214, 192, 143, 255),
        TileType::HouseWindow => color_u8!(122, 186, 215, 255),
        TileType::HouseDoor => color_u8!(111, 74, 46, 255),
        TileType::WoodFloor => color_u8!(145, 99, 60, 255),
        TileType::HouseChair => color_u8!(120, 78, 46, 255),
    }
}

fn draw_death_animations(animations: &[DeathAnimation]) {
    for anim in animations {
        let progress = 1.0 - (anim.timer as f32 / 20.0);
        let size = px(8.0) + progress * px(40.0);
        let alpha = (anim.timer as f32 / 20.0).clamp(0.0, 1.0);
        let cx = anim.x - size / 2.0;
        let cy = anim.y + HUD_H - size / 2.0;
        draw_rectangle(cx, cy, size, size, Color::new(1.0, 1.0, 1.0, alpha * 0.5));
        draw_rectangle_lines(
            cx,
            cy,
            size,
            size,
            px(2.0),
            Color::new(1.0, 1.0, 0.5, alpha),
        );
    }
}

fn draw_player(sprites: &Sprites, player: &Player, frame: i32) {
    let x = player.x.round();
    let y = player.y.round() + HUD_H;
    if player.invuln_timer > 0 && (player.invuln_timer / 3) % 2 == 0 {
        return;
    }
    if sprites.draw_player(player, x, y, frame) {
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
        let x_offset = x;
        let y_offset = y - px(32.0);
        match dir {
            Dir::Up => {
                draw_rectangle(
                    x_offset + px(22.0),
                    y_offset - px(10.0),
                    px(4.0),
                    px(18.0),
                    blade,
                );
                draw_rectangle(
                    x_offset + px(20.0),
                    y_offset + px(7.0),
                    px(8.0),
                    px(2.0),
                    hilt,
                );
            }
            Dir::Down => {
                draw_rectangle(
                    x_offset + px(22.0),
                    y_offset + px(30.0),
                    px(4.0),
                    px(18.0),
                    blade,
                );
                draw_rectangle(
                    x_offset + px(20.0),
                    y_offset + px(29.0),
                    px(8.0),
                    px(2.0),
                    hilt,
                );
            }
            Dir::Left => {
                draw_rectangle(
                    x_offset - px(12.0),
                    y_offset + px(22.0),
                    px(18.0),
                    px(4.0),
                    blade,
                );
                draw_rectangle(
                    x_offset + px(5.0),
                    y_offset + px(20.0),
                    px(2.0),
                    px(8.0),
                    hilt,
                );
            }
            Dir::Right => {
                draw_rectangle(
                    x_offset + px(42.0),
                    y_offset + px(22.0),
                    px(18.0),
                    px(4.0),
                    blade,
                );
                draw_rectangle(
                    x_offset + px(41.0),
                    y_offset + px(20.0),
                    px(2.0),
                    px(8.0),
                    hilt,
                );
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

fn draw_enemy(sprites: &Sprites, enemy: &Enemy, theme_id: Option<i32>) {
    let x = enemy.x.round();
    let y = enemy.y.round() + HUD_H;
    if enemy.flash_timer > 0 && (enemy.flash_timer / 2) % 2 == 0 {
        draw_rectangle(x, y, enemy.w, enemy.h, WHITE);
        return;
    }
    if sprites.draw_enemy(theme_id, enemy, x, y) {
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

fn draw_pickups(sprites: &Sprites, pickups: &[Pickup], theme_id: Option<i32>) {
    for pickup in pickups {
        let draw_w = pickup.w * PICKUP_RENDER_SCALE;
        let draw_h = pickup.h * PICKUP_RENDER_SCALE;
        let x = pickup.x.round() - (draw_w - pickup.w) * 0.5;
        let y = pickup.y.round() + HUD_H + (pickup.timer as f32 * 0.1).sin() * px(1.5)
            - (draw_h - pickup.h) * 0.5;
        if !sprites.draw_pickup(theme_id, pickup.pickup_type, x, y, draw_w, draw_h) {
            let color = match pickup.pickup_type {
                PickupType::Heart | PickupType::HeartContainer => RED,
                PickupType::Key => YELLOW,
                PickupType::BossKey => ORANGE,
                PickupType::BombAmmo | PickupType::Bombs => DARKGRAY,
                PickupType::Gem => SKYBLUE,
                PickupType::Ladder => color_u8!(166, 120, 72, 255),
                PickupType::Hammer => color_u8!(124, 124, 136, 255),
                PickupType::Raft => color_u8!(126, 90, 52, 255),
                PickupType::StrongArmGlove => color_u8!(168, 118, 64, 255),
                PickupType::PortalTool => color_u8!(126, 84, 178, 255),
                PickupType::DragonPiece => color_u8!(226, 194, 92, 255),
                PickupType::Sword => color_u8!(210, 224, 232, 255),
                PickupType::TideChart => color_u8!(112, 170, 214, 255),
                PickupType::EmberCrystal => color_u8!(231, 110, 62, 255),
                PickupType::VoidCompass => color_u8!(188, 146, 230, 255),
                PickupType::CrystalOfSeeing => color_u8!(182, 241, 255, 255),
            };
            match pickup.pickup_type {
                PickupType::Gem => draw_gem_icon(x, y, draw_w / px(16.0), SKYBLUE),
                PickupType::Ladder => {
                    draw_ladder_icon(x + px(4.0), y + px(2.0), draw_w / px(18.0), color)
                }
                PickupType::Hammer => draw_hammer_icon(x + px(2.0), y + px(2.0), draw_w / px(16.0)),
                PickupType::Raft => draw_raft_icon(x + px(2.0), y + px(4.0), draw_w / px(16.0)),
                PickupType::StrongArmGlove => {
                    draw_glove_icon(x + px(2.0), y + px(2.0), draw_w / px(16.0))
                }
                PickupType::PortalTool => {
                    draw_portal_icon(x + px(2.0), y + px(2.0), draw_w / px(16.0))
                }
                PickupType::DragonPiece => {
                    draw_dragon_piece_icon(x + px(3.0), y + px(2.0), draw_w / px(16.0))
                }
                PickupType::Sword => draw_hammer_icon(x + px(2.0), y + px(2.0), draw_w / px(16.0)),
                PickupType::TideChart => draw_rectangle(
                    x + px(3.0),
                    y + px(3.0),
                    draw_w - px(6.0),
                    draw_h - px(6.0),
                    color,
                ),
                PickupType::EmberCrystal => draw_gem_icon(x, y, draw_w / px(16.0), color),
                PickupType::VoidCompass => {
                    draw_portal_icon(x + px(2.0), y + px(2.0), draw_w / px(16.0))
                }
                PickupType::CrystalOfSeeing => draw_gem_icon(x, y, draw_w / px(16.0), color),
                _ => draw_rectangle(x, y, draw_w, draw_h, color),
            }
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

fn draw_sword_icon(x: f32, y: f32, scale: f32, tint: Color) {
    let blade = tint;
    let hilt = color_u8!(196, 160, 74, 255);
    draw_rectangle(x + scale * 4.0, y, scale * 4.0, scale * 12.0, blade);
    draw_rectangle(
        x + scale * 2.0,
        y + scale * 10.0,
        scale * 8.0,
        scale * 2.0,
        hilt,
    );
    draw_rectangle(
        x + scale * 5.0,
        y + scale * 12.0,
        scale * 2.0,
        scale * 4.0,
        hilt,
    );
}

fn draw_gem_icon(x: f32, y: f32, scale: f32, tint: Color) {
    let top = vec2(x + scale * 6.0, y);
    let left = vec2(x, y + scale * 6.0);
    let right = vec2(x + scale * 12.0, y + scale * 6.0);
    let bottom = vec2(x + scale * 6.0, y + scale * 12.0);
    let center = vec2(x + scale * 6.0, y + scale * 6.0);
    let dark = color_u8!(44, 92, 168, 255);
    let mid = tint;
    let light = color_u8!(170, 230, 255, 255);

    draw_triangle(top, left, center, light);
    draw_triangle(top, center, right, mid);
    draw_triangle(left, bottom, center, mid);
    draw_triangle(center, bottom, right, dark);

    draw_line(
        top.x,
        top.y,
        right.x,
        right.y,
        scale,
        color_u8!(20, 56, 124, 255),
    );
    draw_line(
        right.x,
        right.y,
        bottom.x,
        bottom.y,
        scale,
        color_u8!(20, 56, 124, 255),
    );
    draw_line(
        bottom.x,
        bottom.y,
        left.x,
        left.y,
        scale,
        color_u8!(20, 56, 124, 255),
    );
    draw_line(
        left.x,
        left.y,
        top.x,
        top.y,
        scale,
        color_u8!(20, 56, 124, 255),
    );
    draw_line(
        top.x,
        top.y,
        bottom.x,
        bottom.y,
        scale * 0.7,
        color_u8!(110, 200, 245, 255),
    );
    draw_line(
        left.x,
        left.y,
        right.x,
        right.y,
        scale * 0.7,
        color_u8!(110, 200, 245, 255),
    );
    draw_circle(x + scale * 4.0, y + scale * 3.0, scale * 0.9, WHITE);
}

fn draw_ladder_icon(x: f32, y: f32, scale: f32, tint: Color) {
    let dark = color_u8!(90, 62, 34, 255);
    draw_rectangle(x, y, scale * 2.0, scale * 14.0, dark);
    draw_rectangle(x + scale * 8.0, y, scale * 2.0, scale * 14.0, dark);
    for rung in [2.0, 5.0, 8.0, 11.0] {
        draw_rectangle(
            x + scale * 1.5,
            y + scale * rung,
            scale * 7.0,
            scale * 1.2,
            tint,
        );
    }
}

fn draw_hammer_icon(x: f32, y: f32, scale: f32) {
    let metal = color_u8!(150, 154, 168, 255);
    let metal_shadow = color_u8!(90, 96, 110, 255);
    let wood = color_u8!(126, 82, 44, 255);
    draw_rectangle(x + scale * 5.0, y, scale * 6.0, scale * 4.0, metal);
    draw_rectangle(
        x + scale * 8.0,
        y + scale * 3.0,
        scale * 2.0,
        scale * 10.0,
        wood,
    );
    draw_rectangle(
        x + scale * 5.0,
        y + scale * 3.0,
        scale * 3.0,
        scale * 2.0,
        metal_shadow,
    );
}

fn draw_raft_icon(x: f32, y: f32, scale: f32) {
    let wood = color_u8!(132, 92, 54, 255);
    let rope = color_u8!(206, 178, 114, 255);
    for plank in [0.0, 4.0, 8.0] {
        draw_rectangle(
            x + plank * scale * 0.4,
            y + plank * 0.0,
            scale * 3.0,
            scale * 10.0,
            wood,
        );
    }
    draw_rectangle(x, y + scale * 2.0, scale * 10.0, scale * 1.2, rope);
    draw_rectangle(x, y + scale * 7.0, scale * 10.0, scale * 1.2, rope);
}

fn draw_glove_icon(x: f32, y: f32, scale: f32) {
    let leather = color_u8!(162, 118, 68, 255);
    let dark = color_u8!(92, 62, 30, 255);
    draw_rectangle(
        x + scale * 3.0,
        y + scale * 2.0,
        scale * 6.0,
        scale * 8.0,
        leather,
    );
    for finger in [0.0, 2.0, 4.0, 6.0] {
        draw_rectangle(
            x + scale * (3.0 + finger),
            y,
            scale * 1.5,
            scale * 4.0,
            leather,
        );
    }
    draw_rectangle_lines(
        x + scale * 3.0,
        y + scale * 2.0,
        scale * 6.0,
        scale * 8.0,
        scale * 0.6,
        dark,
    );
}

fn draw_portal_icon(x: f32, y: f32, scale: f32) {
    let outer = color_u8!(110, 82, 196, 255);
    let inner = color_u8!(182, 148, 255, 255);
    draw_circle(x + scale * 6.0, y + scale * 6.0, scale * 5.0, outer);
    draw_circle(
        x + scale * 6.0,
        y + scale * 6.0,
        scale * 2.8,
        color_u8!(18, 12, 32, 255),
    );
    draw_circle_lines(
        x + scale * 6.0,
        y + scale * 6.0,
        scale * 5.0,
        scale * 1.0,
        inner,
    );
}

fn draw_dragon_piece_icon(x: f32, y: f32, scale: f32) {
    let outer = color_u8!(219, 186, 94, 255);
    let inner = color_u8!(255, 230, 153, 255);
    let dark = color_u8!(130, 96, 28, 255);
    draw_triangle(
        vec2(x + scale * 6.0, y),
        vec2(x, y + scale * 10.0),
        vec2(x + scale * 12.0, y + scale * 12.0),
        outer,
    );
    draw_triangle(
        vec2(x + scale * 5.5, y + scale * 2.0),
        vec2(x + scale * 2.0, y + scale * 9.0),
        vec2(x + scale * 10.0, y + scale * 10.0),
        inner,
    );
    draw_line(
        x + scale * 6.0,
        y,
        x + scale * 12.0,
        y + scale * 12.0,
        scale * 0.8,
        dark,
    );
}

fn draw_equipped_item_icon(sprites: &Sprites, item: EquippedItem, x: f32, y: f32) {
    match item {
        EquippedItem::None => {}
        EquippedItem::Sword => draw_sword_icon(x + px(4.0), y + px(1.0), px(0.8), LIGHTGRAY),
        EquippedItem::Bombs => {
            if !sprites.draw_hud_bomb(x + px(1.0), y + px(1.0), px(16.0)) {
                draw_rectangle(x + px(3.0), y + px(3.0), px(12.0), px(12.0), DARKGRAY);
            }
        }
        EquippedItem::Hammer => draw_hammer_icon(x + px(1.0), y + px(2.0), px(1.0)),
    }
}

fn draw_action_slot(
    sprites: &Sprites,
    player: &Player,
    x: f32,
    y: f32,
    title: &str,
    control: &str,
    item: EquippedItem,
) {
    let w = px(38.0);
    let h = px(32.0);
    let header_h = px(10.0);
    let has_item = item != EquippedItem::None;
    let border = if has_item {
        color_u8!(197, 170, 119, 255)
    } else {
        color_u8!(96, 104, 118, 255)
    };

    draw_rectangle(x, y, w, h, color_u8!(27, 31, 42, 255));
    draw_rectangle(x, y, w, header_h, color_u8!(48, 56, 73, 255));
    draw_rectangle_lines(x, y, w, h, px(1.0), border);

    let title_size = px(7.0);
    let title_dims = measure_text(title, None, title_size as u16, 1.0);
    draw_text(
        title,
        x + (w - title_dims.width) / 2.0,
        y + px(8.0),
        title_size,
        LIGHTGRAY,
    );

    if has_item {
        draw_equipped_item_icon(sprites, item, x + px(9.0), y + px(12.0));
    } else {
        let empty_size = px(14.0);
        let empty = "--";
        let empty_dims = measure_text(empty, None, empty_size as u16, 1.0);
        draw_text(
            empty,
            x + (w - empty_dims.width) / 2.0,
            y + px(26.0),
            empty_size,
            GRAY,
        );
    }

    if item == EquippedItem::Bombs && player.has_bombs {
        let count = format!("x{}", player.bomb_count);
        let count_size = px(8.0);
        let count_dims = measure_text(&count, None, count_size as u16, 1.0);
        draw_text(
            &count,
            x + w - count_dims.width - px(3.0),
            y + h - px(3.0),
            count_size,
            WHITE,
        );
    }

    let control_size = px(9.0);
    let control_dims = measure_text(control, None, control_size as u16, 1.0);
    draw_text(
        control,
        x + (w - control_dims.width) / 2.0,
        y + h + px(11.0),
        control_size,
        if has_item { WHITE } else { LIGHTGRAY },
    );
}

fn draw_hud(sprites: &Sprites, player: &Player, world: &WorldSnapshot) {
    draw_rectangle(0.0, 0.0, GAME_W, HUD_H, color_u8!(17, 17, 17, 255));
    draw_rectangle(
        0.0,
        HUD_H - px(2.0),
        GAME_W,
        px(2.0),
        color_u8!(100, 100, 100, 255),
    );
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
    draw_action_slot(
        sprites,
        player,
        px(16.0),
        px(10.0),
        "MAIN",
        "Z / SPC",
        player.main_item,
    );
    draw_action_slot(
        sprites,
        player,
        px(62.0),
        px(10.0),
        "SIDE",
        "X",
        player.side_item,
    );
    let gem_text = format!("x{}", player.gems);
    let gem_font_size = px(16.0);
    let gem_measure = measure_text(&gem_text, None, gem_font_size as u16, 1.0);
    let gem_scale = px(1.0);
    let gem_icon_w = gem_scale * 12.0;
    let gem_gap = px(4.0);
    let gem_text_x = GAME_W - px(12.0) - gem_measure.width;
    let gem_x = gem_text_x - gem_gap - gem_icon_w;
    let gem_y = px(30.0);
    draw_gem_icon(gem_x, gem_y, gem_scale, SKYBLUE);
    draw_text(&gem_text, gem_text_x, px(44.0), gem_font_size, SKYBLUE);
    if player.dragon_pieces > 0 {
        let x = px(160.0);
        draw_dragon_piece_icon(x, px(29.0), px(1.0));
        draw_text(
            &format!("x{}", player.dragon_pieces),
            x + px(18.0),
            px(44.0),
            px(16.0),
            color_u8!(226, 194, 92, 255),
        );
    }
    if player.has_ladder {
        draw_ladder_icon(px(160.0), px(6.0), px(1.0), color_u8!(186, 145, 96, 255));
        draw_text(
            "LADDER",
            px(176.0),
            px(20.0),
            px(16.0),
            color_u8!(186, 145, 96, 255),
        );
    }
    if player.keys > 0 || world.in_dungeon {
        if !sprites.draw_hud_key(px(106.0), px(6.0), px(16.0)) {
            draw_rectangle(px(108.0), px(8.0), px(12.0), px(12.0), YELLOW);
        }
        draw_text(
            &format!("x{}", player.keys),
            px(126.0),
            px(20.0),
            px(16.0),
            YELLOW,
        );
    }
    if player.has_boss_key {
        let boss_key_color = color_u8!(220, 40, 40, 255);
        if !sprites.draw_hud_boss_key(px(106.0), px(28.0), px(24.0), boss_key_color) {
            draw_rectangle(px(108.0), px(30.0), px(18.0), px(18.0), boss_key_color);
        }
        draw_text("BOSS", px(134.0), px(44.0), px(16.0), boss_key_color);
    }
    // location label stays clear of the item group on the left.
    let location = world_data::location_name(
        world.screen_x,
        world.screen_y,
        world.in_dungeon,
        world.dungeon_id,
        world.in_interior,
        &world.interior_id,
    );
    draw_text(&location, px(178.0), px(20.0), px(14.0), LIGHTGRAY);
}

fn draw_overworld_map_panel(
    world: &WorldSnapshot,
    area_x: f32,
    area_y: f32,
    area_w: f32,
    area_h: f32,
    frame: i32,
) {
    let gap_x = px(MAP_TILE_GAP);
    let gap_y = px(MAP_TILE_GAP);
    let cell_w = px(MAP_TILE_SIZE);
    let cell_h = px(MAP_TILE_SIZE);
    let minimap_w = WORLD_W as f32 * cell_w + (WORLD_W as f32 - 1.0) * gap_x;
    let minimap_h = WORLD_H as f32 * cell_h + (WORLD_H as f32 - 1.0) * gap_y;
    let base_x = area_x + (area_w - minimap_w) / 2.0;
    let base_y = area_y + (area_h - minimap_h) / 2.0;
    for sy in 0..WORLD_H {
        for sx in 0..WORLD_W {
            let key = format!("{sx},{sy}");
            let x = base_x + sx as f32 * (cell_w + gap_x);
            let y = base_y + sy as f32 * (cell_h + gap_y);
            let current = world.screen_x == sx && world.screen_y == sy;
            let visited = world.visited.contains(&key);
            draw_rectangle(
                x,
                y,
                cell_w,
                cell_h,
                if current {
                    // Neon yellow indicates the player's current screen (pulsing)
                    let pulse = ((frame as f32 * 0.12).sin() * 0.5 + 0.5) as f32;
                    let glow = (200.0 + pulse * 55.0) as u8;
                    color_u8!(glow, glow, 64, 255)
                } else if visited {
                    biome_color(world_data::location_name(sx, sy, false, 0, false, "").as_str())
                } else {
                    color_u8!(34, 34, 34, 255)
                },
            );
            // mark caves and dungeon entrances when running in dev mode
            if world.dev_mode {
                if let Some(kind) = world_data::cave_kind(sx, sy) {
                    let cx = x + cell_w / 2.0;
                    let cy = y + cell_h / 2.0;
                    draw_circle(cx, cy, cell_w * 0.3, BLACK);
                    let label = match kind {
                        world_data::CaveKind::Sword => "S",
                        world_data::CaveKind::Heart => "H",
                        world_data::CaveKind::Shrine => "I",
                        world_data::CaveKind::Sanctum => "E",
                        world_data::CaveKind::Bombs => "B",
                        world_data::CaveKind::Shop => "$",
                        world_data::CaveKind::AncientKey => "K",
                        world_data::CaveKind::TideChart => "T",
                        world_data::CaveKind::EmberCrystal => "E",
                        world_data::CaveKind::VoidCompass => "V",
                        world_data::CaveKind::StarSigil => "*",
                        world_data::CaveKind::DragonCodex => "D",
                        world_data::CaveKind::CrystalOfSeeing => "C",
                    };
                    draw_text(label, cx - px(4.0), cy + px(4.0), px(10.0), WHITE);
                }
                let did = world_data::dungeon_at(sx, sy);
                if did != 0 {
                    let cx = x + cell_w - px(4.0);
                    let cy = y + px(4.0);
                    draw_rectangle(cx - px(3.0), cy - px(3.0), px(6.0), px(6.0), RED);
                }
            }
        }
    }
}

fn draw_dungeon_map_panel(
    world: &WorldSnapshot,
    area_x: f32,
    area_y: f32,
    area_w: f32,
    area_h: f32,
    frame: i32,
) {
    const MAP_CELL: f32 = 18.0;
    const MAP_GAP: f32 = 2.0;
    let gap_x = px(MAP_GAP);
    let gap_y = px(MAP_GAP);
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
    let cols = (max_x - min_x + 1).max(1) as f32;
    let rows = (max_y - min_y + 1).max(1) as f32;

    // Use a fixed cell scale so the dungeon map size and zoom doesn't jump
    // depending on the current dungeon layout.
    let cell_w = px(MAP_TILE_SIZE);
    let cell_h = px(MAP_TILE_SIZE);

    let minimap_w = cols * cell_w + (cols - 1.0) * gap_x;
    let minimap_h = rows * cell_h + (rows - 1.0) * gap_y;
    let base_x = area_x + (area_w - minimap_w) / 2.0;
    let base_y = area_y + (area_h - minimap_h) / 2.0;
    for key in &world.dungeon_rooms {
        if let Some((sx, sy)) = parse_key(key) {
            let rx = (sx - min_x) as f32;
            let ry = (sy - min_y) as f32;
            let x = base_x + rx * (cell_w + gap_x);
            let y = base_y + ry * (cell_h + gap_y);
            let current = world.screen_x == sx && world.screen_y == sy;
            let visit_key = format!("d{}:{key}", world.dungeon_id);
            let visited = world.visited.contains(&visit_key);
            draw_rectangle(
                x,
                y,
                cell_w,
                cell_h,
                if current {
                    // Neon yellow indicates the player's current room (pulsing)
                    let pulse = ((frame as f32 * 0.12).sin() * 0.5 + 0.5) as f32;
                    let glow = (200.0 + pulse * 55.0) as u8;
                    color_u8!(glow, glow, 64, 255)
                } else if visited {
                    color_u8!(68, 68, 102, 255)
                } else {
                    color_u8!(34, 34, 34, 255)
                },
            );
            draw_rectangle_lines(x, y, cell_w, cell_h, px(1.0), color_u8!(123, 132, 147, 255));
        }
    }
}

fn draw_inventory_stat(
    sprites: &Sprites,
    player: &Player,
    x: f32,
    y: f32,
    w: f32,
    label: &str,
    active: bool,
    count: Option<i32>,
) {
    let color = if active { WHITE } else { GRAY };
    match label {
        "Sword" if player.has_sword => {
            draw_sword_icon(x + px(2.0), y - px(13.0), px(1.1), LIGHTGRAY);
        }
        "Gems" => {
            draw_gem_icon(x + px(2.0), y - px(13.0), px(0.9), SKYBLUE);
        }
        "Bombs" if player.has_bombs => {
            let _ = sprites.draw_hud_bomb(x, y - px(10.0), px(22.0));
        }
        "Hammer" if player.has_hammer => {
            draw_hammer_icon(x + px(4.0), y - px(11.0), px(0.9));
        }
        "Dragon Pieces" => {
            draw_dragon_piece_icon(x + px(3.0), y - px(11.0), px(0.85));
        }
        "Keys" if player.keys > 0 => {
            let _ = sprites.draw_hud_key(x + px(2.0), y - px(8.0), px(18.0));
        }
        "Boss Key" if player.has_boss_key => {
            let _ =
                sprites.draw_hud_boss_key(x, y - px(10.0), px(22.0), color_u8!(220, 40, 40, 255));
        }
        "Ladder" if player.has_ladder => {
            draw_ladder_icon(
                x + px(4.0),
                y - px(11.0),
                px(0.9),
                color_u8!(181, 141, 91, 255),
            );
        }
        _ => {}
    }
    let font_size = px(14.0);
    let value = match count {
        Some(amount) => format!("x{amount}"),
        None => {
            if active {
                "YES".to_string()
            } else {
                "NO".to_string()
            }
        }
    };
    let value_width = measure_text(&value, None, font_size as u16, 1.0).width;
    let value_x = x + w - px(4.0) - value_width;
    let label_max_width = (value_x - (x + px(28.0)) - px(10.0)).max(px(40.0));
    let fitted_label = fit_text_to_width(label, label_max_width, font_size);
    draw_text(&fitted_label, x + px(28.0), y, font_size, color);
    draw_text(&value, value_x, y, font_size, color);
}

fn fit_text_to_width(text: &str, max_width: f32, font_size: f32) -> String {
    if measure_text(text, None, font_size as u16, 1.0).width <= max_width {
        return text.to_string();
    }

    let ellipsis = "...";
    let ellipsis_width = measure_text(ellipsis, None, font_size as u16, 1.0).width;
    if ellipsis_width >= max_width {
        return ellipsis.to_string();
    }

    let mut fitted = String::new();
    for ch in text.chars() {
        let mut candidate = fitted.clone();
        candidate.push(ch);
        candidate.push_str(ellipsis);
        if measure_text(&candidate, None, font_size as u16, 1.0).width > max_width {
            break;
        }
        fitted.push(ch);
    }
    fitted.push_str(ellipsis);
    fitted
}

fn biome_color(name: &str) -> Color {
    if name.contains("FROST") {
        color_u8!(170, 190, 210, 255)
    } else if name.contains("COAST") || name.contains("LAKE") || name.contains("ISLAND") {
        color_u8!(66, 110, 172, 255)
    } else if name.contains("RUIN") || name.contains("CANYON") {
        color_u8!(129, 95, 78, 255)
    } else if name.contains("DUST") || name.contains("SALT") {
        color_u8!(194, 166, 101, 255)
    } else if name.contains("WOOD") || name.contains("FOREST") {
        color_u8!(58, 122, 70, 255)
    } else if name.contains("BADLAND") || name.contains("RIDGE") || name.contains("CLIFF") {
        color_u8!(112, 112, 112, 255)
    } else {
        color_u8!(88, 150, 80, 255)
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
