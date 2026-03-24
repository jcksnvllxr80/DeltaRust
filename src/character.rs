use crate::model::{Dir, NpcKind, Player, PlayerState};
use macroquad::prelude::{Color, Image, WHITE};
use macroquad::rand::gen_range;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

mod color_serde {
    use macroquad::prelude::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    struct ColorArray([f32; 4]);

    pub fn serialize<S: Serializer>(color: &Color, s: S) -> Result<S::Ok, S::Error> {
        ColorArray([color.r, color.g, color.b, color.a]).serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Color, D::Error> {
        let ColorArray(arr) = ColorArray::deserialize(d)?;
        Ok(Color::new(arr[0], arr[1], arr[2], arr[3]))
    }
}

pub const FRAME_SIZE: u16 = 48;
pub const SHEET_SIZE: u16 = FRAME_SIZE * 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimMode {
    Idle,
    Walk,
    Attack,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewAnimation {
    IdleDown,
    IdleLeft,
    IdleRight,
    IdleUp,
    WalkDown,
    WalkLeft,
    WalkRight,
    WalkUp,
    SwingDown,
    SwingLeft,
    SwingRight,
    SwingUp,
}

impl PreviewAnimation {
    pub fn label(self) -> &'static str {
        match self {
            PreviewAnimation::IdleDown => "Idle Down",
            PreviewAnimation::IdleLeft => "Idle Left",
            PreviewAnimation::IdleRight => "Idle Right",
            PreviewAnimation::IdleUp => "Idle Up",
            PreviewAnimation::WalkDown => "Walk Down",
            PreviewAnimation::WalkLeft => "Walk Left",
            PreviewAnimation::WalkRight => "Walk Right",
            PreviewAnimation::WalkUp => "Walk Up",
            PreviewAnimation::SwingDown => "Thrust Down",
            PreviewAnimation::SwingLeft => "Thrust Left",
            PreviewAnimation::SwingRight => "Thrust Right",
            PreviewAnimation::SwingUp => "Thrust Up",
        }
    }

    pub fn apply_to_preview(self, player: &mut Player) {
        player.dir = match self {
            PreviewAnimation::IdleDown
            | PreviewAnimation::WalkDown
            | PreviewAnimation::SwingDown => Dir::Down,
            PreviewAnimation::IdleLeft
            | PreviewAnimation::WalkLeft
            | PreviewAnimation::SwingLeft => Dir::Left,
            PreviewAnimation::IdleRight
            | PreviewAnimation::WalkRight
            | PreviewAnimation::SwingRight => Dir::Right,
            PreviewAnimation::IdleUp
            | PreviewAnimation::WalkUp
            | PreviewAnimation::SwingUp => Dir::Up,
        };

        match self {
            PreviewAnimation::IdleDown
            | PreviewAnimation::IdleLeft
            | PreviewAnimation::IdleRight
            | PreviewAnimation::IdleUp => {
                player.state = PlayerState::Idle;
                player.attack_timer = 0;
                player.walk_frame = 0;
            }
            PreviewAnimation::WalkDown
            | PreviewAnimation::WalkLeft
            | PreviewAnimation::WalkRight
            | PreviewAnimation::WalkUp => {
                player.state = PlayerState::Walking;
                player.attack_timer = 0;
                player.walk_frame = 0;
            }
            PreviewAnimation::SwingDown
            | PreviewAnimation::SwingLeft
            | PreviewAnimation::SwingRight
            | PreviewAnimation::SwingUp => {
                player.state = PlayerState::Attacking;
                player.walk_frame = 0;
                player.attack_timer = 0;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaceShape {
    Round,
    Square,
    Narrow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EyeExpression {
    Normal,
    Angry,
    Sad,
    Happy,
    Surprised,
    Confused,
    Thinking,
    Tired,
    Straining,
    Wide,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairStyle {
    Short,
    Long,
    Spiky,
    Bald,
    Ponytail,
    Mohawk,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildType {
    Normal,
    Stocky,
    Slim,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClothesStyle {
    Tunic,
    Robe,
    Vest,
    Cloak,
    Bare,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HelmetStyle {
    None,
    Iron,
    Greathelm,
    Pointy,
    Hood,
    Cap,
    Crown,
    Bandana,
    Horned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmorPiece {
    Leather,
    Chainmail,
    Chestplate,
    Shoulderpads,
    Fullplate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShieldStyle {
    Round,
    Kite,
    Tower,
    Buckler,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponStyle {
    None,
    Sword,
    Greatsword,
    Axe,
    Hammer,
    Spear,
    Staff,
    Wand,
    Bow,
    Crossbow,
    Dagger,
    Boomerang,
    Scythe,
    Flail,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffhandStyle {
    None,
    Torch,
    Orb,
    Dagger,
    Potion,
    Book,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct HeroPalette {
    #[serde(with = "color_serde")]
    pub skin: Color,
    #[serde(with = "color_serde")]
    pub hair: Color,
    #[serde(with = "color_serde")]
    pub shirt: Color,
    #[serde(with = "color_serde")]
    pub pants: Color,
    #[serde(with = "color_serde")]
    pub boots: Color,
    #[serde(with = "color_serde")]
    pub armor: Color,
    #[serde(with = "color_serde")]
    pub cape: Color,
    #[serde(with = "color_serde")]
    pub weapon: Color,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterAppearance {
    #[serde(default)]
    pub gender: Gender,
    pub palette: HeroPalette,
    pub face: FaceShape,
    pub eyes: EyeExpression,
    pub hair_style: HairStyle,
    pub build: BuildType,
    pub clothes: ClothesStyle,
    pub helmet: HelmetStyle,
    pub armors: HashSet<ArmorPiece>,
    pub has_cape: bool,
    pub shield_on: bool,
    pub shield_style: ShieldStyle,
    pub weapon: WeaponStyle,
    pub offhand: OffhandStyle,
}

impl Default for CharacterAppearance {
    fn default() -> Self {
        Self {
            gender: Gender::Male,
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
            face: FaceShape::Round,
            eyes: EyeExpression::Normal,
            hair_style: HairStyle::Short,
            build: BuildType::Normal,
            clothes: ClothesStyle::Tunic,
            helmet: HelmetStyle::None,
            armors: HashSet::new(),
            has_cape: false,
            shield_on: false,
            shield_style: ShieldStyle::Round,
            weapon: WeaponStyle::None,
            offhand: OffhandStyle::None,
        }
    }
}

impl CharacterAppearance {
    pub fn random() -> Self {
        Self {
            gender: Gender::Male,
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
            face: FACE_SHAPES[gen_range(0, FACE_SHAPES.len())],
            eyes: EYE_EXPRESSIONS[gen_range(0, EYE_EXPRESSIONS.len())],
            hair_style: HAIR_STYLES[gen_range(0, HAIR_STYLES.len())],
            build: BUILD_TYPES[gen_range(0, BUILD_TYPES.len())],
            clothes: CLOTHES_STYLES[gen_range(0, CLOTHES_STYLES.len())],
            helmet: HELMET_STYLES[gen_range(0, HELMET_STYLES.len())],
            armors: random_armors(),
            has_cape: false,
            shield_on: false,
            shield_style: SHIELD_STYLES[gen_range(0, SHIELD_STYLES.len())],
            weapon: WEAPON_STYLES[gen_range(0, WEAPON_STYLES.len())],
            offhand: OFFHAND_STYLES[gen_range(0, OFFHAND_STYLES.len())],
        }
    }
}

pub fn npc_appearance(kind: NpcKind) -> CharacterAppearance {
    let mut appearance = CharacterAppearance::default();
    appearance.weapon = WeaponStyle::None;
    appearance.offhand = OffhandStyle::None;
    appearance.shield_on = false;
    appearance.has_cape = false;
    appearance.armors.clear();
    match kind {
        NpcKind::Elara => {
            appearance.palette.skin = rgb(235, 194, 156);
            appearance.palette.hair = rgb(92, 52, 24);
            appearance.palette.shirt = rgb(82, 138, 84);
            appearance.palette.pants = rgb(98, 72, 46);
            appearance.palette.boots = rgb(62, 42, 24);
            appearance.face = FaceShape::Round;
            appearance.eyes = EyeExpression::Happy;
            appearance.hair_style = HairStyle::Long;
            appearance.clothes = ClothesStyle::Robe;
            appearance.offhand = OffhandStyle::Potion;
        }
        NpcKind::Barnett => {
            appearance.palette.skin = rgb(220, 182, 142);
            appearance.palette.hair = rgb(68, 42, 20);
            appearance.palette.shirt = rgb(88, 118, 156);
            appearance.palette.pants = rgb(92, 78, 54);
            appearance.palette.boots = rgb(56, 38, 22);
            appearance.face = FaceShape::Square;
            appearance.eyes = EyeExpression::Thinking;
            appearance.hair_style = HairStyle::Short;
            appearance.clothes = ClothesStyle::Vest;
            appearance.offhand = OffhandStyle::Book;
        }
        NpcKind::Maren => {
            appearance.palette.skin = rgb(214, 172, 132);
            appearance.palette.hair = rgb(44, 22, 10);
            appearance.palette.shirt = rgb(132, 98, 70);
            appearance.palette.pants = rgb(62, 54, 48);
            appearance.palette.boots = rgb(44, 30, 18);
            appearance.face = FaceShape::Square;
            appearance.eyes = EyeExpression::Normal;
            appearance.hair_style = HairStyle::Ponytail;
            appearance.clothes = ClothesStyle::Vest;
            appearance.weapon = WeaponStyle::Hammer;
        }
        NpcKind::Oswin => {
            appearance.palette.skin = rgb(230, 196, 156);
            appearance.palette.hair = rgb(196, 188, 164);
            appearance.palette.shirt = rgb(112, 96, 82);
            appearance.palette.pants = rgb(84, 74, 66);
            appearance.palette.boots = rgb(52, 40, 28);
            appearance.face = FaceShape::Narrow;
            appearance.eyes = EyeExpression::Tired;
            appearance.hair_style = HairStyle::Bald;
            appearance.clothes = ClothesStyle::Robe;
        }
        NpcKind::Corvin => {
            appearance.palette.skin = rgb(198, 160, 128);
            appearance.palette.hair = rgb(42, 30, 22);
            appearance.palette.shirt = rgb(96, 94, 122);
            appearance.palette.pants = rgb(68, 68, 84);
            appearance.palette.boots = rgb(42, 34, 28);
            appearance.palette.armor = rgb(158, 170, 180);
            appearance.face = FaceShape::Narrow;
            appearance.eyes = EyeExpression::Angry;
            appearance.hair_style = HairStyle::Short;
            appearance.clothes = ClothesStyle::Cloak;
            appearance.has_cape = true;
            appearance.offhand = OffhandStyle::Book;
        }
        NpcKind::Petra => {
            appearance.palette.skin = rgb(224, 188, 150);
            appearance.palette.hair = rgb(92, 52, 18);
            appearance.palette.shirt = rgb(136, 108, 82);
            appearance.palette.pants = rgb(78, 62, 48);
            appearance.palette.boots = rgb(48, 34, 24);
            appearance.face = FaceShape::Round;
            appearance.eyes = EyeExpression::Wide;
            appearance.hair_style = HairStyle::Mohawk;
            appearance.clothes = ClothesStyle::Vest;
            appearance.offhand = OffhandStyle::Torch;
        }
        NpcKind::Aldric => {
            appearance.palette.skin = rgb(224, 192, 160);
            appearance.palette.hair = rgb(214, 196, 124);
            appearance.palette.shirt = rgb(74, 126, 154);
            appearance.palette.pants = rgb(76, 88, 112);
            appearance.palette.boots = rgb(48, 34, 24);
            appearance.palette.armor = rgb(170, 188, 196);
            appearance.face = FaceShape::Square;
            appearance.eyes = EyeExpression::Normal;
            appearance.hair_style = HairStyle::Short;
            appearance.clothes = ClothesStyle::Vest;
            appearance.helmet = HelmetStyle::Cap;
            appearance.offhand = OffhandStyle::Book;
        }
        NpcKind::Sael => {
            appearance.palette.skin = rgb(202, 164, 132);
            appearance.palette.hair = rgb(28, 18, 16);
            appearance.palette.shirt = rgb(80, 112, 138);
            appearance.palette.pants = rgb(70, 84, 104);
            appearance.palette.boots = rgb(44, 32, 24);
            appearance.face = FaceShape::Round;
            appearance.eyes = EyeExpression::Confused;
            appearance.hair_style = HairStyle::Long;
            appearance.clothes = ClothesStyle::Cloak;
            appearance.has_cape = true;
            appearance.offhand = OffhandStyle::Torch;
        }
        NpcKind::Dax => {
            appearance.palette.skin = rgb(192, 144, 116);
            appearance.palette.hair = rgb(116, 52, 22);
            appearance.palette.shirt = rgb(146, 96, 62);
            appearance.palette.pants = rgb(82, 68, 56);
            appearance.palette.boots = rgb(50, 36, 24);
            appearance.face = FaceShape::Square;
            appearance.eyes = EyeExpression::Straining;
            appearance.hair_style = HairStyle::Spiky;
            appearance.build = BuildType::Stocky;
            appearance.clothes = ClothesStyle::Vest;
            appearance.weapon = WeaponStyle::Hammer;
        }
        NpcKind::Vel => {
            appearance.palette.skin = rgb(208, 186, 166);
            appearance.palette.hair = rgb(170, 194, 228);
            appearance.palette.shirt = rgb(124, 118, 164);
            appearance.palette.pants = rgb(88, 84, 112);
            appearance.palette.boots = rgb(46, 36, 30);
            appearance.face = FaceShape::Narrow;
            appearance.eyes = EyeExpression::Thinking;
            appearance.hair_style = HairStyle::Long;
            appearance.clothes = ClothesStyle::Robe;
            appearance.offhand = OffhandStyle::Orb;
        }
        NpcKind::CelestialMerchant => {
            appearance.palette.skin = rgb(226, 202, 182);
            appearance.palette.hair = rgb(246, 240, 190);
            appearance.palette.shirt = rgb(146, 126, 196);
            appearance.palette.pants = rgb(100, 90, 140);
            appearance.palette.boots = rgb(54, 40, 34);
            appearance.palette.cape = rgb(224, 206, 120);
            appearance.face = FaceShape::Round;
            appearance.eyes = EyeExpression::Happy;
            appearance.hair_style = HairStyle::Long;
            appearance.clothes = ClothesStyle::Robe;
            appearance.has_cape = true;
            appearance.offhand = OffhandStyle::Orb;
        }
        NpcKind::Senna => {
            appearance.palette.skin = rgb(234, 212, 192);
            appearance.palette.hair = rgb(204, 216, 242);
            appearance.palette.shirt = rgb(138, 144, 182);
            appearance.palette.pants = rgb(94, 98, 128);
            appearance.palette.boots = rgb(50, 40, 34);
            appearance.face = FaceShape::Narrow;
            appearance.eyes = EyeExpression::Sad;
            appearance.hair_style = HairStyle::Long;
            appearance.clothes = ClothesStyle::Cloak;
            appearance.has_cape = true;
        }
        NpcKind::Wren => {
            appearance.palette.skin = rgb(210, 174, 146);
            appearance.palette.hair = rgb(190, 178, 126);
            appearance.palette.shirt = rgb(138, 126, 90);
            appearance.palette.pants = rgb(90, 82, 62);
            appearance.palette.boots = rgb(48, 36, 26);
            appearance.face = FaceShape::Square;
            appearance.eyes = EyeExpression::Tired;
            appearance.hair_style = HairStyle::Ponytail;
            appearance.clothes = ClothesStyle::Cloak;
            appearance.offhand = OffhandStyle::Potion;
        }
        NpcKind::GnomeHealer => {
            // Rendered procedurally — appearance is unused.
        }
    }
    appearance
}

#[derive(Clone, Debug)]
pub struct CharacterCreator {
    pub appearance: CharacterAppearance,
    pub selected_field: usize,
    pub preview_animation: PreviewAnimation,
}

impl CharacterCreator {
    pub fn new(appearance: CharacterAppearance) -> Self {
        Self {
            appearance,
            selected_field: 0,
            preview_animation: PreviewAnimation::WalkDown,
        }
    }

    pub fn randomize(&mut self) {
        self.appearance = CharacterAppearance::random();
    }

    pub fn move_selection(&mut self, delta: i32) {
        let count = CREATOR_FIELDS.len() as i32;
        self.selected_field = (self.selected_field as i32 + delta).rem_euclid(count) as usize;
    }

    pub fn adjust_selected(&mut self, delta: i32) -> bool {
        match CREATOR_FIELDS[self.selected_field] {
            CreatorField::Gender => {
                self.appearance.gender = match self.appearance.gender {
                    Gender::Male => Gender::Female,
                    Gender::Female => Gender::Male,
                };
                apply_gender_preset(&mut self.appearance);
            }
            CreatorField::Skin => {
                cycle_color(&mut self.appearance.palette.skin, &SKIN_COLORS, delta)
            }
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
            CreatorField::ArmorColor => {
                cycle_color(&mut self.appearance.palette.armor, &ARMOR_COLORS, delta)
            }
            CreatorField::CapeColor => {
                cycle_color(&mut self.appearance.palette.cape, &CAPE_COLORS, delta)
            }
            CreatorField::WeaponColor => {
                cycle_color(&mut self.appearance.palette.weapon, &WEAPON_COLORS, delta)
            }
            CreatorField::Face => cycle_enum(&mut self.appearance.face, &FACE_SHAPES, delta),
            CreatorField::Eyes => cycle_enum(&mut self.appearance.eyes, &EYE_EXPRESSIONS, delta),
            CreatorField::HairStyle => {
                cycle_enum(&mut self.appearance.hair_style, &HAIR_STYLES, delta)
            }
            CreatorField::Build => cycle_enum(&mut self.appearance.build, &BUILD_TYPES, delta),
            CreatorField::Clothes => {
                cycle_enum(&mut self.appearance.clothes, &CLOTHES_STYLES, delta)
            }
            CreatorField::Helmet => cycle_enum(&mut self.appearance.helmet, &HELMET_STYLES, delta),
            CreatorField::Chainmail => {
                toggle_armor(&mut self.appearance.armors, ArmorPiece::Chainmail)
            }
            CreatorField::Leather => toggle_armor(&mut self.appearance.armors, ArmorPiece::Leather),
            CreatorField::Chestplate => {
                toggle_armor(&mut self.appearance.armors, ArmorPiece::Chestplate)
            }
            CreatorField::Shoulderpads => {
                toggle_armor(&mut self.appearance.armors, ArmorPiece::Shoulderpads)
            }
            CreatorField::Fullplate => {
                toggle_armor(&mut self.appearance.armors, ArmorPiece::Fullplate)
            }
            CreatorField::Cape => self.appearance.has_cape = !self.appearance.has_cape,
            CreatorField::Shield => self.appearance.shield_on = !self.appearance.shield_on,
            CreatorField::ShieldStyle => {
                cycle_enum(&mut self.appearance.shield_style, &SHIELD_STYLES, delta)
            }
            CreatorField::Weapon => cycle_enum(&mut self.appearance.weapon, &WEAPON_STYLES, delta),
            CreatorField::PreviewAnimation => {
                cycle_enum(&mut self.preview_animation, &PREVIEW_ANIMATIONS, delta);
                return false;
            }
            CreatorField::Offhand => {
                cycle_enum(&mut self.appearance.offhand, &OFFHAND_STYLES, delta)
            }
        }
        true
    }

    pub fn field_text(&self, index: usize) -> String {
        match CREATOR_FIELDS[index] {
            CreatorField::Gender => format!(
                "Gender: {}",
                match self.appearance.gender {
                    Gender::Male => "Male",
                    Gender::Female => "Female",
                }
            ),
            CreatorField::Skin => format!("Skin: {}", color_label(self.appearance.palette.skin)),
            CreatorField::HairColor => {
                format!("Hair Color: {}", color_label(self.appearance.palette.hair))
            }
            CreatorField::ShirtColor => {
                format!(
                    "Shirt Color: {}",
                    color_label(self.appearance.palette.shirt)
                )
            }
            CreatorField::PantsColor => {
                format!(
                    "Pants Color: {}",
                    color_label(self.appearance.palette.pants)
                )
            }
            CreatorField::BootsColor => {
                format!(
                    "Boots Color: {}",
                    color_label(self.appearance.palette.boots)
                )
            }
            CreatorField::ArmorColor => {
                format!(
                    "Armor Color: {}",
                    color_label(self.appearance.palette.armor)
                )
            }
            CreatorField::CapeColor => {
                format!("Cape Color: {}", color_label(self.appearance.palette.cape))
            }
            CreatorField::WeaponColor => {
                format!(
                    "Weapon Color: {}",
                    color_label(self.appearance.palette.weapon)
                )
            }
            CreatorField::Face => format!("Face: {:?}", self.appearance.face),
            CreatorField::Eyes => format!("Eyes: {:?}", self.appearance.eyes),
            CreatorField::HairStyle => format!("Hair: {:?}", self.appearance.hair_style),
            CreatorField::Build => format!("Build: {:?}", self.appearance.build),
            CreatorField::Clothes => format!("Clothes: {:?}", self.appearance.clothes),
            CreatorField::Helmet => format!("Helmet: {:?}", self.appearance.helmet),
            CreatorField::Chainmail => {
                format!(
                    "Armor Chainmail: {}",
                    toggle_text(self.appearance.armors.contains(&ArmorPiece::Chainmail))
                )
            }
            CreatorField::Leather => {
                format!(
                    "Armor Leather: {}",
                    toggle_text(self.appearance.armors.contains(&ArmorPiece::Leather))
                )
            }
            CreatorField::Chestplate => {
                format!(
                    "Armor Chestplate: {}",
                    toggle_text(self.appearance.armors.contains(&ArmorPiece::Chestplate))
                )
            }
            CreatorField::Shoulderpads => {
                format!(
                    "Armor Shoulders: {}",
                    toggle_text(self.appearance.armors.contains(&ArmorPiece::Shoulderpads))
                )
            }
            CreatorField::Fullplate => {
                format!(
                    "Armor Fullplate: {}",
                    toggle_text(self.appearance.armors.contains(&ArmorPiece::Fullplate))
                )
            }
            CreatorField::Cape => format!("Cape: {}", toggle_text(self.appearance.has_cape)),
            CreatorField::Shield => format!("Shield: {}", toggle_text(self.appearance.shield_on)),
            CreatorField::ShieldStyle => {
                format!("Shield Style: {:?}", self.appearance.shield_style)
            }
            CreatorField::Weapon => format!("Weapon: {:?}", self.appearance.weapon),
            CreatorField::PreviewAnimation => {
                format!("Preview: {}", self.preview_animation.label())
            }
            CreatorField::Offhand => format!("Offhand: {:?}", self.appearance.offhand),
        }
    }

    pub fn field_count(&self) -> usize {
        CREATOR_FIELDS.len()
    }

    pub fn field_color(&self, index: usize) -> Option<Color> {
        match CREATOR_FIELDS[index] {
            CreatorField::Gender => None,
            CreatorField::Skin => Some(self.appearance.palette.skin),
            CreatorField::HairColor => Some(self.appearance.palette.hair),
            CreatorField::ShirtColor => Some(self.appearance.palette.shirt),
            CreatorField::PantsColor => Some(self.appearance.palette.pants),
            CreatorField::BootsColor => Some(self.appearance.palette.boots),
            CreatorField::ArmorColor => Some(self.appearance.palette.armor),
            CreatorField::CapeColor => Some(self.appearance.palette.cape),
            CreatorField::WeaponColor => Some(self.appearance.palette.weapon),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum CreatorField {
    Gender,
    Skin,
    HairColor,
    ShirtColor,
    PantsColor,
    BootsColor,
    ArmorColor,
    CapeColor,
    WeaponColor,
    Face,
    Eyes,
    HairStyle,
    Build,
    Clothes,
    Helmet,
    Chainmail,
    Leather,
    Chestplate,
    Shoulderpads,
    Fullplate,
    Cape,
    Shield,
    ShieldStyle,
    Weapon,
    PreviewAnimation,
    Offhand,
}

const CREATOR_FIELDS: [CreatorField; 26] = [
    CreatorField::Gender,
    CreatorField::Skin,
    CreatorField::HairColor,
    CreatorField::ShirtColor,
    CreatorField::PantsColor,
    CreatorField::BootsColor,
    CreatorField::ArmorColor,
    CreatorField::CapeColor,
    CreatorField::WeaponColor,
    CreatorField::Face,
    CreatorField::Eyes,
    CreatorField::HairStyle,
    CreatorField::Build,
    CreatorField::Clothes,
    CreatorField::Helmet,
    CreatorField::Chainmail,
    CreatorField::Leather,
    CreatorField::Chestplate,
    CreatorField::Shoulderpads,
    CreatorField::Fullplate,
    CreatorField::Cape,
    CreatorField::Shield,
    CreatorField::ShieldStyle,
    CreatorField::Weapon,
    CreatorField::PreviewAnimation,
    CreatorField::Offhand,
];

pub struct HeroSheets {
    pub idle: Image,
    pub walk: Image,
    pub attack: Image,
    pub idle_armed: Image,
    pub walk_armed: Image,
    pub attack_armed: Image,
    pub idle_sword: Image,
    pub walk_sword: Image,
    pub attack_sword: Image,
}

pub fn generate_hero_sheets(appearance: &CharacterAppearance) -> HeroSheets {
    let mut sword_appearance = appearance.clone();
    sword_appearance.weapon = WeaponStyle::Sword;
    HeroSheets {
        idle: build_sheet(appearance, AnimMode::Idle, false),
        walk: build_sheet(appearance, AnimMode::Walk, false),
        attack: build_sheet(appearance, AnimMode::Attack, false),
        idle_armed: build_sheet(appearance, AnimMode::Idle, true),
        walk_armed: build_sheet(appearance, AnimMode::Walk, true),
        attack_armed: build_sheet(appearance, AnimMode::Attack, true),
        idle_sword: build_sheet(&sword_appearance, AnimMode::Idle, true),
        walk_sword: build_sheet(&sword_appearance, AnimMode::Walk, true),
        attack_sword: build_sheet(&sword_appearance, AnimMode::Attack, true),
    }
}

fn build_sheet(appearance: &CharacterAppearance, mode: AnimMode, armed: bool) -> Image {
    let mut image = Image::gen_image_color(SHEET_SIZE, SHEET_SIZE, Color::new(0.0, 0.0, 0.0, 0.0));
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
        AnimMode::Attack => [0, 0, -1, 0][frame],
    };
    let leg_swing_left = match mode {
        AnimMode::Idle => [0, 0, 0, 0][frame],
        AnimMode::Walk => [2, 0, -2, 0][frame],
        AnimMode::Attack => [0, 0, 0, 0][frame],
    };
    let leg_swing_right = match mode {
        AnimMode::Idle => [0, 0, 0, 0][frame],
        AnimMode::Walk => [-2, 0, 2, 0][frame],
        AnimMode::Attack => [0, 0, 0, 0][frame],
    };
    let arm_swing_left = match mode {
        AnimMode::Idle => [-1, -1, 0, 0][frame],
        AnimMode::Walk => [-2, 0, 2, 0][frame],
        AnimMode::Attack => [0, 0, 0, 0][frame],
    };
    let arm_swing_right = match mode {
        AnimMode::Idle => [1, 1, 0, 0][frame],
        AnimMode::Walk => [2, 0, -2, 0][frame],
        AnimMode::Attack => [0, 1, 2, 1][frame],
    };
    let side_front_leg = match mode {
        AnimMode::Idle => [0, 0, 0, 0][frame],
        AnimMode::Walk => [3, 0, -3, 0][frame],
        AnimMode::Attack => [0, 0, 0, 0][frame],
    };
    let side_back_leg = match mode {
        AnimMode::Idle => [0, 0, 0, 0][frame],
        AnimMode::Walk => [-3, 0, 3, 0][frame],
        AnimMode::Attack => [0, 0, 0, 0][frame],
    };
    let side_front_arm = match mode {
        AnimMode::Idle => [-1, -1, 0, 0][frame],
        AnimMode::Walk => [-3, 0, 3, 0][frame],
        AnimMode::Attack => [0, 1, 2, 1][frame],
    };
    let side_back_arm = match mode {
        AnimMode::Idle => [1, 1, 0, 0][frame],
        AnimMode::Walk => [3, 0, -3, 0][frame],
        AnimMode::Attack => [0, 0, 0, 0][frame],
    };
    let build = match appearance.build {
        BuildType::Normal => 0,
        BuildType::Stocky => 2,
        BuildType::Slim => -2,
    };
    let hair_dark = shade(appearance.palette.hair, 0.72);
    let shirt_dark = shade(appearance.palette.shirt, 0.72);
    let pants_dark = shade(appearance.palette.pants, 0.7);
    let boots_dark = shade(appearance.palette.boots, 0.65);

    draw_shadow(&mut px, 24, 40 + bob, 8 + build);
    if dir == 1 || dir == 2 {
        let body_left = 16;
        let body_w = 10 + build;
        let body_y = 17 + bob;
        let leg_y = 29 + bob;
        let front_leg_x = body_left + 3;
        let back_leg_x = body_left + 1;

        for y in 0..9 {
            let t = y as f32 / 8.0;
            let y_off = (side_back_leg as f32 * t).round() as i32;
            let color = if y > 5 {
                pants_dark
            } else {
                shade(appearance.palette.pants, 0.82)
            };
            rect(&mut px, back_leg_x, leg_y + y + y_off, 4, 1, color);
        }
        hline(
            &mut px,
            body_left,
            body_left + 5,
            leg_y + 8 + side_back_leg,
            boots_dark,
        );

        for y in 0..8 {
            let t = y as f32 / 7.0;
            let arm_x = back_leg_x + (side_back_arm as f32 * t).round() as i32;
            let color = if y > 5 {
                shade(appearance.palette.skin, 0.75)
            } else if appearance.clothes == ClothesStyle::Bare {
                shade(appearance.palette.skin, 0.8)
            } else {
                shade(appearance.palette.shirt, 0.72)
            };
            rect(&mut px, arm_x, body_y + 2 + y, 2, 1, color);
        }

        if appearance.has_cape {
            draw_cape_side(&mut px, appearance.palette.cape, body_left, body_y);
        }

        for y in 0..13 {
            let row_w = if y < 2 {
                body_w - 2
            } else if y < 4 {
                body_w - 1
            } else {
                body_w
            };
            let color = if appearance.clothes == ClothesStyle::Bare {
                if y > 4 {
                    shade(appearance.palette.skin, 0.75)
                } else {
                    appearance.palette.skin
                }
            } else if y > 8 {
                shirt_dark
            } else {
                appearance.palette.shirt
            };
            rect(&mut px, body_left, body_y + y, row_w, 1, color);
        }
        draw_clothes_detail_side(&mut px, appearance, body_y, body_left, body_w);
        if !appearance.armors.is_empty() {
            draw_armor_pieces(
                &mut px,
                body_left,
                body_y,
                body_w,
                &appearance.armors,
                appearance.palette.armor,
            );
        }
        hline(
            &mut px,
            body_left,
            body_left + body_w - 1,
            leg_y,
            shade(appearance.palette.boots, 1.1),
        );

        for y in 0..9 {
            let t = y as f32 / 8.0;
            let y_off = (side_front_leg as f32 * t).round() as i32;
            let color = if y > 5 {
                pants_dark
            } else {
                appearance.palette.pants
            };
            rect(&mut px, front_leg_x, leg_y + y + y_off, 4, 1, color);
        }
        let foot_y = leg_y + 8 + side_front_leg;
        hline(
            &mut px,
            body_left + 2,
            body_left + 9,
            foot_y,
            appearance.palette.boots,
        );
        pixel(&mut px, body_left + 9, foot_y + 1, boots_dark);

        for y in 0..8 {
            let t = y as f32 / 7.0;
            let arm_x = front_leg_x + (side_front_arm as f32 * t).round() as i32;
            let color = if y > 5 {
                shade(appearance.palette.skin, 0.75)
            } else if appearance.clothes == ClothesStyle::Bare {
                shade(appearance.palette.skin, 0.75)
            } else {
                appearance.palette.shirt
            };
            rect(&mut px, arm_x, body_y + 2 + y, 2, 1, color);
        }
        let hand_x = front_leg_x + side_front_arm;
        pixel(&mut px, hand_x, body_y + 9, appearance.palette.skin);
        pixel(&mut px, hand_x + 1, body_y + 9, appearance.palette.skin);

        if appearance.shield_on && mode != AnimMode::Attack {
            let back_hand_x = back_leg_x + side_back_arm;
            draw_shield_shape(
                &mut px,
                back_hand_x - 3,
                body_y + 4,
                appearance.shield_style,
                appearance.palette.armor,
            );
        }
        if armed {
            if mode == AnimMode::Attack {
                draw_attack_weapon_side(
                    &mut px,
                    appearance.weapon,
                    appearance.palette.weapon,
                    hand_x + 1,
                    body_y + 9,
                    frame,
                );
            } else {
                draw_weapon_shape(
                    &mut px,
                    hand_x + 2,
                    body_y + 9,
                    appearance.weapon,
                    appearance.palette.weapon,
                    1,
                );
            }
        }
        if appearance.offhand != OffhandStyle::None && mode != AnimMode::Attack {
            draw_offhand_shape(
                &mut px,
                hand_x - 1,
                body_y + 8,
                appearance.offhand,
                appearance.palette.weapon,
            );
        }

        rect(
            &mut px,
            body_left + 3,
            body_y - 1,
            3,
            1,
            appearance.palette.skin,
        );
        draw_head_side(&mut px, appearance, body_left, body_y - 6);
        if dir == 1 {
            mirror_horizontally(&mut px);
        }
        return px;
    }

    let body_w = 14 + build;
    let body_x = 24 - body_w / 2;
    let body_y = 17 + bob;
    let leg_y = 29 + bob;
    let arm_y = 18 + bob;
    let leg_w = (4 + (build / 2)).max(3);
    let left_leg_x = body_x + 1;
    let right_leg_x = body_x + body_w - leg_w - 1;

    for y in 0..9 {
        let t = y as f32 / 8.0;
        let left_off = (leg_swing_left as f32 * t).round() as i32;
        let right_off = (leg_swing_right as f32 * t).round() as i32;
        let color = if y > 5 {
            pants_dark
        } else {
            appearance.palette.pants
        };
        rect(&mut px, left_leg_x, leg_y + y + left_off, leg_w, 1, color);
        rect(
            &mut px,
            right_leg_x,
            leg_y + y + right_off,
            leg_w,
            1,
            pants_dark,
        );
    }
    hline(
        &mut px,
        left_leg_x - 1,
        left_leg_x + leg_w,
        leg_y + 8 + leg_swing_left,
        appearance.palette.boots,
    );
    hline(
        &mut px,
        right_leg_x - 1,
        right_leg_x + leg_w,
        leg_y + 8 + leg_swing_right,
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
            if y > 4 {
                shade(appearance.palette.skin, 0.75)
            } else {
                appearance.palette.skin
            }
        } else if y > 8 {
            shirt_dark
        } else {
            appearance.palette.shirt
        };
        rect(&mut px, row_x, body_y + y, row_w, 1, color);
    }
    if dir == 0 {
        draw_clothes_detail_front(&mut px, appearance, body_y, body_x, body_w);
    }
    if !appearance.armors.is_empty() {
        draw_armor_pieces(
            &mut px,
            body_x,
            body_y,
            body_w,
            &appearance.armors,
            appearance.palette.armor,
        );
    }
    hline(
        &mut px,
        body_x,
        body_x + body_w - 1,
        leg_y,
        shade(appearance.palette.boots, 1.1),
    );
    if dir == 0 {
        pixel(&mut px, 24, leg_y, hex("#d4a843"));
        pixel(&mut px, 23, leg_y, hex("#c49333"));
    }

    if dir == 3 {
        let arm_color = if appearance.clothes == ClothesStyle::Bare {
            appearance.palette.skin
        } else {
            shade(appearance.palette.shirt, 0.8)
        };
        for y in 0..7 {
            let color = if y > 4 {
                shade(appearance.palette.skin, 0.75)
            } else {
                arm_color
            };
            rect(&mut px, body_x - 3, arm_y + y + arm_swing_left, 2, 1, color);
            rect(
                &mut px,
                body_x + body_w + 1,
                arm_y + y + arm_swing_right,
                2,
                1,
                color,
            );
        }
    }

    if appearance.has_cape {
        draw_cape_front_back(
            &mut px,
            appearance.palette.cape,
            dir == 3,
            body_x,
            body_w,
            body_y,
        );
    }

    if dir == 0 {
        let arm_color = if appearance.clothes == ClothesStyle::Bare {
            appearance.palette.skin
        } else {
            appearance.palette.shirt
        };
        for y in 0..7 {
            let color = if y > 4 {
                shade(appearance.palette.skin, 0.75)
            } else {
                arm_color
            };
            rect(&mut px, body_x - 3, arm_y + y + arm_swing_left, 2, 1, color);
            rect(
                &mut px,
                body_x + body_w + 1,
                arm_y + y + arm_swing_right,
                2,
                1,
                color,
            );
        }
        pixel(
            &mut px,
            body_x - 3,
            arm_y + 6 + arm_swing_left,
            appearance.palette.skin,
        );
        pixel(
            &mut px,
            body_x - 2,
            arm_y + 7 + arm_swing_left,
            appearance.palette.skin,
        );
        pixel(
            &mut px,
            body_x + body_w + 2,
            arm_y + 6 + arm_swing_right,
            appearance.palette.skin,
        );
        pixel(
            &mut px,
            body_x + body_w + 1,
            arm_y + 7 + arm_swing_right,
            appearance.palette.skin,
        );
    }

    if appearance.shield_on && dir == 0 && mode != AnimMode::Attack {
        draw_shield_shape(
            &mut px,
            body_x - 5,
            arm_y + arm_swing_left,
            appearance.shield_style,
            appearance.palette.armor,
        );
    }
    if armed {
        if mode == AnimMode::Attack {
            let (hand_x, hand_y, thrust_dir) = if dir == 0 {
                (
                    body_x + body_w + 2,
                    arm_y + 6 + arm_swing_right,
                    ThrustDir::Down,
                )
            } else {
                (body_x + body_w / 2, body_y + 1, ThrustDir::Up)
            };
            draw_attack_weapon_front_back(
                &mut px,
                appearance.weapon,
                appearance.palette.weapon,
                hand_x,
                hand_y,
                thrust_dir,
                frame,
            );
        } else {
            let (wx, wy, facing) = if dir == 0 {
                (body_x + body_w + 3, arm_y + 6 + arm_swing_right, 1)
            } else {
                (body_x - 5, arm_y + arm_swing_left, -1)
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
    }
    if appearance.offhand != OffhandStyle::None && mode != AnimMode::Attack {
        let (ox, oy) = if dir == 0 {
            (body_x - 4, arm_y + 7 + arm_swing_left)
        } else {
            (body_x + body_w + 2, arm_y + 6 + arm_swing_right)
        };
        draw_offhand_shape(
            &mut px,
            ox,
            oy,
            appearance.offhand,
            appearance.palette.weapon,
        );
    }

    if dir == 0 {
        hline(&mut px, 22, 25, body_y - 1, appearance.palette.skin);
    }
    draw_head_front_back(
        &mut px,
        appearance,
        dir == 3,
        body_x + 1,
        body_y - 6,
        body_w - 2,
        hair_dark,
    );
    px
}

fn draw_clothes_detail_front(
    px: &mut [Color],
    appearance: &CharacterAppearance,
    body_y: i32,
    body_x: i32,
    body_w: i32,
) {
    match appearance.clothes {
        ClothesStyle::Robe => {
            hline(
                px,
                body_x - 1,
                body_x + body_w,
                body_y + 12,
                shade(appearance.palette.shirt, 0.55),
            );
            hline(
                px,
                body_x - 2,
                body_x + body_w + 1,
                body_y + 11,
                shade(appearance.palette.shirt, 0.5),
            );
        }
        ClothesStyle::Vest => rect(
            px,
            body_x + 2,
            body_y + 1,
            body_w - 4,
            9,
            shade(appearance.palette.shirt, 0.65),
        ),
        _ => {}
    }
}

fn draw_clothes_detail_side(
    px: &mut [Color],
    appearance: &CharacterAppearance,
    body_y: i32,
    body_x: i32,
    body_w: i32,
) {
    match appearance.clothes {
        ClothesStyle::Robe => {
            hline(
                px,
                body_x - 1,
                body_x + body_w + 1,
                body_y + 12,
                shade(appearance.palette.shirt, 0.5),
            );
            hline(
                px,
                body_x - 2,
                body_x + body_w + 2,
                body_y + 11,
                shade(appearance.palette.shirt, 0.45),
            );
        }
        ClothesStyle::Vest => rect(
            px,
            body_x + 1,
            body_y + 1,
            body_w - 2,
            9,
            shade(appearance.palette.shirt, 0.65),
        ),
        _ => {}
    }
}

fn draw_cape_front_back(
    px: &mut [Color],
    cape: Color,
    back_view: bool,
    body_x: i32,
    body_w: i32,
    body_y: i32,
) {
    let cape_dark = shade(cape, 0.7);
    let cape_deep = shade(cape, 0.5);
    if back_view {
        for y in 0..20 {
            let width = if y < 2 {
                body_w
            } else if y < 5 {
                body_w + 2
            } else if y < 12 {
                body_w + 4
            } else {
                (body_w + 4 - (y - 12) * 2).max(2)
            };
            let row_x = 24 - width / 2;
            let color = if y > 14 {
                cape_deep
            } else if y > 8 {
                cape_dark
            } else {
                cape
            };
            hline(px, row_x, row_x + width, body_y + y, color);
        }
    } else {
        for y in 2..18 {
            let hang = (y - 2).min(6) / 3;
            pixel(px, body_x - 2 - hang, body_y + y, cape);
            pixel(px, body_x - 1, body_y + y, cape_dark);
            pixel(px, body_x + body_w + 1, body_y + y, cape_dark);
            pixel(px, body_x + body_w + 1 + hang, body_y + y, cape);
        }
    }
}

fn draw_cape_side(px: &mut [Color], cape: Color, body_x: i32, body_y: i32) {
    let cape_dark = shade(cape, 0.7);
    let cape_deep = shade(cape, 0.5);
    for y in 0..22 {
        let color = if y > 16 {
            cape_deep
        } else if y > 10 {
            cape_dark
        } else {
            cape
        };
        pixel(px, body_x - 1, body_y + y, color);
        if y > 1 {
            pixel(px, body_x - 2, body_y + y, cape_dark);
        }
        if y > 3 {
            pixel(px, body_x - 3, body_y + y, cape_deep);
        }
    }
}

fn mirror_horizontally(px: &mut [Color]) {
    let width = FRAME_SIZE as usize;
    let height = FRAME_SIZE as usize;
    for y in 0..height {
        for x in 0..width / 2 {
            let left = y * width + x;
            let right = y * width + (width - 1 - x);
            px.swap(left, right);
        }
    }
}

fn draw_head_front_back(
    px: &mut [Color],
    appearance: &CharacterAppearance,
    back: bool,
    x: i32,
    y: i32,
    w: i32,
    _hair_dark: Color,
) {
    if back {
        let rows: &[(i32, i32)] = match appearance.face {
            FaceShape::Round => &[
                (3, 3),
                (3, 3),
                (3, 3),
                (2, 2),
                (1, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (1, 1),
                (1, 1),
                (2, 2),
                (3, 3),
            ],
            FaceShape::Square => &[
                (3, 3),
                (3, 3),
                (3, 3),
                (1, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (1, 1),
                (2, 2),
            ],
            FaceShape::Narrow => &[
                (3, 3),
                (3, 3),
                (3, 3),
                (2, 2),
                (1, 1),
                (1, 1),
                (1, 1),
                (1, 1),
                (1, 1),
                (1, 1),
                (2, 2),
                (3, 3),
            ],
        };
        for (row, (li, ri)) in rows.iter().enumerate() {
            let x1 = x + *li;
            let x2 = x + w - 1 - *ri;
            if x2 >= x1 {
                hline(
                    px,
                    x1,
                    x2,
                    y + row as i32,
                    if appearance.hair_style == HairStyle::Bald {
                        appearance.palette.skin
                    } else {
                        appearance.palette.hair
                    },
                );
            }
        }
        if appearance.hair_style != HairStyle::Bald {
            for row in 10..12 {
                let (li, ri) = rows[row];
                let x1 = x + li + 3;
                let x2 = x + w - 1 - ri - 3;
                if x2 >= x1 {
                    hline(px, x1, x2, y + row as i32, appearance.palette.skin);
                }
            }
        }
        pixel(px, x - 1, y + 5, appearance.palette.skin);
        pixel(px, x + w, y + 5, appearance.palette.skin);
        draw_helmet_shape(px, appearance.helmet, y, x, w, appearance.palette.armor);
        return;
    }

    let face_w = match appearance.face {
        FaceShape::Narrow => w - 2,
        FaceShape::Square => w + 1,
        FaceShape::Round => w,
    };
    let face_x = 24 - face_w / 2;
    let rows: &[(i32, i32)] = match appearance.face {
        FaceShape::Round => &[
            (3, 3),
            (3, 3),
            (3, 3),
            (2, 2),
            (1, 1),
            (0, 0),
            (0, 0),
            (0, 0),
            (1, 1),
            (1, 1),
            (2, 2),
            (3, 3),
        ],
        FaceShape::Square => &[
            (3, 3),
            (3, 3),
            (3, 3),
            (1, 1),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (1, 1),
            (2, 2),
        ],
        FaceShape::Narrow => &[
            (3, 3),
            (3, 3),
            (3, 3),
            (2, 2),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (2, 2),
            (3, 3),
        ],
    };
    for row in 3..12 {
        let (li, ri) = rows[row];
        let x1 = face_x + li;
        let x2 = face_x + face_w - 1 - ri;
        if x2 >= x1 {
            hline(px, x1, x2, y + row as i32, appearance.palette.skin);
        }
    }
    pixel(px, face_x - 1, y + 5, appearance.palette.skin);
    pixel(px, face_x + face_w, y + 5, appearance.palette.skin);
    pixel(px, face_x + 1, y + 8, shade(appearance.palette.skin, 0.75));
    pixel(
        px,
        face_x + face_w - 2,
        y + 8,
        shade(appearance.palette.skin, 0.75),
    );
    draw_eyes_front(px, appearance, y, face_x, face_w);
    pixel(
        px,
        face_x + face_w / 2,
        y + 8,
        shade(appearance.palette.skin, 0.75),
    );
    draw_hair_front(
        px,
        appearance.hair_style,
        y,
        face_x,
        face_w,
        appearance.palette.hair,
    );
    draw_helmet_shape(
        px,
        appearance.helmet,
        y,
        face_x,
        face_w,
        appearance.palette.armor,
    );
}

fn draw_head_side(px: &mut [Color], appearance: &CharacterAppearance, x: i32, y: i32) {
    let skull_rows = [
        (2, 7),
        (1, 9),
        (0, 11),
        (0, 11),
        (0, 11),
        (0, 11),
        (0, 11),
        (0, 11),
        (0, 10),
        (1, 9),
        (2, 7),
    ];
    for (row, (ox, width)) in skull_rows.iter().enumerate() {
        rect(
            px,
            x + *ox,
            y + row as i32,
            *width,
            1,
            appearance.palette.skin,
        );
    }
    pixel(px, x + 9, y + 7, shade(appearance.palette.skin, 0.75));
    pixel(px, x + 9, y + 8, shade(appearance.palette.skin, 0.75));
    pixel(px, x + 8, y + 9, shade(appearance.palette.skin, 0.75));
    pixel(px, x, y + 5, appearance.palette.skin);
    pixel(px, x, y + 6, appearance.palette.skin);
    pixel(px, x, y + 7, appearance.palette.skin);
    pixel(px, x - 1, y + 6, appearance.palette.skin);
    pixel(px, x + 10, y + 6, shade(appearance.palette.skin, 0.75));
    pixel(px, x + 11, y + 7, shade(appearance.palette.skin, 0.75));
    draw_face_side(px, appearance, y, x);
    draw_hair(
        px,
        appearance.hair_style,
        x,
        y,
        11,
        appearance.palette.hair,
        true,
    );
    draw_helmet_shape(px, appearance.helmet, y, x, 11, appearance.palette.armor);
}

fn draw_eyes_front(
    px: &mut [Color],
    appearance: &CharacterAppearance,
    head_y: i32,
    face_x: i32,
    face_w: i32,
) {
    let lx = face_x + ((face_w as f32 * 0.26).floor() as i32);
    let rx = face_x + ((face_w as f32 * 0.60).floor() as i32);
    let mx = face_x + face_w / 2;
    let mouth_y = head_y + 10;
    let brow = shade(appearance.palette.skin, 0.55);
    let skin_shadow = shade(appearance.palette.skin, 0.75);
    let eye = hex("#1a1a2a");
    let mouth = hex("#cc2222");

    match appearance.eyes {
        EyeExpression::Normal => {
            hline(px, lx - 1, lx + 1, head_y + 5, brow);
            hline(px, rx - 1, rx + 1, head_y + 5, brow);
            pixel(px, lx, head_y + 6, WHITE);
            pixel(px, lx, head_y + 7, eye);
            pixel(px, lx + 1, head_y + 7, eye);
            pixel(px, rx, head_y + 6, WHITE);
            pixel(px, rx, head_y + 7, eye);
            pixel(px, rx + 1, head_y + 7, eye);
            hline(px, mx - 1, mx + 1, mouth_y, skin_shadow);
        }
        EyeExpression::Angry => {
            pixel(px, lx - 1, head_y + 5, brow);
            pixel(px, lx, head_y + 4, brow);
            pixel(px, lx + 1, head_y + 4, brow);
            pixel(px, rx - 1, head_y + 4, brow);
            pixel(px, rx, head_y + 4, brow);
            pixel(px, rx + 1, head_y + 5, brow);
            hline(px, lx, lx + 1, head_y + 7, eye);
            hline(px, rx, rx + 1, head_y + 7, eye);
            hline(px, mx - 2, mx + 2, mouth_y, skin_shadow);
            pixel(px, mx - 1, mouth_y, eye);
            pixel(px, mx, mouth_y, eye);
            pixel(px, mx + 1, mouth_y, eye);
        }
        EyeExpression::Sad => {
            pixel(px, lx - 1, head_y + 4, brow);
            pixel(px, lx, head_y + 4, brow);
            pixel(px, lx + 1, head_y + 5, brow);
            pixel(px, rx - 1, head_y + 5, brow);
            pixel(px, rx, head_y + 4, brow);
            pixel(px, rx + 1, head_y + 4, brow);
            pixel(px, lx, head_y + 6, WHITE);
            pixel(px, lx, head_y + 7, eye);
            pixel(px, lx + 1, head_y + 7, eye);
            pixel(px, rx, head_y + 6, WHITE);
            pixel(px, rx, head_y + 7, eye);
            pixel(px, rx + 1, head_y + 7, eye);
            hline(px, mx - 1, mx + 1, mouth_y, skin_shadow);
            pixel(px, mx - 2, mouth_y + 1, skin_shadow);
            pixel(px, mx + 2, mouth_y + 1, skin_shadow);
            pixel(px, lx + 1, head_y + 9, hex("#4499ff"));
            pixel(px, lx + 1, head_y + 10, hex("#2277dd"));
        }
        EyeExpression::Happy => {
            hline(px, lx - 1, lx + 1, head_y + 5, brow);
            hline(px, rx - 1, rx + 1, head_y + 5, brow);
            hline(px, lx, lx + 2, head_y + 7, eye);
            pixel(px, lx, head_y + 6, WHITE);
            pixel(px, lx + 2, head_y + 6, WHITE);
            hline(px, rx, rx + 2, head_y + 7, eye);
            pixel(px, rx, head_y + 6, WHITE);
            pixel(px, rx + 2, head_y + 6, WHITE);
            hline(px, mx - 1, mx + 1, mouth_y, mouth);
            pixel(px, mx - 2, mouth_y - 1, skin_shadow);
            pixel(px, mx + 2, mouth_y - 1, skin_shadow);
        }
        _ => {
            hline(px, lx - 1, lx + 1, head_y + 5, brow);
            hline(px, rx - 1, rx + 1, head_y + 5, brow);
            pixel(px, lx, head_y + 6, WHITE);
            pixel(px, lx, head_y + 7, eye);
            pixel(px, lx + 1, head_y + 7, eye);
            pixel(px, rx, head_y + 6, WHITE);
            pixel(px, rx, head_y + 7, eye);
            pixel(px, rx + 1, head_y + 7, eye);
            pixel(px, mx, mouth_y, mouth);
        }
    }
}

fn draw_face_side(px: &mut [Color], appearance: &CharacterAppearance, head_y: i32, head_x: i32) {
    let skin_shadow = shade(appearance.palette.skin, 0.75);
    let brow = shade(appearance.palette.skin, 0.55);
    let eye = hex("#1a1a2a");
    let mouth = hex("#cc2222");
    match appearance.eyes {
        EyeExpression::Angry | EyeExpression::Straining => {
            pixel(px, head_x + 6, head_y + 3, brow);
            pixel(px, head_x + 7, head_y + 4, brow);
            pixel(px, head_x + 8, head_y + 4, brow);
            hline(px, head_x + 6, head_x + 8, head_y + 5, eye);
        }
        EyeExpression::Sad => {
            pixel(px, head_x + 6, head_y + 3, brow);
            pixel(px, head_x + 7, head_y + 3, brow);
            pixel(px, head_x + 8, head_y + 4, brow);
            pixel(px, head_x + 7, head_y + 4, WHITE);
            pixel(px, head_x + 8, head_y + 4, WHITE);
            pixel(px, head_x + 7, head_y + 5, eye);
            pixel(px, head_x + 8, head_y + 5, eye);
            pixel(px, head_x + 7, head_y + 7, hex("#4499ff"));
            pixel(px, head_x + 7, head_y + 8, hex("#2277cc"));
        }
        EyeExpression::Happy => {
            pixel(px, head_x + 6, head_y + 5, eye);
            pixel(px, head_x + 7, head_y + 4, eye);
            pixel(px, head_x + 8, head_y + 5, eye);
            pixel(px, head_x + 6, head_y + 6, WHITE);
            pixel(px, head_x + 8, head_y + 6, WHITE);
        }
        _ => {
            pixel(px, head_x + 6, head_y + 3, brow);
            pixel(px, head_x + 7, head_y + 3, brow);
            pixel(px, head_x + 8, head_y + 3, brow);
            pixel(px, head_x + 7, head_y + 4, WHITE);
            pixel(px, head_x + 8, head_y + 4, WHITE);
            pixel(px, head_x + 7, head_y + 5, eye);
            pixel(px, head_x + 8, head_y + 5, eye);
        }
    }
    match appearance.eyes {
        EyeExpression::Happy => {
            pixel(px, head_x + 7, head_y + 8, skin_shadow);
            pixel(px, head_x + 8, head_y + 9, mouth);
            pixel(px, head_x + 9, head_y + 8, skin_shadow);
        }
        EyeExpression::Sad | EyeExpression::Tired => {
            pixel(px, head_x + 7, head_y + 9, skin_shadow);
            pixel(px, head_x + 8, head_y + 8, skin_shadow);
            pixel(px, head_x + 9, head_y + 9, skin_shadow);
        }
        EyeExpression::Surprised => {
            pixel(px, head_x + 8, head_y + 8, mouth);
            pixel(px, head_x + 8, head_y + 9, mouth);
            pixel(px, head_x + 7, head_y + 8, skin_shadow);
            pixel(px, head_x + 9, head_y + 8, skin_shadow);
        }
        EyeExpression::Angry | EyeExpression::Straining => {
            hline(px, head_x + 6, head_x + 9, head_y + 8, skin_shadow);
            pixel(px, head_x + 7, head_y + 8, eye);
            pixel(px, head_x + 8, head_y + 8, eye);
        }
        _ => {
            hline(px, head_x + 7, head_x + 9, head_y + 8, skin_shadow);
        }
    }
}

fn draw_hair_front(
    px: &mut [Color],
    style: HairStyle,
    head_y: i32,
    face_x: i32,
    face_w: i32,
    color: Color,
) {
    if style == HairStyle::Bald {
        return;
    }
    hline(px, face_x, face_x + face_w - 1, head_y, color);
    hline(px, face_x - 1, face_x + face_w, head_y + 1, color);
    pixel(px, face_x - 1, head_y + 2, color);
    pixel(px, face_x + face_w, head_y + 2, color);
    pixel(px, face_x - 1, head_y + 3, color);
    pixel(px, face_x + face_w, head_y + 3, color);
    match style {
        HairStyle::Short => {
            hline(px, face_x, face_x + face_w - 1, head_y + 2, color);
            hline(px, face_x + 1, face_x + face_w / 2, head_y + 3, color);
        }
        HairStyle::Long => {
            for dy in 0..5 {
                hline(px, face_x - 1, face_x + face_w, head_y + dy, color);
            }
            vline(px, face_x - 1, head_y + 3, head_y + 15, color);
            vline(px, face_x + face_w, head_y + 3, head_y + 15, color);
        }
        HairStyle::Spiky => {
            hline(px, face_x, face_x + face_w - 1, head_y + 2, color);
            pixel(px, face_x + 1, head_y - 1, color);
            pixel(px, face_x + 3, head_y - 2, color);
            pixel(px, face_x + face_w / 2, head_y - 3, color);
            pixel(px, face_x + face_w / 2 + 1, head_y - 2, color);
            pixel(px, face_x + face_w - 3, head_y - 2, color);
            pixel(px, face_x + face_w - 1, head_y - 1, color);
            hline(px, face_x, face_x + face_w - 1, head_y - 1, color);
        }
        HairStyle::Ponytail => {
            hline(px, face_x, face_x + face_w - 1, head_y + 2, color);
            hline(px, face_x + 1, face_x + face_w / 2, head_y + 3, color);
            vline(px, face_x + face_w - 2, head_y + 2, head_y + 16, color);
            vline(px, face_x + face_w - 1, head_y + 2, head_y + 14, color);
            pixel(px, face_x + face_w - 3, head_y + 14, color);
            pixel(px, face_x + face_w - 3, head_y + 15, color);
        }
        HairStyle::Mohawk => {
            let mid = face_x + face_w / 2;
            for dy in -3..3 {
                pixel(px, mid, head_y + dy, color);
            }
            pixel(px, mid - 1, head_y, color);
            pixel(px, mid + 1, head_y, color);
            pixel(px, mid - 1, head_y + 1, color);
            pixel(px, mid + 1, head_y + 1, color);
        }
        HairStyle::Bald => {}
    }
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
        HairStyle::Ponytail => {
            rect(px, x, y, w, 4, color);
            if back {
                rect(px, x + w - 2, y + 4, 2, 8, color);
                pixel(px, x + w - 3, y + 10, color);
            }
        }
        HairStyle::Mohawk => {
            let mid = x + w / 2;
            for dy in -2..4 {
                pixel(px, mid, y + dy, color);
            }
            pixel(px, mid - 1, y + 1, color);
            pixel(px, mid + 1, y + 1, color);
        }
        HairStyle::Bald => {}
    }
}

fn draw_helmet_shape(px: &mut [Color], style: HelmetStyle, x: i32, y: i32, w: i32, color: Color) {
    match style {
        HelmetStyle::None => {}
        HelmetStyle::Iron => rect(px, x, y, w, 4, color),
        HelmetStyle::Greathelm => rect(px, x - 1, y - 1, w + 2, 12, color),
        HelmetStyle::Pointy => {
            for row in 0..6 {
                let row_w = row + 1;
                hline(
                    px,
                    x + w / 2 - row_w / 2,
                    x + w / 2 + row_w / 2,
                    y - 6 + row,
                    shade(color, 0.9),
                );
            }
        }
        HelmetStyle::Hood => rect(px, x - 1, y, w + 2, 5, shade(color, 0.82)),
        HelmetStyle::Cap => {
            hline(px, x, x + w - 1, y, shade(color, 0.9));
            hline(px, x - 1, x + w, y + 1, shade(color, 0.8));
        }
        HelmetStyle::Crown => {
            rect(px, x, y + 2, w, 2, color);
            for point in 0..3 {
                rect(px, x + 1 + point * 4, y, 2, 2, shade(color, 1.2));
            }
        }
        HelmetStyle::Bandana => {
            hline(px, x, x + w - 1, y + 3, shade(color, 0.8));
            hline(px, x - 1, x + w, y + 4, shade(color, 0.75));
        }
        HelmetStyle::Horned => {
            hline(px, x, x + w - 1, y, color);
            hline(px, x, x + w - 1, y + 1, shade(color, 0.8));
            vline(px, x + 1, y - 3, y, shade(color, 1.2));
            vline(px, x + w - 2, y - 3, y, shade(color, 1.2));
        }
    }
}

fn draw_armor_pieces(
    px: &mut [Color],
    x: i32,
    y: i32,
    w: i32,
    armors: &HashSet<ArmorPiece>,
    color: Color,
) {
    for armor in ARMOR_PIECES {
        if armors.contains(&armor) {
            draw_armor_piece(px, x, y, w, armor, color);
        }
    }
}

fn draw_armor_piece(px: &mut [Color], x: i32, y: i32, w: i32, armor: ArmorPiece, color: Color) {
    match armor {
        ArmorPiece::Leather => {
            rect(px, x + 1, y + 1, w - 2, 10, color);
            hline(px, x + 2, x + w - 3, y + 3, shade(color, 0.4));
            hline(px, x + 2, x + w - 3, y + 6, shade(color, 0.4));
            hline(px, x + 2, x + w - 3, y + 9, shade(color, 0.4));
        }
        ArmorPiece::Chainmail => {
            for row in 1..11 {
                for col in x + 1..x + w - 1 {
                    pixel(
                        px,
                        col,
                        y + row,
                        if (col + row) % 2 == 0 {
                            color
                        } else {
                            shade(color, 0.65)
                        },
                    );
                }
            }
        }
        ArmorPiece::Chestplate => {
            rect(px, x + 1, y + 1, w - 2, 9, color);
            hline(px, x + 1, x + w - 2, y + 1, shade(color, 1.2));
            hline(px, x + 2, x + w - 3, y + 9, shade(color, 0.6));
        }
        ArmorPiece::Shoulderpads => {
            rect(px, x - 3, y, 5, 5, color);
            rect(px, x + w - 2, y, 5, 5, color);
        }
        ArmorPiece::Fullplate => {
            rect(px, x, y, w, 12, shade(color, 0.95));
            hline(px, x, x + w - 1, y, shade(color, 1.2));
            hline(px, x, x + w - 1, y + 7, shade(color, 0.6));
            if w > 4 {
                vline(px, x + w / 2, y + 2, y + 10, shade(color, 0.6));
            }
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
        ShieldStyle::Buckler => rect(px, x, y, 4, 7, color),
    }
}

fn draw_weapon_shape(
    px: &mut [Color],
    x: i32,
    y: i32,
    weapon: WeaponStyle,
    color: Color,
    facing: i32,
) {
    let dark = shade(color, 0.6);
    match weapon {
        WeaponStyle::None => {}
        WeaponStyle::Sword => {
            vline(px, x, y - 6, y + 4, color);
            hline(px, x - 2, x + 2, y + 3, dark);
        }
        WeaponStyle::Greatsword => {
            rect(px, x - 1, y - 9, 3, 14, color);
            hline(px, x - 3, x + 3, y + 2, dark);
        }
        WeaponStyle::Axe => {
            vline(px, x, y - 4, y + 5, dark);
            rect(px, x + facing, y - 3, 3, 4, color);
        }
        WeaponStyle::Hammer => {
            vline(px, x, y - 7, y + 3, dark);
            rect(px, x - 2, y - 11, 7, 5, color);
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
        WeaponStyle::Wand => {
            vline(px, x, y - 8, y + 1, dark);
            pixel(px, x, y - 10, color);
            pixel(px, x - 1, y - 9, color);
            pixel(px, x + 1, y - 9, color);
        }
        WeaponStyle::Bow => {
            vline(px, x - 1, y - 6, y + 5, dark);
            vline(px, x + 2, y - 7, y + 7, WHITE);
        }
        WeaponStyle::Crossbow => {
            vline(px, x, y - 3, y + 4, dark);
            hline(px, x - 5, x + 6, y - 2, shade(color, 0.8));
        }
        WeaponStyle::Dagger => {
            vline(px, x, y - 4, y, color);
            pixel(px, x - 1, y - 3, color);
        }
        WeaponStyle::Boomerang => {
            hline(px, x, x + 3, y - 4, color);
            hline(px, x - 2, x + 1, y - 1, shade(color, 0.8));
        }
        WeaponStyle::Scythe => {
            vline(px, x, y - 10, y + 4, dark);
            hline(px, x - 5, x + 2, y - 12, color);
        }
        WeaponStyle::Flail => {
            vline(px, x, y - 3, y + 2, dark);
            pixel(px, x, y - 4, shade(color, 0.8));
            pixel(px, x + 1, y - 5, shade(color, 0.8));
            rect(px, x - 1, y - 12, 4, 4, color);
        }
    }
}

#[derive(Clone, Copy)]
enum ThrustDir {
    Up,
    Down,
    Right,
}

fn draw_attack_weapon_front_back(
    px: &mut [Color],
    weapon: WeaponStyle,
    color: Color,
    hand_x: i32,
    hand_y: i32,
    dir: ThrustDir,
    frame: usize,
) {
    draw_thrust_weapon(px, hand_x, hand_y, weapon, color, dir, frame);
}

fn draw_attack_weapon_side(
    px: &mut [Color],
    weapon: WeaponStyle,
    color: Color,
    hand_x: i32,
    hand_y: i32,
    frame: usize,
) {
    draw_thrust_weapon(px, hand_x, hand_y, weapon, color, ThrustDir::Right, frame);
}

fn draw_thrust_weapon(
    px: &mut [Color],
    hand_x: i32,
    hand_y: i32,
    weapon: WeaponStyle,
    color: Color,
    dir: ThrustDir,
    frame: usize,
) {
    match weapon {
        WeaponStyle::None => {}
        WeaponStyle::Sword | WeaponStyle::Greatsword | WeaponStyle::Dagger => {
            draw_thrust_blade(px, hand_x, hand_y, weapon, color, dir, frame)
        }
        WeaponStyle::Spear => draw_thrust_spear(px, hand_x, hand_y, color, dir, frame),
        WeaponStyle::Staff | WeaponStyle::Wand => {
            draw_thrust_staff(px, hand_x, hand_y, weapon, color, dir, frame)
        }
        _ => draw_thrust_weighted_weapon(px, hand_x, hand_y, weapon, color, dir, frame),
    }
}

fn draw_thrust_blade(
    px: &mut [Color],
    hand_x: i32,
    hand_y: i32,
    weapon: WeaponStyle,
    color: Color,
    dir: ThrustDir,
    frame: usize,
) {
    let (dx, dy) = thrust_vector(dir);
    let (pxv, pyv) = thrust_perp(dir);
    let raw_tip = (
        hand_x + dx * attack_reach(weapon, frame),
        hand_y + dy * attack_reach(weapon, frame),
    );
    let (tip_x, tip_y) = if weapon == WeaponStyle::Dagger {
        midpoint((hand_x, hand_y), raw_tip)
    } else {
        raw_tip
    };
    let dark = shade(color, 0.55);
    let handle_end = (hand_x - dx * 2, hand_y - dy * 2);

    line(px, hand_x, hand_y, tip_x, tip_y, color);
    if weapon == WeaponStyle::Greatsword {
        line(
            px,
            hand_x + pxv,
            hand_y + pyv,
            tip_x + pxv,
            tip_y + pyv,
            shade(color, 0.85),
        );
    }
    line(px, hand_x, hand_y, handle_end.0, handle_end.1, dark);
    line(
        px,
        hand_x - pxv * 2,
        hand_y - pyv * 2,
        hand_x + pxv * 2,
        hand_y + pyv * 2,
        dark,
    );
}

fn draw_thrust_spear(
    px: &mut [Color],
    hand_x: i32,
    hand_y: i32,
    color: Color,
    dir: ThrustDir,
    frame: usize,
) {
    let (dx, dy) = thrust_vector(dir);
    let (pxv, pyv) = thrust_perp(dir);
    let dark = shade(color, 0.55);
    let reach = attack_reach(WeaponStyle::Spear, frame);
    let tip_x = hand_x + dx * reach;
    let tip_y = hand_y + dy * reach;
    let butt_x = hand_x - dx * 3;
    let butt_y = hand_y - dy * 3;
    line(px, butt_x, butt_y, tip_x, tip_y, dark);
    pixel(px, tip_x, tip_y, color);
    pixel(px, tip_x - dx + pxv, tip_y - dy + pyv, color);
    pixel(px, tip_x - dx - pxv, tip_y - dy - pyv, color);
}

fn draw_thrust_staff(
    px: &mut [Color],
    hand_x: i32,
    hand_y: i32,
    weapon: WeaponStyle,
    color: Color,
    dir: ThrustDir,
    frame: usize,
) {
    let (dx, dy) = thrust_vector(dir);
    let (pxv, pyv) = thrust_perp(dir);
    let dark = shade(color, 0.55);
    let reach = attack_reach(weapon, frame);
    let tip_x = hand_x + dx * reach;
    let tip_y = hand_y + dy * reach;
    let butt_x = hand_x - dx * 3;
    let butt_y = hand_y - dy * 3;
    line(px, butt_x, butt_y, tip_x, tip_y, dark);
    match weapon {
        WeaponStyle::Staff => {
            rect(px, tip_x - pxv, tip_y - pyv, 1 + pxv.abs() * 2, 1 + pyv.abs() * 2, color);
            pixel(px, tip_x + dx, tip_y + dy, shade(color, 1.2));
        }
        WeaponStyle::Wand => {
            pixel(px, tip_x, tip_y, color);
            pixel(px, tip_x + pxv, tip_y + pyv, shade(color, 1.3));
            pixel(px, tip_x - pxv, tip_y - pyv, shade(color, 1.3));
        }
        _ => {}
    }
}

fn draw_thrust_weighted_weapon(
    px: &mut [Color],
    hand_x: i32,
    hand_y: i32,
    weapon: WeaponStyle,
    color: Color,
    dir: ThrustDir,
    frame: usize,
) {
    let (dx, dy) = thrust_vector(dir);
    let (pxv, pyv) = thrust_perp(dir);
    let dark = shade(color, 0.55);
    let reach = attack_reach(weapon, frame);
    let head_x = hand_x + dx * reach;
    let head_y = hand_y + dy * reach;
    let butt_x = hand_x - dx * 3;
    let butt_y = hand_y - dy * 3;
    line(px, butt_x, butt_y, head_x, head_y, dark);

    match weapon {
        WeaponStyle::Axe => {
            line(
                px,
                head_x,
                head_y,
                head_x + pxv * 3 - dx,
                head_y + pyv * 3 - dy,
                color,
            );
            line(
                px,
                head_x,
                head_y,
                head_x + pxv * 2,
                head_y + pyv * 2,
                shade(color, 0.85),
            );
        }
        WeaponStyle::Hammer => {
            line(
                px,
                head_x - pxv * 2,
                head_y - pyv * 2,
                head_x + pxv * 2,
                head_y + pyv * 2,
                color,
            );
            line(
                px,
                head_x - pxv * 2 + dx,
                head_y - pyv * 2 + dy,
                head_x + pxv * 2 + dx,
                head_y + pyv * 2 + dy,
                color,
            );
        }
        WeaponStyle::Bow => {
            line(
                px,
                head_x - pxv * 2,
                head_y - pyv * 2,
                head_x + pxv * 2,
                head_y + pyv * 2,
                color,
            );
            line(
                px,
                head_x - pxv * 2,
                head_y - pyv * 2,
                head_x + pxv * 2,
                head_y + pyv * 2,
                WHITE,
            );
        }
        WeaponStyle::Crossbow => {
            line(
                px,
                head_x - pxv * 3,
                head_y - pyv * 3,
                head_x + pxv * 3,
                head_y + pyv * 3,
                shade(color, 0.85),
            );
        }
        WeaponStyle::Boomerang => {
            line(px, head_x, head_y, head_x + pxv * 2, head_y + pyv * 2, color);
            line(px, head_x, head_y, head_x - pxv * 2, head_y - pyv * 2, shade(color, 0.85));
        }
        WeaponStyle::Scythe => {
            line(
                px,
                head_x,
                head_y,
                head_x + pxv * 3 - dx,
                head_y + pyv * 3 - dy,
                color,
            );
            line(
                px,
                head_x + pxv * 3 - dx,
                head_y + pyv * 3 - dy,
                head_x + pxv * 2 - dx * 2,
                head_y + pyv * 2 - dy * 2,
                color,
            );
        }
        WeaponStyle::Flail => {
            let chain_x = hand_x + dx * (reach - 2);
            let chain_y = hand_y + dy * (reach - 2);
            line(px, hand_x, hand_y, chain_x, chain_y, dark);
            rect(px, head_x - 1, head_y - 1, 3, 3, color);
        }
        _ => {
            draw_weapon_shape(px, hand_x, hand_y, weapon, color, if dx < 0 { -1 } else { 1 });
        }
    }
}

fn midpoint(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    ((a.0 + b.0) / 2, (a.1 + b.1) / 2)
}

fn thrust_vector(dir: ThrustDir) -> (i32, i32) {
    match dir {
        ThrustDir::Up => (0, -1),
        ThrustDir::Down => (0, 1),
        ThrustDir::Right => (1, 0),
    }
}

fn thrust_perp(dir: ThrustDir) -> (i32, i32) {
    match dir {
        ThrustDir::Up | ThrustDir::Down => (1, 0),
        ThrustDir::Right => (0, 1),
    }
}

fn attack_reach(weapon: WeaponStyle, frame: usize) -> i32 {
    let base = match frame {
        0 => 4,
        1 => 7,
        2 => 10,
        _ => 6,
    };
    match weapon {
        WeaponStyle::Dagger | WeaponStyle::Wand => base - 2,
        WeaponStyle::Greatsword | WeaponStyle::Spear | WeaponStyle::Scythe => base + 3,
        WeaponStyle::Staff | WeaponStyle::Bow | WeaponStyle::Crossbow => base + 1,
        WeaponStyle::Hammer | WeaponStyle::Flail => base + 2,
        _ => base,
    }
}

fn draw_offhand_shape(px: &mut [Color], x: i32, y: i32, offhand: OffhandStyle, color: Color) {
    match offhand {
        OffhandStyle::None => {}
        OffhandStyle::Torch => {
            vline(px, x, y - 4, y, shade(hex("#8B6914"), 1.0));
            pixel(px, x, y - 5, hex("#ff8800"));
            pixel(px, x - 1, y - 5, hex("#ffcc00"));
            pixel(px, x + 1, y - 5, hex("#ff4400"));
            pixel(px, x, y - 6, hex("#ffee88"));
        }
        OffhandStyle::Orb => {
            rect(px, x - 1, y - 3, 3, 3, shade(color, 0.8));
            pixel(px, x, y - 4, shade(color, 1.4));
            pixel(px, x - 1, y - 2, shade(color, 1.3));
        }
        OffhandStyle::Dagger => {
            vline(px, x, y - 4, y, color);
            pixel(px, x - 1, y - 3, color);
        }
        OffhandStyle::Potion => {
            rect(px, x - 1, y - 3, 3, 4, hex("#2266ff"));
            pixel(px, x, y - 4, hex("#aaddff"));
            pixel(px, x - 1, y - 2, hex("#4488ff"));
        }
        OffhandStyle::Book => {
            rect(px, x - 2, y - 4, 4, 5, hex("#8B6914"));
            rect(px, x - 1, y - 3, 2, 3, hex("#f5e6c8"));
            pixel(px, x - 2, y - 4, shade(hex("#8B6914"), 1.3));
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

fn line(px: &mut [Color], x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
    let mut x = x1;
    let mut y = y1;
    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        pixel(px, x, y, color);
        if x == x2 && y == y2 {
            break;
        }
        let e2 = err * 2;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
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

fn toggle_armor(armors: &mut HashSet<ArmorPiece>, armor: ArmorPiece) {
    if !armors.insert(armor) {
        armors.remove(&armor);
    }
}

fn random_armors() -> HashSet<ArmorPiece> {
    let mut armors = HashSet::new();
    for armor in ARMOR_PIECES {
        if gen_range(0, 2) == 1 {
            armors.insert(armor);
        }
    }
    armors
}

fn almost_same_color(a: Color, b: Color) -> bool {
    (a.r - b.r).abs() < 0.001 && (a.g - b.g).abs() < 0.001 && (a.b - b.b).abs() < 0.001
}

fn toggle_text(value: bool) -> &'static str {
    if value { "On" } else { "Off" }
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

const FACE_SHAPES: [FaceShape; 3] = [FaceShape::Round, FaceShape::Square, FaceShape::Narrow];
const EYE_EXPRESSIONS: [EyeExpression; 10] = [
    EyeExpression::Normal,
    EyeExpression::Angry,
    EyeExpression::Sad,
    EyeExpression::Happy,
    EyeExpression::Surprised,
    EyeExpression::Confused,
    EyeExpression::Thinking,
    EyeExpression::Tired,
    EyeExpression::Straining,
    EyeExpression::Wide,
];
const HAIR_STYLES: [HairStyle; 6] = [
    HairStyle::Short,
    HairStyle::Long,
    HairStyle::Spiky,
    HairStyle::Bald,
    HairStyle::Ponytail,
    HairStyle::Mohawk,
];
const BUILD_TYPES: [BuildType; 3] = [BuildType::Normal, BuildType::Stocky, BuildType::Slim];
const CLOTHES_STYLES: [ClothesStyle; 5] = [
    ClothesStyle::Tunic,
    ClothesStyle::Robe,
    ClothesStyle::Vest,
    ClothesStyle::Cloak,
    ClothesStyle::Bare,
];
const HELMET_STYLES: [HelmetStyle; 9] = [
    HelmetStyle::None,
    HelmetStyle::Iron,
    HelmetStyle::Greathelm,
    HelmetStyle::Pointy,
    HelmetStyle::Hood,
    HelmetStyle::Cap,
    HelmetStyle::Crown,
    HelmetStyle::Bandana,
    HelmetStyle::Horned,
];
const ARMOR_PIECES: [ArmorPiece; 5] = [
    ArmorPiece::Chainmail,
    ArmorPiece::Leather,
    ArmorPiece::Chestplate,
    ArmorPiece::Shoulderpads,
    ArmorPiece::Fullplate,
];
const SHIELD_STYLES: [ShieldStyle; 4] = [
    ShieldStyle::Round,
    ShieldStyle::Kite,
    ShieldStyle::Tower,
    ShieldStyle::Buckler,
];
const PREVIEW_ANIMATIONS: [PreviewAnimation; 12] = [
    PreviewAnimation::IdleDown,
    PreviewAnimation::IdleLeft,
    PreviewAnimation::IdleRight,
    PreviewAnimation::IdleUp,
    PreviewAnimation::WalkDown,
    PreviewAnimation::WalkLeft,
    PreviewAnimation::WalkRight,
    PreviewAnimation::WalkUp,
    PreviewAnimation::SwingDown,
    PreviewAnimation::SwingLeft,
    PreviewAnimation::SwingRight,
    PreviewAnimation::SwingUp,
];
const WEAPON_STYLES: [WeaponStyle; 14] = [
    WeaponStyle::None,
    WeaponStyle::Sword,
    WeaponStyle::Greatsword,
    WeaponStyle::Axe,
    WeaponStyle::Hammer,
    WeaponStyle::Spear,
    WeaponStyle::Staff,
    WeaponStyle::Wand,
    WeaponStyle::Bow,
    WeaponStyle::Crossbow,
    WeaponStyle::Dagger,
    WeaponStyle::Boomerang,
    WeaponStyle::Scythe,
    WeaponStyle::Flail,
];
const OFFHAND_STYLES: [OffhandStyle; 6] = [
    OffhandStyle::None,
    OffhandStyle::Torch,
    OffhandStyle::Orb,
    OffhandStyle::Dagger,
    OffhandStyle::Potion,
    OffhandStyle::Book,
];

const SKIN_COLORS: [Color; 5] = [
    rgb(255, 204, 153),
    rgb(244, 164, 96),
    rgb(198, 134, 66),
    rgb(141, 85, 36),
    rgb(74, 44, 23),
];
const HAIR_COLORS: [Color; 10] = [
    rgb(42, 21, 0),
    rgb(122, 59, 16),
    rgb(200, 134, 10),
    rgb(218, 165, 32),
    rgb(221, 34, 68),
    rgb(136, 51, 204),
    rgb(34, 170, 204),
    rgb(204, 204, 204),
    rgb(255, 255, 255),
    rgb(17, 17, 17),
];
fn apply_gender_preset(appearance: &mut CharacterAppearance) {
    match appearance.gender {
        Gender::Female => {
            appearance.hair_style = HairStyle::Long;
            appearance.palette.shirt = rgb(220, 100, 150);
            appearance.palette.pants = rgb(200, 120, 155);
            appearance.palette.boots = rgb(160, 70, 100);
        }
        Gender::Male => {
            appearance.hair_style = HairStyle::Short;
            appearance.palette.shirt = rgb(34, 102, 170);
            appearance.palette.pants = rgb(85, 51, 17);
            appearance.palette.boots = rgb(42, 26, 10);
        }
    }
}

const SHIRT_COLORS: [Color; 10] = [
    rgb(34, 102, 170),
    rgb(34, 153, 68),
    rgb(170, 34, 34),
    rgb(119, 34, 170),
    rgb(204, 136, 34),
    rgb(34, 51, 68),
    rgb(170, 170, 170),
    rgb(26, 26, 26),
    rgb(245, 230, 200),
    rgb(220, 100, 150),
];
const PANTS_COLORS: [Color; 8] = [
    rgb(85, 51, 17),
    rgb(51, 68, 85),
    rgb(34, 34, 34),
    rgb(85, 102, 51),
    rgb(136, 68, 34),
    rgb(136, 136, 170),
    rgb(204, 153, 51),
    rgb(200, 120, 155),
];
const BOOTS_COLORS: [Color; 6] = [
    rgb(42, 26, 10),
    rgb(92, 58, 30),
    rgb(26, 42, 58),
    rgb(58, 58, 58),
    rgb(138, 106, 58),
    rgb(160, 70, 100),
];
const ARMOR_COLORS: [Color; 7] = [
    rgb(136, 153, 170),
    rgb(200, 168, 74),
    rgb(204, 68, 34),
    rgb(34, 85, 170),
    rgb(34, 136, 68),
    rgb(51, 17, 17),
    rgb(232, 232, 240),
];
const CAPE_COLORS: [Color; 6] = [
    rgb(170, 34, 34),
    rgb(34, 102, 170),
    rgb(34, 34, 34),
    rgb(51, 136, 51),
    rgb(119, 34, 170),
    rgb(204, 153, 51),
];
const WEAPON_COLORS: [Color; 8] = [
    rgb(176, 200, 216),
    rgb(200, 168, 74),
    rgb(204, 68, 34),
    rgb(68, 170, 204),
    rgb(204, 68, 204),
    rgb(68, 204, 102),
    rgb(255, 238, 68),
    rgb(255, 255, 255),
];
