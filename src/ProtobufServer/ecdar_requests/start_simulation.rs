use std::panic::AssertUnwindSafe;

use crate::DataReader::component_loader::{ComponentContainer, self};

use crate::ProtobufServer::services::{SimulationStartRequest, SimulationStepResponse, SimulationState};
use crate::extract_system_rep::get_system_recipe;
use crate::parse_queries::parse_to_expression_tree;

use log::trace;

use tonic::{Request, Response, Status};

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub async fn handle_start_simulation(
        &self,
        request: AssertUnwindSafe<Request<SimulationStartRequest>>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        trace!("Received query: {:?}", request);
        
        let request_message = request.0.into_inner();
        let component_info = request_message.components_info.unwrap();
        let composition = request_message.component_composition;

        // Extract components from the request message
        let mut component_container = ComponentContainer::from(&component_info).unwrap();

        // Combine components as specified in the composition string
        let mut dimension = 0;
        let composition_as_expression_tree = parse_to_expression_tree(&composition);
        let transition_system = get_system_recipe(&composition_as_expression_tree[0], &mut component_container, &mut dimension, &mut None).compile(dimension);

        // Send the combine component to the Simulation module
        let initial_decision_point = get_initial_decision_from(transition_system);

        // Serialize and respond with the SimulationState result from the simulation module
        let initial_simulation_state = SimulationState{
            component_composition: composition,
            components_info: Some(component_info),
            decision_points: vec![initial_decision_point],
        };
        let simulation_step_response = SimulationStepResponse { new_state: Some(initial_simulation_state) };

        Ok(Response::new(simulation_step_response))
    }
}
