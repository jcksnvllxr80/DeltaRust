use macroquad::prelude::{Color, Image, BLACK, WHITE};
use macroquad::rand::gen_range;

pub const FRAME_SIZE: u16 = 48;
pub const SHEET_SIZE: u16 = FRAME_SIZE * 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimMode {
    Idle,
    Walk,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HairStyle {
    Short,
    Long,
    Spiky,
    Bald,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildType {
    Normal,
    Stocky,
    Slim,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClothesStyle {
    Tunic,
    Robe,
    Vest,
    Bare,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelmetStyle {
    None,
    Iron,
    Hood,
    Crown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmorStyle {
    None,
    Leather,
    Chainmail,
    Chestplate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShieldStyle {
    Round,
    Kite,
    Tower,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponStyle {
    Sword,
    Axe,
    Spear,
    Staff,
}

#[derive(Clone, Copy, Debug)]
pub struct HeroPalette {
    pub skin: Color,
    pub hair: Color,
    pub shirt: Color,
    pub pants: Color,
    pub boots: Color,
    pub armor: Color,
    pub cape: Color,
    pub weapon: Color,
}

#[derive(Clone, Debug)]
pub struct CharacterAppearance {
    pub palette: HeroPalette,
    pub hair_style: HairStyle,
    pub build: BuildType,
    pub clothes: ClothesStyle,
    pub helmet: HelmetStyle,
    pub armor: ArmorStyle,
    pub has_cape: bool,
    pub shield_on: bool,
    pub shield_style: ShieldStyle,
    pub weapon: WeaponStyle,
}

impl Default for CharacterAppearance {
    fn default() -> Self {
        Self {
            palette: HeroPalette {
                skin: rgb(255, 204, 153),
                hair: rgb(42, 21, 0),
                shirt: rgb(34, 102, 170),
                pants: rgb(85, 51, 17),
                boots: rgb(42, 26, 10),
                armor: rgb(136, 153, 170),
                cape: rgb(170, 34, 34),
                weapon: rgb(176, 200, 216),
            },
            hair_style: HairStyle::Short,
            build: BuildType::Normal,
            clothes: ClothesStyle::Tunic,
            helmet: HelmetStyle::None,
            armor: ArmorStyle::None,
            has_cape: false,
            shield_on: false,
            shield_style: ShieldStyle::Round,
            weapon: WeaponStyle::Sword,
        }
    }
}

impl CharacterAppearance {
    pub fn random() -> Self {
        Self {
            palette: HeroPalette {
                skin: pick_color(&SKIN_COLORS),
                hair: pick_color(&HAIR_COLORS),
                shirt: pick_color(&SHIRT_COLORS),
                pants: pick_color(&PANTS_COLORS),
                boots: pick_color(&BOOTS_COLORS),
                armor: pick_color(&ARMOR_COLORS),
                cape: pick_color(&CAPE_COLORS),
                weapon: pick_color(&WEAPON_COLORS),
            },
            hair_style: HAIR_STYLES[gen_range(0, HAIR_STYLES.len())],
            build: BUILD_TYPES[gen_range(0, BUILD_TYPES.len())],
            clothes: CLOTHES_STYLES[gen_range(0, CLOTHES_STYLES.len())],
            helmet: HELMET_STYLES[gen_range(0, HELMET_STYLES.len())],
            armor: ARMOR_STYLES[gen_range(0, ARMOR_STYLES.len())],
            has_cape: gen_range(0, 2) == 1,
            shield_on: gen_range(0, 2) == 1,
            shield_style: SHIELD_STYLES[gen_range(0, SHIELD_STYLES.len())],
            weapon: WEAPON_STYLES[gen_range(0, WEAPON_STYLES.len())],
        }
    }
}

#[derive(Clone, Debug)]
pub struct CharacterCreator {
    pub appearance: CharacterAppearance,
    pub selected_field: usize,
}

impl CharacterCreator {
    pub fn new(appearance: CharacterAppearance) -> Self {
        Self {
            appearance,
            selected_field: 0,
        }
    }

    pub fn randomize(&mut self) {
        self.appearance = CharacterAppearance::random();
    }

    pub fn move_selection(&mut self, delta: i32) {
        let count = CREATOR_FIELDS.len() as i32;
        self.selected_field = (self.selected_field as i32 + delta).rem_euclid(count) as usize;
    }

    pub fn adjust_selected(&mut self, delta: i32) {
        match CREATOR_FIELDS[self.selected_field] {
            CreatorField::Skin => cycle_color(&mut self.appearance.palette.skin, &SKIN_COLORS, delta),
            CreatorField::HairColor => {
                cycle_color(&mut self.appearance.palette.hair, &HAIR_COLORS, delta)
            }
            CreatorField::ShirtColor => {
                cycle_color(&mut self.appearance.palette.shirt, &SHIRT_COLORS, delta)
            }
            CreatorField::PantsColor => {
                cycle_color(&mut self.appearance.palette.pants, &PANTS_COLORS, delta)
            }
            CreatorField::BootsColor => {
                cycle_color(&mut self.appearance.palette.boots, &BOOTS_COLORS, delta)
            }
            CreatorField::HairStyle => cycle_enum(&mut self.appearance.hair_style, &HAIR_STYLES, delta),
            CreatorField::Build => cycle_enum(&mut self.appearance.build, &BUILD_TYPES, delta),
            CreatorField::Clothes => cycle_enum(&mut self.appearance.clothes, &CLOTHES_STYLES, delta),
        }
    }

    pub fn field_text(&self, index: usize) -> String {
        match CREATOR_FIELDS[index] {
            CreatorField::Skin => format!("Skin: {}", color_label(self.appearance.palette.skin)),
            CreatorField::HairColor => {
                format!("Hair Color: {}", color_label(self.appearance.palette.hair))
            }
            CreatorField::ShirtColor => {
                format!("Shirt Color: {}", color_label(self.appearance.palette.shirt))
            }
            CreatorField::PantsColor => {
                format!("Pants Color: {}", color_label(self.appearance.palette.pants))
            }
            CreatorField::BootsColor => {
                format!("Boots Color: {}", color_label(self.appearance.palette.boots))
            }
            CreatorField::HairStyle => format!("Hair: {:?}", self.appearance.hair_style),
            CreatorField::Build => format!("Build: {:?}", self.appearance.build),
            CreatorField::Clothes => format!("Clothes: {:?}", self.appearance.clothes),
        }
    }

    pub fn field_count(&self) -> usize {
        CREATOR_FIELDS.len()
    }

    pub fn field_color(&self, index: usize) -> Option<Color> {
        match CREATOR_FIELDS[index] {
            CreatorField::Skin => Some(self.appearance.palette.skin),
            CreatorField::HairColor => Some(self.appearance.palette.hair),
            CreatorField::ShirtColor => Some(self.appearance.palette.shirt),
            CreatorField::PantsColor => Some(self.appearance.palette.pants),
            CreatorField::BootsColor => Some(self.appearance.palette.boots),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum CreatorField {
    Skin,
    HairColor,
    ShirtColor,
    PantsColor,
    BootsColor,
    HairStyle,
    Build,
    Clothes,
}

const CREATOR_FIELDS: [CreatorField; 8] = [
    CreatorField::Skin,
    CreatorField::HairColor,
    CreatorField::ShirtColor,
    CreatorField::PantsColor,
    CreatorField::BootsColor,
    CreatorField::HairStyle,
    CreatorField::Build,
    CreatorField::Clothes,
];

pub struct HeroSheets {
    pub idle: Image,
    pub walk: Image,
    pub idle_armed: Image,
    pub walk_armed: Image,
}

pub fn generate_hero_sheets(appearance: &CharacterAppearance) -> HeroSheets {
    HeroSheets {
        idle: build_sheet(appearance, AnimMode::Idle, false),
        walk: build_sheet(appearance, AnimMode::Walk, false),
        idle_armed: build_sheet(appearance, AnimMode::Idle, true),
        walk_armed: build_sheet(appearance, AnimMode::Walk, true),
    }
}

fn build_sheet(appearance: &CharacterAppearance, mode: AnimMode, armed: bool) -> Image {
    let mut image = Image::gen_image_color(
        SHEET_SIZE,
        SHEET_SIZE,
        Color::new(0.0, 0.0, 0.0, 0.0),
    );
    for dir in 0..4 {
        for frame in 0..4 {
            let canvas = draw_frame(appearance, dir, frame, mode, armed);
            for y in 0..FRAME_SIZE {
                for x in 0..FRAME_SIZE {
                    let color = canvas[(y as usize * FRAME_SIZE as usize) + x as usize];
                    if color.a > 0.0 {
                        image.set_pixel(
                            (frame as u32 * FRAME_SIZE as u32 + x as u32) as u32,
                            (dir as u32 * FRAME_SIZE as u32 + y as u32) as u32,
                            color,
                        );
                    }
                }
            }
        }
    }
    image
}

fn draw_frame(
    appearance: &CharacterAppearance,
    dir: usize,
    frame: usize,
    mode: AnimMode,
    armed: bool,
) -> Vec<Color> {
    let mut px = vec![Color::new(0.0, 0.0, 0.0, 0.0); FRAME_SIZE as usize * FRAME_SIZE as usize];
    let bob = match mode {
        AnimMode::Idle => [0, 0, -1, -1][frame],
        AnimMode::Walk => [-1, 0, -1, 0][frame],
    };
    let swing = match mode {
        AnimMode::Idle => [0, 0, 0, 0][frame],
        AnimMode::Walk => [2, 0, -2, 0][frame],
    };
    let build = match appearance.build {
        BuildType::Normal => 0,
        BuildType::Stocky => 2,
        BuildType::Slim => -2,
    };
    let body_w = 14 + build;
    let body_x = 24 - body_w / 2;
    let body_y = 17 + bob;
    let leg_y = 29 + bob;
    let hair_dark = shade(appearance.palette.hair, 0.72);
    let shirt_dark = shade(appearance.palette.shirt, 0.72);
    let pants_dark = shade(appearance.palette.pants, 0.7);
    let boots_dark = shade(appearance.palette.boots, 0.65);

    draw_shadow(&mut px, 24, 40 + bob, 8 + build);
    if appearance.has_cape {
        for y in 0..18 {
            let width = if y < 4 { body_w - 2 } else { body_w + 2 };
            let start = 24 - width / 2;
            let color = if y > 12 {
                shade(appearance.palette.cape, 0.45)
            } else if y > 7 {
                shade(appearance.palette.cape, 0.65)
            } else {
                appearance.palette.cape
            };
            rect(&mut px, start, body_y + y, width, 1, color);
        }
    }

    if dir == 1 || dir == 2 {
        let front = if dir == 1 { -1 } else { 1 };
        for y in 0..9 {
            rect(&mut px, 17, leg_y + y + swing / 2, 4, 1, pants_dark);
            rect(&mut px, 20, leg_y + y - swing / 2, 4, 1, appearance.palette.pants);
        }
        hline(&mut px, 16, 21, leg_y + 8 + swing / 2, boots_dark);
        hline(&mut px, 19, 26, leg_y + 8 - swing / 2, appearance.palette.boots);
        for y in 0..13 {
            let row_w = if y < 2 { body_w - 2 } else { body_w };
            let color = if appearance.clothes == ClothesStyle::Bare {
                appearance.palette.skin
            } else if y > 8 {
                shirt_dark
            } else {
                appearance.palette.shirt
            };
            rect(&mut px, 16, body_y + y, row_w, 1, color);
        }
        draw_head_side(&mut px, appearance, 18, body_y - 6);
        if appearance.shield_on {
            draw_shield_shape(
                &mut px,
                27,
                body_y + 4,
                appearance.shield_style,
                appearance.palette.armor,
            );
        }
        if armed {
            draw_weapon_shape(
                &mut px,
                30 + front,
                body_y + 5,
                appearance.weapon,
                appearance.palette.weapon,
                front,
            );
        }
        return px;
    }

    for y in 0..9 {
        rect(&mut px, body_x + 1, leg_y + y + swing, 4, 1, appearance.palette.pants);
        rect(&mut px, body_x + body_w - 5, leg_y + y - swing, 4, 1, pants_dark);
    }
    hline(&mut px, body_x, body_x + 5, leg_y + 8 + swing, appearance.palette.boots);
    hline(
        &mut px,
        body_x + body_w - 6,
        body_x + body_w - 1,
        leg_y + 8 - swing,
        boots_dark,
    );
    for y in 0..13 {
        let row_w = if y < 2 {
            body_w - 4
        } else if y < 5 {
            body_w - 2
        } else {
            body_w
        };
        let row_x = 24 - row_w / 2;
        let color = if appearance.clothes == ClothesStyle::Bare {
            appearance.palette.skin
        } else if y > 8 {
            shirt_dark
        } else {
            appearance.palette.shirt
        };
        rect(&mut px, row_x, body_y + y, row_w, 1, color);
    }
    hline(&mut px, body_x + 2, body_x + body_w - 3, body_y + 8, hex("#c49333"));
    if appearance.armor != ArmorStyle::None {
        draw_armor_shape(&mut px, body_x, body_y, body_w, appearance.armor, appearance.palette.armor);
    }
    rect(&mut px, body_x - 3, body_y + 1 + swing, 2, 7, appearance.palette.shirt);
    rect(
        &mut px,
        body_x + body_w + 1,
        body_y + 1 - swing,
        2,
        7,
        appearance.palette.shirt,
    );
    if appearance.shield_on && dir == 0 {
        draw_shield_shape(
            &mut px,
            body_x - 6,
            body_y + 2 + swing,
            appearance.shield_style,
            appearance.palette.armor,
        );
    }
    if armed {
        let (wx, wy, facing) = if dir == 0 {
            (body_x + body_w + 4, body_y + 2 - swing, 1)
        } else {
            (body_x - 6, body_y + 2 + swing, -1)
        };
        draw_weapon_shape(
            &mut px,
            wx,
            wy,
            appearance.weapon,
            appearance.palette.weapon,
            facing,
        );
    }
    draw_head_front_back(&mut px, appearance, dir == 3, body_x + 1, body_y - 6, body_w - 2, hair_dark);
    px
}

fn draw_head_front_back(
    px: &mut [Color],
    appearance: &CharacterAppearance,
    back: bool,
    x: i32,
    y: i32,
    w: i32,
    hair_dark: Color,
) {
    rect(px, x, y + 2, w, 8, appearance.palette.skin);
    match appearance.helmet {
        HelmetStyle::None => draw_hair(px, appearance.hair_style, x, y, w, appearance.palette.hair, back),
        style => draw_helmet_shape(px, style, x, y, w, appearance.palette.armor),
    }
    if back {
        hline(px, x + 1, x + w - 2, y + 5, hair_dark);
    } else {
        pixel(px, x + 2, y + 5, WHITE);
        pixel(px, x + 3, y + 5, BLACK);
        pixel(px, x + w - 4, y + 5, WHITE);
        pixel(px, x + w - 3, y + 5, BLACK);
    }
}

fn draw_head_side(px: &mut [Color], appearance: &CharacterAppearance, x: i32, y: i32) {
    rect(px, x, y + 2, 10, 8, appearance.palette.skin);
    match appearance.helmet {
        HelmetStyle::None => draw_hair(px, appearance.hair_style, x, y, 10, appearance.palette.hair, false),
        style => draw_helmet_shape(px, style, x, y, 10, appearance.palette.armor),
    }
    pixel(px, x + 6, y + 5, WHITE);
    pixel(px, x + 7, y + 5, BLACK);
}

fn draw_hair(px: &mut [Color], style: HairStyle, x: i32, y: i32, w: i32, color: Color, back: bool) {
    match style {
        HairStyle::Short => rect(px, x, y, w, 4, color),
        HairStyle::Long => {
            rect(px, x, y, w, 4, color);
            if back {
                rect(px, x + 1, y + 4, w - 2, 4, shade(color, 0.72));
            }
        }
        HairStyle::Spiky => {
            rect(px, x, y + 1, w, 3, color);
            for spike in 0..4 {
                pixel(px, x + 1 + spike * 3, y, color);
            }
        }
        HairStyle::Bald => {}
    }
}

fn draw_helmet_shape(px: &mut [Color], style: HelmetStyle, x: i32, y: i32, w: i32, color: Color) {
    match style {
        HelmetStyle::None => {}
        HelmetStyle::Iron => rect(px, x, y, w, 4, color),
        HelmetStyle::Hood => rect(px, x - 1, y, w + 2, 5, shade(color, 0.82)),
        HelmetStyle::Crown => {
            rect(px, x, y + 2, w, 2, color);
            for point in 0..3 {
                rect(px, x + 1 + point * 4, y, 2, 2, shade(color, 1.2));
            }
        }
    }
}

fn draw_armor_shape(px: &mut [Color], x: i32, y: i32, w: i32, armor: ArmorStyle, color: Color) {
    match armor {
        ArmorStyle::None => {}
        ArmorStyle::Leather => {
            hline(px, x + 1, x + w - 2, y + 3, shade(color, 0.7));
            hline(px, x + 1, x + w - 2, y + 8, shade(color, 0.7));
        }
        ArmorStyle::Chainmail => {
            for row in 3..11 {
                for col in (x + 1..x + w - 1).step_by(2) {
                    pixel(px, col, y + row, shade(color, 0.65));
                }
            }
        }
        ArmorStyle::Chestplate => {
            rect(px, x + 2, y + 2, w - 4, 8, color);
            hline(px, x + 2, x + w - 3, y + 2, shade(color, 1.2));
        }
    }
}

fn draw_shield_shape(px: &mut [Color], x: i32, y: i32, style: ShieldStyle, color: Color) {
    match style {
        ShieldStyle::Round => {
            for (row, range) in [[1, 3], [0, 4], [0, 4], [0, 4], [1, 3]].iter().enumerate() {
                hline(px, x + range[0], x + range[1], y + row as i32, color);
            }
        }
        ShieldStyle::Kite => {
            rect(px, x, y, 4, 7, color);
            pixel(px, x + 1, y + 7, color);
            pixel(px, x + 2, y + 7, color);
        }
        ShieldStyle::Tower => rect(px, x, y, 4, 10, color),
    }
}

fn draw_weapon_shape(px: &mut [Color], x: i32, y: i32, weapon: WeaponStyle, color: Color, facing: i32) {
    let dark = shade(color, 0.6);
    match weapon {
        WeaponStyle::Sword => {
            vline(px, x, y - 6, y + 4, color);
            hline(px, x - 2, x + 2, y + 3, dark);
        }
        WeaponStyle::Axe => {
            vline(px, x, y - 4, y + 5, dark);
            rect(px, x + facing, y - 3, 3, 4, color);
        }
        WeaponStyle::Spear => {
            vline(px, x, y - 8, y + 4, dark);
            pixel(px, x, y - 9, color);
            pixel(px, x - 1, y - 8, color);
            pixel(px, x + 1, y - 8, color);
        }
        WeaponStyle::Staff => {
            vline(px, x, y - 7, y + 5, dark);
            rect(px, x - 1, y - 9, 3, 3, color);
        }
    }
}

fn draw_shadow(px: &mut [Color], cx: i32, cy: i32, rx: i32) {
    for y in -3..=3 {
        for x in -rx..=rx {
            let nx = x as f32 / rx.max(1) as f32;
            let ny = y as f32 / 3.0;
            if nx * nx + ny * ny <= 1.0 {
                pixel(px, cx + x, cy + y, Color::new(0.0, 0.0, 0.0, 0.15));
            }
        }
    }
}

fn pixel(px: &mut [Color], x: i32, y: i32, color: Color) {
    if x < 0 || y < 0 || x >= FRAME_SIZE as i32 || y >= FRAME_SIZE as i32 {
        return;
    }
    px[y as usize * FRAME_SIZE as usize + x as usize] = color;
}

fn hline(px: &mut [Color], x1: i32, x2: i32, y: i32, color: Color) {
    for x in x1..=x2 {
        pixel(px, x, y, color);
    }
}

fn vline(px: &mut [Color], x: i32, y1: i32, y2: i32, color: Color) {
    for y in y1..=y2 {
        pixel(px, x, y, color);
    }
}

fn rect(px: &mut [Color], x: i32, y: i32, w: i32, h: i32, color: Color) {
    for dy in 0..h {
        for dx in 0..w {
            pixel(px, x + dx, y + dy, color);
        }
    }
}

fn shade(color: Color, factor: f32) -> Color {
    Color::new(
        (color.r * factor).clamp(0.0, 1.0),
        (color.g * factor).clamp(0.0, 1.0),
        (color.b * factor).clamp(0.0, 1.0),
        color.a,
    )
}

fn hex(value: &str) -> Color {
    let s = value.trim_start_matches('#');
    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(255);
    Color::from_rgba(r, g, b, 255)
}

fn pick_color(list: &[Color]) -> Color {
    list[gen_range(0, list.len())]
}

fn cycle_color(current: &mut Color, list: &[Color], delta: i32) {
    let current_index = list
        .iter()
        .position(|c| almost_same_color(*c, *current))
        .unwrap_or(0);
    let next = (current_index as i32 + delta).rem_euclid(list.len() as i32) as usize;
    *current = list[next];
}

fn cycle_enum<T: Copy + PartialEq>(current: &mut T, list: &[T], delta: i32) {
    let current_index = list.iter().position(|v| *v == *current).unwrap_or(0);
    let next = (current_index as i32 + delta).rem_euclid(list.len() as i32) as usize;
    *current = list[next];
}

fn almost_same_color(a: Color, b: Color) -> bool {
    (a.r - b.r).abs() < 0.001 && (a.g - b.g).abs() < 0.001 && (a.b - b.b).abs() < 0.001
}

fn color_label(color: Color) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        (color.r * 255.0).round() as u8,
        (color.g * 255.0).round() as u8,
        (color.b * 255.0).round() as u8
    )
}

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

const HAIR_STYLES: [HairStyle; 4] = [HairStyle::Short, HairStyle::Long, HairStyle::Spiky, HairStyle::Bald];
const BUILD_TYPES: [BuildType; 3] = [BuildType::Normal, BuildType::Stocky, BuildType::Slim];
const CLOTHES_STYLES: [ClothesStyle; 4] = [
    ClothesStyle::Tunic,
    ClothesStyle::Robe,
    ClothesStyle::Vest,
    ClothesStyle::Bare,
];
const HELMET_STYLES: [HelmetStyle; 4] = [
    HelmetStyle::None,
    HelmetStyle::Iron,
    HelmetStyle::Hood,
    HelmetStyle::Crown,
];
const ARMOR_STYLES: [ArmorStyle; 4] = [
    ArmorStyle::None,
    ArmorStyle::Leather,
    ArmorStyle::Chainmail,
    ArmorStyle::Chestplate,
];
const SHIELD_STYLES: [ShieldStyle; 3] = [
    ShieldStyle::Round,
    ShieldStyle::Kite,
    ShieldStyle::Tower,
];
const WEAPON_STYLES: [WeaponStyle; 4] = [
    WeaponStyle::Sword,
    WeaponStyle::Axe,
    WeaponStyle::Spear,
    WeaponStyle::Staff,
];

const SKIN_COLORS: [Color; 5] = [
    rgb(255, 204, 153),
    rgb(244, 164, 96),
    rgb(198, 134, 66),
    rgb(141, 85, 36),
    rgb(74, 44, 23),
];
const HAIR_COLORS: [Color; 6] = [
    rgb(42, 21, 0),
    rgb(122, 59, 16),
    rgb(200, 134, 10),
    rgb(218, 165, 32),
    rgb(136, 51, 204),
    rgb(204, 204, 204),
];
const SHIRT_COLORS: [Color; 6] = [
    rgb(34, 102, 170),
    rgb(34, 153, 68),
    rgb(170, 34, 34),
    rgb(119, 34, 170),
    rgb(204, 136, 34),
    rgb(245, 230, 200),
];
const PANTS_COLORS: [Color; 6] = [
    rgb(85, 51, 17),
    rgb(51, 68, 85),
    rgb(34, 34, 34),
    rgb(85, 102, 51),
    rgb(136, 68, 34),
    rgb(136, 136, 170),
];
const BOOTS_COLORS: [Color; 5] = [
    rgb(42, 26, 10),
    rgb(92, 58, 30),
    rgb(26, 42, 58),
    rgb(58, 58, 58),
    rgb(138, 106, 58),
];
const ARMOR_COLORS: [Color; 5] = [
    rgb(136, 153, 170),
    rgb(200, 168, 74),
    rgb(204, 68, 34),
    rgb(34, 85, 170),
    rgb(232, 232, 240),
];
const CAPE_COLORS: [Color; 5] = [
    rgb(170, 34, 34),
    rgb(34, 102, 170),
    rgb(34, 34, 34),
    rgb(51, 136, 51),
    rgb(119, 34, 170),
];
const WEAPON_COLORS: [Color; 5] = [
    rgb(176, 200, 216),
    rgb(200, 168, 74),
    rgb(204, 68, 34),
    rgb(68, 170, 204),
    rgb(255, 255, 255),
];
