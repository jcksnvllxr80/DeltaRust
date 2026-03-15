mod audio;
mod character;
mod config;
mod constants;
mod game;
mod generated_overworld;
mod logger;
mod model;
mod render;
mod sprites;
mod world;
mod world_data;

use crate::constants::window_conf;
use crate::game::Game;
use crate::logger::LogLevel;

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

fn parse_log_level(args: &[String]) -> (LogLevel, Vec<String>) {
    let mut warnings = Vec::new();

    // 1. CLI --log-level flag (highest priority)
    if let Some(raw) = find_arg_value(args, "--log-level") {
        if let Some(level) = LogLevel::parse(&raw) {
            return (level, warnings);
        }
        warnings.push(format!(
            "invalid --log-level value '{raw}', falling back to environment/config/default"
        ));
    }

    // 2. Environment variable
    if let Ok(raw) = std::env::var("DELTA_RUST_LOG") {
        if let Some(level) = LogLevel::parse(&raw) {
            return (level, warnings);
        }
        warnings.push(format!(
            "invalid DELTA_RUST_LOG value '{raw}', falling back to config/default"
        ));
    }

    // 3. Config file
    let cfg_level = &config::get().general.log_level;
    if let Some(level) = LogLevel::parse(cfg_level) {
        return (level, warnings);
    }

    (LogLevel::Info, warnings)
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Load config first (before logger, since config has log_level).
    config::init(&args);
    let cfg = config::get();

    // Determine log level: CLI --log-level > env DELTA_RUST_LOG > config.toml > default
    let (log_level, log_warnings) = parse_log_level(&args);
    match logger::init(log_level) {
        Ok(path) => {
            crate::log_info!(
                "logger ready level={} file={} args={:?}",
                logger::current_level().as_str(),
                path.display(),
                args
            );
            for warning in log_warnings {
                crate::log_warn!("{}", warning);
            }
        }
        Err(error) => {
            eprintln!("failed to initialize logger: {error}");
        }
    }

    let dev_mode = cfg.general.dev_mode;
    let all_items = cfg.general.all_items;
    let full_hearts = cfg.general.full_hearts;
    crate::log_info!(
        "startup flags dev_mode={} all_items={} full_hearts={}",
        dev_mode,
        all_items,
        full_hearts
    );
    let mut game = Game::new(dev_mode, all_items, full_hearts).await;
    loop {
        game.update();
        game.draw();
        macroquad::prelude::next_frame().await;
    }
}
