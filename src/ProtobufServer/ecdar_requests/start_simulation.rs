use std::panic::AssertUnwindSafe;

use crate::DataReader::component_loader::ComponentContainer;

use crate::ProtobufServer::ecdar_requests::helpers;
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
        let transition_system = helpers::simulation_info_to_transition_system(simulation_info);
        
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
    fn from(decision_point: DecisionPoint) -> Self {
        todo!();
    }
}
