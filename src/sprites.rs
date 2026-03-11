use crate::character::{generate_hero_sheets, CharacterAppearance};
use crate::constants::{PIXEL_SCALE, TILE};
use crate::model::{Dir, Enemy, EnemyType, PickupType, Player, PlayerState, Projectile, TileType};
use macroquad::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Sprites {
    hero_idle: Option<Sheet>,
    hero_walk: Option<Sheet>,
    hero_idle_sword: Option<Sheet>,
    hero_walk_sword: Option<Sheet>,
    enemies: Option<Sheet>,
    tiles: Option<Sheet>,
    items: Option<Sheet>,
    biome_sheets: HashMap<i32, BiomeSpriteSet>,
    title_dragon: Option<Texture2D>,
    layout: SpriteLayout,
}

impl Sprites {
    pub async fn load(appearance: &CharacterAppearance) -> Self {
        let layout = load_layout();
        let generated = generate_hero_sheets(appearance);
        let hero_idle = Some(sheet_from_image(&generated.idle));
        let hero_walk = Some(sheet_from_image(&generated.walk));
        let hero_idle_sword = Some(sheet_from_image(&generated.idle_armed));
        let hero_walk_sword = Some(sheet_from_image(&generated.walk_armed));
        let enemies = load_sheet(&layout.enemies.sheet).await;
        let tiles = load_sheet(&layout.tiles.sheet).await;
        let items = load_sheet(&layout.items.sheet).await;
        let biome_sheets = load_biome_sprite_sets().await;
        let title_dragon = load_image_texture(&concept_asset_path("dragon.png")).await;
        Self {
            hero_idle,
            hero_walk,
            hero_idle_sword,
            hero_walk_sword,
            enemies,
            tiles,
            items,
            biome_sheets,
            title_dragon,
            layout,
        }
    }

    pub fn set_hero_appearance(&mut self, appearance: &CharacterAppearance) {
        let generated = generate_hero_sheets(appearance);
        self.hero_idle = Some(sheet_from_image(&generated.idle));
        self.hero_walk = Some(sheet_from_image(&generated.walk));
        self.hero_idle_sword = Some(sheet_from_image(&generated.idle_armed));
        self.hero_walk_sword = Some(sheet_from_image(&generated.walk_armed));
    }

    pub fn draw_player(&self, player: &Player, x: f32, y: f32) -> bool {
        let (sheet, frames) = match player.state {
            PlayerState::Walking => {
                let sheet = if player.has_sword {
                    self.hero_walk_sword.as_ref().or(self.hero_walk.as_ref())
                } else {
                    self.hero_walk.as_ref()
                };
                let Some(sheet) = sheet else {
                    return false;
                };
                (sheet, self.hero_frames(player.dir))
            }
            _ => {
                let sheet = if player.has_sword {
                    self.hero_idle_sword.as_ref().or(self.hero_idle.as_ref())
                } else {
                    self.hero_idle.as_ref()
                };
                let Some(sheet) = sheet else {
                    return false;
                };
                (sheet, self.hero_frames(player.dir))
            }
        };
        if frames.is_empty() {
            return false;
        }
        let frame_index = if player.state == PlayerState::Walking {
            (player.walk_frame as usize) % frames.len()
        } else {
            0
        };
        draw_centered_frame(
            sheet,
            &frames[frame_index],
            x,
            y,
            self.layout.hero.dest_scale.unwrap_or(PIXEL_SCALE),
            self.layout.hero.base_w.unwrap_or(frames[frame_index].w),
            self.layout.hero.base_h.unwrap_or(frames[frame_index].h),
        );
        true
    }

    fn hero_frames(&self, dir: Dir) -> &[FrameRect] {
        match dir {
            Dir::Down => &self.layout.hero.down,
            Dir::Up => &self.layout.hero.up,
            Dir::Left => &self.layout.hero.left,
            Dir::Right => &self.layout.hero.right,
        }
    }

