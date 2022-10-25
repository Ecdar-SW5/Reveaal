use std::collections::HashMap;
use std::panic::AssertUnwindSafe;

use crate::component::Component;
use crate::logging;
use crate::logging::*;
use crate::xml_parser::parse_xml_from_str;
use crate::DataReader::component_loader::ComponentContainer;
use crate::DataReader::json_reader::json_to_component;
use crate::DataReader::json_writer::component_to_json;
use crate::DataReader::parse_queries;
use crate::ModelObjects::queries::Query;
use crate::ProtobufServer::services::component::Rep;
use crate::ProtobufServer::services::query_response::query_ok::Result as ProtobufResult;
use crate::ProtobufServer::services::query_response::query_ok::{
    ComponentResult, ConsistencyResult, DeterminismResult, Information, RefinementResult,
};
use crate::ProtobufServer::services::query_response::QueryOk;
use crate::ProtobufServer::services::query_response::Response as QueryOkOrErrorResponse;
use crate::ProtobufServer::services::{
    Component as ProtobufComponent, QueryRequest, QueryResponse,
};
use crate::System::executable_query::QueryResult;
use crate::System::{extract_system_rep, input_enabler};
use log::{logger, trace};
use simplelog::WriteLogger;
use tonic::{Request, Response, Status};

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub async fn handle_send_query(
        &self,
        request: AssertUnwindSafe<Request<QueryRequest>>,
    ) -> Result<Response<QueryResponse>, Status> {
        trace!("Received query: {:?}", request);
        let query_request = request.0.into_inner();
        let components_info = query_request.components_info.as_ref().unwrap();
        let proto_components = &components_info.components;
        let query = parse_query(&query_request)?;

        let parsed_components: Vec<Component> = proto_components
            .iter()
            .map(|p: &ProtobufComponent| parse_components_if_some(p))
            .flatten()
            .flatten()
            .collect::<Vec<Component>>();
        /*
              for proto_component in proto_components {
                  let components = parse_components_if_some(proto_component)?;
                  for component in components {
                      parsed_components.push(component);
                  }
              }
        */

        let mut component_container =
            create_component_container(parsed_components, query_request.should_reduce_clocks);

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

        let msgs = get_messages(); // TODO: Not raw
        println!("{:?}", msgs);

        let reply = QueryResponse {
            response: Some(QueryOkOrErrorResponse::QueryOk(QueryOk {
                query_id: query_request.query_id,
                result: convert_ecdar_result(&result),
                info: vec![Information {
                    subject: "test".to_string(),
                    message: "nsoedundo".to_string(),
                }], //TODO: Should be whatever is written through `info!` (logging)
            })),
        };

        Ok(Response::new(reply))
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

fn parse_components_if_some(
    proto_component: &ProtobufComponent,
) -> Result<Vec<Component>, tonic::Status> {
    if let Some(rep) = &proto_component.rep {
        match rep {
            Rep::Json(json) => parse_json_component(json),
            Rep::Xml(xml) => Ok(parse_xml_components(xml)),
        }
    } else {
        Ok(vec![])
    }
}

fn parse_json_component(json: &str) -> Result<Vec<Component>, tonic::Status> {
    match json_to_component(json) {
        Ok(comp) => Ok(vec![comp]),
        Err(_) => Err(tonic::Status::invalid_argument(
            "Failed to parse json component",
        )),
    }
}

fn parse_xml_components(xml: &str) -> Vec<Component> {
    let (comps, _, _) = parse_xml_from_str(xml);
    comps
}

fn create_component_container(
    components: Vec<Component>,
    should_clock_reduce: bool,
) -> ComponentContainer {
    let mut comp_hashmap = HashMap::<String, Component>::new();
    for mut component in components {
        trace!("Adding comp {} to container", component.get_name());
        if should_clock_reduce {
            component.reduce_clocks(component.find_redundant_clocks())
        }

        component.create_edge_io_split();
        let inputs: Vec<_> = component
            .get_input_actions()
            .into_iter()
            .map(|channel| channel.name)
            .collect();
        input_enabler::make_input_enabled(&mut component, &inputs);
        comp_hashmap.insert(component.get_name().to_string(), component);
    }
    ComponentContainer::new(comp_hashmap)
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
