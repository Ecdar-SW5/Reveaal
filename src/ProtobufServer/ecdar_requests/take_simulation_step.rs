use tonic::Status;

use crate::{
    DataReader::component_loader::ModelCache,
    ProtobufServer::{
        ecdar_requests::helpers,
        services::{
            DecisionPoint as ProtoDecisionPoint, SimulationStepRequest, SimulationStepResponse,
        },
        ConcreteEcdarBackend,
    },
    Simulation::{decision::Decision, transition_decision::TransitionDecision},
};
impl ConcreteEcdarBackend {
    pub fn handle_take_simulation_step(
        request: SimulationStepRequest,
        _cache: ModelCache, // TODO should be used...
    ) -> Result<SimulationStepResponse, Status> {
        let request_message = request;
        let simulation_info = request_message.simulation_info.unwrap();

        let transition_system = helpers::simulation_info_to_transition_system(simulation_info);

        let chosen_decision = request_message.chosen_decision.unwrap();
        let chosen_decision: Decision = Decision::from(chosen_decision, &transition_system);
        let chosen_decision: TransitionDecision =
            TransitionDecision::from(&chosen_decision, &transition_system);

        let decision_point = chosen_decision.resolve(transition_system.clone()); // TODO remove clone

        let decision_point =
            ProtoDecisionPoint::from_transition_decision_point(&decision_point, &transition_system);

        let simulation_step_response = SimulationStepResponse {
            new_decision_point: Some(decision_point),
        };

        Ok(simulation_step_response)
    }
}
