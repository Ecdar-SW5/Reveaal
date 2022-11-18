use crate::ProtobufServer::ecdar_requests::helpers::*;
use crate::ProtobufServer::services::ecdar_backend_server::EcdarBackend;

use crate::DataReader::component_loader::ModelCache;
use crate::ProtobufServer::services::{
    Decision, Edge, QueryRequest, QueryResponse, SimulationStartRequest, SimulationStepRequest,
    SimulationStepResponse, UserTokenResponse,
};

use super::services::DecisionPoint;
use super::threadpool::ThreadPool;
use futures::FutureExt;
use std::panic::UnwindSafe;
use std::sync::atomic::{AtomicI32, Ordering};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ConcreteEcdarBackend {
    thread_pool: ThreadPool,
    model_cache: ModelCache,
    num: AtomicI32,
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
        let id = self.num.fetch_add(1, Ordering::SeqCst);
        let token_response = UserTokenResponse { user_id: id };
        Result::Ok(Response::new(token_response))
    }

    async fn send_query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryResponse>, Status> {
        let cache = self.model_cache.clone();
        let res =
            catch_unwind(self.thread_pool.enqueue(move || {
                ConcreteEcdarBackend::handle_send_query(request.into_inner(), cache)
            }))
            .await;
        res.map(Response::new)
    }
    //Function currently returns dummy data
    async fn start_simulation(
        &self,
        request: Request<SimulationStartRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        //Return Decision point
        Ok(Response::new(SimulationStepResponse {
            new_decision_point: Some(create_decision_point_from_L5()),
        }))
    }

    //Function currently returns dummy data
    async fn take_simulation_step(
        &self,
        request: Request<SimulationStepRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        let edge_one = String::from("E27");
        let edge_two = String::from("E29");
        let new_decision_point: DecisionPoint = match request
            .into_inner()
            .chosen_decision
            .unwrap()
            .edge
            .unwrap()
            .id
        {
            edge_one => create_decision_point_from_L5(),
            edge_two => create_decision_point_from_L4(),
            _ => panic!("Chosen edge is not valid"),
        };
        Ok(Response::new(SimulationStepResponse {
            new_decision_point: Some(new_decision_point),
        }))
    }
}
