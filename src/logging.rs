use log::{LevelFilter, SetLoggerError};
use simplelog::*;

#[cfg(feature = "logging")]
pub fn setup_logger() -> Result<(), SetLoggerError> {
    let conf = ConfigBuilder::new()
        .set_time_format_custom(&[])
        .set_target_level(LevelFilter::Info)
        .add_filter_allow_str("clock-reduction")
        .build();
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            conf,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        //WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()), //TODO: Write to something that should be sent through gRPC
    ])
}
