use std::panic::AssertUnwindSafe;

use crate::DataReader::component_loader::ComponentContainer;

use crate::ProtobufServer::services::{SimulationStartRequest, SimulationStepResponse};

use log::trace;

use tonic::{Request, Response, Status};

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub async fn handle_start_simulation(
        &self,
        request: AssertUnwindSafe<Request<SimulationStartRequest>>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        trace!("Received query: {:?}", request);

        let request_message = request.0.into_inner();
        let component_info = request_message.components_info.unwrap();

        // Extract components from the request message
        let _component_container = ComponentContainer::from(&component_info);

        // Combine components as specified in the composition string

        // let system: TransitionSystemPtr;
        // let comb = combine_components(system, PruningStrategy::Reachable);

        // Send the combine component to the Simulation module

        // Serialize and respond with the SimulatioState result form the simulation module

        // Ok(Response::new(None);
        Err(Status::unimplemented(""))
    }
}