    pub fn draw_enemy(&self, biome_id: Option<i32>, enemy: &Enemy, x: f32, y: f32) -> bool {
        let themed = biome_id
            .and_then(|id| self.biome_sheets.get(&id))
            .and_then(|set| {
                let sheet = set.enemies.as_ref()?;
                let anim = themed_enemy_animation(biome_id, enemy.enemy_type)?;
                Some((sheet, anim))
            });
        let (sheet, anim) = if let Some((sheet, anim)) = themed {
            (sheet, anim)
        } else {
            let Some(sheet) = &self.enemies else {
                return false;
            };
            let anim = match enemy.enemy_type {
                EnemyType::Slime => self.layout.enemies.slime.clone(),
                EnemyType::Octorok => self.layout.enemies.octorok.clone(),
                EnemyType::Bat => self.layout.enemies.bat.clone(),
                EnemyType::Darknut => self.layout.enemies.darknut.clone(),
                EnemyType::Boss => self.layout.enemies.boss.clone(),
            };
            (sheet, anim)
        };
        let frame_index = if anim.frames.len() > 1 {
            ((enemy.timer / anim.frame_time.max(1)) as usize) % anim.frames.len()
        } else {
            0
        };
        draw_centered_frame(
            sheet,
            &anim.frames[frame_index],
            x,
            y,
            anim.dest_scale.unwrap_or(PIXEL_SCALE),
            anim.frames[frame_index].w,
            anim.frames[frame_index].h,
        );
        true
    }

    pub fn draw_tile(&self, biome_id: Option<i32>, tile: TileType, x: f32, y: f32, color: Color) -> bool {
        let themed = biome_id
            .and_then(|id| self.biome_sheets.get(&id))
            .and_then(|set| {
                let sheet = set.tiles.as_ref()?;
                let frame = themed_tile_frame(biome_id, tile)?;
                Some((sheet, frame))
            });
        let (sheet, frame) = if let Some((sheet, frame)) = themed {
            (sheet, frame)
        } else {
            let Some(sheet) = &self.tiles else {
                return false;
            };
            (sheet, base_tile_frame(&self.layout, tile))
        };
        draw_frame_to_size(sheet, frame, x, y, TILE, TILE, color);
        true
    }

    pub fn draw_pickup(
        &self,
        biome_id: Option<i32>,
        pickup: PickupType,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> bool {
        let themed = biome_id
            .and_then(|id| self.biome_sheets.get(&id))
            .and_then(|set| {
                let sheet = set.items.as_ref()?;
                let frame = themed_pickup_frame(biome_id, pickup)?;
                Some((sheet, frame))
            });
        let (sheet, frame) = if let Some((sheet, frame)) = themed {
            (sheet, frame)
        } else {
            let Some(sheet) = &self.items else {
                return false;
            };
            let Some(frame) = base_pickup_frame(&self.layout, pickup) else {
                return false;
            };
            (sheet, frame)
        };
        draw_frame_to_size(sheet, frame, x, y, w, h, WHITE);
        true
    }

    pub fn draw_bomb(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let Some(sheet) = &self.items else {
            return false;
        };
        draw_frame_to_size(sheet, &self.layout.items.bomb, x, y, w, h, WHITE);
        true
    }

    pub fn draw_projectile(&self, projectile: &Projectile, x: f32, y: f32) -> bool {
        let Some(sheet) = &self.items else {
            return false;
        };
        let frame = if projectile.from_enemy {
            &self.layout.items.enemy_projectile
        } else {
            &self.layout.items.player_projectile
        };
        draw_frame_to_size(sheet, frame, x, y, projectile.w, projectile.h, WHITE);
        true
    }

    pub fn draw_hud_heart(&self, state: HeartState, x: f32, y: f32, size: f32) -> bool {
        let Some(sheet) = &self.items else {
            return false;
        };
        let frame = match state {
            HeartState::Full => &self.layout.items.hud_heart_full,
            HeartState::Half => &self.layout.items.hud_heart_half,
            HeartState::Empty => &self.layout.items.hud_heart_empty,
        };
        draw_frame_to_size(sheet, frame, x, y, size, size, WHITE);
        true
    }

    pub fn draw_hud_bomb(&self, x: f32, y: f32, size: f32) -> bool {
        let Some(sheet) = &self.items else {
            return false;
        };
        draw_frame_to_size(sheet, &self.layout.items.hud_bomb, x, y, size, size, WHITE);
        true
    }

    pub fn draw_hud_key(&self, x: f32, y: f32, size: f32) -> bool {
        let Some(sheet) = &self.items else {
            return false;
        };
        draw_frame_to_size(sheet, &self.layout.items.hud_key, x, y, size, size, WHITE);
        true
    }

    pub fn draw_hud_boss_key(&self, x: f32, y: f32, size: f32, color: Color) -> bool {
        let Some(sheet) = &self.items else {
            return false;
        };
        draw_frame_to_size(sheet, &self.layout.items.hud_boss_key, x, y, size, size, color);
        true
    }

    pub fn draw_title_dragon(&self, x: f32, y: f32, max_w: f32, max_h: f32) -> bool {
        let Some(texture) = &self.title_dragon else {
            return false;
        };
        let source_w = texture.width();
        let source_h = texture.height();
        if source_w <= 0.0 || source_h <= 0.0 {
            return false;
        }

        let scale = (max_w / source_w).min(max_h / source_h);
        let draw_w = source_w * scale;
        let draw_h = source_h * scale;
        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                ..Default::default()
            },
        );
        true
    }
}

