use crate::audio::{Audio, MusicTrack};
use crate::constants::{
    ATTACK_DURATION, COLS, GAME_H, GAME_W, KNOCKBACK_FRAMES, KNOCKBACK_SPEED, PIXEL_SCALE,
    PLAYER_SPEED, ROWS, TILE, TRANS_SPEED, WORLD_H, WORLD_W,
};
use crate::model::{
    Bomb, Dir, Enemy, EnemySpawn, EnemyType, GameState, Pickup, PickupType, Player, PlayerState,
    Projectile, TileType, Transition,
};
use crate::render;
use crate::sprites::Sprites;
use crate::world::World;
use macroquad::prelude::*;

fn px(value: f32) -> f32 {
    value * PIXEL_SCALE
}

pub struct Game {
    pub state: GameState,
    pub frame: i32,
    pub transition: Transition,
    pub message_text: String,
    pub dungeon_overworld_x: i32,
    pub dungeon_overworld_y: i32,
    pub player: Player,
    pub world: World,
    pub enemies: Vec<Enemy>,
    pub pickups: Vec<Pickup>,
    pub bombs: Vec<Bomb>,
    pub projectiles: Vec<Projectile>,
    pub audio: Audio,
    pub sprites: Sprites,
}

impl Game {
    pub async fn new() -> Self {
        let mut game = Self {
            state: GameState::Title,
            frame: 0,
            transition: Transition::default(),
            message_text: String::new(),
            dungeon_overworld_x: 0,
            dungeon_overworld_y: 2,
            player: Player::new(),
            world: World::new(),
            enemies: vec![],
            pickups: vec![],
            bombs: vec![],
            projectiles: vec![],
            audio: Audio::load().await,
            sprites: Sprites::load().await,
        };
        game.spawn_for_screen();
        game
    }

