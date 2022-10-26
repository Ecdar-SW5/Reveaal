use std::panic::AssertUnwindSafe;

use crate::DataReader::component_loader::ComponentContainer;

use crate::TransitionSystems::CompiledComponent;
use crate::ProtobufServer::services::{SimulationStartRequest, SimulationStepResponse};
use crate::Simulation::transition_decision::TransitionDecision;

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
        let transition_system = CompiledComponent::from_component_loader(&mut component_container, &composition);

        // Send the combine component to the Simulation module
        let _initial_decision_point = TransitionDecision::initial(transition_system);

        // Serialize and respond with the SimulationState result from the simulation module
        let simulation_step_response = SimulationStepResponse {
            new_decision_point: None,
        };

        Ok(Response::new(simulation_step_response))
    }
}