#[derive(Clone, Copy)]
pub enum HeartState {
    Full,
    Half,
    Empty,
}

struct Sheet {
    texture: Texture2D,
}

#[derive(Clone, Deserialize)]
struct SpriteLayout {
    hero: HeroLayout,
    enemies: EnemyLayout,
    tiles: TileLayout,
    items: ItemLayout,
}

#[derive(Clone, Deserialize)]
struct HeroLayout {
    dest_scale: Option<f32>,
    base_w: Option<f32>,
    base_h: Option<f32>,
    down: Vec<FrameRect>,
    up: Vec<FrameRect>,
    left: Vec<FrameRect>,
    right: Vec<FrameRect>,
}

#[derive(Clone, Deserialize)]
struct EnemyLayout {
    sheet: String,
    slime: AnimationLayout,
    octorok: AnimationLayout,
    bat: AnimationLayout,
    darknut: AnimationLayout,
    boss: AnimationLayout,
}

#[derive(Clone, Deserialize)]
struct TileLayout {
    sheet: String,
    grass: FrameRect,
    tree: FrameRect,
    water: FrameRect,
    rock: FrameRect,
    sand: FrameRect,
    path: FrameRect,
    cave: FrameRect,
    dungeon: FrameRect,
    cracked: FrameRect,
    bush: FrameRect,
    bridge: FrameRect,
    wall: FrameRect,
    floor: FrameRect,
    door_locked: FrameRect,
    door: FrameRect,
    stairs: FrameRect,
    chest: FrameRect,
    goal: FrameRect,
    boss_door: FrameRect,
    floor_alt: FrameRect,
}

#[derive(Clone, Deserialize)]
struct ItemLayout {
    sheet: String,
    heart_pickup: FrameRect,
    heart_container: FrameRect,
    key: FrameRect,
    boss_key: FrameRect,
    bomb_ammo: FrameRect,
    bombs: FrameRect,
    bomb: FrameRect,
    enemy_projectile: FrameRect,
    player_projectile: FrameRect,
    hud_heart_full: FrameRect,
    hud_heart_half: FrameRect,
    hud_heart_empty: FrameRect,
    hud_bomb: FrameRect,
    hud_key: FrameRect,
    hud_boss_key: FrameRect,
}

struct BiomeSpriteSet {
    tiles: Option<Sheet>,
    enemies: Option<Sheet>,
    items: Option<Sheet>,
}

#[derive(Clone, Deserialize)]
struct AnimationLayout {
    frame_time: i32,
    dest_scale: Option<f32>,
    frames: Vec<FrameRect>,
}

#[derive(Clone, Deserialize)]
struct FrameRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn frame(x: f32, y: f32, w: f32, h: f32) -> FrameRect {
    FrameRect { x, y, w, h }
}

fn anim(frame_time: i32, frames: &[(f32, f32, f32, f32)]) -> AnimationLayout {
    AnimationLayout {
        frame_time,
        dest_scale: Some(3.0),
        frames: frames.iter().map(|(x, y, w, h)| frame(*x, *y, *w, *h)).collect(),
    }
}

fn base_tile_frame(layout: &SpriteLayout, tile: TileType) -> &FrameRect {
    match tile {
        TileType::Grass => &layout.tiles.grass,
        TileType::Tree => &layout.tiles.tree,
        TileType::Water => &layout.tiles.water,
        TileType::Rock => &layout.tiles.rock,
        TileType::Sand => &layout.tiles.sand,
        TileType::Path => &layout.tiles.path,
        TileType::Cave => &layout.tiles.cave,
        TileType::Dungeon => &layout.tiles.dungeon,
        TileType::Cracked => &layout.tiles.cracked,
        TileType::Bush => &layout.tiles.bush,
        TileType::Bridge => &layout.tiles.bridge,
        TileType::Wall => &layout.tiles.wall,
        TileType::Floor => &layout.tiles.floor,
        TileType::DoorLocked => &layout.tiles.door_locked,
        TileType::Door => &layout.tiles.door,
        TileType::Stairs => &layout.tiles.stairs,
        TileType::Chest => &layout.tiles.chest,
        TileType::Goal => &layout.tiles.goal,
        TileType::BossDoor => &layout.tiles.boss_door,
        TileType::FloorAlt => &layout.tiles.floor_alt,
    }
}

