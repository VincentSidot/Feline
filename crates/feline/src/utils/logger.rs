use colored::Colorize;
use log::{Level, LevelFilter, Log};
use std::{fmt, io::Write};

use super::config::Config;

struct Logger;

impl Logger {
    fn level_allowed(level: Level, filter: LevelFilter) -> bool {
        match filter {
            LevelFilter::Off => false,
            LevelFilter::Error => level <= Level::Error,
            LevelFilter::Warn => level <= Level::Warn,
            LevelFilter::Info => level <= Level::Info,
            LevelFilter::Debug => level <= Level::Debug,
            LevelFilter::Trace => true,
        }
    }
}

pub(super) fn logfmt(
    level: Level,
    target: &str,
    file: &str,
    line: u32,
    args: &fmt::Arguments,
) -> String {
    let color_balise = match level {
        Level::Error => "[ERRO]".red(),
        Level::Warn => "[WARN]".yellow(),
        Level::Info => "[INFO]".green(),
        Level::Debug => "[DBUG]".blue(),
        Level::Trace => "[TRCE]".purple(),
    };

    format!(
        "{} ({} -> {}:{}) - {}",
        color_balise, target, file, line, args
    )
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let target = metadata.target();
        let filt = Config::get_log_level(target);

        Self::level_allowed(metadata.level(), filt)
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let file = record.file().unwrap_or("unknown");
            let line = record.line().unwrap_or(0);
            let target = record.target();

            let text = logfmt(record.level(), target, file, line, &record.args());
            // GUI apps can outlive or detach from the launching terminal. Ignore broken pipe
            // errors so logging does not crash the process.
            let mut stdout = std::io::stdout().lock();
            let _ = writeln!(stdout, "{}", text);
        }
    }

    fn flush(&self) {}
}

/// Initializes the logger
///
/// This function sets up the logger with a default level of INFO.
/// The log level can be controlled via the LOG_LEVEL environment variable.
/// For example: `LOG_LEVEL=debug` or `LOG_LEVEL=websearch=trace`
pub fn init() {
    static LOGGER: Logger = Logger;

    // Set global max to the highest level we might emit (so overrides can work).
    let global_max = Config::get_max_log_level();

    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(global_max))
        .expect("Failed to set logger");

    log::info!("Logger initialized");
}
