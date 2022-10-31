use std::panic::AssertUnwindSafe;

use crate::DataReader::component_loader::ComponentContainer;

use crate::ProtobufServer::services::{SimulationStartRequest, SimulationStepResponse, DecisionPoint as ProtoDecisionPoint};
use crate::Simulation::decision_point::DecisionPoint;
use crate::Simulation::transition_decision_point::TransitionDecisionPoint;
use crate::TransitionSystems::CompiledComponent;

use log::trace;

use tonic::{Request, Response, Status};

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub async fn handle_start_simulation(
        request: AssertUnwindSafe<Request<SimulationStartRequest>>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        trace!("Received query: {:?}", request);

        let request_message = request.0.into_inner();
        let simulation_info = request_message.simulation_info.unwrap();
        let composition = simulation_info.component_composition;
        let component_info = simulation_info.components_info.unwrap();

        // Extract components from the request message
        let mut component_container = ComponentContainer::from(&component_info).unwrap();

        // Build transition_system as specified in the composition string
        let transition_system =
            CompiledComponent::from_component_loader(&mut component_container, &composition);

        // Find Initial TransitionDecisionPoint in transition system
        let initial= TransitionDecisionPoint::initial(transition_system).unwrap();

        // Convert initial from TransitionDecision to DecisionPoint
        let initial:DecisionPoint = initial.into();

        // Convert initial from DecisionPoint to ProtoDecisionPoint
        let initial:ProtoDecisionPoint = initial.into();

        // Respond with initial
        let simulation_step_response = SimulationStepResponse {
            new_decision_point: Some(initial),
        };
        Ok(Response::new(simulation_step_response))
    }
}

impl From<DecisionPoint> for ProtoDecisionPoint {
    fn from(_: DecisionPoint) -> Self {
        todo!()
    }
}