fn base_pickup_frame(layout: &SpriteLayout, pickup: PickupType) -> Option<&FrameRect> {
    Some(match pickup {
        PickupType::Heart => &layout.items.heart_pickup,
        PickupType::HeartContainer => &layout.items.heart_container,
        PickupType::Key => &layout.items.key,
        PickupType::BossKey => &layout.items.boss_key,
        PickupType::BombAmmo => &layout.items.bomb_ammo,
        PickupType::Bombs => &layout.items.bombs,
        PickupType::Gem | PickupType::Ladder | PickupType::Hammer | PickupType::DragonPiece => return None,
    })
}

fn themed_tile_frame(biome_id: Option<i32>, tile: TileType) -> Option<&'static FrameRect> {
    if biome_id == Some(2) {
        Some(match tile {
            TileType::Grass => &frame_const::ASH_PLAIN,
            TileType::Tree | TileType::Wall => &frame_const::RUIN_WALL,
            TileType::Water => &frame_const::DEEP_ASH,
            TileType::Rock | TileType::Sand | TileType::Bush => &frame_const::ASH_RUBBLE,
            TileType::Path | TileType::FloorAlt => &frame_const::ASH_DRIFT,
            TileType::Cave | TileType::Dungeon | TileType::Door | TileType::DoorLocked | TileType::BossDoor => {
                &frame_const::RUIN_DOORWAY
            }
            TileType::Cracked => &frame_const::CRACKED_FLAGSTONE,
            TileType::Bridge | TileType::Stairs => &frame_const::BELLTOWER,
            TileType::Floor | TileType::Goal | TileType::Chest => &frame_const::RUIN_FLOOR,
        })
    } else {
        None
    }
}

fn themed_enemy_animation(biome_id: Option<i32>, enemy_type: EnemyType) -> Option<AnimationLayout> {
    if biome_id == Some(2) {
        Some(match enemy_type {
            EnemyType::Slime => anim(16, &[(0.0, 0.0, 16.0, 16.0), (16.0, 0.0, 16.0, 16.0)]),
            EnemyType::Octorok => anim(10, &[(0.0, 32.0, 16.0, 16.0), (16.0, 32.0, 16.0, 16.0)]),
            EnemyType::Bat => anim(12, &[(0.0, 48.0, 16.0, 16.0), (16.0, 48.0, 16.0, 16.0)]),
            EnemyType::Darknut => anim(22, &[(0.0, 16.0, 16.0, 16.0), (16.0, 16.0, 16.0, 16.0)]),
            EnemyType::Boss => anim(22, &[(0.0, 16.0, 16.0, 16.0), (16.0, 16.0, 16.0, 16.0)]),
        })
    } else {
        None
    }
}

fn themed_pickup_frame(biome_id: Option<i32>, pickup: PickupType) -> Option<&'static FrameRect> {
    if biome_id == Some(2) {
        match pickup {
            PickupType::Gem => Some(&frame_const::ASH_GOLD_POUCH),
            PickupType::Key => Some(&frame_const::SOLDIER_BADGE),
            PickupType::Bombs | PickupType::BombAmmo => Some(&frame_const::SALVAGE_TOKEN),
            _ => None,
        }
    } else {
        None
    }
}

mod frame_const {
    use super::FrameRect;

