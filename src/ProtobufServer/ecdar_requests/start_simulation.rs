use crate::DataReader::component_loader::ModelCache;
use crate::ProtobufServer::ecdar_requests::helpers;
use crate::ProtobufServer::services::{
    DecisionPoint as ProtoDecisionPoint, SimulationStartRequest, SimulationStepResponse,
};
use crate::Simulation::transition_decision_point::TransitionDecisionPoint;
use log::trace;

use tonic::Status;

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub fn handle_start_simulation(
        request: SimulationStartRequest,
        _cache: ModelCache, // TODO should be used...
    ) -> Result<SimulationStepResponse, Status> {
        fn option_to_vec<T>(o: Option<T>) -> Vec<T> {
            match o {
                Some(e) => vec![e],
                None => vec![],
            }
        }
        trace!("Received query: {:?}", request);

        let simulation_info = match request.simulation_info {
            Some(v) => v,
            None => return Err(Status::invalid_argument("simulation_info was empty")),
        };

        let transition_system = helpers::simulation_info_to_transition_system(simulation_info);

        let initial = TransitionDecisionPoint::initial(&transition_system)
            .map(|i| ProtoDecisionPoint::from_transition_decision_point(&i, &transition_system));

        Ok(SimulationStepResponse {
            new_decision_points: option_to_vec(initial),
        })
    }
}
