//use colored::{ColoredString, Colorize};
use log::{LevelFilter, SetLoggerError};
use simplelog::*;

#[cfg(feature = "logging")]
pub fn setup_logger() -> Result<(), SetLoggerError> {
    /*
    fn colored_level(level: log::Level) -> ColoredString {
        match level {
            log::Level::Error => level.to_string().red(),
            log::Level::Warn => level.to_string().yellow(),
            log::Level::Info => level.to_string().cyan(),
            log::Level::Debug => level.to_string().blue(),
            log::Level::Trace => level.to_string().magenta(),
        }
    }
     */

    let conf = ConfigBuilder::new()
        .set_time_format_custom(&[])
        .set_target_level(LevelFilter::Info)
        .add_filter_allow_str("clock-reduction")
        .build();
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, conf, TerminalMode::Stdout, ColorChoice::Auto),
            //WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()), //TODO: Write to something that should be sent through gRPC
        ]
    )
    /*
    let target = if let Some(w) = target {
        Target::Pipe(w)
    } else {
        Target::default()
    };

    env_logger::Builder::from_default_env()
        .target(target)
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
     */
}
