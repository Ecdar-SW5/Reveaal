use crate::ProtobufServer::services::ecdar_backend_server::EcdarBackend;

use crate::DataReader::component_loader::ModelCache;
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
    model_cache: ModelCache,
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

impl ConcreteEcdarBackend {
    async fn handleRequest<RequestT, ResponseT>(
        &self,
        request: Request<RequestT>,
        handler: impl Fn(RequestT, ModelCache) -> Result<ResponseT, Status> + Send + 'static,
    ) -> Result<Response<ResponseT>, Status>
    where
        ResponseT: Send + 'static,
        RequestT: Send + 'static,
    {
        let cache = self.model_cache.clone();
        let res = catch_unwind(
            self.thread_pool
                .enqueue(move || handler(request.into_inner(), cache)),
        )
        .await;
        res.map(Response::new)
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
        self.handleRequest(request, ConcreteEcdarBackend::handle_send_query)
            .await
    }

    async fn start_simulation(
        &self,
        request: Request<SimulationStartRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        self.handleRequest(request, Self::handle_start_simulation)
            .await
    }

    async fn take_simulation_step(
        &self,
        request: Request<SimulationStepRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        self.handleRequest(request, Self::handle_take_simulation_step)
            .await
    }
}
