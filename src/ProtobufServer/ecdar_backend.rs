use crate::ProtobufServer::services::ecdar_backend_server::EcdarBackend;

use crate::ProtobufServer::services::{
    QueryRequest, QueryResponse, SimulationStartRequest, SimulationStepRequest,
    SimulationStepResponse, UserTokenResponse,
};
use futures::FutureExt;
use std::panic::UnwindSafe;
use tonic::{Request, Response, Status};

use super::threadpool::ThreadPool;

#[derive(Debug, Default)]
pub struct ConcreteEcdarBackend {
    thread_pool: ThreadPool,
}

async fn catch_unwind<T, O>(future: T) -> Result<O, Status>
where
    T: UnwindSafe + futures::Future<Output = Result<O, Status>>,
{
    fn downcast_to_string(e: Box<dyn std::any::Any + Send>) -> String {
        match e.downcast::<String>() {
            Ok(v) => *v,
            Err(e) => match e.downcast::<&str>() {
                Ok(v) => v.to_string(),
                _ => "Unknown Source of Error".to_owned(),
            },
        }
    }

    match future.catch_unwind().await {
        Ok(response) => response,
        Err(e) => Err(Status::internal(format!(
            "{}, please report this bug to the developers",
            downcast_to_string(e)
        ))),
    }
}

#[tonic::async_trait]
impl EcdarBackend for ConcreteEcdarBackend {
    async fn get_user_token(
        &self,
        _request: Request<()>,
    ) -> Result<Response<UserTokenResponse>, Status> {
        unimplemented!();
    }

    async fn send_query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        let res = catch_unwind(self.thread_pool.enqueue(request.into_inner())).await;
        res.map(Response::new)
    }

    async fn start_simulation(
        &self,
        request: Request<SimulationStartRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        let request = std::panic::AssertUnwindSafe(request);
        catch_unwind(self.handle_start_simulation(request)).await
    }

    async fn take_simulation_step(
        &self,
        _request: Request<SimulationStepRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(Status::unimplemented(""))
        // let request = std::panic::AssertUnwindSafe(request);
        // catch_unwind(self.handle_step_simulation_step(request)).await
    }
}
