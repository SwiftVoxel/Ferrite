use chrono::Local;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    fn as_str(self) -> &'static str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }

    /// ANSI color code to use for this level, if any.
    fn color_code(self) -> Option<&'static str> {
        match self {
            Level::Warn => Some("93"),  // bright yellow
            Level::Error => Some("91"), // bright red
            _ => None,
        }
    }
}

const RESET: &str = "\x1b[0m";

/// Logs a line in the format: [17:51:25] [main/INFO] Starting Ferrite...
/// Warn and Error lines are colored (bright yellow / bright red).
pub fn log(thread: &str, level: Level, message: &str) {
    let time = Local::now().format("%H:%M:%S");
    let line = format!("[{time}] [{thread}/{}] {message}", level.as_str());

    match level.color_code() {
        Some(code) => println!("\x1b[{code}m{line}{RESET}"),
        None => println!("{line}"),
    }
}

#[macro_export]
macro_rules! log_info {
    ($thread:expr, $($arg:tt)*) => {
        $crate::logger::log($thread, $crate::logger::Level::Info, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($thread:expr, $($arg:tt)*) => {
        $crate::logger::log($thread, $crate::logger::Level::Warn, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($thread:expr, $($arg:tt)*) => {
        $crate::logger::log($thread, $crate::logger::Level::Error, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($thread:expr, $($arg:tt)*) => {
        $crate::logger::log($thread, $crate::logger::Level::Debug, &format!($($arg)*))
    };
}
