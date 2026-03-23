use crate::audio::{Audio, MusicTrack};
use crate::character::{CharacterAppearance, CharacterCreator, EyeExpression};
use crate::constants::{
    COLS, GAME_H, GAME_W, HUD_H, PIXEL_SCALE, ROWS, TILE, WORLD_H, WORLD_W, attack_duration,
    knockback_frames, knockback_speed, player_speed, trans_speed,
};
use crate::model::{
    Bomb, DeathAnimation, Dir, Enemy, EnemySpawn, EnemyType, EquippedItem, GameState, ItemSlot,
    NpcKind, Pickup, PickupType, Player, PlayerState, Projectile, PropKind, TileType, Transition,
    WorldProp,
};
use crate::render;
use crate::save::{self, SaveData, SaveSlotSummary};
use crate::sprites::Sprites;
use crate::world::World;
use crate::world_data;
use macroquad::prelude::*;

fn px(value: f32) -> f32 {
    value * PIXEL_SCALE
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapMode {
    Overworld,
    Dungeon,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InteractionSource {
    CurrentTile(i32, i32),
    FrontTile(i32, i32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileInteraction {
    EnterDungeon,
    EnterInterior,
    ExitDungeon,
    ExitInterior,
    CaveInteract,
    Victory,
}

pub struct Game {
    pub state: GameState,
    pub frame: i32,
    pub title_menu_selection: usize,
    pub transition: Transition,
    pub message_text: String,
    pub dungeon_overworld_x: i32,
    pub dungeon_overworld_y: i32,
    pub pending_dungeon: i32,
    pub player: Player,
    pub world: World,
    pub enemies: Vec<Enemy>,
    pub pickups: Vec<Pickup>,
    pub props: Vec<WorldProp>,
    pub bombs: Vec<Bomb>,
    pub projectiles: Vec<Projectile>,
    pub death_animations: Vec<DeathAnimation>,
    pub audio: Audio,
    pub sprites: Sprites,
    pub appearance: CharacterAppearance,
    pub creator: CharacterCreator,
    pub inventory_tab: InventoryTab,
    pub inventory_selection: usize,
    pub inventory_scroll_dir: i8,
    pub inventory_scroll_timer: i32,
    pub inventory_scroll_delay: i32,
    pub controls_scroll: f32,
    pub controls_scroll_dir: i8,
    pub controls_scroll_timer: i32,
    pub controls_scroll_delay: i32,
    pub inventory_map_mode: MapMode,
    pub all_items_mode: bool,
    pub full_hearts_mode: bool,
    pub god_mode: bool,
    pub console_open: bool,
    pub console_input: String,
    pub console_feedback: String,
    pub console_backspace_timer: i32,
    pub console_backspace_delay: i32,
    pub pause_menu_selection: usize,
    pub pause_confirm: Option<PauseConfirm>,
    pub save_message_timer: i32,
    pub save_slot_selection: usize,
    pub save_load_action_selected: SaveLoadAction,
    pub pending_save_slot: Option<usize>,
    pub pending_load_slot: Option<usize>,
    pub inventory_from_title: bool,
    blocked_interaction: Option<InteractionSource>,
    pub time_minutes: i32,
    pub day_number: i32,
    time_tick: i32,
    pub time_speed: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryTab {
    Inventory,
    Map,
    SaveLoad,
    Controls,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SaveLoadAction {
    Save,
    Load,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PauseConfirm {
    MainMenu,
    Exit,
}

impl Game {
    pub async fn new(dev_mode: bool, all_items_mode: bool, full_hearts_mode: bool) -> Self {
        let appearance = CharacterAppearance::default();
        let mut game = Self {
            state: GameState::Title,
            frame: 0,
            title_menu_selection: 0,
            transition: Transition::default(),
            message_text: String::new(),
            dungeon_overworld_x: 0,
            dungeon_overworld_y: 2,
            pending_dungeon: 0,
            player: Player::new(),
            world: World::new(dev_mode),
            enemies: vec![],
            pickups: vec![],
            props: vec![],
            bombs: vec![],
            projectiles: vec![],
            death_animations: vec![],
            audio: Audio::load().await,
            sprites: Sprites::load(&appearance).await,
            appearance: appearance.clone(),
            creator: CharacterCreator::new(appearance),
            inventory_tab: InventoryTab::Inventory,
            inventory_selection: 0,
            inventory_scroll_dir: 0,
            inventory_scroll_timer: 0,
            inventory_scroll_delay: 0,
            controls_scroll: 0.0,
            controls_scroll_dir: 0,
            controls_scroll_timer: 0,
            controls_scroll_delay: 0,
            inventory_map_mode: MapMode::Overworld,
            all_items_mode,
            full_hearts_mode,
            god_mode: false,
            console_open: false,
            console_input: String::new(),
            console_feedback: String::new(),
            console_backspace_timer: 0,
            console_backspace_delay: 0,
            pause_menu_selection: 0,
            pause_confirm: None,
            save_message_timer: 0,
            save_slot_selection: 0,
            save_load_action_selected: SaveLoadAction::Save,
            pending_save_slot: None,
            pending_load_slot: None,
            inventory_from_title: false,
            blocked_interaction: None,
            time_minutes: 360, // 6:00 AM
            day_number: 0,
            time_tick: 0,
            time_speed: 1,
        };
        game.apply_starting_loadout();
        game.spawn_for_screen();
        crate::log_info!(
            "game initialized dev_mode={} all_items_mode={} full_hearts_mode={}",
            dev_mode,
            all_items_mode,
            full_hearts_mode
        );
        game
    }

    pub fn update(&mut self) {
        self.frame += 1;
        if matches!(self.state, GameState::Playing) {
            self.time_tick += self.time_speed;
            if self.time_tick >= 60 {
                self.time_tick = 0;
                let prev = self.time_minutes;
                self.time_minutes = (self.time_minutes + 1) % 1440;
                if prev == 1439 {
                    self.day_number += 1;
                }
            }
        }
        if !matches!(self.state, GameState::Playing) || !self.console_open {
            drain_char_input();
        }
        match self.state {
            GameState::Title => {
                self.audio.play_music(MusicTrack::Title);
                let has_save = save::has_any_save();
                let max_option = if has_save { 3 } else { 2 };
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                    self.title_menu_selection = self.title_menu_selection.saturating_sub(1);
                }
                if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                    self.title_menu_selection = (self.title_menu_selection + 1).min(max_option);
                }
                if start_pressed() {
                    if has_save {
                        match self.title_menu_selection {
                            0 => self.continue_game(),
                            1 => self.start_new_game(),
                            2 => self.begin_character_create(),
                            _ => miniquad::window::request_quit(),
                        }
                    } else {
                        match self.title_menu_selection {
                            0 => self.start_new_game(),
                            1 => self.begin_character_create(),
                            _ => miniquad::window::request_quit(),
                        }
                    }
                }
            }
            GameState::CharacterCreate => self.update_character_create(),
            GameState::Playing => self.update_playing(),
            GameState::Inventory => self.update_inventory(),
            GameState::Transition => self.update_transition(),
            GameState::DungeonEnter => self.update_dungeon_enter(),
            GameState::DungeonExit => self.update_dungeon_exit(),
            GameState::Message => {
                if start_pressed() {
                    crate::log_debug!(
                        "dismiss_message blocked_source={:?} text={:?}",
                        self.active_interaction_source(),
                        self.message_text.replace('\n', " | ")
                    );
                    self.blocked_interaction = self.active_interaction_source();
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver | GameState::Victory => {
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Z) {
                    crate::log_info!("returning to title from {:?}", self.state);
                    self.state = GameState::Title;
                    self.frame = 0;
                }
            }
            GameState::PauseMenu => self.update_pause_menu(),
        }
    }

    pub fn draw(&self) {
        match self.state {
            GameState::Title => render::draw_title(
                &self.sprites,
                self.frame,
                self.title_menu_selection,
                save::has_any_save(),
            ),
            GameState::CharacterCreate => {
                render::draw_character_creator(&self.sprites, &self.creator, self.frame)
            }
            GameState::Playing => self.draw_game(),
            GameState::Inventory => {
                if self.inventory_from_title {
                    render::draw_load_menu(
                        self.save_slot_selection,
                        &self.save_slots(),
                        self.pending_load_slot,
                    )
                } else {
                    render::draw_inventory(
                        &self.sprites,
                        &self.timed_snapshot(),
                        &self.player,
                        self.inventory_tab,
                        self.inventory_map_mode,
                        self.inventory_selection,
                        self.frame,
                        self.save_message_timer,
                        self.save_slot_selection,
                        self.save_load_action_selected,
                        &self.save_slots(),
                        self.pending_save_slot,
                        self.pending_load_slot,
                        self.controls_scroll,
                    )
                }
            }
            GameState::Transition => render::draw_transition(
                &self.sprites,
                &self.timed_snapshot(),
                &self.transition,
                self.world
                    .screen_tiles(self.transition.new_screen_x, self.transition.new_screen_y),
                &self.player,
            ),
            GameState::DungeonEnter | GameState::DungeonExit => {
                self.draw_game();
                render::draw_fade_overlay((self.transition.progress / 30.0).min(1.0));
            }
            GameState::Message => {
                self.draw_game();
                render::draw_message_box(&self.message_text);
            }
            GameState::GameOver => render::draw_game_over(self.frame),
            GameState::Victory => render::draw_victory(self.frame),
            GameState::PauseMenu => {
                self.draw_game();
                render::draw_pause_menu(
                    &self.sprites,
                    self.frame,
                    self.pause_menu_selection,
                    self.pause_confirm.map(|c| matches!(c, PauseConfirm::MainMenu)),
                    self.pause_confirm.is_some(),
                );
            }
        }
    }

    fn timed_snapshot(&self) -> crate::model::WorldSnapshot {
        let mut s = self.world.snapshot();
        s.time_minutes = self.time_minutes;
        s.day_number = self.day_number;
        s
    }

    fn draw_game(&self) {
        render::draw_game(
            &self.sprites,
            self.frame,
            &self.timed_snapshot(),
            &self.player,
            &self.enemies,
            &self.pickups,
            &self.props,
            &self.bombs,
            &self.projectiles,
            &self.death_animations,
        );
        if self.console_open {
            render::draw_console(&self.console_input, &self.console_feedback, self.frame);
        }
    }

    fn start_new_game(&mut self) {
        // preserve dev_mode flag when resetting world
        let dev = self.world.dev_mode;
        self.world = World::new(dev);
        self.player = Player::new();
        self.apply_starting_loadout();
        self.sprites.set_hero_appearance(&self.appearance);
        self.spawn_for_screen();
        self.reset_items();
        self.load_screen_items();
        self.audio.play_music(MusicTrack::Overworld);
        self.state = GameState::Playing;
        self.frame = 0;
        self.inventory_tab = InventoryTab::Inventory;
        self.inventory_selection = 0;
        self.save_slot_selection = 0;
        self.save_load_action_selected = SaveLoadAction::Save;
        self.pending_save_slot = None;
        self.pending_load_slot = None;
        crate::log_info!(
            "start_new_game dev_mode={} all_items_mode={} full_hearts_mode={}",
            dev,
            self.all_items_mode,
            self.full_hearts_mode
        );
    }

    fn apply_starting_loadout(&mut self) {
        if self.full_hearts_mode {
            self.player.grant_full_hearts();
        }
        if self.all_items_mode {
            self.player.grant_all_items();
        }
    }

    fn save_slots(&self) -> Vec<SaveSlotSummary> {
        save::list_saves()
    }

    fn begin_character_create(&mut self) {
        self.creator = CharacterCreator::new(self.appearance.clone());
        self.sprites.set_hero_appearance(&self.creator.appearance);
        self.state = GameState::CharacterCreate;
        self.frame = 0;
        crate::log_info!("enter_character_create");
    }

    fn update_character_create(&mut self) {
        self.audio.play_music(MusicTrack::Title);
        let mut changed = false;
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.creator.move_selection(-1);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.creator.move_selection(1);
        }
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            changed |= self.creator.adjust_selected(-1);
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            changed |= self.creator.adjust_selected(1);
        }
        if is_key_pressed(KeyCode::R) {
            self.creator.randomize();
            changed = true;
            crate::log_debug!("character_creator_randomized");
        }
        if changed {
            self.sprites.set_hero_appearance(&self.creator.appearance);
        }
        if is_key_pressed(KeyCode::Escape) {
            self.sprites.set_hero_appearance(&self.appearance);
            self.state = GameState::Title;
            self.frame = 0;
            crate::log_info!("character_create_cancelled");
            return;
        }
        if start_pressed() {
            self.appearance = self.creator.appearance.clone();
            self.sprites.set_hero_appearance(&self.appearance);
            self.state = GameState::Title;
            self.title_menu_selection = 0;
            self.frame = 0;
            crate::log_info!("character_create_saved");
        }
    }

    fn update_playing(&mut self) {
        if console_toggle_pressed() {
            self.console_open = !self.console_open;
            self.console_input.clear();
            drain_char_input();
            crate::log_debug!("console_toggled open={}", self.console_open);
            return;
        }
        if self.console_open {
            self.update_console_input();
            return;
        }
        if is_key_pressed(KeyCode::Escape) {
            self.state = GameState::PauseMenu;
            self.pause_menu_selection = 0;
            self.pause_confirm = None;
            let mut sad = self.appearance.clone();
            sad.eyes = EyeExpression::Sad;
            self.sprites.set_hero_appearance(&sad);
            crate::log_debug!("pause_menu_opened");
            return;
        }
        if inventory_pressed() {
            self.inventory_tab = InventoryTab::Inventory;
            self.inventory_selection = 0;
            self.inventory_from_title = false;
            self.state = GameState::Inventory;
            crate::log_debug!("open_inventory selection_reset=true");
            return;
        }
        if is_key_pressed(KeyCode::M) {
            self.inventory_tab = InventoryTab::Map;
            self.inventory_map_mode = if self.world.in_dungeon { MapMode::Dungeon } else { MapMode::Overworld };
            self.inventory_from_title = false;
            self.state = GameState::Inventory;
            crate::log_debug!("open_map shortcut");
            return;
        }
        if let Some((dir, nx, ny)) = self.update_player() {
            self.start_transition(dir, nx, ny);
            return;
        }

        self.clear_blocked_interaction();

        if start_pressed() {
            let (tile_x, tile_y) = self.player_tile();
            let ladder_source = InteractionSource::CurrentTile(tile_x, tile_y);
            if !self.is_interaction_blocked(ladder_source) && self.try_use_ladder_point() {
                return;
            }

            let (front_x, front_y, _) = self.front_tile();
            let npc_source = InteractionSource::FrontTile(front_x, front_y);
            if !self.is_interaction_blocked(npc_source) && self.try_interact_npc() {
                return;
            }
        }

        match self.check_tile_interaction() {
            Some((TileInteraction::EnterDungeon, source))
                if !self.is_interaction_blocked(source) =>
            {
                if self.can_enter_current_dungeon() {
                    self.enter_dungeon();
                }
                return;
            }
            Some((TileInteraction::EnterInterior, source))
                if !self.is_interaction_blocked(source) && start_pressed() =>
            {
                self.enter_interior();
                return;
            }
            Some((TileInteraction::ExitDungeon, source))
                if !self.is_interaction_blocked(source) =>
            {
                self.exit_dungeon();
                return;
            }
            Some((TileInteraction::ExitInterior, source))
                if !self.is_interaction_blocked(source) && start_pressed() =>
            {
                self.exit_interior();
                return;
            }
            Some((TileInteraction::CaveInteract, source))
                if !self.is_interaction_blocked(source) && start_pressed() =>
            {
                self.handle_cave();
                return;
            }
            Some((TileInteraction::Victory, source)) if !self.is_interaction_blocked(source) => {
                self.state = GameState::Victory;
                self.frame = 0;
                crate::log_info!(
                    "victory_triggered screen=({}, {})",
                    self.world.screen_x,
                    self.world.screen_y
                );
                return;
            }
            _ => {}
        }
        self.update_enemies();
        self.update_room_props();
        self.update_items();
        self.update_death_animations();
        self.check_damage();
        self.check_pickups();
        if self.player.hp <= 0 {
            self.state = GameState::GameOver;
            self.frame = 0;
            crate::log_warn!(
                "game_over screen=({}, {})",
                self.world.screen_x,
                self.world.screen_y
            );
        }
    }

    fn restore_sprites(&mut self) {
        self.sprites.set_hero_appearance(&self.appearance);
    }

    fn update_pause_menu(&mut self) {
        const MENU_COUNT: usize = 3;
        if self.pause_confirm.is_some() {
            if start_pressed() || is_key_pressed(KeyCode::Y) {
                match self.pause_confirm {
                    Some(PauseConfirm::MainMenu) => {
                        crate::log_info!("pause_menu: returning to title");
                        self.restore_sprites();
                        self.state = GameState::Title;
                        self.frame = 0;
                        self.pause_confirm = None;
                    }
                    Some(PauseConfirm::Exit) => {
                        crate::log_info!("pause_menu: exit to desktop");
                        std::process::exit(0);
                    }
                    None => {}
                }
            }
            if is_key_pressed(KeyCode::Escape)
                || is_key_pressed(KeyCode::N)
                || is_key_pressed(KeyCode::X)
            {
                self.pause_confirm = None;
            }
            return;
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.pause_menu_selection = self.pause_menu_selection.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.pause_menu_selection = (self.pause_menu_selection + 1).min(MENU_COUNT - 1);
        }
        if is_key_pressed(KeyCode::Escape) {
            self.restore_sprites();
            self.state = GameState::Playing;
            crate::log_debug!("pause_menu: resumed");
            return;
        }
        if start_pressed() {
            match self.pause_menu_selection {
                0 => {
                    self.restore_sprites();
                    self.state = GameState::Playing;
                    crate::log_debug!("pause_menu: resumed via select");
                }
                1 => self.pause_confirm = Some(PauseConfirm::MainMenu),
                _ => self.pause_confirm = Some(PauseConfirm::Exit),
            }
        }
    }

    fn update_inventory(&mut self) {
        // In load-from-title mode: Enter loads immediately, no confirmation step.
        if self.inventory_from_title && start_pressed() {
            let slots = self.save_slots();
            let selected = self.save_slot_selection.min(slots.len().saturating_sub(1));
            if let Some(slot) = slots.get(selected).filter(|s| s.exists) {
                let slot_idx = slot.slot;
                self.inventory_from_title = false;
                self.load_save_slot(slot_idx);
            }
            return;
        }
        if let Some(slot) = self.pending_save_slot {
            if is_key_pressed(KeyCode::Escape)
                || is_key_pressed(KeyCode::N)
                || is_key_pressed(KeyCode::X)
            {
                self.pending_save_slot = None;
                crate::log_debug!("cancel_save_confirmation slot={}", slot + 1);
                return;
            }
            if start_pressed() || is_key_pressed(KeyCode::Y) || is_key_pressed(KeyCode::Z) {
                self.pending_save_slot = None;
                self.perform_save(slot);
                return;
            }
        }
        if let Some(slot) = self.pending_load_slot {
            if is_key_pressed(KeyCode::Escape)
                || is_key_pressed(KeyCode::N)
                || is_key_pressed(KeyCode::X)
            {
                self.pending_load_slot = None;
                crate::log_debug!("cancel_load_confirmation slot={}", slot + 1);
                return;
            }
            if start_pressed() || is_key_pressed(KeyCode::Y) || is_key_pressed(KeyCode::Z) {
                self.pending_load_slot = None;
                self.load_save_slot(slot);
                return;
            }
        }
        if inventory_pressed() || is_key_pressed(KeyCode::Escape) {
            if self.inventory_from_title {
                self.inventory_from_title = false;
                self.state = GameState::Title;
                crate::log_debug!("close_load_menu: back to title");
            } else {
                self.state = GameState::Playing;
                crate::log_debug!("close_inventory");
            }
            return;
        }
        if self.save_message_timer != 0 {
            self.save_message_timer -= self.save_message_timer.signum();
        }
        // Block tab switching when opened from title (load-only mode)
        if !self.inventory_from_title
            && (is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::E))
        {
            self.pending_save_slot = None;
            self.pending_load_slot = None;
            let direction = if is_key_pressed(KeyCode::Q) { -1 } else { 1 };
            self.inventory_tab = match (self.inventory_tab, direction) {
                (InventoryTab::Inventory, -1) => InventoryTab::Controls,
                (InventoryTab::Inventory, 1) => InventoryTab::Map,
                (InventoryTab::Map, -1) => InventoryTab::Inventory,
                (InventoryTab::Map, 1) => InventoryTab::SaveLoad,
                (InventoryTab::SaveLoad, -1) => InventoryTab::Map,
                (InventoryTab::SaveLoad, 1) => InventoryTab::Controls,
                (InventoryTab::Controls, -1) => InventoryTab::SaveLoad,
                (InventoryTab::Controls, 1) => InventoryTab::Inventory,
                _ => self.inventory_tab,
            };
            if self.inventory_tab == InventoryTab::Map {
                self.inventory_map_mode = if self.world.in_dungeon {
                    MapMode::Dungeon
                } else {
                    MapMode::Overworld
                };
            }
            if self.inventory_tab == InventoryTab::Controls {
                self.controls_scroll = 0.0;
            }
            crate::log_verbose!(
                "inventory_tab_switched tab={:?} map_mode={:?}",
                self.inventory_tab,
                self.inventory_map_mode
            );
            return;
        }

        // Tab clicks (disabled when opened from title for load-only mode)
        if !self.inventory_from_title && is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let outer_x = px(16.0);
            let outer_y = px(14.0);
            let inv_tab = Rect::new(outer_x + px(12.0), outer_y + px(30.0), px(67.0), px(22.0));
            let map_tab = Rect::new(outer_x + px(85.0), outer_y + px(30.0), px(53.0), px(22.0));
            if inv_tab.contains(vec2(mx, my)) {
                self.inventory_tab = InventoryTab::Inventory;
                self.pending_save_slot = None;
                self.pending_load_slot = None;
                crate::log_verbose!("inventory_tab_clicked tab=Inventory");
                return;
            }
            if map_tab.contains(vec2(mx, my)) {
                self.inventory_tab = InventoryTab::Map;
                self.pending_save_slot = None;
                self.pending_load_slot = None;
                self.inventory_map_mode = if self.world.in_dungeon {
                    MapMode::Dungeon
                } else {
                    MapMode::Overworld
                };
                crate::log_verbose!(
                    "inventory_tab_clicked tab=Map map_mode={:?}",
                    self.inventory_map_mode
                );
                return;
            }
            let saveload_tab =
                Rect::new(outer_x + px(144.0), outer_y + px(30.0), px(67.0), px(22.0));
            if saveload_tab.contains(vec2(mx, my)) {
                self.inventory_tab = InventoryTab::SaveLoad;
                self.save_load_action_selected = SaveLoadAction::Save;
                self.pending_save_slot = None;
                self.pending_load_slot = None;
                crate::log_verbose!("inventory_tab_clicked tab=SaveLoad");
                return;
            }
            let controls_tab =
                Rect::new(outer_x + px(217.0), outer_y + px(30.0), px(67.0), px(22.0));
            if controls_tab.contains(vec2(mx, my)) {
                self.inventory_tab = InventoryTab::Controls;
                self.pending_save_slot = None;
                self.pending_load_slot = None;
                self.controls_scroll = 0.0;
                crate::log_verbose!("inventory_tab_clicked tab=Controls");
                return;
            }
        }

        if self.inventory_tab == InventoryTab::Map {
            // When in a dungeon, allow switching between over-world and dungeon maps.
            if self.world.in_dungeon {
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                    self.inventory_map_mode = MapMode::Overworld;
                    crate::log_verbose!("inventory_map_mode=Overworld");
                }
                if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                    self.inventory_map_mode = MapMode::Dungeon;
                    crate::log_verbose!("inventory_map_mode=Dungeon");
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    let outer_x = px(16.0);
                    let outer_y = px(14.0);
                    let outer_w = GAME_W - px(32.0);
                    let content_x = outer_x + px(12.0);
                    let content_y = outer_y + px(51.0);
                    let content_w = outer_w - px(24.0);
                    let btn_w = px(80.0);
                    let btn_h = px(18.0);
                    let btn_x = content_x + content_w - btn_w - px(12.0);
                    let btn_y = content_y + px(18.0);
                    let overworld_btn = Rect::new(btn_x, btn_y, btn_w, btn_h);
                    let dungeon_btn = Rect::new(btn_x, btn_y + btn_h + px(6.0), btn_w, btn_h);
                    if overworld_btn.contains(vec2(mx, my)) {
                        self.inventory_map_mode = MapMode::Overworld;
                        crate::log_verbose!("inventory_map_button=Overworld");
                    }
                    if dungeon_btn.contains(vec2(mx, my)) {
                        self.inventory_map_mode = MapMode::Dungeon;
                        crate::log_verbose!("inventory_map_button=Dungeon");
                    }
                }
            } else {
                self.inventory_map_mode = MapMode::Overworld;
            }

            if start_pressed() {
                self.inventory_tab = InventoryTab::Inventory;
            }
            return;
        }

        if self.inventory_tab == InventoryTab::SaveLoad {
            self.update_save_load_selection();
            
            // Handle action button presses (Z/Enter/Space for selected action)
            if is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Space) || start_pressed() {
                if self.save_load_action_selected == SaveLoadAction::Save {
                    self.pending_save_slot = Some(self.save_slot_selection);
                    crate::log_info!(
                        "prompt_save_confirmation slot={}",
                        self.save_slot_selection + 1
                    );
                } else {
                    let slots = self.save_slots();
                    let selected = self.save_slot_selection.min(slots.len().saturating_sub(1));
                    if let Some(slot) = slots.get(selected).filter(|s| s.exists) {
                        self.pending_load_slot = Some(slot.slot);
                        crate::log_info!("prompt_load_confirmation slot={}", slot.slot + 1);
                    }
                }
            }
            
            // Legacy L key for load
            if is_key_pressed(KeyCode::L) {
                let slots = self.save_slots();
                let selected = self.save_slot_selection.min(slots.len().saturating_sub(1));
                if let Some(slot) = slots.get(selected).filter(|s| s.exists) {
                    self.pending_load_slot = Some(slot.slot);
                    crate::log_info!("prompt_load_confirmation slot={}", slot.slot + 1);
                }
            }
            return;
        }

        if self.inventory_tab == InventoryTab::Controls {
            // Controls panel supports scrolling when content exceeds visible height.
            const INITIAL_SCROLL_DELAY: i32 = 45;
            const REPEAT_SCROLL_DELAY: i32 = 15;
            let row_h = 22.0 * PIXEL_SCALE;
            let section_gap = 14.0 * PIXEL_SCALE;
            let content_h = (GAME_H + HUD_H - 28.0 * PIXEL_SCALE) - 63.0 * PIXEL_SCALE;
            let visible_height = content_h - 36.0 * PIXEL_SCALE;
            // There are 6 section headers and 15 data rows, so 21 total lines.
            let total_height = section_gap * 6.0 + row_h * 21.0;
            // Add a bit of buffer so the final row can fully scroll into view.
            let max_scroll = (total_height - visible_height + row_h * 1.5).max(0.0);

            // Key repeat scrolling (hold up/down)
            let current_dir = if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                -1
            } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                1
            } else {
                0
            };
            if current_dir == 0 {
                self.controls_scroll_dir = 0;
                self.controls_scroll_timer = 0;
                self.controls_scroll_delay = INITIAL_SCROLL_DELAY;
            } else {
                if current_dir != self.controls_scroll_dir {
                    self.controls_scroll_dir = current_dir;
                    self.controls_scroll_timer = 0;
                    self.controls_scroll_delay = INITIAL_SCROLL_DELAY;
                }
                if self.controls_scroll_timer <= 0 {
                    if self.controls_scroll_dir < 0 {
                        self.controls_scroll = (self.controls_scroll - row_h).max(0.0);
                    } else {
                        self.controls_scroll = (self.controls_scroll + row_h).min(max_scroll);
                    }
                    self.controls_scroll_timer = self.controls_scroll_delay;
                    self.controls_scroll_delay = REPEAT_SCROLL_DELAY;
                }
                self.controls_scroll_timer = (self.controls_scroll_timer - 1).max(0);
            }

            // Mouse wheel scrolling
            let wheel_delta = mouse_wheel().1;
            if wheel_delta > 0.0 {
                self.controls_scroll = (self.controls_scroll - row_h).max(0.0);
            } else if wheel_delta < 0.0 {
                self.controls_scroll = (self.controls_scroll + row_h).min(max_scroll);
            }

            self.controls_scroll = self.controls_scroll.clamp(0.0, max_scroll);
            return;
        }

        let entry_count = self.player.inventory_entries(&crate::world_data::dungeon_ids()).len();
        const INITIAL_SCROLL_DELAY: i32 = 45; // frames before repeat starts
        const REPEAT_SCROLL_DELAY: i32 = 15; // frames between repeated moves

        // Mouse wheel scroll support (vertical axis). Positive scrolls up.
        let wheel_delta = mouse_wheel().1;
        if wheel_delta != 0.0 {
            if wheel_delta > 0.0 {
                self.inventory_selection = self.inventory_selection.saturating_sub(1);
            } else if self.inventory_selection + 1 < entry_count {
                self.inventory_selection += 1;
            }
            self.inventory_scroll_dir = 0;
            self.inventory_scroll_timer = 0;
            self.inventory_scroll_delay = INITIAL_SCROLL_DELAY;
        }

        let current_dir = if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            -1
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            1
        } else {
            0
        };

        if current_dir == 0 {
            self.inventory_scroll_dir = 0;
            self.inventory_scroll_timer = 0;
            self.inventory_scroll_delay = INITIAL_SCROLL_DELAY;
        } else {
            if current_dir != self.inventory_scroll_dir {
                self.inventory_scroll_dir = current_dir;
                self.inventory_scroll_timer = 0;
                self.inventory_scroll_delay = INITIAL_SCROLL_DELAY;
            }
            if self.inventory_scroll_timer <= 0 {
                if current_dir < 0 {
                    self.inventory_selection = self.inventory_selection.saturating_sub(1);
                } else if self.inventory_selection + 1 < entry_count {
                    self.inventory_selection += 1;
                }
                self.inventory_scroll_timer = self.inventory_scroll_delay;
                self.inventory_scroll_delay = REPEAT_SCROLL_DELAY;
            }
            self.inventory_scroll_timer = (self.inventory_scroll_timer - 1).max(0);
        }
        if start_pressed() || is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Space) {
            self.assign_selected_inventory_item(ItemSlot::Main);
        }
        if is_key_pressed(KeyCode::X) {
            self.assign_selected_inventory_item(ItemSlot::Side);
        }
    }

    fn assign_selected_inventory_item(&mut self, slot: ItemSlot) {
        let Some(entry) = self
            .player
            .inventory_entries(&crate::world_data::dungeon_ids())
            .get(self.inventory_selection)
            .copied()
        else {
            return;
        };
        if !entry.owned || !entry.equipable {
            return;
        }
        let Some(item) = entry.item.equipped_item() else {
            return;
        };
        self.assign_item_to_slot(slot, item);
    }

    fn assign_item_to_slot(&mut self, slot: ItemSlot, item: EquippedItem) {
        let current = match slot {
            ItemSlot::Main => self.player.main_item,
            ItemSlot::Side => self.player.side_item,
        };
        if current == item {
            match slot {
                ItemSlot::Main => self.player.main_item = EquippedItem::None,
                ItemSlot::Side => self.player.side_item = EquippedItem::None,
            }
            return;
        }

        let other = match slot {
            ItemSlot::Main => self.player.side_item,
            ItemSlot::Side => self.player.main_item,
        };
        if other == item {
            match slot {
                ItemSlot::Main => self.player.side_item = current,
                ItemSlot::Side => self.player.main_item = current,
            }
        }

        match slot {
            ItemSlot::Main => self.player.main_item = item,
            ItemSlot::Side => self.player.side_item = item,
        }
    }

    fn update_save_load_selection(&mut self) {
        let slot_count = save::slot_count();
        if slot_count == 0 {
            self.save_slot_selection = 0;
            return;
        }

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.save_slot_selection = self.save_slot_selection.saturating_sub(1);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.save_slot_selection = (self.save_slot_selection + 1).min(slot_count - 1);
        }

        // Left/Right or A/D keys switch between Save and Load actions
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.save_load_action_selected = SaveLoadAction::Save;
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.save_load_action_selected = SaveLoadAction::Load;
        }

        let wheel_delta = mouse_wheel().1;
        if wheel_delta > 0.0 {
            self.save_slot_selection = self.save_slot_selection.saturating_sub(1);
        } else if wheel_delta < 0.0 {
            self.save_slot_selection = (self.save_slot_selection + 1).min(slot_count - 1);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let outer_x = px(16.0);
            let outer_y = px(14.0);
            let outer_w = GAME_W - px(32.0);
            let content_x = outer_x + px(12.0);
            let content_y = outer_y + px(51.0);
            let content_w = outer_w - px(24.0);
            let padding = px(16.0);
            let list_w = (content_w - padding * 3.0) / 2.0;
            let row_x = content_x + padding;
            let row_w = list_w - px(24.0);
            let row_h = px(58.0);
            let row_gap = px(10.0);
            let start_y = content_y + px(24.0);
            for slot in 0..slot_count {
                let y = start_y + slot as f32 * (row_h + row_gap);
                let rect = Rect::new(row_x, y, row_w, row_h);
                if rect.contains(vec2(mx, my)) {
                    self.save_slot_selection = slot;
                    break;
                }
            }

            // Action buttons in the detail panel
            let detail_x = content_x + padding + list_w + padding;
            let detail_w = list_w;
            let action_btn_h = px(22.0);
            let action_btn_w = (detail_w - px(48.0)) / 2.0;
            let action_btn_y = content_y + px(190.0);
            let save_btn = Rect::new(
                detail_x + px(16.0),
                action_btn_y,
                action_btn_w,
                action_btn_h,
            );
            let load_btn = Rect::new(
                detail_x + px(16.0) + action_btn_w + px(8.0),
                action_btn_y,
                action_btn_w,
                action_btn_h,
            );
            if save_btn.contains(vec2(mx, my)) {
                self.pending_save_slot = Some(self.save_slot_selection);
                crate::log_info!(
                    "prompt_save_confirmation slot={}",
                    self.save_slot_selection + 1
                );
            } else if load_btn.contains(vec2(mx, my)) {
                let slots = self.save_slots();
                let sel = self.save_slot_selection.min(slots.len().saturating_sub(1));
                if let Some(slot) = slots.get(sel).filter(|s| s.exists) {
                    self.pending_load_slot = Some(slot.slot);
                    crate::log_info!("prompt_load_confirmation slot={}", slot.slot + 1);
                }
            }
        }
    }

    fn to_save_data(&self) -> SaveData {
        SaveData {
            player: self.player.clone(),
            screen_x: self.world.screen_x,
            screen_y: self.world.screen_y,
            in_dungeon: self.world.in_dungeon,
            dungeon_id: self.world.dungeon_id,
            in_interior: self.world.in_interior,
            interior_id: self.world.interior_id.clone(),
            visited: self.world.visited.clone(),
            cleared_rooms: self.world.cleared_rooms.clone(),
            opened_chests: self.world.opened_chests.clone(),
            destroyed_tiles: self.world.destroyed_tiles.clone(),
            appearance: self.appearance.clone(),
            dungeon_overworld_x: self.dungeon_overworld_x,
            dungeon_overworld_y: self.dungeon_overworld_y,
            visited_screens: self.world.visited_screens.clone(),
            time_minutes: self.time_minutes,
            day_number: self.day_number,
        }
    }

    fn load_from_save(&mut self, data: SaveData) {
        self.player = data.player;
        self.player.state = PlayerState::Idle;
        self.player.attack_timer = 0;
        self.player.invuln_timer = 0;
        self.player.hurt_timer = 0;
        self.player.knock_dx = 0.0;
        self.player.knock_dy = 0.0;
        self.appearance = data.appearance;
        self.sprites.set_hero_appearance(&self.appearance);
        self.dungeon_overworld_x = data.dungeon_overworld_x;
        self.dungeon_overworld_y = data.dungeon_overworld_y;
        self.world.in_dungeon = data.in_dungeon;
        self.world.dungeon_id = data.dungeon_id;
        self.world.in_interior = data.in_interior;
        self.world.interior_id = data.interior_id;
        self.world.visited = data.visited;
        self.world.cleared_rooms = data.cleared_rooms;
        self.world.opened_chests = data.opened_chests;
        self.world.destroyed_tiles = data.destroyed_tiles;
        if data.visited_screens.len() == WORLD_H as usize
            && data.visited_screens.iter().all(|row| row.len() == WORLD_W as usize)
        {
            self.world.visited_screens = data.visited_screens;
        } else {
            self.world.visited_screens = vec![vec![false; WORLD_W as usize]; WORLD_H as usize];
        }
        self.time_minutes = data.time_minutes;
        self.day_number = data.day_number;
        self.time_tick = 0;
        self.world.load_screen(data.screen_x, data.screen_y);
        self.spawn_for_screen();
        self.reset_items();
        self.load_screen_items();
    }

    fn perform_save(&mut self, slot: usize) {
        let data = self.to_save_data();
        match save::save_game(slot, &data) {
            Ok(()) => {
                self.save_message_timer = 120;
                crate::log_info!("game saved successfully slot={}", slot + 1);
            }
            Err(e) => {
                crate::log_warn!("save failed: {}", e);
                self.save_message_timer = -120;
            }
        }
    }

    fn load_save_slot(&mut self, slot: usize) -> bool {
        match save::load_game(slot) {
            Ok(data) => {
                let dev = self.world.dev_mode;
                self.world = World::new(dev);
                self.load_from_save(data);
                self.audio.play_music(if self.world.in_dungeon {
                    MusicTrack::Dungeon
                } else {
                    MusicTrack::Overworld
                });
                self.state = GameState::Playing;
                self.frame = 0;
                self.inventory_tab = InventoryTab::Inventory;
                self.inventory_selection = 0;
                self.pending_save_slot = None;
                self.pending_load_slot = None;
                crate::log_info!("game loaded from slot={}", slot + 1);
                true
            }
            Err(e) => {
                crate::log_warn!("load failed: {}", e);
                self.save_message_timer = -120;
                false
            }
        }
    }

    fn continue_game(&mut self) {
        // Open the load menu so the player can choose which save to load.
        self.save_message_timer = 0;
        self.state = GameState::Inventory;
        self.inventory_tab = InventoryTab::SaveLoad;
        self.inventory_selection = 0;
        self.save_slot_selection = 0;
        self.save_load_action_selected = SaveLoadAction::Load;
        self.pending_save_slot = None;
        self.pending_load_slot = None;
        self.inventory_from_title = true;
    }

    fn auto_assign_item(&mut self, item: EquippedItem, preferred_slot: ItemSlot) {
        if self.player.main_item == item || self.player.side_item == item {
            return;
        }

        let preferred_empty = match preferred_slot {
            ItemSlot::Main => self.player.main_item == EquippedItem::None,
            ItemSlot::Side => self.player.side_item == EquippedItem::None,
        };
        if preferred_empty {
            self.assign_item_to_slot(preferred_slot, item);
            return;
        }

        let alternate_slot = match preferred_slot {
            ItemSlot::Main => ItemSlot::Side,
            ItemSlot::Side => ItemSlot::Main,
        };
        let alternate_empty = match alternate_slot {
            ItemSlot::Main => self.player.main_item == EquippedItem::None,
            ItemSlot::Side => self.player.side_item == EquippedItem::None,
        };
        if alternate_empty {
            self.assign_item_to_slot(alternate_slot, item);
        }
    }

    fn try_use_item(&mut self, item: EquippedItem) -> bool {
        match item {
            EquippedItem::None => false,
            EquippedItem::Sword => {
                if !self.player.has_sword {
                    return false;
                }
                self.player.begin_attack();
                self.audio.sword();
                true
            }
            EquippedItem::Bombs => {
                if self.player.has_bombs && self.player.bomb_count > 0 {
                    self.player.bomb_count -= 1;
                    let mut bx = self.player.x + px(2.0);
                    let mut by = self.player.y + px(2.0);
                    match self.player.dir {
                        Dir::Up => by -= TILE,
                        Dir::Down => by += TILE,
                        Dir::Left => bx -= TILE,
                        Dir::Right => bx += TILE,
                    }
                    self.spawn_bomb(bx, by);
                }
                false
            }
            EquippedItem::Hammer => {
                self.try_use_hammer();
                false
            }
        }
    }

    fn try_use_hammer(&mut self) {
        if !self.player.has_hammer {
            return;
        }
        let (front_x, front_y, tile) = self.front_tile();
        if tile != TileType::Cracked {
            return;
        }
        self.world.destroy_tile(
            front_x as usize,
            front_y as usize,
            if self.world.in_dungeon || self.world.in_interior {
                TileType::Floor
            } else {
                TileType::Path
            },
        );
        self.audio.pickup();
    }

    fn update_transition(&mut self) {
        self.transition.progress += trans_speed();
        let total = match self.transition.dir {
            Some(Dir::Left | Dir::Right) => GAME_W,
            Some(Dir::Up | Dir::Down) => GAME_H,
            None => 0.0,
        };
        if self.transition.progress >= total {
            self.world
                .load_screen(self.transition.new_screen_x, self.transition.new_screen_y);
            self.player.x = self.transition.player_new_x;
            self.player.y = self.transition.player_new_y;
            self.spawn_for_screen();
            self.reset_items();
            self.load_screen_items();
            self.state = GameState::Playing;
            crate::log_debug!(
                "transition_complete dir={:?} new_screen=({}, {})",
                self.transition.dir,
                self.transition.new_screen_x,
                self.transition.new_screen_y
            );
        }
    }

    fn update_dungeon_enter(&mut self) {
        self.transition.progress += 4.0;
        if self.transition.progress >= 60.0 {
            crate::log_info!("dungeon_enter_complete id={}", self.pending_dungeon);
            self.world.enter_dungeon(self.pending_dungeon);
            self.player.x = 7.0 * TILE;
            self.player.y = 9.0 * TILE;
            self.player.dir = Dir::Up;
            // boss_keys persists per-dungeon; no reset needed on dungeon enter
            self.spawn_for_screen();
            self.reset_items();
            self.load_screen_items();
            self.audio.play_music(MusicTrack::Dungeon);
            self.state = GameState::Playing;
        }
    }

    fn update_dungeon_exit(&mut self) {
        self.transition.progress += 4.0;
        if self.transition.progress >= 60.0 {
            crate::log_info!(
                "dungeon_exit_complete target_screen=({}, {})",
                self.dungeon_overworld_x,
                self.dungeon_overworld_y
            );
            self.world
                .exit_dungeon(self.dungeon_overworld_x, self.dungeon_overworld_y);
            self.player.x = 5.0 * TILE;
            self.player.y = 5.0 * TILE;
            self.player.dir = Dir::Down;
            self.spawn_for_screen();
            self.reset_items();
            self.load_screen_items();
            self.audio.play_music(MusicTrack::Overworld);
            self.state = GameState::Playing;
        }
    }

    fn update_player(&mut self) -> Option<(Dir, f32, f32)> {
        if self.player.invuln_timer > 0 {
            self.player.invuln_timer -= 1;
        }
        if self.player.hurt_timer > 0 {
            self.player.hurt_timer -= 1;
            let hb = self.player.hitbox();
            let nx = self.player.x + self.player.knock_dx;
            let ny = self.player.y + self.player.knock_dy;
            if !self.dynamic_collides(nx + px(2.0), self.player.y + px(4.0), hb.w, hb.h) {
                self.player.x = nx;
            }
            if !self.dynamic_collides(self.player.x + px(2.0), ny + px(4.0), hb.w, hb.h) {
                self.player.y = ny;
            }
            self.player.x = self.player.x.clamp(0.0, GAME_W - TILE);
            self.player.y = self.player.y.clamp(0.0, GAME_H - TILE);
            return None;
        }
        if self.player.attack_timer > 0 {
            self.player.attack_timer -= 1;
            if self.player.attack_timer == attack_duration() - 3 {
                self.sword_hit_check();
            }
            if self.player.attack_timer <= 0 {
                self.player.state = PlayerState::Idle;
            }
            return None;
        }
        if is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Space) {
            if self.try_use_item(self.player.main_item) {
                return None;
            }
        }
        if is_key_pressed(KeyCode::X) {
            if self.try_use_item(self.player.side_item) {
                return None;
            }
        }
        let mut dx = 0.0;
        let mut dy = 0.0;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            dy = -player_speed();
            self.player.dir = Dir::Up;
            self.player.last_axis = Some('y');
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            dy = player_speed();
            self.player.dir = Dir::Down;
            self.player.last_axis = Some('y');
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dx = -player_speed();
            self.player.dir = Dir::Left;
            self.player.last_axis = Some('x');
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dx = player_speed();
            self.player.dir = Dir::Right;
            self.player.last_axis = Some('x');
        }
        if dx != 0.0 && dy != 0.0 {
            let factor = 1.0 / 2.0f32.sqrt();
            dx *= factor;
            dy *= factor;
        }
        if dx != 0.0 || dy != 0.0 {
            self.try_push_boulder();
            self.player.state = PlayerState::Walking;
            self.player.walk_timer += 1;
            if self.player.walk_timer >= 24 {
                self.player.walk_timer = 0;
                self.player.walk_frame = (self.player.walk_frame + 1) % 4;
            }
            let hb = self.player.hitbox();
            let new_x = self.player.x + dx;
            let new_y = self.player.y + dy;
            if !self.dynamic_collides(new_x + px(2.0), self.player.y + px(4.0), hb.w, hb.h) {
                self.player.x = new_x;
            }
            if !self.dynamic_collides(self.player.x + px(2.0), new_y + px(4.0), hb.w, hb.h) {
                self.player.y = new_y;
            }
            let (_, _, front_tile) = self.front_tile();
            if front_tile == TileType::DoorLocked && self.player.keys > 0 {
                self.player.keys -= 1;
                self.open_all_doors(TileType::DoorLocked);
                self.audio.door();
                self.show_message("Door opened!");
            }
            if front_tile == TileType::BossDoor && self.player.has_boss_key_for(self.world.dungeon_id) {
                self.open_all_doors(TileType::BossDoor);
                self.audio.door();
                self.show_message("Boss door opened!");
            }
        } else {
            self.player.state = PlayerState::Idle;
            self.player.walk_timer = 0;
        }
        self.check_screen_edge()
    }

    fn check_screen_edge(&self) -> Option<(Dir, f32, f32)> {
        if self.player.x < -2.0 {
            Some((Dir::Left, GAME_W - TILE - 1.0, self.player.y))
        } else if self.player.x + TILE > GAME_W + 2.0 {
            Some((Dir::Right, 1.0, self.player.y))
        } else if self.player.y < -2.0 {
            Some((Dir::Up, self.player.x, GAME_H - TILE - 1.0))
        } else if self.player.y + TILE > GAME_H + 2.0 {
            Some((Dir::Down, self.player.x, 1.0))
        } else {
            None
        }
    }

    fn check_tile_interaction(&self) -> Option<(TileInteraction, InteractionSource)> {
        let (cx, cy) = self.player_tile();
        let (fx, fy, front_tile) = self.front_tile();

        match front_tile {
            TileType::HouseDoor if self.world.in_interior => {
                return Some((
                    TileInteraction::ExitInterior,
                    InteractionSource::FrontTile(fx, fy),
                ));
            }
            TileType::Cave
                if !self.world.in_dungeon
                    && !self.world.in_interior
                    && world_data::interior_entrance_at(
                        self.world.screen_x,
                        self.world.screen_y,
                        fx,
                        fy,
                    )
                    .is_some() =>
            {
                return Some((
                    TileInteraction::EnterInterior,
                    InteractionSource::FrontTile(fx, fy),
                ));
            }
            TileType::HouseDoor
                if !self.world.in_dungeon
                    && !self.world.in_interior
                    && world_data::interior_entrance_at(
                        self.world.screen_x,
                        self.world.screen_y,
                        fx,
                        fy,
                    )
                    .is_some() =>
            {
                return Some((
                    TileInteraction::EnterInterior,
                    InteractionSource::FrontTile(fx, fy),
                ));
            }
            _ => {}
        }

        match self.world.get_tile(cx, cy) {
            TileType::Dungeon => Some((
                TileInteraction::EnterDungeon,
                InteractionSource::CurrentTile(cx, cy),
            )),
            TileType::Stairs => Some((
                TileInteraction::ExitDungeon,
                InteractionSource::CurrentTile(cx, cy),
            )),
            TileType::Door if self.world.in_interior => Some((
                TileInteraction::ExitInterior,
                InteractionSource::CurrentTile(cx, cy),
            )),
            TileType::Door
                if !self.world.in_dungeon
                    && !self.world.in_interior
                    && world_data::interior_entrance_at(
                        self.world.screen_x,
                        self.world.screen_y,
                        cx,
                        cy,
                    )
                    .is_some() =>
            {
                Some((
                    TileInteraction::EnterInterior,
                    InteractionSource::CurrentTile(cx, cy),
                ))
            }
            TileType::Cave
                if !self.world.in_dungeon
                    && !self.world.in_interior
                    && world_data::interior_entrance_at(
                        self.world.screen_x,
                        self.world.screen_y,
                        cx,
                        cy,
                    )
                    .is_some() =>
            {
                Some((
                    TileInteraction::EnterInterior,
                    InteractionSource::CurrentTile(cx, cy),
                ))
            }
            TileType::Cave => Some((
                TileInteraction::CaveInteract,
                InteractionSource::CurrentTile(cx, cy),
            )),
            TileType::Goal => Some((
                TileInteraction::Victory,
                InteractionSource::CurrentTile(cx, cy),
            )),
            _ => None,
        }
    }

    fn player_tile(&self) -> (i32, i32) {
        (
            ((self.player.x + 8.0) / TILE).floor() as i32,
            ((self.player.y + 8.0) / TILE).floor() as i32,
        )
    }

    fn active_interaction_source(&self) -> Option<InteractionSource> {
        let (tile_x, tile_y) = self.player_tile();
        let (front_x, front_y, _) = self.front_tile();

        if self.props.iter().any(|prop| {
            matches!(prop.kind, PropKind::LadderPoint)
                && prop.tile_x == tile_x
                && prop.tile_y == tile_y
                && prop.target_tile_x.is_some()
                && prop.target_tile_y.is_some()
        }) {
            return Some(InteractionSource::CurrentTile(tile_x, tile_y));
        }

        if self.props.iter().any(|prop| {
            matches!(prop.kind, PropKind::Npc(_))
                && prop.tile_x == front_x
                && prop.tile_y == front_y
        }) {
            return Some(InteractionSource::FrontTile(front_x, front_y));
        }

        self.check_tile_interaction().map(|(_, source)| source)
    }

    fn clear_blocked_interaction(&mut self) {
        if self.blocked_interaction.is_some()
            && self.active_interaction_source() != self.blocked_interaction
        {
            self.blocked_interaction = None;
        }
    }

    fn is_interaction_blocked(&self, source: InteractionSource) -> bool {
        self.blocked_interaction == Some(source)
    }

    fn can_enter_current_dungeon(&mut self) -> bool {
        let dungeon_id = world_data::dungeon_at(self.world.screen_x, self.world.screen_y);
        if dungeon_id == 0 {
            return true;
        }
        if dungeon_id == 3 {
            if !self.player.has_hammer {
                self.show_message("The vault lock panel needs the HAMMER.");
                return false;
            }
            if !self.player.has_ancient_key {
                self.show_message("You need the ANCIENT KEY.");
                return false;
            }
        } else if dungeon_id == 4 {
            if !self.player.has_raft {
                self.show_message("You need the RAFT.");
                return false;
            }
            if !self.player.has_tide_chart {
                self.show_message("You need the TIDE CHART.");
                return false;
            }
        } else if dungeon_id == 5 {
            if !self.player.has_strong_arm_glove {
                self.show_message("You need the STRONG ARM GLOVE.");
                return false;
            }
            if !self.player.has_ember_crystal {
                self.show_message("You need the EMBER CRYSTAL.");
                return false;
            }
        } else if dungeon_id == 6 && !self.player.has_void_compass {
            self.show_message("You need the VOID COMPASS.");
            return false;
        } else if dungeon_id == 7 {
            if !self.player.has_raft {
                self.show_message("You need the RAFT.");
                return false;
            }
            if !self.player.has_portal_tool {
                self.show_message("You need the PORTAL TOOL.");
                return false;
            }
            if !self.player.has_star_sigil {
                self.show_message("You need the STAR SIGIL.");
                return false;
            }
        } else if dungeon_id == 8 {
            if !self.player.has_dragon_codex {
                self.show_message("You need the DRAGON CODEX.");
                return false;
            }
            if !self.player.has_crystal_of_seeing {
                self.show_message("You need the CRYSTAL OF SEEING.");
                return false;
            }
            if self.player.dragon_pieces < 7 {
                self.show_message("You need the seven DRAGON PIECES.");
                return false;
            }
        }
        true
    }

    fn front_tile(&self) -> (i32, i32, TileType) {
        let mut fx = ((self.player.x + 8.0) / TILE).floor() as i32;
        let mut fy = ((self.player.y + 8.0) / TILE).floor() as i32;
        match self.player.dir {
            Dir::Up => fy -= 1,
            Dir::Down => fy += 1,
            Dir::Left => fx -= 1,
            Dir::Right => fx += 1,
        }
        (fx, fy, self.world.get_tile(fx, fy))
    }

    fn dynamic_collides(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        if self.player_collides(x, y, w, h) {
            return true;
        }
        let rect = Rect::new(x, y, w, h);
        self.props.iter().any(|prop| {
            if !matches!(
                prop.kind,
                crate::model::PropKind::Boulder | crate::model::PropKind::Npc(_)
            ) {
                return false;
            }
            let px = prop.tile_x as f32 * TILE;
            let py = prop.tile_y as f32 * TILE;
            rect.overlaps(&Rect::new(px, py, TILE, TILE))
        })
    }

    fn player_collides(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let l = (x / TILE).floor() as i32;
        let r = ((x + w - 1.0) / TILE).floor() as i32;
        let t = (y / TILE).floor() as i32;
        let b = ((y + h - 1.0) / TILE).floor() as i32;
        for row in t..=b {
            for col in l..=r {
                let tile = self.world.get_tile(col, row);
                let solid = matches!(
                    tile,
                    TileType::Tree
                        | TileType::Rock
                        | TileType::Cracked
                        | TileType::Wall
                        | TileType::HouseRoof
                        | TileType::HouseRoofLeft
                        | TileType::HouseRoofRight
                        | TileType::HouseWall
                        | TileType::HouseWindow
                        | TileType::HouseDoor
                        | TileType::HouseChair
                        | TileType::DoorLocked
                        | TileType::BossDoor
                        | TileType::Chest
                ) || (tile == TileType::Water && !self.player.has_raft);
                if solid {
                    return true;
                }
            }
        }
        false
    }

    fn boulder_index_at(&self, tile_x: i32, tile_y: i32) -> Option<usize> {
        self.props.iter().position(|prop| {
            matches!(prop.kind, crate::model::PropKind::Boulder)
                && prop.tile_x == tile_x
                && prop.tile_y == tile_y
        })
    }

    fn try_push_boulder(&mut self) {
        let (front_x, front_y, _) = self.front_tile();
        let Some(index) = self.boulder_index_at(front_x, front_y) else {
            return;
        };
        let (dx, dy) = match self.player.dir {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };
        let target_x = self.props[index].tile_x + dx;
        let target_y = self.props[index].tile_y + dy;
        if self.world.is_solid(target_x, target_y)
            || self.boulder_index_at(target_x, target_y).is_some()
        {
            return;
        }
        self.props[index].tile_x = target_x;
        self.props[index].tile_y = target_y;
    }

    fn sword_hit_check(&mut self) {
        let sword = match self.player.dir {
            Dir::Up => Rect::new(
                self.player.x - px(4.0),
                self.player.y - px(18.0),
                px(22.0),
                px(24.0),
            ),
            Dir::Down => Rect::new(
                self.player.x - px(4.0),
                self.player.y + px(12.0),
                px(22.0),
                px(24.0),
            ),
            Dir::Left => Rect::new(
                self.player.x - px(24.0),
                self.player.y - px(4.0),
                px(24.0),
                px(22.0),
            ),
            Dir::Right => Rect::new(
                self.player.x + px(16.0),
                self.player.y - px(4.0),
                px(24.0),
                px(22.0),
            ),
        };
        let hits: Vec<usize> = self
            .enemies
            .iter()
            .enumerate()
            .filter_map(|(index, enemy)| {
                if enemy.active
                    && enemy.hurt_timer <= 0
                    && sword.overlaps(&Rect::new(enemy.x, enemy.y, enemy.w, enemy.h))
                {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();
        for index in hits {
            self.damage_enemy(index, 1, self.player.dir);
        }
        let (col, row, tile) = self.front_tile();
        if tile == TileType::Bush {
            self.world.destroy_tile(
                col as usize,
                row as usize,
                if self.world.in_dungeon || self.world.in_interior {
                    TileType::Floor
                } else {
                    TileType::Grass
                },
            );
            if rand::gen_range(0.0, 1.0) < crate::config::get().combat.bush_heart_drop_chance {
                self.spawn_pickup(
                    col as f32 * TILE + px(3.0),
                    row as f32 * TILE + px(3.0),
                    PickupType::Heart,
                );
            }
        }
        if tile == TileType::Chest {
            self.open_chest(col as usize, row as usize);
        }
    }

    fn open_chest(&mut self, col: usize, row: usize) {
        let mut item_type = PickupType::BombAmmo;
        for item in self.world.get_screen_items() {
            if item.tile_x == col && item.tile_y == row {
                item_type = item.pickup_type;
            }
        }
        self.world.mark_chest_opened(col, row);
        match item_type {
            PickupType::Bombs => {
                self.player.has_bombs = true;
                self.player.bomb_count = crate::config::get().combat.bomb_starting_ammo;
                self.auto_assign_item(EquippedItem::Bombs, ItemSlot::Side);
                self.show_message("You found BOMBS!");
            }
            PickupType::HeartContainer => {
                self.player.max_hp += 2;
                self.player.hp = self.player.max_hp;
                self.show_message("Heart Container!\nHP increased!");
            }
            PickupType::BombAmmo => {
                self.player.bomb_count = (self.player.bomb_count
                    + crate::config::get().combat.bomb_chest_ammo)
                    .min(self.player.max_bombs);
                self.show_message("Found 4 bombs!");
            }
            PickupType::Key => {
                self.player.keys += 1;
                self.show_message("Found a KEY!");
            }
            PickupType::Gem => {
                self.player.gems += pickup_value(item_type);
                self.show_message("Found a GEM!");
            }
            PickupType::Ladder => {
                self.player.has_ladder = true;
                self.show_message("You found the LADDER!");
            }
            PickupType::Hammer => {
                self.player.has_hammer = true;
                self.auto_assign_item(EquippedItem::Hammer, ItemSlot::Side);
                self.show_message("You found the HAMMER!");
            }
            PickupType::Raft => {
                self.player.has_raft = true;
                self.show_message("You found the RAFT!");
            }
            PickupType::StrongArmGlove => {
                self.player.has_strong_arm_glove = true;
                self.show_message("You found the STRONG ARM GLOVE!");
            }
            PickupType::PortalTool => {
                self.player.has_portal_tool = true;
                self.show_message("You found the PORTAL TOOL!");
            }
            PickupType::DragonPiece => {
                self.player.dragon_pieces += 1;
                self.show_message("Dragon piece claimed!");
            }
            PickupType::BossKey
            | PickupType::Heart
            | PickupType::Sword
            | PickupType::TideChart
            | PickupType::EmberCrystal
            | PickupType::VoidCompass
            | PickupType::CrystalOfSeeing => {}
        }
        if matches!(item_type, PickupType::Gem | PickupType::DragonPiece) {
            self.audio.currency();
        } else {
            self.audio.pickup();
        }
    }

    fn open_all_doors(&mut self, tile_type: TileType) {
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.world.tiles[row][col] == tile_type {
                    self.world.destroy_tile(
                        col,
                        row,
                        if self.world.in_dungeon || self.world.in_interior {
                            TileType::Floor
                        } else {
                            TileType::Path
                        },
                    );
                }
            }
        }
    }

    fn start_transition(&mut self, dir: Dir, nx: f32, ny: f32) {
        let (mut next_x, mut next_y) = (self.world.screen_x, self.world.screen_y);
        match dir {
            Dir::Left => next_x -= 1,
            Dir::Right => next_x += 1,
            Dir::Up => next_y -= 1,
            Dir::Down => next_y += 1,
        }
        if next_x < 0 || next_y < 0 || !self.world.screen_exists(next_x, next_y) {
            self.player.x = self.player.x.clamp(0.0, GAME_W - TILE);
            self.player.y = self.player.y.clamp(0.0, GAME_H - TILE);
            return;
        }
        let Some((player_new_x, player_new_y)) =
            self.find_transition_target(dir, next_x, next_y, nx, ny)
        else {
            self.player.x = self.player.x.clamp(0.0, GAME_W - TILE);
            self.player.y = self.player.y.clamp(0.0, GAME_H - TILE);
            return;
        };
        self.transition = Transition {
            dir: Some(dir),
            progress: 0.0,
            old_tiles: self.world.tiles.clone(),
            new_screen_x: next_x,
            new_screen_y: next_y,
            player_new_x,
            player_new_y,
        };
        self.state = GameState::Transition;
    }

    fn find_transition_target(
        &self,
        dir: Dir,
        next_x: i32,
        next_y: i32,
        nx: f32,
        ny: f32,
    ) -> Option<(f32, f32)> {
        let next_tiles = self.world.screen_tiles(next_x, next_y)?;
        match dir {
            Dir::Left | Dir::Right => {
                let start_row = ((self.player.y + 8.0) / TILE).floor() as i32;
                let current_edge = if matches!(dir, Dir::Left) {
                    0
                } else {
                    COLS - 1
                };
                let next_edge = if matches!(dir, Dir::Left) {
                    COLS - 1
                } else {
                    0
                };
                let mut candidates: Vec<i32> = (0..ROWS).map(|row| row as i32).collect();
                candidates.sort_by_key(|row| (row - start_row).abs());
                for row in candidates {
                    let row_usize = row as usize;
                    if self.transition_tile_open(self.world.tiles[row_usize][current_edge])
                        && self.transition_tile_open(next_tiles[row_usize][next_edge])
                    {
                        return Some((nx, row as f32 * TILE));
                    }
                }
            }
            Dir::Up | Dir::Down => {
                let start_col = ((self.player.x + 8.0) / TILE).floor() as i32;
                let current_edge = if matches!(dir, Dir::Up) { 0 } else { ROWS - 1 };
                let next_edge = if matches!(dir, Dir::Up) { ROWS - 1 } else { 0 };
                let mut candidates: Vec<i32> = (0..COLS).map(|col| col as i32).collect();
                candidates.sort_by_key(|col| (col - start_col).abs());
                for col in candidates {
                    let col_usize = col as usize;
                    if self.transition_tile_open(self.world.tiles[current_edge][col_usize])
                        && self.transition_tile_open(next_tiles[next_edge][col_usize])
                    {
                        return Some((col as f32 * TILE, ny));
                    }
                }
            }
        }
        None
    }

    fn transition_tile_open(&self, tile: TileType) -> bool {
        !matches!(
            tile,
            TileType::Tree
                | TileType::Rock
                | TileType::Cracked
                | TileType::Wall
                | TileType::HouseRoof
                | TileType::HouseRoofLeft
                | TileType::HouseRoofRight
                | TileType::HouseWall
                | TileType::HouseWindow
                | TileType::HouseDoor
                | TileType::HouseChair
                | TileType::DoorLocked
                | TileType::BossDoor
                | TileType::Chest
        ) && (tile != TileType::Water || self.player.has_raft)
    }

    fn enter_dungeon(&mut self) {
        self.dungeon_overworld_x = self.world.screen_x;
        self.dungeon_overworld_y = self.world.screen_y;
        self.pending_dungeon = world_data::dungeon_at(self.world.screen_x, self.world.screen_y);
        self.transition.progress = 0.0;
        self.state = GameState::DungeonEnter;
        crate::log_info!(
            "begin_dungeon_enter id={} from_screen=({}, {})",
            self.pending_dungeon,
            self.dungeon_overworld_x,
            self.dungeon_overworld_y
        );
    }

    fn exit_dungeon(&mut self) {
        self.transition.progress = 0.0;
        self.state = GameState::DungeonExit;
        crate::log_info!(
            "begin_dungeon_exit from_dungeon={} to_screen=({}, {})",
            self.world.dungeon_id,
            self.dungeon_overworld_x,
            self.dungeon_overworld_y
        );
    }

    fn enter_interior(&mut self) {
        let (front_x, front_y, _) = self.front_tile();
        let current_x = ((self.player.x + 8.0) / TILE).floor() as i32;
        let current_y = ((self.player.y + 8.0) / TILE).floor() as i32;
        let interior_id = world_data::interior_entrance_at(
            self.world.screen_x,
            self.world.screen_y,
            front_x,
            front_y,
        )
        .or_else(|| {
            world_data::interior_entrance_at(
                self.world.screen_x,
                self.world.screen_y,
                current_x,
                current_y,
            )
        });
        let Some(interior_id) = interior_id else {
            return;
        };
        self.world.enter_interior(interior_id);
        let (spawn_x, spawn_y) = world_data::interior_spawn_tile(interior_id);
        self.player.x = spawn_x as f32 * TILE;
        self.player.y = spawn_y as f32 * TILE;
        self.player.dir = Dir::Up;
        self.spawn_for_screen();
        self.reset_items();
        self.load_screen_items();
        crate::log_info!(
            "enter_interior id={} spawn=({}, {})",
            interior_id,
            spawn_x,
            spawn_y
        );
    }

    fn exit_interior(&mut self) {
        let interior_id = self.world.interior_id.clone();
        self.world.exit_interior();
        if let Some((tile_x, tile_y)) = world_data::interior_exit_overworld_tile(&interior_id) {
            self.player.x = tile_x as f32 * TILE;
            self.player.y = tile_y as f32 * TILE;
        }
        self.player.dir = Dir::Down;
        self.spawn_for_screen();
        self.reset_items();
        self.load_screen_items();
        crate::log_info!("exit_interior id={}", interior_id);
    }

    fn handle_cave(&mut self) {
        match world_data::cave_kind(self.world.screen_x, self.world.screen_y) {
            Some(cave_kind) => self.handle_cave_kind(cave_kind),
            None => self.show_message("A mysterious\ncave..."),
        }
    }

    fn handle_cave_kind(&mut self, cave_kind: world_data::CaveKind) {
        crate::log_debug!(
            "handle_cave_kind kind={:?} screen=({}, {}) interior={} dungeon={}",
            cave_kind,
            self.world.screen_x,
            self.world.screen_y,
            self.world.in_interior,
            self.world.in_dungeon
        );
        let cave_key = if self.world.in_interior {
            format!("cave:{}", self.world.interior_id)
        } else {
            format!(
                "cave:{}",
                world_data::screen_key(self.world.screen_x, self.world.screen_y)
            )
        };
        match cave_kind {
            world_data::CaveKind::Sword => {
                if !self.player.has_sword {
                    self.player.has_sword = true;
                    self.auto_assign_item(EquippedItem::Sword, ItemSlot::Main);
                    self.show_message("You found a SWORD!");
                } else {
                    self.show_message("The cave is empty.");
                }
            }
            world_data::CaveKind::Heart => {
                if !self.world.opened_chests.contains_key(&cave_key) {
                    self.world.opened_chests.insert(cave_key, vec![(3, 4)]);
                    self.player.max_hp += 2;
                    self.player.hp = self.player.max_hp;
                    self.show_message("Heart Container!\nHP increased!");
                } else {
                    self.show_message("The cave is empty.");
                }
            }
            world_data::CaveKind::Shop => {
                if !self.world.opened_chests.contains_key(&cave_key) {
                    self.world.opened_chests.insert(cave_key, vec![]);
                    self.player.max_bombs = crate::config::get().combat.bomb_max_capacity;
                    self.player.bomb_count = (self.player.bomb_count
                        + crate::config::get().combat.bomb_starting_ammo)
                        .min(self.player.max_bombs);
                    self.show_message("Bomb bag upgrade!\nMax bombs increased!");
                } else {
                    self.show_message("The shop is closed.");
                }
            }
            world_data::CaveKind::AncientKey => {
                if !self.player.has_ancient_key {
                    self.player.has_ancient_key = true;
                    self.show_message("You found the ANCIENT KEY!");
                } else {
                    self.show_message("Corvin already sold you the key.");
                }
            }
            world_data::CaveKind::TideChart => {
                if !self.player.has_tide_chart {
                    self.player.has_tide_chart = true;
                    self.show_message("You found the TIDE CHART!");
                } else {
                    self.show_message("The lighthouse is empty.");
                }
            }
            world_data::CaveKind::EmberCrystal => {
                if !self.player.has_strong_arm_glove {
                    self.show_message("The crystal is too hot to touch.");
                } else if !self.player.has_ember_crystal {
                    self.player.has_ember_crystal = true;
                    self.show_message("You claimed the EMBER CRYSTAL!");
                } else {
                    self.show_message("Only cooling shards remain.");
                }
            }
            world_data::CaveKind::VoidCompass => {
                if !self.player.has_void_compass {
                    self.player.has_void_compass = true;
                    self.show_message("You found the VOID COMPASS!");
                } else {
                    self.show_message("The rift has gone still.");
                }
            }
            world_data::CaveKind::StarSigil => {
                if !self.player.has_star_sigil {
                    self.player.has_star_sigil = true;
                    self.show_message("You received the STAR SIGIL!");
                } else {
                    self.show_message("The merchant has already moved on.");
                }
            }
            world_data::CaveKind::DragonCodex => {
                if !self.player.has_dragon_codex {
                    self.player.has_dragon_codex = true;
                    self.show_message("You assembled the DRAGON CODEX!");
                } else {
                    self.show_message("Wren has no more pages for you.");
                }
            }
            world_data::CaveKind::CrystalOfSeeing => {
                if !self.player.has_crystal_of_seeing {
                    self.player.has_crystal_of_seeing = true;
                    self.show_message("You found the CRYSTAL OF SEEING!");
                } else {
                    self.show_message("The niche is empty.");
                }
            }
            world_data::CaveKind::Shrine => {
                if !self.world.opened_chests.contains_key(&cave_key) {
                    self.world.opened_chests.insert(cave_key, vec![]);
                    self.player.max_hp += 2;
                    self.player.hp = self.player.max_hp;
                    self.show_message("Island shrine!\nHP increased!");
                } else {
                    self.show_message("The shrine is quiet.");
                }
            }
            world_data::CaveKind::Sanctum => {
                if !self.world.opened_chests.contains_key(&cave_key) {
                    self.world.opened_chests.insert(cave_key, vec![]);
                    self.player.max_hp += 2;
                    self.player.hp = self.player.max_hp;
                    self.show_message("Heart Container!\nHP increased!");
                } else {
                    self.show_message("The cave is empty.");
                }
            }
            world_data::CaveKind::Bombs => {
                if !self.world.opened_chests.contains_key(&cave_key) {
                    self.world.opened_chests.insert(cave_key, vec![]);
                    if !self.player.has_bombs {
                        self.player.has_bombs = true;
                        self.player.bomb_count = crate::config::get().combat.bomb_starting_ammo;
                        self.auto_assign_item(EquippedItem::Bombs, ItemSlot::Side);
                        self.show_message("You found BOMBS!");
                    } else {
                        self.player.bomb_count = (self.player.bomb_count
                            + crate::config::get().combat.bomb_starting_ammo)
                            .min(self.player.max_bombs);
                        self.show_message("Found 8 bombs!");
                    }
                } else {
                    self.show_message("The cave is empty.");
                }
            }
        }
    }

    fn update_enemies(&mut self) {
        let px = self.player.x + 8.0;
        let py = self.player.y + 8.0;
        let mut shots = vec![];
        for enemy in &mut self.enemies {
            if !enemy.active {
                continue;
            }
            if enemy.hurt_timer > 0 {
                enemy.hurt_timer -= 1;
                enemy.x += enemy.knock_x;
                enemy.y += enemy.knock_y;
                enemy.flash_timer = enemy.hurt_timer;
                continue;
            }
            enemy.timer += 1;
            match enemy.enemy_type {
                EnemyType::Slime => ai_slime(enemy, &self.world),
                EnemyType::Octorok => {
                    if let Some(shot) = ai_octorok(enemy, &self.world, px, py) {
                        shots.push(shot);
                    }
                }
                EnemyType::Bat => ai_bat(enemy, px, py),
                EnemyType::Darknut => ai_darknut(enemy, &self.world, px, py),
                EnemyType::Boss => shots.extend(ai_boss(enemy, &self.world, px, py)),
            }
        }
        for (x, y, dx, dy) in shots {
            self.spawn_projectile(x, y, dx, dy, true);
        }
        self.enemies.retain(|enemy| enemy.active);
    }

    fn update_items(&mut self) {
        for pickup in &mut self.pickups {
            pickup.timer += 1;
            if pickup_times_out(pickup.pickup_type)
                && pickup.timer > crate::config::get().combat.pickup_lifetime_frames
            {
                pickup.collected = true;
            }
        }
        let mut exploded = vec![];
        for bomb in &mut self.bombs {
            if !bomb.exploded {
                bomb.timer -= 1;
                if bomb.timer <= 0 {
                    bomb.exploded = true;
                    bomb.explosion_timer = crate::config::get().combat.bomb_explosion_frames;
                    exploded.push((bomb.x, bomb.y, bomb.w, bomb.h));
                }
            } else {
                bomb.explosion_timer -= 1;
            }
        }
        for (x, y, w, h) in exploded {
            self.bomb_explode(x, y, w, h);
        }
        for projectile in &mut self.projectiles {
            projectile.x += projectile.dx;
            projectile.y += projectile.dy;
            projectile.timer -= 1;
            let tile_x = ((projectile.x + projectile.w / 2.0) / TILE).floor() as i32;
            let tile_y = ((projectile.y + projectile.h / 2.0) / TILE).floor() as i32;
            if self.world.is_solid(tile_x, tile_y)
                || projectile.timer <= 0
                || projectile.x < -8.0
                || projectile.x > GAME_W + 8.0
                || projectile.y < -8.0
                || projectile.y > GAME_H + 8.0
            {
                projectile.active = false;
            }
        }
        self.pickups.retain(|pickup| !pickup.collected);
        self.bombs
            .retain(|bomb| !bomb.exploded || bomb.explosion_timer > 0);
        self.projectiles.retain(|projectile| projectile.active);
    }

    fn bomb_explode(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let cx = x + w / 2.0;
        let cy = y + h / 2.0;
        for row in 0..ROWS {
            for col in 0..COLS {
                let tx = col as f32 * TILE + TILE / 2.0;
                let ty = row as f32 * TILE + TILE / 2.0;
                if vec2(tx - cx, ty - cy).length() < crate::config::get().combat.bomb_radius
                    && self.world.tiles[row][col] == TileType::Cracked
                {
                    self.world.destroy_tile(
                        col,
                        row,
                        if self.world.in_dungeon || self.world.in_interior {
                            TileType::Floor
                        } else {
                            TileType::Path
                        },
                    );
                }
            }
        }
    }

    fn update_death_animations(&mut self) {
        for anim in &mut self.death_animations {
            anim.timer -= 1;
        }
        self.death_animations.retain(|anim| anim.timer > 0);
    }

    fn check_damage(&mut self) {
        if self.player.invuln_timer <= 0
            && self.player.hurt_timer <= 0
            && self.player.attack_timer <= 0
        {
            let player_rect = self.player.hitbox();
            if let Some(enemy) = self.enemies.iter().find(|enemy| {
                enemy.active
                    && enemy.hurt_timer <= 0
                    && player_rect.overlaps(&Rect::new(enemy.x, enemy.y, enemy.w, enemy.h))
            }) {
                let dx = self.player.x - enemy.x;
                let dy = self.player.y - enemy.y;
                let knock_dir = if dx.abs() > dy.abs() {
                    if dx > 0.0 { Dir::Right } else { Dir::Left }
                } else if dy > 0.0 {
                    Dir::Down
                } else {
                    Dir::Up
                };
                self.player_take_damage(1, knock_dir);
            }
        }
        if self.player.invuln_timer <= 0 && self.player.hurt_timer <= 0 {
            let player_rect = self.player.hitbox();
            if let Some(projectile) = self.projectiles.iter_mut().find(|p| {
                p.active && p.from_enemy && player_rect.overlaps(&Rect::new(p.x, p.y, p.w, p.h))
            }) {
                projectile.active = false;
                self.player_take_damage(1, self.player.dir);
            }
        }
        let hits: Vec<usize> = self
            .enemies
            .iter()
            .enumerate()
            .filter_map(|(index, enemy)| {
                if enemy.active
                    && enemy.hurt_timer <= 0
                    && self.bombs.iter().any(|bomb| {
                        bomb.exploded
                            && bomb.explosion_timer >= 15
                            && Rect::new(bomb.x - 8.0, bomb.y - 8.0, bomb.w + 16.0, bomb.h + 16.0)
                                .overlaps(&Rect::new(enemy.x, enemy.y, enemy.w, enemy.h))
                    })
                {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();
        for index in hits {
            self.damage_enemy(index, 3, self.player.dir);
        }
    }

    fn player_take_damage(&mut self, amount: i32, from_dir: Dir) {
        if self.god_mode || self.world.dev_mode || self.player.invuln_timer > 0 {
            return;
        }
        self.player.hp -= amount;
        self.player.invuln_timer = crate::config::get().player.invuln_frames;
        self.player.hurt_timer = knockback_frames();
        self.player.state = PlayerState::Hurt;
        self.player.knock_dx = 0.0;
        self.player.knock_dy = 0.0;
        match from_dir {
            Dir::Up => self.player.knock_dy = knockback_speed(),
            Dir::Down => self.player.knock_dy = -knockback_speed(),
            Dir::Left => self.player.knock_dx = knockback_speed(),
            Dir::Right => self.player.knock_dx = -knockback_speed(),
        }
        self.audio.player_hit();
    }

    fn damage_enemy(&mut self, index: usize, amount: i32, knock_dir: Dir) {
        if index >= self.enemies.len() || self.enemies[index].hurt_timer > 0 {
            return;
        }
        let enemy = &mut self.enemies[index];
        enemy.hp -= amount;
        enemy.hurt_timer = 20;
        enemy.flash_timer = 20;
        enemy.knock_x = 0.0;
        enemy.knock_y = 0.0;
        match knock_dir {
            Dir::Up => enemy.knock_y = -2.0,
            Dir::Down => enemy.knock_y = 2.0,
            Dir::Left => enemy.knock_x = -2.0,
            Dir::Right => enemy.knock_x = 2.0,
        }
        if enemy.hp <= 0 {
            enemy.active = false;
            self.on_enemy_death(index);
            self.audio.enemy_die();
        }
    }

    fn on_enemy_death(&mut self, index: usize) {
        let enemy = self.enemies[index].clone();
        self.death_animations.push(DeathAnimation {
            x: enemy.x + enemy.w / 2.0,
            y: enemy.y + enemy.h / 2.0,
            timer: 20,
        });
        let roll = rand::gen_range(0.0, 1.0);
        match enemy.enemy_type {
            EnemyType::Boss => {
                self.spawn_pickup(enemy.x + 4.0, enemy.y + 4.0, PickupType::HeartContainer);
            }
            _ if roll < crate::config::get().combat.enemy_heart_drop_chance => {
                self.spawn_pickup(enemy.x, enemy.y, PickupType::Heart)
            }
            _ if roll < 0.35 => self.spawn_pickup(enemy.x, enemy.y, PickupType::BombAmmo),
            _ if roll < 0.5 => self.spawn_pickup(enemy.x, enemy.y, PickupType::Gem),
            _ => {}
        }
        if self.world.in_dungeon {
            let remaining = self
                .enemies
                .iter()
                .enumerate()
                .filter(|(i, enemy)| *i != index && enemy.active)
                .count();
            if remaining == 0 {
                let cleared_key = self.world.cleared_room_key();
                self.world.cleared_rooms.insert(cleared_key);
                self.on_dungeon_room_cleared();
            }
        }
    }

    fn check_pickups(&mut self) {
        let player_rect = Rect::new(
            self.player.x + px(3.0),
            self.player.y + px(4.0),
            px(10.0),
            px(12.0),
        );
        let collected: Vec<(usize, PickupType, Option<String>)> = self
            .pickups
            .iter()
            .enumerate()
            .filter_map(|(index, pickup)| {
                if !pickup.collected
                    && player_rect.overlaps(&Rect::new(pickup.x, pickup.y, pickup.w, pickup.h))
                {
                    Some((index, pickup.pickup_type, pickup.key.clone()))
                } else {
                    None
                }
            })
            .collect();
        for (index, pickup, key) in collected {
            if self.collect_pickup(pickup) {
                if let Some(key) = key {
                    self.world.opened_chests.insert(key, vec![]);
                }
                if let Some(item) = self.pickups.get_mut(index) {
                    item.collected = true;
                }
            }
        }
    }

    fn collect_pickup(&mut self, pickup: PickupType) -> bool {
        match pickup {
            PickupType::Heart => self.player.hp = (self.player.hp + 2).min(self.player.max_hp),
            PickupType::HeartContainer => {
                self.player.max_hp += 2;
                self.player.hp = self.player.max_hp;
                self.show_message("Heart Container!\nHP increased!");
            }
            PickupType::Key => self.player.keys += 1,
            PickupType::BossKey => {
                self.player.grant_boss_key_for(self.world.dungeon_id);
                self.show_message("Boss Key found!");
            }
            PickupType::BombAmmo => {
                if self.player.has_bombs {
                    self.player.bomb_count = (self.player.bomb_count
                        + crate::config::get().combat.bomb_chest_ammo)
                        .min(self.player.max_bombs);
                }
            }
            PickupType::Bombs => {
                self.player.has_bombs = true;
                self.player.bomb_count = crate::config::get().combat.bomb_starting_ammo;
                self.auto_assign_item(EquippedItem::Bombs, ItemSlot::Side);
                self.show_message("You found BOMBS!");
            }
            PickupType::Gem => {
                self.player.gems += pickup_value(pickup);
            }
            PickupType::Ladder => {
                self.player.has_ladder = true;
                self.show_message("You found the LADDER!");
            }
            PickupType::Hammer => {
                self.player.has_hammer = true;
                self.auto_assign_item(EquippedItem::Hammer, ItemSlot::Side);
                self.show_message("You found the HAMMER!");
            }
            PickupType::Raft => {
                self.player.has_raft = true;
                self.show_message("You found the RAFT!");
            }
            PickupType::StrongArmGlove => {
                self.player.has_strong_arm_glove = true;
                self.show_message("You found the STRONG ARM GLOVE!");
            }
            PickupType::PortalTool => {
                self.player.has_portal_tool = true;
                self.show_message("You found the PORTAL TOOL!");
            }
            PickupType::DragonPiece => {
                self.player.dragon_pieces += 1;
                self.show_message("You claimed a dragon piece!");
            }
            PickupType::Sword => {
                self.player.has_sword = true;
                self.auto_assign_item(EquippedItem::Sword, ItemSlot::Main);
                self.show_message("You found a SWORD!");
            }
            PickupType::TideChart => {
                self.player.has_tide_chart = true;
                self.show_message("You found the TIDE CHART!");
            }
            PickupType::EmberCrystal => {
                if !self.player.has_strong_arm_glove {
                    self.show_message("The crystal is too hot to touch.");
                    return false;
                }
                self.player.has_ember_crystal = true;
                self.show_message("You claimed the EMBER CRYSTAL!");
            }
            PickupType::VoidCompass => {
                self.player.has_void_compass = true;
                self.show_message("You found the VOID COMPASS!");
            }
            PickupType::CrystalOfSeeing => {
                self.player.has_crystal_of_seeing = true;
                self.show_message("You found the CRYSTAL OF SEEING!");
            }
        }
        if matches!(
            pickup,
            PickupType::Key | PickupType::Gem | PickupType::DragonPiece
        ) {
            self.audio.currency();
        } else {
            self.audio.pickup();
        }
        true
    }

    fn spawn_for_screen(&mut self) {
        self.enemies = self
            .world
            .get_enemy_spawns()
            .into_iter()
            .map(create_enemy)
            .collect();
        self.props = world_data::screen_props(
            self.world.screen_x,
            self.world.screen_y,
            self.world.in_dungeon,
            self.world.dungeon_id,
            self.world.in_interior,
            &self.world.interior_id,
        );
    }

    fn reset_items(&mut self) {
        self.pickups.clear();
        self.bombs.clear();
        self.projectiles.clear();
    }

    fn load_screen_items(&mut self) {
        if self.world.in_interior {
            for item in self.world.get_screen_items() {
                let key = self.screen_pickup_state_key(item.tile_x, item.tile_y);
                if self.world.opened_chests.contains_key(&key) {
                    continue;
                }
                if self.player_already_has_pickup(item.pickup_type) {
                    self.world.opened_chests.insert(key, vec![]);
                    continue;
                }
                self.spawn_pickup_with_key(
                    item.tile_x as f32 * TILE,
                    item.tile_y as f32 * TILE + px(4.0),
                    item.pickup_type,
                    key,
                );
            }
            return;
        }
        if self.world.in_dungeon && !self.player.has_boss_key_for(self.world.dungeon_id) {
            if let Some((tile_x, tile_y)) = world_data::boss_key_spawn_tile(
                self.world.dungeon_id,
                self.world.screen_x,
                self.world.screen_y,
            ) {
                if self.props.is_empty() {
                    self.spawn_pickup(
                        tile_x as f32 * TILE,
                        tile_y as f32 * TILE + px(4.0),
                        PickupType::BossKey,
                    );
                }
            } else {
                let should_spawn = match self.world.dungeon_id {
                    2 => self.world.screen_x == 2 && self.world.screen_y == 1,
                    3 => self.world.screen_x == 1 && self.world.screen_y == 0,
                    4 => self.world.screen_x == 1 && self.world.screen_y == 0,
                    _ => false,
                };
                if should_spawn {
                    self.spawn_pickup(8.0 * TILE, 5.0 * TILE + px(4.0), PickupType::BossKey);
                }
            }
        }
    }

    fn update_console_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.console_open = false;
            self.console_input.clear();
            crate::log_debug!("console_closed");
            return;
        }
        // Backspace repeat behavior
        const BACKSPACE_INITIAL_DELAY: i32 = 30;
        const BACKSPACE_REPEAT_DELAY: i32 = 15;

        if is_key_pressed(KeyCode::Backspace) {
            self.console_input.pop();
            self.console_backspace_timer = BACKSPACE_INITIAL_DELAY;
            self.console_backspace_delay = BACKSPACE_REPEAT_DELAY;
        } else if is_key_down(KeyCode::Backspace) {
            if self.console_backspace_timer <= 0 {
                self.console_input.pop();
                self.console_backspace_timer = self.console_backspace_delay;
            } else {
                self.console_backspace_timer -= 1;
            }
        } else {
            self.console_backspace_timer = 0;
        }
        while let Some(ch) = get_char_pressed() {
            if !ch.is_control() && self.console_input.len() < 96 {
                self.console_input.push(ch);
            }
        }
        if is_key_pressed(KeyCode::Enter) {
            let command = self.console_input.trim().to_string();
            self.console_feedback = self.execute_console_command(&command);
            self.console_input.clear();
        }
    }

    fn execute_console_command(&mut self, command: &str) -> String {
        if command.is_empty() {
            return "Enter a command.".to_string();
        }

        let mut parts = command.split_whitespace();
        let Some(name) = parts.next() else {
            return "Enter a command.".to_string();
        };

        match name.to_ascii_lowercase().as_str() {
            "help" => console_help_text().to_string(),
            "teleport" => {
                let coords = command[name.len()..].trim();
                let mut values = coords.split(',').map(str::trim);
                let (Some(x_str), Some(y_str), None) =
                    (values.next(), values.next(), values.next())
                else {
                    return "Usage: teleport x,y".to_string();
                };

                let Ok(screen_x) = x_str.parse::<i32>() else {
                    return "Teleport X must be a whole number.".to_string();
                };
                let Ok(screen_y) = y_str.parse::<i32>() else {
                    return "Teleport Y must be a whole number.".to_string();
                };

                if screen_x < 0 || screen_x >= WORLD_W || screen_y < 0 || screen_y >= WORLD_H {
                    return format!(
                        "Teleport target must be within 0..{}, 0..{}.",
                        WORLD_W - 1,
                        WORLD_H - 1
                    );
                }

                match self.teleport_to_screen(screen_x, screen_y) {
                    Ok(()) => {
                        crate::log_info!("console_teleport screen=({}, {})", screen_x, screen_y);
                        format!("Teleported to {},{}.", screen_x, screen_y)
                    }
                    Err(err) => err,
                }
            }
            "god_mode" => {
                let Some(value) = parts.next() else {
                    return "Usage: god_mode 0/1".to_string();
                };
                if parts.next().is_some() {
                    return "Usage: god_mode 0/1".to_string();
                }

                match value {
                    "0" => {
                        self.god_mode = false;
                        crate::log_info!("console_god_mode enabled=false");
                        "God mode disabled.".to_string()
                    }
                    "1" => {
                        self.god_mode = true;
                        crate::log_info!("console_god_mode enabled=true");
                        "God mode enabled.".to_string()
                    }
                    _ => "Usage: god_mode 0/1".to_string(),
                }
            }
            "get_item" => {
                let item_name = parts.next().map(|s| s.to_lowercase());
                let Some(item_name) = item_name else {
                    return "Usage: get_item <item_name>".to_string();
                };
                if parts.next().is_some() {
                    return "Usage: get_item <item_name>".to_string();
                }

                match item_name.as_str() {
                    "sword" => {
                        if !self.player.has_sword {
                            self.player.has_sword = true;
                            self.auto_assign_item(EquippedItem::Sword, ItemSlot::Main);
                            crate::log_info!("console_get_item item=sword");
                            "Got SWORD!".to_string()
                        } else {
                            "Already have SWORD.".to_string()
                        }
                    }
                    "bombs" => {
                        if !self.player.has_bombs {
                            self.player.has_bombs = true;
                            self.player.bomb_count = crate::config::get().combat.bomb_starting_ammo;
                            self.auto_assign_item(EquippedItem::Bombs, ItemSlot::Side);
                            crate::log_info!("console_get_item item=bombs");
                            "Got BOMBS!".to_string()
                        } else if self.player.bomb_count < self.player.max_bombs {
                            self.player.bomb_count = self.player.max_bombs;
                            "Bombs refilled!".to_string()
                        } else {
                            "Already have BOMBS (full).".to_string()
                        }
                    }
                    s if s.starts_with("boss_key_") => {
                        let suffix = &s["boss_key_".len()..];
                        match suffix.parse::<i32>() {
                            Ok(id) if crate::world_data::dungeon_ids().contains(&id) => {
                                if !self.player.has_boss_key_for(id) {
                                    self.player.grant_boss_key_for(id);
                                    crate::log_info!("console_get_item item=boss_key_{id}");
                                    format!("Got BOSS KEY for dungeon {id}!")
                                } else {
                                    format!("Already have BOSS KEY for dungeon {id}.")
                                }
                            }
                            _ => format!("Unknown dungeon: {suffix}. Valid: boss_key_1 … boss_key_8"),
                        }
                    }
                    "keys" => {
                        self.player.keys += 1;
                        crate::log_info!("console_get_item item=keys");
                        "Got KEY!".to_string()
                    }
                    "gems" => {
                        self.player.gems += 10;
                        crate::log_info!("console_get_item item=gems");
                        "Got 10 GEMS!".to_string()
                    }
                    "dragon_pieces" => {
                        if self.player.dragon_pieces < 7 {
                            self.player.dragon_pieces = 7;
                            crate::log_info!("console_get_item item=dragon_pieces");
                            "Got all 7 DRAGON PIECES!".to_string()
                        } else {
                            "Already have all DRAGON PIECES.".to_string()
                        }
                    }
                    "ancient_key" => {
                        if !self.player.has_ancient_key {
                            self.player.has_ancient_key = true;
                            crate::log_info!("console_get_item item=ancient_key");
                            "Got ANCIENT KEY!".to_string()
                        } else {
                            "Already have ANCIENT KEY.".to_string()
                        }
                    }
                    "tide_chart" => {
                        if !self.player.has_tide_chart {
                            self.player.has_tide_chart = true;
                            crate::log_info!("console_get_item item=tide_chart");
                            "Got TIDE CHART!".to_string()
                        } else {
                            "Already have TIDE CHART.".to_string()
                        }
                    }
                    "ember_crystal" => {
                        if !self.player.has_strong_arm_glove {
                            "Need STRONG ARM GLOVE first.".to_string()
                        } else if !self.player.has_ember_crystal {
                            self.player.has_ember_crystal = true;
                            crate::log_info!("console_get_item item=ember_crystal");
                            "Got EMBER CRYSTAL!".to_string()
                        } else {
                            "Already have EMBER CRYSTAL.".to_string()
                        }
                    }
                    "void_compass" => {
                        if !self.player.has_void_compass {
                            self.player.has_void_compass = true;
                            crate::log_info!("console_get_item item=void_compass");
                            "Got VOID COMPASS!".to_string()
                        } else {
                            "Already have VOID COMPASS.".to_string()
                        }
                    }
                    "star_sigil" => {
                        if !self.player.has_star_sigil {
                            self.player.has_star_sigil = true;
                            crate::log_info!("console_get_item item=star_sigil");
                            "Got STAR SIGIL!".to_string()
                        } else {
                            "Already have STAR SIGIL.".to_string()
                        }
                    }
                    "dragon_codex" => {
                        if !self.player.has_dragon_codex {
                            self.player.has_dragon_codex = true;
                            crate::log_info!("console_get_item item=dragon_codex");
                            "Got DRAGON CODEX!".to_string()
                        } else {
                            "Already have DRAGON CODEX.".to_string()
                        }
                    }
                    "crystal_of_seeing" => {
                        if !self.player.has_crystal_of_seeing {
                            self.player.has_crystal_of_seeing = true;
                            crate::log_info!("console_get_item item=crystal_of_seeing");
                            "Got CRYSTAL OF SEEING!".to_string()
                        } else {
                            "Already have CRYSTAL OF SEEING.".to_string()
                        }
                    }
                    "ladder" => {
                        if !self.player.has_ladder {
                            self.player.has_ladder = true;
                            crate::log_info!("console_get_item item=ladder");
                            "Got LADDER!".to_string()
                        } else {
                            "Already have LADDER.".to_string()
                        }
                    }
                    "hammer" => {
                        if !self.player.has_hammer {
                            self.player.has_hammer = true;
                            self.auto_assign_item(EquippedItem::Hammer, ItemSlot::Side);
                            crate::log_info!("console_get_item item=hammer");
                            "Got HAMMER!".to_string()
                        } else {
                            "Already have HAMMER.".to_string()
                        }
                    }
                    "raft" => {
                        if !self.player.has_raft {
                            self.player.has_raft = true;
                            crate::log_info!("console_get_item item=raft");
                            "Got RAFT!".to_string()
                        } else {
                            "Already have RAFT.".to_string()
                        }
                    }
                    "lantern" => {
                        if !self.player.has_lantern {
                            self.player.has_lantern = true;
                            crate::log_info!("console_get_item item=lantern");
                            "Got LANTERN!".to_string()
                        } else {
                            "Already have LANTERN.".to_string()
                        }
                    }
                    "strong_arm_glove" | "strong_glove" => {
                        if !self.player.has_strong_arm_glove {
                            self.player.has_strong_arm_glove = true;
                            crate::log_info!("console_get_item item=strong_arm_glove");
                            "Got STRONG ARM GLOVE!".to_string()
                        } else {
                            "Already have STRONG ARM GLOVE.".to_string()
                        }
                    }
                    "portal_tool" => {
                        if !self.player.has_portal_tool {
                            self.player.has_portal_tool = true;
                            crate::log_info!("console_get_item item=portal_tool");
                            "Got PORTAL TOOL!".to_string()
                        } else {
                            "Already have PORTAL TOOL.".to_string()
                        }
                    }
                    _ => {
                        let valid_items = [
                            "sword",
                            "bombs",
                            "boss_key_1 … boss_key_8",
                            "keys",
                            "gems",
                            "dragon_pieces",
                            "ancient_key",
                            "tide_chart",
                            "ember_crystal",
                            "void_compass",
                            "star_sigil",
                            "dragon_codex",
                            "crystal_of_seeing",
                            "ladder",
                            "hammer",
                            "raft",
                            "lantern",
                            "strong_arm_glove",
                            "portal_tool",
                        ];
                        format!("Unknown item: {item_name}. Valid: {:?}", valid_items)
                    }
                }
            }
            "show_map_poi" => {
                self.world.dev_mode = !self.world.dev_mode;
                if self.world.dev_mode {
                    "Map POI markers enabled.".to_string()
                } else {
                    "Map POI markers disabled.".to_string()
                }
            }
            "time_speed" => {
                let arg = command[name.len()..].trim();
                match arg.parse::<i32>() {
                    Ok(n) if n >= 1 => {
                        self.time_speed = n;
                        self.time_tick = 0;
                        format!("Time speed set to {n}x")
                    }
                    _ => "Usage: time_speed <integer>  (e.g. time_speed 4)".to_string(),
                }
            }
            "set_time" => {
                let arg = command[name.len()..].trim();
                let parts: Vec<&str> = arg.splitn(2, ':').collect();
                match (parts.first(), parts.get(1)) {
                    (Some(h), Some(m)) => match (h.parse::<i32>(), m.parse::<i32>()) {
                        (Ok(h), Ok(m)) if (0..24).contains(&h) && (0..60).contains(&m) => {
                            self.time_minutes = h * 60 + m;
                            self.time_tick = 0;
                            format!("Time set to {:02}:{:02}", h, m)
                        }
                        _ => "Usage: set_time HH:MM  (e.g. set_time 14:30)".to_string(),
                    },
                    _ => "Usage: set_time HH:MM  (e.g. set_time 6:00)".to_string(),
                }
            }
            _ => format!("Unknown command: {command}"),
        }
    }

    fn teleport_to_screen(&mut self, screen_x: i32, screen_y: i32) -> Result<(), String> {
        self.world.in_dungeon = false;
        self.world.dungeon_id = 0;
        self.world.in_interior = false;
        self.world.interior_id.clear();
        self.world.load_screen(screen_x, screen_y);
        self.spawn_for_screen();
        self.reset_items();
        self.load_screen_items();

        let Some((spawn_x, spawn_y)) = self.find_console_spawn_position() else {
            return Err(format!(
                "No walkable spawn found on screen {},{}.",
                screen_x, screen_y
            ));
        };

        self.player.x = spawn_x;
        self.player.y = spawn_y;
        self.player.dir = Dir::Down;
        self.player.state = PlayerState::Idle;
        self.player.attack_timer = 0;
        self.player.invuln_timer = 0;
        self.player.hurt_timer = 0;
        self.player.knock_dx = 0.0;
        self.player.knock_dy = 0.0;
        self.player.last_axis = None;
        self.blocked_interaction = None;
        self.audio.play_music(MusicTrack::Overworld);
        Ok(())
    }

    fn find_console_spawn_position(&self) -> Option<(f32, f32)> {
        let hitbox = self.player.hitbox();
        let center_x = COLS as i32 / 2;
        let center_y = ROWS as i32 / 2;
        let mut best: Option<(f32, f32, i32)> = None;

        for tile_y in 0..ROWS as i32 {
            for tile_x in 0..COLS as i32 {
                let world_x = tile_x as f32 * TILE;
                let world_y = tile_y as f32 * TILE;
                if self.dynamic_collides(world_x + px(2.0), world_y + px(4.0), hitbox.w, hitbox.h) {
                    continue;
                }

                let distance = (tile_x - center_x).abs() + (tile_y - center_y).abs();
                if best.is_none_or(|(_, _, best_distance)| distance < best_distance) {
                    best = Some((world_x, world_y, distance));
                }
            }
        }

        best.map(|(x, y, _)| (x, y))
    }

    /// Called when all enemies in a dungeon room are defeated.
    fn on_dungeon_room_cleared(&mut self) {
        for reward in world_data::room_clear_rewards(
            self.world.dungeon_id,
            self.world.screen_x,
            self.world.screen_y,
        ) {
            self.spawn_pickup(
                reward.tile_x as f32 * TILE,
                reward.tile_y as f32 * TILE,
                reward.pickup_type,
            );
        }
    }

    fn update_room_props(&mut self) {
        use crate::model::PropKind;

        let plate_positions: Vec<(i32, i32)> = self
            .props
            .iter()
            .filter(|prop| matches!(prop.kind, PropKind::PressurePlate))
            .map(|prop| (prop.tile_x, prop.tile_y))
            .collect();
        if plate_positions.is_empty() || self.player.has_boss_key_for(self.world.dungeon_id) {
            return;
        }
        let all_pressed = plate_positions
            .iter()
            .all(|(x, y)| self.boulder_index_at(*x, *y).is_some());
        if !all_pressed {
            return;
        }
        let boss_key_present = self
            .pickups
            .iter()
            .any(|pickup| !pickup.collected && pickup.pickup_type == PickupType::BossKey);
        if boss_key_present {
            return;
        }
        if let Some((tile_x, tile_y)) = world_data::boss_key_spawn_tile(
            self.world.dungeon_id,
            self.world.screen_x,
            self.world.screen_y,
        ) {
            self.spawn_pickup(
                tile_x as f32 * TILE,
                tile_y as f32 * TILE + px(4.0),
                PickupType::BossKey,
            );
            self.show_message("A hidden panel slides open!");
        }
    }

    fn try_use_ladder_point(&mut self) -> bool {
        if !self.player.has_ladder {
            return false;
        }
        let tile_x = ((self.player.x + 8.0) / TILE).floor() as i32;
        let tile_y = ((self.player.y + 8.0) / TILE).floor() as i32;
        let Some(prop) = self.props.iter().find(|prop| {
            matches!(prop.kind, PropKind::LadderPoint)
                && prop.tile_x == tile_x
                && prop.tile_y == tile_y
                && prop.target_tile_x.is_some()
                && prop.target_tile_y.is_some()
        }) else {
            return false;
        };
        crate::log_debug!(
            "use_ladder_point from=({}, {}) to=({}, {})",
            tile_x,
            tile_y,
            prop.target_tile_x.unwrap(),
            prop.target_tile_y.unwrap()
        );
        self.player.x = prop.target_tile_x.unwrap() as f32 * TILE;
        self.player.y = prop.target_tile_y.unwrap() as f32 * TILE;
        self.show_message("You climb with the LADDER.");
        true
    }

    fn try_interact_npc(&mut self) -> bool {
        let (tile_x, tile_y, _) = self.front_tile();
        let Some(npc_kind) = self.props.iter().find_map(|prop| match prop.kind {
            PropKind::Npc(kind) if prop.tile_x == tile_x && prop.tile_y == tile_y => Some(kind),
            _ => None,
        }) else {
            return false;
        };
        crate::log_debug!(
            "interact_npc kind={:?} at=({}, {})",
            npc_kind,
            tile_x,
            tile_y
        );
        self.handle_npc(npc_kind);
        true
    }

    fn npc_state_key(&self, npc_kind: NpcKind) -> String {
        format!(
            "npc:{}:{npc_kind:?}",
            world_data::screen_key(self.world.screen_x, self.world.screen_y),
        )
    }

    fn spend_gems(&mut self, amount: i32) -> bool {
        if self.player.gems < amount {
            return false;
        }
        self.player.gems -= amount;
        self.audio.currency();
        true
    }

    fn handle_npc(&mut self, npc_kind: NpcKind) {
        let state_key = self.npc_state_key(npc_kind);
        let first_time = !self.world.opened_chests.contains_key(&state_key);
        match npc_kind {
            NpcKind::Elara => {
                if self.player.hp < self.player.max_hp {
                    if self.spend_gems(5) {
                        self.player.hp = self.player.max_hp;
                        self.show_message("Elara brews a forest tonic.\nFully healed for 5 gems.");
                    } else {
                        self.show_message("Elara: 5 gems for a healing tonic.");
                    }
                } else {
                    self.show_message(
                        "Elara: The Fen cave is old.\nCome back if you need healing.",
                    );
                }
            }
            NpcKind::Barnett => {
                self.show_message(
                    "Barnett: East leads to Ashenfall.\nNorth reaches the Highlands.",
                );
            }
            NpcKind::Maren => {
                if !self.player.has_hammer {
                    if self.spend_gems(24) {
                        self.player.has_hammer = true;
                        self.auto_assign_item(EquippedItem::Hammer, ItemSlot::Side);
                        self.show_message(
                            "Maren sells you a HAMMER.\nIt feels heavy and reliable.",
                        );
                    } else {
                        self.show_message("Maren: A HAMMER costs 24 gems.");
                    }
                } else if self.player.has_bombs && self.player.bomb_count < self.player.max_bombs {
                    if self.spend_gems(5) {
                        self.player.bomb_count =
                            (self.player.bomb_count + 8).min(self.player.max_bombs);
                        self.show_message("Maren refills your bomb satchel for 5 gems.");
                    } else {
                        self.show_message("Maren: 5 gems for a bomb refill.");
                    }
                } else {
                    self.show_message("Maren: The east district post still reeks of old ash.");
                }
            }
            NpcKind::Oswin => {
                self.show_message(
                    "Oswin: Ringing the old bell changes things.\nThe ruins reward curiosity.",
                );
            }
            NpcKind::Corvin => {
                if !self.player.has_ancient_key {
                    if self.spend_gems(40) {
                        self.player.has_ancient_key = true;
                        self.show_message("Corvin sells you the ANCIENT KEY for 40 gems.");
                    } else {
                        self.show_message("Corvin: The ANCIENT KEY is 40 gems.");
                    }
                } else {
                    self.show_message("Corvin: The vents are still burning beneath the Vault.");
                }
            }
            NpcKind::Petra => {
                self.show_message(
                    "Petra: Watch the vent timings.\nThe biggest one always pulses hottest.",
                );
            }
            NpcKind::Aldric => {
                if !self.player.has_raft {
                    if self.spend_gems(35) {
                        self.player.has_raft = true;
                        self.show_message("Aldric sells you a RAFT for 35 gems.");
                    } else {
                        self.show_message("Aldric: A serviceable RAFT costs 35 gems.");
                    }
                } else {
                    self.show_message("Aldric: Check the Tide Chart before diving the gate.");
                }
            }
            NpcKind::Sael => {
                if self.player.max_bombs < crate::config::get().combat.bomb_max_capacity {
                    if self.spend_gems(20) {
                        self.player.max_bombs = crate::config::get().combat.bomb_max_capacity;
                        self.player.bomb_count = self
                            .player
                            .bomb_count
                            .max(crate::config::get().combat.bomb_starting_ammo);
                        self.show_message("Sael upgrades your bomb bag.\nMax bombs increased!");
                    } else {
                        self.show_message("Sael: Deep gear isn't cheap.\n20 gems for the upgrade.");
                    }
                } else {
                    self.show_message("Sael: Dawn water shows the Citadel's shape from above.");
                }
            }
            NpcKind::Dax => {
                if !self.player.has_strong_arm_glove {
                    if self.spend_gems(45) {
                        self.player.has_strong_arm_glove = true;
                        self.show_message("Dax sells you the STRONG ARM GLOVE for 45 gems.");
                    } else {
                        self.show_message("Dax: The STRONG ARM GLOVE costs 45 gems.");
                    }
                } else {
                    self.show_message("Dax: The true crystal is the one that feels alive.");
                }
            }
            NpcKind::Vel => {
                self.show_message(
                    "Vel: Stable rifts point the way.\nFollow the shimmer to the Shade.",
                );
            }
            NpcKind::CelestialMerchant => {
                if !self.player.has_star_sigil {
                    if self.spend_gems(60) {
                        self.player.has_star_sigil = true;
                        self.show_message("The Celestial Merchant sells you the STAR SIGIL.");
                    } else {
                        self.show_message("Celestial Merchant: The STAR SIGIL is 60 gems.");
                    }
                } else {
                    self.show_message("Celestial Merchant: The Spire has your measure now.");
                }
            }
            NpcKind::Senna => {
                self.show_message("Senna: The dark seventh star isn't gone.\nIt's waiting.");
            }
            NpcKind::Wren => {
                if !self.player.has_dragon_codex {
                    if self.spend_gems(30) {
                        self.player.has_dragon_codex = true;
                        self.show_message("Wren pieces together the DRAGON CODEX.");
                    } else {
                        self.show_message("Wren: The final codex pages cost 30 gems.");
                    }
                } else if first_time {
                    self.show_message(
                        "Wren: You've come far.\nThe rest is between you and the mountain.",
                    );
                } else {
                    self.show_message("Wren keeps the last camp warm and quiet.");
                }
            }
        }
        if first_time {
            self.world.opened_chests.insert(state_key, vec![]);
        }
    }

    fn show_message(&mut self, text: &str) {
        self.message_text = text.to_string();
        self.audio.message();
        self.state = GameState::Message;
        crate::log_debug!("show_message text={:?}", text.replace('\n', " | "));
    }

    fn spawn_pickup(&mut self, x: f32, y: f32, pickup_type: PickupType) {
        self.spawn_pickup_internal(x, y, pickup_type, None);
    }

    fn spawn_pickup_with_key(&mut self, x: f32, y: f32, pickup_type: PickupType, key: String) {
        self.spawn_pickup_internal(x, y, pickup_type, Some(key));
    }

    fn spawn_pickup_internal(
        &mut self,
        x: f32,
        y: f32,
        pickup_type: PickupType,
        key: Option<String>,
    ) {
        self.pickups.push(Pickup {
            x,
            y,
            w: px(10.0),
            h: px(10.0),
            pickup_type,
            key,
            timer: 0,
            collected: false,
        });
    }

    fn screen_pickup_state_key(&self, tile_x: usize, tile_y: usize) -> String {
        format!(
            "pickup:{}:{}:{}",
            self.world.cleared_room_key(),
            tile_x,
            tile_y
        )
    }

    fn player_already_has_pickup(&self, pickup_type: PickupType) -> bool {
        match pickup_type {
            PickupType::Sword => self.player.has_sword,
            PickupType::TideChart => self.player.has_tide_chart,
            PickupType::EmberCrystal => self.player.has_ember_crystal,
            PickupType::VoidCompass => self.player.has_void_compass,
            PickupType::CrystalOfSeeing => self.player.has_crystal_of_seeing,
            _ => false,
        }
    }

    fn spawn_bomb(&mut self, x: f32, y: f32) {
        self.bombs.push(Bomb {
            x,
            y,
            w: px(12.0),
            h: px(12.0),
            timer: crate::config::get().combat.bomb_fuse_frames,
            exploded: false,
            explosion_timer: 0,
        });
    }

    fn spawn_projectile(&mut self, x: f32, y: f32, dx: f32, dy: f32, from_enemy: bool) {
        self.projectiles.push(Projectile {
            x,
            y,
            w: px(24.0),
            h: px(24.0),
            dx,
            dy,
            from_enemy,
            active: true,
            timer: 120,
        });
    }
}

fn create_enemy(spawn: EnemySpawn) -> Enemy {
    let ecfg = &crate::config::get().enemies;
    let (hp, speed, w, h, shoot_cooldown) = match spawn.enemy_type {
        EnemyType::Slime => (
            ecfg.slime.hp,
            ecfg.slime.speed * PIXEL_SCALE,
            px(12.0),
            px(12.0),
            0,
        ),
        EnemyType::Octorok => (
            ecfg.octorok.hp,
            ecfg.octorok.speed * PIXEL_SCALE,
            px(14.0),
            px(14.0),
            120,
        ),
        EnemyType::Bat => (
            ecfg.bat.hp,
            ecfg.bat.speed * PIXEL_SCALE,
            px(10.0),
            px(10.0),
            0,
        ),
        EnemyType::Darknut => (
            ecfg.darknut.hp,
            ecfg.darknut.speed * PIXEL_SCALE,
            px(14.0),
            px(14.0),
            0,
        ),
        EnemyType::Boss => (
            ecfg.boss.hp,
            ecfg.boss.speed * PIXEL_SCALE,
            px(24.0),
            px(24.0),
            60,
        ),
    };
    Enemy {
        enemy_type: spawn.enemy_type,
        x: spawn.x,
        y: spawn.y,
        w,
        h,
        hp,
        max_hp: hp,
        speed,
        dir: Dir::Down,
        move_timer: 0,
        hurt_timer: 0,
        knock_x: 0.0,
        knock_y: 0.0,
        flash_timer: 0,
        shoot_cooldown,
        active: true,
        timer: 0,
        vx: 0.0,
        vy: 0.0,
    }
}

fn random_dir() -> Dir {
    match rand::gen_range(0, 4) {
        0 => Dir::Up,
        1 => Dir::Down,
        2 => Dir::Left,
        _ => Dir::Right,
    }
}

fn move_enemy(enemy: &mut Enemy, world: &World) {
    let mut dx = 0.0;
    let mut dy = 0.0;
    let scaled = enemy.speed * 0.6;
    match enemy.dir {
        Dir::Up => dy = -scaled * 0.3,
        Dir::Down => dy = scaled,
        Dir::Left => dx = -scaled,
        Dir::Right => dx = scaled,
    }
    let nx = enemy.x + dx;
    let ny = enemy.y + dy;
    let margin = if enemy.enemy_type == EnemyType::Boss {
        TILE * 2.0
    } else {
        TILE
    };
    if nx >= margin
        && nx + enemy.w <= GAME_W - margin
        && !world.collides(nx, enemy.y, enemy.w, enemy.h)
    {
        enemy.x = nx;
    } else {
        enemy.move_timer = 0;
    }
    if ny >= margin
        && ny + enemy.h <= GAME_H - margin
        && !world.collides(enemy.x, ny, enemy.w, enemy.h)
    {
        enemy.y = ny;
    } else {
        enemy.move_timer = 0;
    }
}

fn ai_slime(enemy: &mut Enemy, world: &World) {
    enemy.move_timer -= 1;
    if enemy.move_timer <= 0 {
        enemy.dir = random_dir();
        enemy.move_timer = 30 + rand::gen_range(0, 60);
        if rand::gen_range(0.0, 1.0) < 0.3 {
            return;
        }
    }
    move_enemy(enemy, world);
}

fn ai_octorok(
    enemy: &mut Enemy,
    world: &World,
    player_x: f32,
    player_y: f32,
) -> Option<(f32, f32, f32, f32)> {
    enemy.move_timer -= 1;
    enemy.shoot_cooldown -= 1;
    if enemy.shoot_cooldown <= 0 {
        let dx = player_x - enemy.x;
        let dy = player_y - enemy.y;
        let dist = vec2(dx, dy).length();
        enemy.shoot_cooldown = 150 + rand::gen_range(0, 90);
        enemy.move_timer = 30;
        if dist > 0.0 && dist < px(100.0) {
            return Some((
                enemy.x + enemy.w / 2.0 - px(3.0),
                enemy.y + enemy.h / 2.0 - px(3.0),
                dx / dist * 0.4 * PIXEL_SCALE,
                dy / dist * 0.4 * PIXEL_SCALE,
            ));
        }
    }
    if enemy.move_timer <= 0 {
        enemy.dir = random_dir();
        enemy.move_timer = 40 + rand::gen_range(0, 40);
    }
    move_enemy(enemy, world);
    None
}

fn ai_bat(enemy: &mut Enemy, px: f32, py: f32) {
    enemy.move_timer -= 1;
    if enemy.move_timer <= 0 {
        let dx = px - enemy.x + rand::gen_range(-40.0, 40.0);
        let dy = py - enemy.y + rand::gen_range(-40.0, 40.0);
        let dist = vec2(dx, dy).length();
        if dist > 0.0 {
            enemy.vx = dx / dist * enemy.speed;
            enemy.vy = dy / dist * enemy.speed;
        }
        enemy.move_timer = 20 + rand::gen_range(0, 30);
    }
    enemy.x = (enemy.x + enemy.vx).clamp(TILE, GAME_W - TILE - enemy.w);
    enemy.y = (enemy.y + enemy.vy).clamp(TILE, GAME_H - TILE - enemy.h);
}

fn ai_darknut(enemy: &mut Enemy, world: &World, px: f32, py: f32) {
    enemy.move_timer -= 1;
    if enemy.move_timer <= 0 {
        let dx = px - enemy.x;
        let dy = py - enemy.y;
        enemy.dir = if dx.abs() > dy.abs() {
            if dx > 0.0 { Dir::Right } else { Dir::Left }
        } else if dy > 0.0 {
            Dir::Down
        } else {
            Dir::Up
        };
        enemy.move_timer = 15 + rand::gen_range(0, 20);
    }
    move_enemy(enemy, world);
}

fn ai_boss(
    enemy: &mut Enemy,
    world: &World,
    player_x: f32,
    player_y: f32,
) -> Vec<(f32, f32, f32, f32)> {
    let mut shots = vec![];
    enemy.move_timer -= 1;
    enemy.shoot_cooldown -= 1;
    if enemy.shoot_cooldown <= 0 {
        let base_angle = (player_y - enemy.y).atan2(player_x - enemy.x);
        for offset in [-0.3f32, 0.0, 0.3] {
            let angle = base_angle + offset;
            shots.push((
                enemy.x + enemy.w / 2.0 - px(3.0),
                enemy.y + enemy.h / 2.0 - px(3.0),
                angle.cos() * 0.35 * PIXEL_SCALE,
                angle.sin() * 0.35 * PIXEL_SCALE,
            ));
        }
        enemy.shoot_cooldown = 140 + rand::gen_range(0, 80);
    }
    if enemy.move_timer <= 0 {
        let dx = player_x - enemy.x + rand::gen_range(-30.0, 30.0);
        let dy = player_y - enemy.y + rand::gen_range(-30.0, 30.0);
        enemy.dir = if dx.abs() > dy.abs() {
            if dx > 0.0 { Dir::Right } else { Dir::Left }
        } else if dy > 0.0 {
            Dir::Down
        } else {
            Dir::Up
        };
        enemy.move_timer = 30 + rand::gen_range(0, 40);
    }
    move_enemy(enemy, world);
    shots
}

fn start_pressed() -> bool {
    is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Space)
}

fn inventory_pressed() -> bool {
    is_key_pressed(KeyCode::I) || is_key_pressed(KeyCode::Tab)
}

fn console_toggle_pressed() -> bool {
    is_key_pressed(KeyCode::GraveAccent)
}

fn console_help_text() -> &'static str {
    "Commands: help | teleport x,y | god_mode 0/1 | get_item <item_name> | set_time HH:MM | time_speed <n> | show_map_poi"
}

fn drain_char_input() {
    while get_char_pressed().is_some() {}
}

fn pickup_value(pickup: PickupType) -> i32 {
    match pickup {
        PickupType::Gem => 5,
        _ => 0,
    }
}

fn pickup_times_out(pickup: PickupType) -> bool {
    matches!(
        pickup,
        PickupType::Heart | PickupType::BombAmmo | PickupType::Gem
    )
}
