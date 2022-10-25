#![allow(non_snake_case)]
pub mod DataReader;
pub mod DataTypes;
pub mod EdgeEval;
pub mod ModelObjects;
pub mod ProtobufServer;
pub mod Simulation;
pub mod System;
pub mod TransitionSystems;
pub mod logging;
pub mod tests;

pub use crate::DataReader::component_loader::{
    ComponentLoader, JsonProjectLoader, ProjectLoader, XmlProjectLoader,
};
pub use crate::DataReader::{parse_queries, xml_parser};
pub use crate::ModelObjects::queries::Query;
pub use crate::System::extract_system_rep;
pub use ModelObjects::component;
pub use ModelObjects::queries;
pub use ProtobufServer::start_grpc_server_with_tokio;
pub use System::executable_query::QueryResult;

#[macro_use]
extern crate pest_derive;
extern crate colored;
extern crate core;
extern crate serde;
extern crate serde_xml_rs;
extern crate simple_error;
extern crate xml;
