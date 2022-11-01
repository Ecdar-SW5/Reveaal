use tonic::{Request, Status, Response};

use crate::{ProtobufServer::{ConcreteEcdarBackend, ecdar_requests::helpers, services::{SimulationStepRequest, SimulationStepResponse, DecisionPoint as ProtoDecisionPoint}}, TransitionSystems, Simulation::{transition_decision_point::TransitionDecision, decision_point::{Decision, DecisionPoint}}};
impl ConcreteEcdarBackend {
    async fn handle_take_simulation_step(
        request: Request<SimulationStepRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        let request_message = request.into_inner();
        let simulation_info = request_message.simulation_info.unwrap();
        
        let chosen_decision = request_message.chosen_decision.unwrap();
        let chosen_decision : Decision = chosen_decision.into();
        let chosen_decision : TransitionDecision = chosen_decision.into();

        let transition_system = helpers::simulation_info_to_transition_system(simulation_info);
        let decision_point = chosen_decision.resolve(transition_system);
        
        let decision_point : DecisionPoint = decision_point.into();
        let decicion_point : ProtoDecisionPoint = decision_point.into();

        let simulation_step_response = SimulationStepResponse {
            new_decision_point: Some(decicion_point),
        };

        Ok(Response::new(simulation_step_response))
    }
}
