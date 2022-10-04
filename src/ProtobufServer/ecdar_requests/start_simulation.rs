use std::panic::AssertUnwindSafe;

use crate::DataReader::json_writer::component_to_json;
use crate::DataReader::parse_queries;
use crate::ModelObjects::queries::Query;
use crate::ProtobufServer::services::component::Rep;
use crate::ProtobufServer::services::query_response::Result as ProtobufResult;
use crate::ProtobufServer::services::query_response::{
    ComponentResult, ConsistencyResult, DeterminismResult, RefinementResult,
};
use crate::ProtobufServer::services::{Component, Query as ProtobufQuery, QueryResponse};
use crate::System::executable_query::QueryResult;
use crate::System::extract_system_rep;
use log::trace;
use tonic::{Request, Response, Status};

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub async fn handle_start_simulation(
        &self,
        request: AssertUnwindSafe<Request<()>>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        trace!("Recieved query: {:?}", request);
        let start_simulation_request = request.0.into_inner();

        let components = self.get_components_lock()?;
        let mut component_container = components.borrow_mut();

        Ok(Response::new());
    }
}