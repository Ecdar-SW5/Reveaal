use std::panic::AssertUnwindSafe;

use core::cell::RefCell;
use crate::System::input_enabler;
use crate::DataReader::component_loader::ComponentContainer;
use crate::DataReader::component_loader::ComponentLoader;
use crate::DataReader::json_reader::json_to_component;
use crate::xml_parser::parse_xml_from_str;
use crate::DataReader::json_writer::component_to_json;
use crate::ProtobufServer::services::SimulationStepResponse;
use crate::ProtobufServer::services::component::Rep;
use crate::ProtobufServer::services::{Component as ProtobufComponent, SimulationStartRequest};
use crate::component::Component;
use crate::System::extract_system_rep;
use log::trace;
use tonic::{Request, Response, Status};

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub async fn handle_start_simulation(
        &self,
        request: AssertUnwindSafe<Request<()>>,
    ) -> Result<Response<SimulationStepResponse>, tonic::Status> {
        trace!("Recieved query: {:?}", request);
        let start_simulation_request = request.0.into_inner()?;
        let mut id = 0;


        let reply = {
            simulationid = id += 1;
            initialdecisionpoint = &start_simulation_request.components_info.components.component.locations
        };

        Ok(Response::new((reply)));
    }
}
