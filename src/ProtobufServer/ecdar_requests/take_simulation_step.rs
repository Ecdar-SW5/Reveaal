use tonic::{Request, Response, Status};

use crate::{
    ProtobufServer::{
        ecdar_requests::helpers,
        services::{
            DecisionPoint as ProtoDecisionPoint, SimulationStepRequest, SimulationStepResponse,
        },
        ConcreteEcdarBackend,
    },
    Simulation::{
        decision_point::{self, Decision, DecisionPoint},
        transition_decision_point::TransitionDecision,
    },
    TransitionSystems,
};
impl ConcreteEcdarBackend {
    async fn handle_take_simulation_step(
        request: Request<SimulationStepRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        let request_message = request.into_inner();
        let simulation_info = request_message.simulation_info.unwrap();

        let chosen_decision = request_message.chosen_decision.unwrap();
        let chosen_decision: Decision = chosen_decision.into();
        let chosen_decision: TransitionDecision = chosen_decision.into();

        let transition_system = helpers::simulation_info_to_transition_system(simulation_info);
        let decision_point = chosen_decision.resolve(transition_system.clone()); // TODO remove clone

        let decision_point =
            ProtoDecisionPoint::from_transition_decision_point(&decision_point, &transition_system);

        let simulation_step_response = SimulationStepResponse {
            new_decision_point: Some(decision_point),
        };

        Ok(Response::new(simulation_step_response))
    }
}
