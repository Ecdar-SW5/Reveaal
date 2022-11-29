use crate::DataReader::component_loader::ModelCache;
use crate::DataReader::proto_reader::simulation_info_to_transition_system;
use crate::DataReader::proto_writer::transition_decision_point_to_proto_decision_point;
use crate::ProtobufServer::services::{SimulationStartRequest, SimulationStepResponse};
use crate::ProtobufServer::ConcreteEcdarBackend;
use crate::Simulation::transition_decision_point::TransitionDecisionPoint;

use tonic::Status;

impl ConcreteEcdarBackend {
    /// Handles a start simulation request: Responding with the initial decision point in the transition system given in the `request`.
    pub fn handle_start_simulation(
        request: SimulationStartRequest,
        _cache: ModelCache, // TODO should be used...
    ) -> Result<SimulationStepResponse, Status> {
        fn option_to_vec<T>(option: Option<T>) -> Vec<T> {
            match option {
                Some(item) => vec![item],
                None => vec![],
            }
        }

        let simulation_info = request.simulation_info.unwrap();

        let transition_system = simulation_info_to_transition_system(&simulation_info);

        let initial = TransitionDecisionPoint::initial(&transition_system)
            .map(|i| transition_decision_point_to_proto_decision_point(&i, &transition_system));

        Ok(SimulationStepResponse {
            new_decision_points: option_to_vec(initial),
        })
    }
}
