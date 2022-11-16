use crate::ProtobufServer::services::query_request::settings::ReduceClocksLevel::All;
use crate::ProtobufServer::services::query_request::Settings;

pub mod ClockReduction;
pub mod ModelObjects;
pub mod failure_message;
pub mod grpc;
pub mod reachability;
pub mod refinement;
pub mod sample;
pub mod save_component;
pub const TEST_SETTINGS: Settings = Settings {
    reduce_clocks_level: Some(All(false)),
};