    pub static ASH_PLAIN: FrameRect = FrameRect { x: 0.0, y: 0.0, w: 16.0, h: 16.0 };
    pub static RUIN_WALL: FrameRect = FrameRect { x: 16.0, y: 0.0, w: 16.0, h: 16.0 };
    pub static DEEP_ASH: FrameRect = FrameRect { x: 32.0, y: 0.0, w: 16.0, h: 16.0 };
    pub static BELLTOWER: FrameRect = FrameRect { x: 48.0, y: 0.0, w: 16.0, h: 16.0 };
    pub static RUIN_FLOOR: FrameRect = FrameRect { x: 80.0, y: 0.0, w: 16.0, h: 16.0 };
    pub static ASH_RUBBLE: FrameRect = FrameRect { x: 96.0, y: 0.0, w: 16.0, h: 16.0 };
    pub static RUIN_DOORWAY: FrameRect = FrameRect { x: 112.0, y: 0.0, w: 16.0, h: 16.0 };
    pub static CRACKED_FLAGSTONE: FrameRect = FrameRect { x: 128.0, y: 0.0, w: 16.0, h: 16.0 };
    pub static ASH_DRIFT: FrameRect = FrameRect { x: 0.0, y: 16.0, w: 16.0, h: 16.0 };
    pub static SALVAGE_TOKEN: FrameRect = FrameRect { x: 0.0, y: 0.0, w: 16.0, h: 16.0 };
    pub static SOLDIER_BADGE: FrameRect = FrameRect { x: 0.0, y: 16.0, w: 16.0, h: 16.0 };
    pub static ASH_GOLD_POUCH: FrameRect = FrameRect { x: 32.0, y: 16.0, w: 16.0, h: 16.0 };
}

fn draw_centered_frame(
    sheet: &Sheet,
    frame: &FrameRect,
    x: f32,
    y: f32,
    scale: f32,
    base_frame_w: f32,
    base_frame_h: f32,
) {
    let width = frame.w * scale;
    let height = frame.h * scale;
    let base_width = base_frame_w * PIXEL_SCALE;
    let base_height = base_frame_h * PIXEL_SCALE;
    let draw_x = x - (width - base_width) / 2.0;
    let draw_y = y - (height - base_height);
    draw_frame_to_size(sheet, frame, draw_x, draw_y, width, height, WHITE);
}

fn draw_frame_to_size(sheet: &Sheet, frame: &FrameRect, x: f32, y: f32, w: f32, h: f32, color: Color) {
    draw_texture_ex(
        &sheet.texture,
        x,
        y,
        color,
        DrawTextureParams {
            source: Some(Rect::new(frame.x, frame.y, frame.w, frame.h)),
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        },
    );
}

fn load_layout() -> SpriteLayout {
    let path = sprite_asset_path("layout.toml");
    fs::read_to_string(&path)
        .ok()
        .and_then(|text| toml::from_str(&text).ok())
        .unwrap_or_else(default_layout)
}

async fn load_sheet(name: &str) -> Option<Sheet> {
    let path = sprite_asset_path(name);
    let texture = load_texture(path.to_string_lossy().as_ref()).await.ok()?;
    texture.set_filter(FilterMode::Nearest);
    Some(Sheet { texture })
}

async fn load_biome_sprite_sets() -> HashMap<i32, BiomeSpriteSet> {
    let mut sets = HashMap::new();
    for biome_id in 1..=8 {
        let tiles = load_sheet(&format!("biome/biome{biome_id}/tiles.png")).await;
        let enemies = load_sheet(&format!("biome/biome{biome_id}/enemies.png")).await;
        let items = load_sheet(&format!("biome/biome{biome_id}/items.png")).await;
        if tiles.is_some() || enemies.is_some() || items.is_some() {
            sets.insert(
                biome_id,
                BiomeSpriteSet {
                    tiles,
                    enemies,
                    items,
                },
            );
        }
    }
    sets
}

async fn load_image_texture(path: &Path) -> Option<Texture2D> {
    let texture = load_texture(path.to_string_lossy().as_ref()).await.ok()?;
    texture.set_filter(FilterMode::Nearest);
    Some(texture)
}

fn sprite_asset_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("sprites")
        .join(name)
}

fn concept_asset_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("concept")
        .join("art")
        .join(name)
}

fn sheet_from_image(image: &Image) -> Sheet {
    let texture = Texture2D::from_image(image);
    texture.set_filter(FilterMode::Nearest);
    Sheet { texture }
}