    pub fn update(&mut self) {
        self.frame += 1;
        match self.state {
            GameState::Title => {
                self.audio.play_music(MusicTrack::Title);
                if start_pressed() {
                    self.start_new_game();
                }
            }
            GameState::Playing => self.update_playing(),
            GameState::Transition => self.update_transition(),
            GameState::DungeonEnter => self.update_dungeon_enter(),
            GameState::DungeonExit => self.update_dungeon_exit(),
            GameState::Message => {
                if start_pressed() {
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver | GameState::Victory => {
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Z) {
                    self.state = GameState::Title;
                    self.frame = 0;
                }
            }
        }
    }

    pub fn draw(&self) {
        match self.state {
            GameState::Title => render::draw_title(self.frame),
            GameState::Playing => self.draw_game(),
            GameState::Transition => render::draw_transition(
                &self.sprites,
                &self.world.snapshot(),
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
        }
    }

    fn draw_game(&self) {
        render::draw_game(
            &self.sprites,
            &self.world.snapshot(),
            &self.player,
            &self.enemies,
            &self.pickups,
            &self.bombs,
            &self.projectiles,
        );
    }

    fn start_new_game(&mut self) {
        self.world = World::new();
        self.player = Player::new();
        self.spawn_for_screen();
        self.reset_items();
        self.load_screen_items();
        self.audio.play_music(MusicTrack::Overworld);
        self.state = GameState::Playing;
        self.frame = 0;
    }

    fn update_playing(&mut self) {
        if let Some((dir, nx, ny)) = self.update_player() {
            self.start_transition(dir, nx, ny);
            return;
        }
        match self.check_tile_interaction() {
            Some("enter_dungeon") => {
                self.enter_dungeon();
                return;
            }
            Some("exit_dungeon") => {
                self.exit_dungeon();
                return;
            }
            Some("cave_interact") if start_pressed() => {
                self.handle_cave();
                return;
            }
            Some("victory") => {
                self.state = GameState::Victory;
                self.frame = 0;
                return;
            }
            _ => {}
        }
        self.update_enemies();
        self.update_items();
        self.check_damage();
        self.check_pickups();
        if self.player.hp <= 0 {
            self.state = GameState::GameOver;
            self.frame = 0;
        }
    }

    fn update_transition(&mut self) {
        self.transition.progress += TRANS_SPEED;
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
        }
    }

    fn update_dungeon_enter(&mut self) {
        self.transition.progress += 4.0;
        if self.transition.progress >= 60.0 {
            self.world.enter_dungeon();
            self.player.x = 7.0 * TILE;
            self.player.y = 9.0 * TILE;
            self.player.dir = Dir::Up;
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
            if !self
                .world
                .collides(nx + px(2.0), self.player.y + px(4.0), hb.w, hb.h)
            {
                self.player.x = nx;
            }
            if !self
                .world
                .collides(self.player.x + px(2.0), ny + px(4.0), hb.w, hb.h)
            {
                self.player.y = ny;
            }
            self.player.x = self.player.x.clamp(0.0, GAME_W - TILE);
            self.player.y = self.player.y.clamp(0.0, GAME_H - TILE);
            return None;
        }
        if self.player.attack_timer > 0 {
            self.player.attack_timer -= 1;
            if self.player.attack_timer == ATTACK_DURATION - 3 {
                self.sword_hit_check();
            }
            if self.player.attack_timer <= 0 {
                self.player.state = PlayerState::Idle;
            }
            return None;
        }
        if (is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Space)) && self.player.has_sword {
            self.player.begin_attack();
            self.audio.sword();
            return None;
        }
        if is_key_pressed(KeyCode::X) && self.player.has_bombs && self.player.bomb_count > 0 {
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
        let mut dx = 0.0;
        let mut dy = 0.0;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            dy = -PLAYER_SPEED;
            self.player.dir = Dir::Up;
            self.player.last_axis = Some('y');
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            dy = PLAYER_SPEED;
            self.player.dir = Dir::Down;
            self.player.last_axis = Some('y');
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dx = -PLAYER_SPEED;
            self.player.dir = Dir::Left;
            self.player.last_axis = Some('x');
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dx = PLAYER_SPEED;
            self.player.dir = Dir::Right;
            self.player.last_axis = Some('x');
        }
        if dx != 0.0 && dy != 0.0 {
            let factor = 1.0 / 2.0f32.sqrt();
            dx *= factor;
            dy *= factor;
        }
        if dx != 0.0 || dy != 0.0 {
            self.player.state = PlayerState::Walking;
            self.player.walk_timer += 1;
            if self.player.walk_timer >= 8 {
                self.player.walk_timer = 0;
                self.player.walk_frame = (self.player.walk_frame + 1) % 2;
            }
            let hb = self.player.hitbox();
            let new_x = self.player.x + dx;
            let new_y = self.player.y + dy;
            if !self
                .world
                .collides(new_x + px(2.0), self.player.y + px(4.0), hb.w, hb.h)
            {
                self.player.x = new_x;
            }
            if !self
                .world
                .collides(self.player.x + px(2.0), new_y + px(4.0), hb.w, hb.h)
            {
                self.player.y = new_y;
            }
            let (_, _, front_tile) = self.front_tile();
            if front_tile == TileType::DoorLocked && self.player.keys > 0 {
                self.player.keys -= 1;
                self.open_all_doors(TileType::DoorLocked);
                self.audio.door();
                self.show_message("Door opened!");
            }
            if front_tile == TileType::BossDoor && self.player.has_boss_key {
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

    fn check_tile_interaction(&self) -> Option<&'static str> {
        let cx = ((self.player.x + 8.0) / TILE).floor() as i32;
        let cy = ((self.player.y + 8.0) / TILE).floor() as i32;
        match self.world.get_tile(cx, cy) {
            TileType::Dungeon => Some("enter_dungeon"),
            TileType::Stairs => Some("exit_dungeon"),
            TileType::Cave => Some("cave_interact"),
            TileType::Goal => Some("victory"),
            _ => None,
        }
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

    fn sword_hit_check(&mut self) {
        let sword = match self.player.dir {
            Dir::Up => Rect::new(
                self.player.x + px(3.0),
                self.player.y - px(12.0),
                px(10.0),
                px(14.0),
            ),
            Dir::Down => Rect::new(
                self.player.x + px(3.0),
                self.player.y + px(14.0),
                px(10.0),
                px(14.0),
            ),
            Dir::Left => Rect::new(
                self.player.x - px(12.0),
                self.player.y + px(3.0),
                px(14.0),
                px(10.0),
            ),
            Dir::Right => Rect::new(
                self.player.x + px(14.0),
                self.player.y + px(3.0),
                px(14.0),
                px(10.0),
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
                if self.world.in_dungeon {
                    TileType::Floor
                } else {
                    TileType::Grass
                },
            );
            if rand::gen_range(0.0, 1.0) < 0.2 {
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
                self.player.bomb_count = 8;
                self.show_message("You found BOMBS!\nPress X to use.");
            }
            PickupType::HeartContainer => {
                self.player.max_hp += 2;
                self.player.hp = self.player.max_hp;
                self.show_message("Heart Container!\nHP increased!");
            }
            PickupType::BombAmmo => {
                self.player.bomb_count = (self.player.bomb_count + 4).min(self.player.max_bombs);
                self.show_message("Found 4 bombs!");
            }
            PickupType::Key => {
                self.player.keys += 1;
                self.show_message("Found a KEY!");
            }
            PickupType::BossKey | PickupType::Heart => {}
        }
        self.audio.pickup();
    }

    fn open_all_doors(&mut self, tile_type: TileType) {
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.world.tiles[row][col] == tile_type {
                    self.world.destroy_tile(
                        col,
                        row,
                        if self.world.in_dungeon {
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
        let max_x = if self.world.in_dungeon {
            2
        } else {
            WORLD_W - 1
        };
        let max_y = if self.world.in_dungeon {
            2
        } else {
            WORLD_H - 1
        };
        if next_x < 0
            || next_y < 0
            || next_x > max_x
            || next_y > max_y
            || !self.world.screen_exists(next_x, next_y)
        {
            self.player.x = self.player.x.clamp(0.0, GAME_W - TILE);
            self.player.y = self.player.y.clamp(0.0, GAME_H - TILE);
            return;
        }
        self.transition = Transition {
            dir: Some(dir),
            progress: 0.0,
            old_tiles: self.world.tiles.clone(),
            new_screen_x: next_x,
            new_screen_y: next_y,
            player_new_x: nx,
            player_new_y: ny,
        };
        self.state = GameState::Transition;
    }

    fn enter_dungeon(&mut self) {
        self.dungeon_overworld_x = self.world.screen_x;
        self.dungeon_overworld_y = self.world.screen_y;
        self.transition.progress = 0.0;
        self.state = GameState::DungeonEnter;
    }

    fn exit_dungeon(&mut self) {
        self.transition.progress = 0.0;
        self.state = GameState::DungeonExit;
    }

    fn handle_cave(&mut self) {
        match format!("{},{}", self.world.screen_x, self.world.screen_y).as_str() {
            "1,1" => {
                if !self.player.has_sword {
                    self.player.has_sword = true;
                    self.show_message("You found a sword!\nUse it to fight enemies.");
                } else {
                    self.show_message("The cave is empty.");
                }
            }
            "0,0" => {
                let key = "o:0,0".to_string();
                if !self.world.opened_chests.contains_key(&key) {
                    self.world.opened_chests.insert(key, vec![(3, 4)]);
                    self.player.max_hp += 2;
                    self.player.hp = self.player.max_hp;
                    self.show_message("Heart Container!\nHP increased!");
                } else {
                    self.show_message("The cave is empty.");
                }
            }
            _ => self.show_message("A mysterious\ncave..."),
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
            if pickup.pickup_type != PickupType::Key
                && pickup.pickup_type != PickupType::BossKey
                && pickup.timer > 600
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
                    bomb.explosion_timer = 20;
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
                if vec2(tx - cx, ty - cy).length() < 24.0
                    && self.world.tiles[row][col] == TileType::Cracked
                {
                    self.world.destroy_tile(
                        col,
                        row,
                        if self.world.in_dungeon {
                            TileType::Floor
                        } else {
                            TileType::Path
                        },
                    );
                }
            }
        }
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
        if self.player.invuln_timer > 0 {
            return;
        }
        self.player.hp -= amount;
        self.player.invuln_timer = 60;
        self.player.hurt_timer = KNOCKBACK_FRAMES;
        self.player.state = PlayerState::Hurt;
        self.player.knock_dx = 0.0;
        self.player.knock_dy = 0.0;
        match from_dir {
            Dir::Up => self.player.knock_dy = KNOCKBACK_SPEED,
            Dir::Down => self.player.knock_dy = -KNOCKBACK_SPEED,
            Dir::Left => self.player.knock_dx = KNOCKBACK_SPEED,
            Dir::Right => self.player.knock_dx = -KNOCKBACK_SPEED,
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
        let roll = rand::gen_range(0.0, 1.0);
        match enemy.enemy_type {
            EnemyType::Boss => {
                self.spawn_pickup(enemy.x + 4.0, enemy.y + 4.0, PickupType::HeartContainer);
                self.spawn_pickup(enemy.x + 16.0, enemy.y + 4.0, PickupType::BossKey);
            }
            _ if roll < 0.25 => self.spawn_pickup(enemy.x, enemy.y, PickupType::Heart),
            _ if roll < 0.35 => self.spawn_pickup(enemy.x, enemy.y, PickupType::BombAmmo),
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
                self.world
                    .cleared_rooms
                    .insert(format!("d:{},{}", self.world.screen_x, self.world.screen_y));
                if self.world.screen_x == 0 && self.world.screen_y == 1 {
                    self.spawn_pickup(7.0 * TILE, 5.0 * TILE, PickupType::Key);
                }
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
        let collected: Vec<PickupType> = self
            .pickups
            .iter_mut()
            .filter_map(|pickup| {
                if !pickup.collected
                    && player_rect.overlaps(&Rect::new(pickup.x, pickup.y, pickup.w, pickup.h))
                {
                    pickup.collected = true;
                    Some(pickup.pickup_type)
                } else {
                    None
                }
            })
            .collect();
        for pickup in collected {
            self.collect_pickup(pickup);
        }
    }

    fn collect_pickup(&mut self, pickup: PickupType) {
        match pickup {
            PickupType::Heart => self.player.hp = (self.player.hp + 2).min(self.player.max_hp),
            PickupType::HeartContainer => {
                self.player.max_hp += 2;
                self.player.hp = self.player.max_hp;
                self.show_message("Heart Container!\nHP increased!");
            }
            PickupType::Key => self.player.keys += 1,
            PickupType::BossKey => {
                self.player.has_boss_key = true;
                self.show_message("Boss Key found!");
            }
            PickupType::BombAmmo => {
                if self.player.has_bombs {
                    self.player.bomb_count =
                        (self.player.bomb_count + 4).min(self.player.max_bombs);
                }
            }
            PickupType::Bombs => {
                self.player.has_bombs = true;
                self.player.bomb_count = 8;
                self.show_message("You found BOMBS!\nPress X to use.");
            }
        }
        if pickup == PickupType::Key {
            self.audio.currency();
        } else {
            self.audio.pickup();
        }
    }

    fn spawn_for_screen(&mut self) {
        self.enemies = self
            .world
            .get_enemy_spawns()
            .into_iter()
            .map(create_enemy)
            .collect();
    }

    fn reset_items(&mut self) {
        self.pickups.clear();
        self.bombs.clear();
        self.projectiles.clear();
    }

    fn load_screen_items(&mut self) {
        if self.world.in_dungeon
            && self.world.screen_x == 2
            && self.world.screen_y == 1
            && !self.player.has_boss_key
        {
            self.spawn_pickup(8.0 * TILE, 5.0 * TILE + px(4.0), PickupType::BossKey);
        }
    }

    fn show_message(&mut self, text: &str) {
        self.message_text = text.to_string();
        self.audio.message();
        self.state = GameState::Message;
    }

    fn spawn_pickup(&mut self, x: f32, y: f32, pickup_type: PickupType) {
        self.pickups.push(Pickup {
            x,
            y,
            w: px(10.0),
            h: px(10.0),
            pickup_type,
            timer: 0,
            collected: false,
        });
    }

    fn spawn_bomb(&mut self, x: f32, y: f32) {
        self.bombs.push(Bomb {
            x,
            y,
            w: px(12.0),
            h: px(12.0),
            timer: 90,
            exploded: false,
            explosion_timer: 0,
        });
    }

    fn spawn_projectile(&mut self, x: f32, y: f32, dx: f32, dy: f32, from_enemy: bool) {
        self.projectiles.push(Projectile {
            x,
            y,
            w: px(6.0),
            h: px(6.0),
            dx,
            dy,
            from_enemy,
            active: true,
            timer: 120,
        });
    }
}

fn create_enemy(spawn: EnemySpawn) -> Enemy {
    let (hp, speed, w, h, shoot_cooldown) = match spawn.enemy_type {
        EnemyType::Slime => (1, 0.4 * PIXEL_SCALE, px(12.0), px(12.0), 0),
        EnemyType::Octorok => (2, 0.6 * PIXEL_SCALE, px(14.0), px(14.0), 120),
        EnemyType::Bat => (1, 0.4 * PIXEL_SCALE, px(10.0), px(10.0), 0),
        EnemyType::Darknut => (3, 0.7 * PIXEL_SCALE, px(14.0), px(14.0), 0),
        EnemyType::Boss => (12, 0.5 * PIXEL_SCALE, px(24.0), px(24.0), 60),
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
        enemy.shoot_cooldown = 90 + rand::gen_range(0, 60);
        enemy.move_timer = 30;
        if dist > 0.0 && dist < px(100.0) {
            return Some((
                enemy.x + enemy.w / 2.0 - px(3.0),
                enemy.y + enemy.h / 2.0 - px(3.0),
                dx / dist * 0.9 * PIXEL_SCALE,
                dy / dist * 0.9 * PIXEL_SCALE,
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
                angle.cos() * 0.8 * PIXEL_SCALE,
                angle.sin() * 0.8 * PIXEL_SCALE,
            ));
        }
        enemy.shoot_cooldown = 70 + rand::gen_range(0, 40);
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
