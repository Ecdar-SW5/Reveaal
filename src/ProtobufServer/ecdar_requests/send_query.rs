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
    pub async fn handle_send_query(
        &self,
        request: AssertUnwindSafe<Request<ProtobufQuery>>,
    ) -> Result<Response<QueryResponse>, Status> {
        trace!("Received query: {:?}", request);
        let query_request = request.0.into_inner();

        let query = parse_query(&query_request)?;

        let components = self.get_components_lock()?;
        let mut component_container = components.borrow_mut();

        if query_request.ignored_input_outputs.is_some() {
            return Err(Status::unimplemented(
                "ignored input outputs are currently not supported",
            ));
        }

        let executable_query =
            match extract_system_rep::create_executable_query(&query, &mut *component_container) {
                Ok(query) => query,
                Err(e) => {
                    return Err(Status::invalid_argument(format!(
                        "Creation of query failed: {}",
                        e
                    )))
                }
            };
        let result = executable_query.execute();

        let reply = QueryResponse {
            query: Some(query_request),
            result: convert_ecdar_result(&result),
        };

        Ok(Response::new(reply))
    }
}

fn parse_query(query_request: &ProtobufQuery) -> Result<Query, Status> {
    let mut queries = parse_queries::parse_to_query(&query_request.query);

    if queries.len() != 1 {
        Err(Status::invalid_argument(
            "This procedure takes in exactly 1 query",
        ))
    } else {
        Ok(queries.remove(0))
    }
}

fn convert_ecdar_result(query_result: &QueryResult) -> Option<ProtobufResult> {
    match query_result {
        QueryResult::Refinement(refines) => Some(ProtobufResult::Refinement(RefinementResult {
            success: *refines,
            relation: vec![],
        })),

        QueryResult::Reachability(_, _) =>{
            panic!("Not implemented, but should be implemented");
        }

        QueryResult::GetComponent(comp) => Some(ProtobufResult::Component(ComponentResult {
            component: Some(Component {
                rep: Some(Rep::Json(component_to_json(comp))),
            }),
        })),
        QueryResult::Consistency(is_consistent) => {
            Some(ProtobufResult::Consistency(ConsistencyResult {
                success: *is_consistent,
            }))
        }
        QueryResult::Determinism(is_deterministic) => {
            Some(ProtobufResult::Determinism(DeterminismResult {
                success: *is_deterministic,
            }))
        }
        QueryResult::Error(message) => Some(ProtobufResult::Error(message.clone())),
    }
}
