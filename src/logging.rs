use chrono::Local;
use colored::{ColoredString, Colorize};
use log::{LevelFilter, SetLoggerError};
use std::io::Write;

#[cfg(feature = "logging")]
pub fn setup_logger() -> Result<(), SetLoggerError> {
    fn colored_level(level: log::Level) -> ColoredString {
        match level {
            log::Level::Error => level.to_string().red(),
            log::Level::Warn => level.to_string().yellow(),
            log::Level::Info => level.to_string().cyan(),
            log::Level::Debug => level.to_string().blue(),
            log::Level::Trace => level.to_string().magenta(),
        }
    }

    env_logger::Builder::from_default_env()
        .filter(Some("clock-reduction"), LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{0} {1}:{1} {2}] - {3}",
                Local::now().format("%H:%M:%S").to_string().cyan(),
                record.file().unwrap_or_default(),
                colored_level(record.level()),
                record.args()
            )
        })
        .try_init()
}
