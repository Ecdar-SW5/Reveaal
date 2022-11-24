use tonic::Status;

use crate::{
    DataReader::component_loader::{parse_components_if_some, ModelCache},
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

        let components = simulation_info
            .clone()
            .components_info
            .unwrap()
            .components
            .iter()
            .flat_map(parse_components_if_some)
            .flatten()
            .collect();

        let transition_system = helpers::simulation_info_to_transition_system(simulation_info);

        let chosen_decision = request_message.chosen_decision.unwrap();
        let chosen_decision: Decision =
            Decision::from(chosen_decision, &transition_system, components);
        let chosen_decisions = TransitionDecision::from(&chosen_decision, &transition_system);

        let decision_points: Vec<_> = chosen_decisions
            .into_iter()
            .map(|d| d.resolve(transition_system.clone()))
            .map(|d| ProtoDecisionPoint::from_transition_decision_point(&d, &transition_system))
            .collect();

        let simulation_step_response = SimulationStepResponse {
            new_decision_points: decision_points,
        };

        Ok(simulation_step_response)
    }
}

#[cfg(test)]
mod tests {
    #[ignore]
    fn _take_simulation_step__get_composit_component__should_return_component() {
        // TODO
    }
}
