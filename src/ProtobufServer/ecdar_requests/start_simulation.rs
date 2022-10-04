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
        let start_simulation_request = request.0.into_inner();

        let components = self.get_components_lock()?;
        let component_container = self.get_components_lock()?;

        for proto_component in &start_simulation_request.component {
            let component = self.parse_component_if_some(proto_component)?;

        }


        Ok(Response::new(()));
    }

    fn parse_component_if_some(
        &self,
        proto_component: &ProtobufComponent,
    ) -> Result<Vec<Component>, tonic::Status> {
        if let Some(rep) = &proto_component.rep {
            match rep {
                Rep::Json(json) => parse_json_component(json),
                Rep::Xml(xml) => Ok(parse_xml_components(xml)),
            }
        } else {
            Ok(vec![])
        }
    }
}
fn parse_json_component(json: &str) -> Result<Vec<Component>, tonic::Status> {
    match json_to_component(json) {
        Ok(comp) => Ok(vec![comp]),
        Err(_) => Err(tonic::Status::invalid_argument(
            "Failed to parse json component",
        )),
    }
}

fn parse_xml_components(xml: &str) -> Vec<Component> {
    let (comps, _, _) = parse_xml_from_str(xml);
    comps
}
