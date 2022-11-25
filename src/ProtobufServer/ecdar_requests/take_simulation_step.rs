use tonic::Status;

use crate::{
    DataReader::{
        component_loader::ModelCache,
        proto_reader::{
            components_info_to_components, proto_decision_to_decision,
            simulation_info_to_transition_system,
        },
        proto_writer::transition_decision_point_to_proto_decision_point,
    },
    ProtobufServer::{
        services::{SimulationStepRequest, SimulationStepResponse},
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

        let components =
            components_info_to_components(simulation_info.components_info.as_ref().unwrap());

        let transition_system = simulation_info_to_transition_system(&simulation_info);

        let chosen_decision = request_message.chosen_decision.unwrap();
        let chosen_decision: Decision =
            proto_decision_to_decision(chosen_decision, &transition_system, components);
        let chosen_decisions = TransitionDecision::from(&chosen_decision, &transition_system);

        let decision_points: Vec<_> = chosen_decisions
            .into_iter()
            .map(|d| d.resolve(&transition_system))
            .map(|d| transition_decision_point_to_proto_decision_point(&d, &transition_system))
            .collect();

        let simulation_step_response = SimulationStepResponse {
            new_decision_points: decision_points,
        };

        Ok(simulation_step_response)
    }
}
