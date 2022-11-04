use std::panic::AssertUnwindSafe;

use crate::DataReader::component_loader::ComponentContainer;

use crate::ProtobufServer::ecdar_requests::helpers;
use crate::ProtobufServer::services::{
    DecisionPoint as ProtoDecisionPoint, SimulationStartRequest, SimulationStepResponse,
};
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
        let initial = TransitionDecisionPoint::initial(transition_system).unwrap();

        // Convert initial from TransitionDecision to DecisionPoint
        let initial: DecisionPoint = initial.into();

        // Convert initial from DecisionPoint to ProtoDecisionPoint
        let initial: ProtoDecisionPoint = initial.into();

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

#[cfg(test)]
mod tests {
    use crate::{
        tests::{
            grpc::grpc_helper::create_initial_decision_point,
            Simulation::helper::initial_transition_decision_point_EcdarUniversity_Machine,
        },
        ProtobufServer::services::DecisionPoint as ProtoDecisionPoint,
        Simulation::decision_point::DecisionPoint,
    };

    #[test]
    fn from__good_DecisionPoint__returns_good_ProtoDecisionPoint() {
        // Arrange
        let transitionDecisionPoint = initial_transition_decision_point_EcdarUniversity_Machine();
        let decisionPoint = DecisionPoint::from(transitionDecisionPoint);

        // Act
        let actual = ProtoDecisionPoint::from(decisionPoint);

        // Assert
        let expected = create_initial_decision_point();

        assert_eq!(actual.edges.len(), 2);
        assert!(actual.edges.contains(&expected.edges[0]));
        assert!(actual.edges.contains(&expected.edges[1]));
        assert_eq!(actual.source, expected.source);
    }
}
