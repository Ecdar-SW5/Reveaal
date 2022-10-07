use std::panic::AssertUnwindSafe;

use core::cell::RefCell;
use crate::System::input_enabler;
use crate::DataReader::component_loader::ComponentContainer;
use crate::DataReader::parse_edge::Rule::bool;
use crate::DataReader::component_loader::ComponentLoader;
use crate::DataReader::json_reader::json_to_component;
use crate::xml_parser::parse_xml_from_str;
use crate::DataReader::json_writer::component_to_json;
use crate::ProtobufServer::services::SimulationStepResponse;
use crate::ProtobufServer::services::component::Rep;
use crate::ProtobufServer::services::{Component as ProtobufComponent, SimulationStartRequest, Constraint, ComponentClock, ComponentsInfo, SpecificComponent};
use crate::component::Component;
use crate::System::extract_system_rep;
use std::collections::HashMap;
use log::trace;
use tonic::{Request, Response, Status};

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub async fn handle_start_simulation(
        &self,
        request: AssertUnwindSafe<Request<SimulationStartRequest>>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        trace!("Recieved query: {:?}", request);

        let start_simulation_request = request.0.into_inner();

        // Extract the individual components
        let components_info = start_simulation_request.components_info;
        let components = Vec::new();
        for json_component in components_info.components {
            let component = parse_json_component(json_component);
            components.push(component);
        }

        // Combine components as specified in the composition string
        
        let system: TransitionSystemPtr;
        let comb = combine_components(system, PruningStrategy::Reachable);

        // Send the combine component to the Simulation module

        // Serialize and respond with the SimulatioState result form the simulation module

        Ok(Response::new(None);
    }
}
