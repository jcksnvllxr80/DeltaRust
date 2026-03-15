use std::env;
use std::fmt;
use std::fs::{create_dir_all, File, OpenOptions};
use time::{format_description, OffsetDateTime};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Verbose = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Critical = 5,
}

impl LogLevel {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "verbose" | "trace" => Some(Self::Verbose),
            "debug" => Some(Self::Debug),
            "info" => Some(Self::Info),
            "warn" | "warning" => Some(Self::Warn),
            "error" => Some(Self::Error),
            "critical" | "fatal" => Some(Self::Critical),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Verbose => "VERBOSE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Critical => "CRITICAL",
        }
    }

    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Verbose,
            1 => Self::Debug,
            2 => Self::Info,
            3 => Self::Warn,
            4 => Self::Error,
            _ => Self::Critical,
        }
    }
}

struct Logger {
    level: AtomicU8,
    writer: Mutex<BufWriter<File>>,
    path: PathBuf,
}

static LOGGER: OnceLock<Logger> = OnceLock::new();
static TIME_FORMAT: OnceLock<String> = OnceLock::new();
static TIME_FORMAT_EPOCH: OnceLock<bool> = OnceLock::new();
static TIME_FORMAT_EPOCH_MS: OnceLock<bool> = OnceLock::new();

pub fn default_log_path() -> PathBuf {
    let dir = crate::config::try_get()
        .map(|c| PathBuf::from(&c.general.log_dir))
        .unwrap_or_else(||
            env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("logs")
        );
    dir.join("delta_rust.log")
}

pub fn init(level: LogLevel) -> std::io::Result<PathBuf> {
    init_time_format();

    if let Some(logger) = LOGGER.get() {
        logger.level.store(level as u8, Ordering::Relaxed);
        return Ok(logger.path.clone());
    }

    let path = default_log_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let file = OpenOptions::new().create(true).append(true).open(&path)?;
    let logger = Logger {
        level: AtomicU8::new(level as u8),
        writer: Mutex::new(BufWriter::new(file)),
        path: path.clone(),
    };

    let _ = LOGGER.set(logger);
    log_fmt(
        LogLevel::Info,
        "logger",
        format_args!(
            "session started version={} level={} path={}",
            env!("CARGO_PKG_VERSION"),
            level.as_str(),
            path.display()
        ),
    );

    Ok(path)
}

pub fn current_level() -> LogLevel {
    LOGGER
        .get()
        .map(|logger| LogLevel::from_u8(logger.level.load(Ordering::Relaxed)))
        .unwrap_or(LogLevel::Info)
}

fn translate_time_format(fmt: &str) -> String {
    // Allow a simple user-facing format like "YYYY/MM/DDTHH:MM:SS.sssZ" by translating
    // it into `time` crate's format_description syntax.
    fmt.replace("YYYY", "[year]")
        .replace("MM", "[month]")
        .replace("DD", "[day]")
        .replace("HH", "[hour]")
        .replace("mm", "[minute]")
        .replace("SS", "[second]")
        // `sss` should mean milliseconds (3 digits).
        .replace("sss", "[subsecond digits:3]")
}

fn init_time_format() {
    let raw_fmt = crate::config::try_get()
        .map(|c| c.general.log_time_format.trim().to_string())
        .unwrap_or_else(|| "YYYY/MM/DDTHH:MM:SS.sssZ".into());

    let is_epoch = raw_fmt.eq_ignore_ascii_case("epoch") || raw_fmt.eq_ignore_ascii_case("unix");
    let is_epoch_ms = raw_fmt.eq_ignore_ascii_case("epoch_ms")
        || raw_fmt.eq_ignore_ascii_case("ms")
        || raw_fmt.eq_ignore_ascii_case("milliseconds");

    let _ = TIME_FORMAT_EPOCH.set(is_epoch);
    let _ = TIME_FORMAT_EPOCH_MS.set(is_epoch_ms);

    let fmt = if is_epoch || is_epoch_ms {
        // Store something usable but unused in epoch modes.
        "YYYY/MM/DDTHH:MM:SS.sssZ".to_string()
    } else {
        translate_time_format(&raw_fmt)
    };

    let _ = TIME_FORMAT.set(fmt);
}

fn timestamp() -> String {
    if *TIME_FORMAT_EPOCH_MS.get_or_init(|| false) {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis().to_string())
            .unwrap_or_else(|_| "0".to_string())
    } else if *TIME_FORMAT_EPOCH.get_or_init(|| false) {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_else(|_| "0".to_string())
    } else {
        let fmt = TIME_FORMAT
            .get()
            .map(|s| s.as_str())
            .unwrap_or("[year]/[month]/[day]T[hour]:[minute]:[second].[subsecond]Z");
        let now = OffsetDateTime::now_utc();
        let desc = format_description::parse(fmt).unwrap_or_else(|_| {
            format_description::parse("[year]/[month]/[day]T[hour]:[minute]:[second].[subsecond]Z")
                .unwrap()
        });
        now.format(&desc)
            .unwrap_or_else(|_| "0000/00/00T00:00:00.000Z".to_string())
    }
}

pub fn enabled(level: LogLevel) -> bool {
    LOGGER
        .get()
        .is_some_and(|logger| level >= LogLevel::from_u8(logger.level.load(Ordering::Relaxed)))
}

pub fn log_fmt(level: LogLevel, target: &str, args: fmt::Arguments<'_>) {
    let Some(logger) = LOGGER.get() else {
        return;
    };
    if !enabled(level) {
        return;
    }

    let timestamp = timestamp();

    if let Ok(mut writer) = logger.writer.lock() {
        let _ = writeln!(
            writer,
            "[{}] {:<8} [{}] {}",
            timestamp,
            level.as_str(),
            target,
            args
        );
        let _ = writer.flush();
    }
}

#[macro_export]
macro_rules! log_verbose {
    ($($arg:tt)*) => {{
        if $crate::logger::enabled($crate::logger::LogLevel::Verbose) {
            $crate::logger::log_fmt($crate::logger::LogLevel::Verbose, module_path!(), format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        if $crate::logger::enabled($crate::logger::LogLevel::Debug) {
            $crate::logger::log_fmt($crate::logger::LogLevel::Debug, module_path!(), format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        if $crate::logger::enabled($crate::logger::LogLevel::Info) {
            $crate::logger::log_fmt($crate::logger::LogLevel::Info, module_path!(), format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        if $crate::logger::enabled($crate::logger::LogLevel::Warn) {
            $crate::logger::log_fmt($crate::logger::LogLevel::Warn, module_path!(), format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        if $crate::logger::enabled($crate::logger::LogLevel::Error) {
            $crate::logger::log_fmt($crate::logger::LogLevel::Error, module_path!(), format_args!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! log_critical {
    ($($arg:tt)*) => {{
        if $crate::logger::enabled($crate::logger::LogLevel::Critical) {
            $crate::logger::log_fmt($crate::logger::LogLevel::Critical, module_path!(), format_args!($($arg)*));
        }
    }};
}