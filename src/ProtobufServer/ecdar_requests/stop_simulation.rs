impl ConcreteEcdarBackend {
    pub async fn handle_stop_simulation(
        &self,
        request: AssertUnwindSafe<Request<()>>,
    ) -> Result<Response<SimulationStepResponse>, tonic::Status> {
        trace!("Recieved query: {:?}", request);
        let stop_simulation_request = request.0.into_inner();

        let simulationid = stop_simulation_request.simulation_id;

        Ok((()));
    }
}