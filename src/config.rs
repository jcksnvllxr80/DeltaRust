use serde::Deserialize;
use std::path::PathBuf;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

/// Application configuration loaded from `config.toml`.
///
/// Every field has a sensible default so the game works even without a config
/// file.  Missing sections or keys are silently filled with defaults.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: General,
    pub audio: Audio,
    pub player: PlayerCfg,
    pub combat: Combat,
    pub enemies: Enemies,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct General {
    pub log_level: String,
    pub log_dir: String,
    pub log_time_format: String,
    pub dev_mode: bool,
    pub all_items: bool,
    pub full_hearts: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Audio {
    pub music_volume: f32,
    pub sfx_volume: f32,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct PlayerCfg {
    /// Multiplier applied to PIXEL_SCALE for movement speed.
    pub speed: f32,
    pub max_hp: i32,
    pub full_hearts_hp: i32,
    pub attack_frames: i32,
    pub invuln_frames: i32,
    pub knockback_speed: f32,
    pub knockback_frames: i32,
    pub transition_speed: f32,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Combat {
    pub bomb_fuse_frames: i32,
    pub bomb_explosion_frames: i32,
    pub bomb_radius: f32,
    pub bomb_starting_ammo: i32,
    pub bomb_chest_ammo: i32,
    pub bomb_max_capacity: i32,
    pub bush_heart_drop_chance: f32,
    pub enemy_heart_drop_chance: f32,
    pub pickup_lifetime_frames: i32,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct EnemyStats {
    pub hp: i32,
    /// Multiplier applied to PIXEL_SCALE for movement speed.
    pub speed: f32,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Enemies {
    pub slime: EnemyStats,
    pub octorok: EnemyStats,
    pub bat: EnemyStats,
    pub darknut: EnemyStats,
    pub boss: EnemyStats,
}

// ---------------------------------------------------------------------------
// Defaults
// ---------------------------------------------------------------------------

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General::default(),
            audio: Audio::default(),
            player: PlayerCfg::default(),
            combat: Combat::default(),
            enemies: Enemies::default(),
        }
    }
}

impl Default for General {
    fn default() -> Self {
        Self {
            log_level: "info".into(),
            log_dir: "logs".into(),
            log_time_format: "YYYY/MM/DDTHH:MM:SS.sssZ".into(),
            dev_mode: false,
            all_items: false,
            full_hearts: false,
        }
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            music_volume: 0.45,
            sfx_volume: 0.7,
        }
    }
}

impl Default for PlayerCfg {
    fn default() -> Self {
        Self {
            speed: 0.8,
            max_hp: 6,
            full_hearts_hp: 32,
            attack_frames: 12,
            invuln_frames: 60,
            knockback_speed: 2.5,
            knockback_frames: 10,
            transition_speed: 14.0,
        }
    }
}

impl Default for Combat {
    fn default() -> Self {
        Self {
            bomb_fuse_frames: 90,
            bomb_explosion_frames: 20,
            bomb_radius: 24.0,
            bomb_starting_ammo: 8,
            bomb_chest_ammo: 4,
            bomb_max_capacity: 16,
            bush_heart_drop_chance: 0.2,
            enemy_heart_drop_chance: 0.25,
            pickup_lifetime_frames: 600,
        }
    }
}

impl Default for EnemyStats {
    fn default() -> Self {
        Self { hp: 1, speed: 0.4 }
    }
}

impl Default for Enemies {
    fn default() -> Self {
        Self {
            slime: EnemyStats { hp: 1, speed: 0.4 },
            octorok: EnemyStats { hp: 2, speed: 0.6 },
            bat: EnemyStats { hp: 1, speed: 0.4 },
            darknut: EnemyStats { hp: 3, speed: 0.7 },
            boss: EnemyStats { hp: 12, speed: 0.5 },
        }
    }
}

// ---------------------------------------------------------------------------
// Loading
// ---------------------------------------------------------------------------

/// Try to locate `config.toml` next to the executable, then fall back to the
/// project root (CARGO_MANIFEST_DIR at compile time).  Returns `Config::default()`
/// if the file doesn't exist or can't be parsed.
pub fn load() -> Config {
    let candidates = config_candidates();
    for path in &candidates {
        if path.is_file() {
            match std::fs::read_to_string(path) {
                Ok(text) => match toml::from_str::<Config>(&text) {
                    Ok(cfg) => {
                        crate::log_info!("config loaded from {}", path.display());
                        return cfg;
                    }
                    Err(e) => {
                        eprintln!("warning: failed to parse {}: {e}", path.display());
                        crate::log_warn!("config parse error {}: {e}", path.display());
                    }
                },
                Err(e) => {
                    eprintln!("warning: could not read {}: {e}", path.display());
                    crate::log_warn!("config read error {}: {e}", path.display());
                }
            }
        }
    }
    crate::log_info!("no config.toml found, using defaults");
    Config::default()
}

fn config_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    // Next to the running executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            paths.push(dir.join("config.toml"));
        }
    }
    // Project root (useful during development with `cargo run`)
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    paths.push(manifest.join("config.toml"));
    paths
}

fn find_arg_value(args: &[String], name: &str) -> Option<String> {
    let prefix = format!("{name}=");
    for index in 0..args.len() {
        let arg = &args[index];
        if let Some(value) = arg.strip_prefix(&prefix) {
            return Some(value.to_string());
        }
        if arg == name {
            return args.get(index + 1).cloned();
        }
    }
    None
}

/// Initialize the global config.  Call once at startup before anything reads
/// it.  CLI flags override config-file values.
pub fn init(cli_args: &[String]) {
    let mut cfg = load();

    // CLI flags override config file
    if cli_args.iter().any(|a| a == "--dev-mode" || a == "--dev") {
        cfg.general.dev_mode = true;
    }
    if cli_args.iter().any(|a| a == "--all-items") {
        cfg.general.all_items = true;
    }
    if cli_args.iter().any(|a| a == "--full-hearts") {
        cfg.general.full_hearts = true;
    }

    if let Some(dir) = find_arg_value(cli_args, "--log-dir") {
        cfg.general.log_dir = dir;
    }

    let _ = CONFIG.set(cfg);
}

/// Get a reference to the global config.  Panics if called before `init()`.
pub fn get() -> &'static Config {
    CONFIG.get().expect("config::init() must be called first")
}

/// Like `get()` but returns `None` if config is not initialized.
pub fn try_get() -> Option<&'static Config> {
    CONFIG.get()
}
