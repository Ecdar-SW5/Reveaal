use tonic::{Request, Status, Response};

use crate::ProtobufServer::{ConcreteEcdarBackend, services::{SimulationStepRequest, SimulationStepResponse}};

impl ConcreteEcdarBackend {
    async fn handle_take_simulation_step(
        _request: Request<SimulationStepRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        todo!()
    }
}
