use std::panic::AssertUnwindSafe;

use crate::DataReader::component_loader::ComponentContainer;

use crate::extract_system_rep::get_system_recipe;
use crate::parse_queries::parse_to_expression_tree;
use crate::ProtobufServer::services::{
    DecisionPoint, SimulationStartRequest, SimulationStepResponse,
};

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
        let simulation_info = request_message.simulation_info.unwrap();
        let composition = simulation_info.component_composition;
        let component_info = simulation_info.components_info.unwrap();

        // Extract components from the request message
        let mut component_container = ComponentContainer::from(&component_info).unwrap();

        // Combine components as specified in the composition string
        let mut dimension = 0;
        let composition_as_expression_tree = parse_to_expression_tree(&composition);
        let transition_system = get_system_recipe(
            &composition_as_expression_tree[0],
            &mut component_container,
            &mut dimension,
            &mut None,
        )
        .compile(dimension);

        // Send the combine component to the Simulation module
        let initial_decision_point = DecisionPoint {
            source: todo!(),
            edges: todo!(),
        };
        // get_initial_decision_from(transition_system);

        // Serialize and respond with the SimulationState result from the simulation module
        let simulation_step_response = SimulationStepResponse {
            new_decision_point: Some(initial_decision_point),
        };

        Ok(Response::new(simulation_step_response))
    }
}
