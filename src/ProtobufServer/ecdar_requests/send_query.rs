use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;

use crate::DataReader::component_loader::ComponentContainer;
use crate::DataReader::json_writer::component_to_json;
use crate::DataReader::parse_queries;
use crate::ModelObjects::queries::Query;
use crate::ProtobufServer::services::component::Rep;
use crate::ProtobufServer::services::query_response::query_ok::Result as ProtobufResult;
use crate::ProtobufServer::services::query_response::query_ok::{
    ComponentResult, ConsistencyResult, DeterminismResult, RefinementResult,
};
use crate::ProtobufServer::services::query_response::QueryOk;
use crate::ProtobufServer::services::query_response::Response as QueryOkOrErrorResponse;
use crate::ProtobufServer::services::{
    Component as ProtobufComponent, QueryRequest, QueryResponse,
};
use crate::System::executable_query::QueryResult;
use crate::System::extract_system_rep;
use log::trace;
use tonic::Status;

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub fn handle_send_query(
        query_request: QueryRequest,
        mut model_cache: ModelCache,
    ) -> Result<QueryResponse, Status> {
        trace!("Received query: {:?}", query_request);
        let components_info = query_request.components_info.as_ref().unwrap();
        let proto_components = &components_info.components;
        let query = parse_query(&query_request)?;

        let components_info = query_request.components_info.as_ref().unwrap();
        let mut component_container = ComponentContainer::from(components_info)?;

        if query_request.ignored_input_outputs.is_some() {
            return Err(Status::unimplemented(
                "ignored input outputs are currently not supported",
            ));
        }

        let executable_query =
            match extract_system_rep::create_executable_query(&query, &mut component_container) {
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
            response: Some(QueryOkOrErrorResponse::QueryOk(QueryOk {
                query_id: query_request.query_id,
                result: convert_ecdar_result(&result),
            })),
        };

        Ok(reply)
    }
}

fn parse_query(query_request: &QueryRequest) -> Result<Query, Status> {
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
            reason: "".to_string(),
            relation: vec![],
            state: None,
        })),

        QueryResult::Reachability(_, _) => {
            unimplemented!("Not implemented, but should be implemented");
        }

        QueryResult::GetComponent(comp) => Some(ProtobufResult::Component(ComponentResult {
            component: Some(ProtobufComponent {
                rep: Some(Rep::Json(component_to_json(comp))),
            }),
        })),
        QueryResult::Consistency(is_consistent) => {
            Some(ProtobufResult::Consistency(ConsistencyResult {
                success: *is_consistent,
                reason: "".to_string(),
                state: None,
            }))
        }
        QueryResult::Determinism(is_deterministic) => {
            Some(ProtobufResult::Determinism(DeterminismResult {
                success: *is_deterministic,
                reason: "".to_string(),
                state: None,
            }))
        }
        QueryResult::Error(message) => Some(ProtobufResult::Error(message.clone())),
    }
}