fn default_layout() -> SpriteLayout {
    SpriteLayout {
        hero: HeroLayout {
            dest_scale: Some(2.0),
            base_w: Some(16.0),
            base_h: Some(16.0),
            down: vec![
                frame(0.0, 0.0, 48.0, 48.0),
                frame(48.0, 0.0, 48.0, 48.0),
                frame(96.0, 0.0, 48.0, 48.0),
                frame(144.0, 0.0, 48.0, 48.0),
            ],
            left: vec![
                frame(0.0, 48.0, 48.0, 48.0),
                frame(48.0, 48.0, 48.0, 48.0),
                frame(96.0, 48.0, 48.0, 48.0),
                frame(144.0, 48.0, 48.0, 48.0),
            ],
            right: vec![
                frame(0.0, 96.0, 48.0, 48.0),
                frame(48.0, 96.0, 48.0, 48.0),
                frame(96.0, 96.0, 48.0, 48.0),
                frame(144.0, 96.0, 48.0, 48.0),
            ],
            up: vec![
                frame(0.0, 144.0, 48.0, 48.0),
                frame(48.0, 144.0, 48.0, 48.0),
                frame(96.0, 144.0, 48.0, 48.0),
                frame(144.0, 144.0, 48.0, 48.0),
            ],
        },
        enemies: EnemyLayout {
            sheet: "enemies.png".to_string(),
            slime: AnimationLayout {
                frame_time: 18,
                dest_scale: Some(3.0),
                frames: vec![frame(0.0, 0.0, 16.0, 16.0), frame(16.0, 0.0, 16.0, 16.0)],
            },
            octorok: AnimationLayout {
                frame_time: 16,
                dest_scale: Some(3.0),
                frames: vec![frame(0.0, 16.0, 16.0, 16.0), frame(16.0, 16.0, 16.0, 16.0)],
            },
            bat: AnimationLayout {
                frame_time: 8,
                dest_scale: Some(3.0),
                frames: vec![frame(0.0, 32.0, 16.0, 16.0), frame(16.0, 32.0, 16.0, 16.0)],
            },
            darknut: AnimationLayout {
                frame_time: 16,
                dest_scale: Some(3.0),
                frames: vec![frame(0.0, 48.0, 16.0, 16.0), frame(16.0, 48.0, 16.0, 16.0)],
            },
            boss: AnimationLayout {
                frame_time: 18,
                dest_scale: Some(3.0),
                frames: vec![frame(0.0, 64.0, 24.0, 24.0), frame(24.0, 64.0, 24.0, 24.0)],
            },
        },
        tiles: TileLayout {
            sheet: "tiles.png".to_string(),
            grass: frame(0.0, 0.0, 16.0, 16.0),
            tree: frame(16.0, 0.0, 16.0, 16.0),
            water: frame(32.0, 0.0, 16.0, 16.0),
            rock: frame(48.0, 0.0, 16.0, 16.0),
            sand: frame(64.0, 0.0, 16.0, 16.0),
            path: frame(0.0, 16.0, 16.0, 16.0),
            cave: frame(16.0, 16.0, 16.0, 16.0),
            dungeon: frame(32.0, 16.0, 16.0, 16.0),
            cracked: frame(48.0, 16.0, 16.0, 16.0),
            bush: frame(64.0, 16.0, 16.0, 16.0),
            bridge: frame(0.0, 32.0, 16.0, 16.0),
            wall: frame(16.0, 32.0, 16.0, 16.0),
            floor: frame(32.0, 32.0, 16.0, 16.0),
            door_locked: frame(48.0, 32.0, 16.0, 16.0),
            door: frame(64.0, 32.0, 16.0, 16.0),
            stairs: frame(0.0, 48.0, 16.0, 16.0),
            chest: frame(16.0, 48.0, 16.0, 16.0),
            goal: frame(32.0, 48.0, 16.0, 16.0),
            boss_door: frame(48.0, 48.0, 16.0, 16.0),
            floor_alt: frame(64.0, 48.0, 16.0, 16.0),
        },
        items: ItemLayout {
            sheet: "items.png".to_string(),
            heart_pickup: frame(0.0, 0.0, 16.0, 16.0),
            heart_container: frame(16.0, 0.0, 16.0, 16.0),
            key: frame(32.0, 0.0, 16.0, 16.0),
            boss_key: frame(48.0, 0.0, 16.0, 16.0),
            bomb_ammo: frame(0.0, 16.0, 16.0, 16.0),
            bombs: frame(16.0, 16.0, 16.0, 16.0),
            bomb: frame(32.0, 16.0, 16.0, 16.0),
            enemy_projectile: frame(48.0, 16.0, 16.0, 16.0),
            player_projectile: frame(0.0, 32.0, 16.0, 16.0),
            hud_heart_full: frame(16.0, 32.0, 16.0, 16.0),
            hud_heart_half: frame(32.0, 32.0, 16.0, 16.0),
            hud_heart_empty: frame(48.0, 32.0, 16.0, 16.0),
            hud_bomb: frame(0.0, 48.0, 16.0, 16.0),
            hud_key: frame(16.0, 48.0, 16.0, 16.0),
            hud_boss_key: frame(32.0, 48.0, 16.0, 16.0),
        },
    }
}
