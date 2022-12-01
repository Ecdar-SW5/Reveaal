use tonic::Status;

use crate::{
    DataReader::{
        component_loader::ModelCache,
        proto_reader::{
            components_info_to_components, proto_decision_to_decision,
            simulation_info_to_transition_system,
        },
        proto_writer::decision_point_to_proto_decision_point,
    },
    ProtobufServer::{
        services::{SimulationStepRequest, SimulationStepResponse},
        ConcreteEcdarBackend,
    },
};

impl ConcreteEcdarBackend {
    /// Handles a take simulation step request:
    /// Given a `decision` and transition system in the `request`, walk along the decided edge and respond with the resulting decision points.
    pub fn handle_take_simulation_step(
        request: SimulationStepRequest,
        _cache: ModelCache, // TODO should be used...
    ) -> Result<SimulationStepResponse, Status> {
        let request_message = request;
        let simulation_info = request_message.simulation_info.unwrap();

        let components =
            components_info_to_components(simulation_info.components_info.as_ref().unwrap());

        let system = simulation_info_to_transition_system(&simulation_info);

        let chosen_decision = request_message.chosen_decision.unwrap();
        let chosen_decision = proto_decision_to_decision(chosen_decision, &system, components);

        let decision_points = chosen_decision.resolve(&system);

        let decision_points = decision_points
            .into_iter()
            .map(|dp| decision_point_to_proto_decision_point(&dp, &system))
            .collect();

        let simulation_step_response = SimulationStepResponse {
            new_decision_points: decision_points,
        };

        Ok(simulation_step_response)
    }
}
