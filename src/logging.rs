use crate::ProtobufServer::services::query_response::query_ok::Information;
use core::marker::Send;
use log::{LevelFilter, SetLoggerError};
use simplelog::*;
use std::fmt::Arguments;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

#[cfg(feature = "logging")]
pub fn setup_logger() -> Result<(), SetLoggerError> {
    let info_conf = ConfigBuilder::new()
        .set_time_format_custom(&[])
        .set_target_level(LevelFilter::Info)
        .add_filter_allow_str("clock-reduction")
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            info_conf,
            File::create("logger").expect("Couldn't create file"), //todo: fix
        ),
    ])
}

pub fn get_messages() -> Vec<Information> {
    // read file and format
    vec![]
}
