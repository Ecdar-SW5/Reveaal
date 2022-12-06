use crate::ProtobufServer::services::query_request::Settings;

pub mod ClockReduction;
pub mod ModelObjects;
pub mod Simulation;
pub mod edge_ids;
pub mod failure_message;
pub mod grpc;
pub mod reachability;
pub mod refinement;
pub mod sample;
pub mod save_component;
pub mod system_recipe;

/// The default settings for Testing
pub const TEST_SETTINGS: Settings = Settings {
    disable_clock_reduction: true,
};
