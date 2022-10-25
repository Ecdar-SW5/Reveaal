use crate::ProtobufServer::services::query_response::query_ok::Information;
use colored::Colorize;
use futures::sink::Buffer;
use log::{LevelFilter, SetLoggerError};
use serde_json::ser::Formatter;
use simplelog::*;
use std::borrow::BorrowMut;
use std::fmt::{Arguments, Debug, Display};
use std::fs::File;
use std::io::{IoSlice, Write};
use std::ops::Deref;

static mut T: Option<G> = None;
static mut MSGS: Vec<String> = vec![];

#[cfg(feature = "logging")]
pub fn setup_logger() -> Result<(), SetLoggerError> {
    let info_conf = ConfigBuilder::new()
        .set_time_format_custom(&[])
        .set_target_level(LevelFilter::Info)
        .add_filter_allow_str("clock-reduction")
        .build();

    /*
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        /*
        WriteLogger::new(
            LevelFilter::Info,
            info_conf.clone(),
            File::create("logger").expect("Couldn't create file"), //todo: fix
        ),
         */
        WriteLogger::new(
            LevelFilter::Info,
            info_conf,
            g.static_ref()
        ),
    ])
        */
    WriteLogger::init(LevelFilter::Info, info_conf, G {})
}

pub fn get_messages<'a>() -> &'a Vec<String> {
    // read file and format
    unsafe { &MSGS }
}

pub fn get_g<'a>() -> &'a G {
    unsafe { T.as_ref().unwrap() }
}

#[derive(Debug, Clone)]
pub struct G {}

impl Formatter for G {}

impl Write for G {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let binding = String::from_utf8_lossy(buf);
        let str = binding.trim_start();
        println!("{} -> {:?}", str, str.find('\n'));
        unsafe {
            // TODO: Only if last char
            if let Some(s) = MSGS.last_mut() {
                s.push_str(str)
            } else {
                MSGS.push(str.to_string());
            }
            if str.chars().last() == Some('\n') {
                MSGS.push("".to_string());
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let str = String::from_utf8(buf.to_vec()).expect("FAIL"); // TODO
        unsafe {
            MSGS.push(str);
        }
        Ok(())
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        self.write(fmt.to_string().as_bytes()).expect("FAIL");
        Ok(())
    }
}
