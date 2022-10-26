use crate::ProtobufServer::services::query_response::query_ok::Information;
use log::{LevelFilter, SetLoggerError};
use simplelog::*;
use std::fmt::{Arguments, Debug};
use std::io::Write;

static mut MSGS: Vec<String> = vec![];

#[cfg(feature = "logging")]
pub fn setup_logger(is_server: bool) -> Result<(), SetLoggerError> {
    let info_conf = ConfigBuilder::new()
        .set_time_format_custom(&[])
        .set_target_level(LevelFilter::Info)
        .add_filter_allow_str("clock-reduction")
        .build();
    let loggers: Vec<Box<dyn SharedLogger>> = vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        if is_server {
            WriteLogger::new(LevelFilter::Info, info_conf, G {})
        } else {
            TermLogger::new(
                LevelFilter::Info,
                info_conf,
                TerminalMode::Mixed,
                ColorChoice::Auto,
            )
        },
    ];

    CombinedLogger::init(loggers)

    //WriteLogger::init(LevelFilter::Info, info_conf, G {})
}

fn get_messages_raw() -> impl Iterator<Item = String> + Sized {
    unsafe {
        let out = MSGS.iter().filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });
        MSGS.clear();
        out
    }
}

pub fn get_messages() -> Vec<Information> {
    get_messages_raw()
        .map(|s| {
            let (sub, msg) = s.split_once(':').unwrap_or_else(|| panic!("aaahhh"));
            Information {
                subject: sub.to_string(),
                message: msg.to_string(),
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct G {}

impl Write for G {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let binding = String::from_utf8_lossy(buf);
        let str = binding.trim_start();
        unsafe {
            if let Some(s) = MSGS.last_mut() {
                s.push_str(str)
            } else {
                MSGS.push(str.trim_end().to_string());
            }
            if str.ends_with('\n') {
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
        let _ = self.write(fmt.to_string().as_bytes())?;
        Ok(())
    }
}
