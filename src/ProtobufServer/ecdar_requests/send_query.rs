use std::collections::HashMap;
use std::sync::Arc;

use crate::component::Component;
use crate::xml_parser::parse_xml_from_str;
use crate::DataReader::component_loader::ModelCache;
use crate::DataReader::json_reader::json_to_component;
use crate::DataReader::json_writer::component_to_json;
use crate::DataReader::parse_queries;
use crate::ModelObjects::queries::Query;
use crate::ProtobufServer::services::component::Rep;
use crate::ProtobufServer::services::query_response::query_ok::Result as ProtobufResult;
use crate::ProtobufServer::services::query_response::query_ok::{
    ComponentResult, ConsistencyResult as ProtobufConsistencyResult,
    DeterminismResult as ProtobufDeterminismResult, RefinementResult,
};
use crate::ProtobufServer::services::query_response::QueryOk;
use crate::ProtobufServer::services::query_response::Response as QueryOkOrErrorResponse;
use crate::ProtobufServer::services::{
    Component as ProtobufComponent, ComponentClock as ProtobufComponentClock,
    Conjunction as ProtobufConjunction, Constraint as ProtobufConstraint,
    Disjunction as ProtobufDisjunction, Federation, Location, LocationTuple, QueryRequest,
    QueryResponse, State,
};
use crate::ProtobufServer::ConcreteEcdarBackend;
use crate::System::executable_query::QueryResult;
use crate::System::local_consistency::{ConsistencyFailure, ConsistencyResult, DeterminismResult};
use crate::System::refine::{self, RefinementFailure};
use crate::System::{extract_system_rep, input_enabler};
use crate::TransitionSystems::{self, LocationID};
use edbm::util::constraints::Disjunction;
use log::trace;
use tonic::Status;

impl ConcreteEcdarBackend {
    pub fn handle_send_query(
        query_request: QueryRequest,
        mut model_cache: ModelCache,
    ) -> Result<QueryResponse, Status> {
        trace!("Received query: {:?}", query_request);
        let components_info = query_request.components_info.as_ref().unwrap();
        let proto_components = &components_info.components;
        let query = parse_query(&query_request)?;

        let mut component_container = match model_cache.get_model(components_info.components_hash) {
            Some(model) => model,
            None => {
                let mut parsed_components = vec![];

                for proto_component in proto_components {
                    let components = parse_components_if_some(proto_component)?;
                    for component in components {
                        parsed_components.push(component);
                    }
                }

                let components = create_components(parsed_components);
                model_cache.insert_model(components_info.components_hash, Arc::new(components))
            }
        };

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
                info: vec![],
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

fn create_components(components: Vec<Component>) -> HashMap<String, Component> {
    let mut comp_hashmap = HashMap::<String, Component>::new();
    for mut component in components {
        trace!("Adding comp {} to container", component.get_name());

        component.create_edge_io_split();
        let inputs: Vec<_> = component
            .get_input_actions()
            .into_iter()
            .map(|channel| channel.name)
            .collect();
        input_enabler::make_input_enabled(&mut component, &inputs);
        comp_hashmap.insert(component.get_name().to_string(), component);
    }
    comp_hashmap
}

fn convert_ecdar_result(query_result: &QueryResult) -> Option<ProtobufResult> {
    match query_result {
        QueryResult::Refinement(refines) => match refines {
            refine::RefinementResult::Success => {
                Some(ProtobufResult::Refinement(RefinementResult {
                    success: true,
                    reason: "".to_string(),
                    relation: vec![],
                    state: None,
                    action: "".to_string(), // Empty string is used, when no failing action is available.
                }))
            }
            refine::RefinementResult::Failure(failure) => convert_refinement_failure(failure),
        },

        QueryResult::Reachability(_, _) => {
            unimplemented!("Not implemented, but should be implemented");
        }

        QueryResult::GetComponent(comp) => Some(ProtobufResult::Component(ComponentResult {
            component: Some(ProtobufComponent {
                rep: Some(Rep::Json(component_to_json(comp))),
            }),
        })),
        QueryResult::Consistency(is_consistent) => match is_consistent {
            ConsistencyResult::Success => {
                Some(ProtobufResult::Consistency(ProtobufConsistencyResult {
                    success: true,
                    reason: "".to_string(),
                    state: None,
                    action: "".to_string(),
                }))
            }
            ConsistencyResult::Failure(failure) => match failure {
                ConsistencyFailure::NoInitialLocation | ConsistencyFailure::EmptyInitialState => {
                    Some(ProtobufResult::Consistency(ProtobufConsistencyResult {
                        success: false,
                        reason: failure.to_string(),
                        state: None,
                        action: "".to_string(),
                    }))
                }
                ConsistencyFailure::NotConsistentFrom(location_id, action)
                | ConsistencyFailure::NotDeterministicFrom(location_id, action) => {
                    Some(ProtobufResult::Consistency(ProtobufConsistencyResult {
                        success: false,
                        reason: failure.to_string(),
                        state: Some(State {
                            location_tuple: Some(LocationTuple {
                                locations: vec![Location {
                                    id: location_id.to_string(),
                                    specific_component: None,
                                }],
                            }),
                            federation: None,
                        }),
                        action: action.to_string(),
                    }))
                }
            },
        },
        QueryResult::Determinism(is_deterministic) => match is_deterministic {
            DeterminismResult::Success => {
                Some(ProtobufResult::Determinism(ProtobufDeterminismResult {
                    success: true,
                    reason: "".to_string(),
                    state: None,
                    action: "".to_string(),
                }))
            }
            DeterminismResult::Failure(location_id, action) => {
                Some(ProtobufResult::Determinism(ProtobufDeterminismResult {
                    success: false,
                    reason: "Not deterministic From Location".to_string(),
                    state: Some(State {
                        location_tuple: Some(LocationTuple {
                            locations: vec![Location {
                                id: location_id.to_string(),
                                specific_component: None,
                            }],
                        }),
                        federation: None,
                    }),
                    action: action.to_string(),
                }))
            }
        },
        QueryResult::Error(message) => Some(ProtobufResult::Error(message.clone())),
    }
}

fn convert_refinement_failure(failure: &RefinementFailure) -> Option<ProtobufResult> {
    match failure {
        RefinementFailure::NotDisjointAndNotSubset
        | RefinementFailure::NotDisjoint
        | RefinementFailure::NotSubset
        | RefinementFailure::EmptySpecification
        | RefinementFailure::EmptyImplementation => {
            Some(ProtobufResult::Refinement(RefinementResult {
                success: false,
                relation: vec![],
                state: None,
                reason: failure.to_string(),
                action: "".to_string(),
            }))
        }
        RefinementFailure::CutsDelaySolutions(state_pair)
        | RefinementFailure::InitialState(state_pair)
        | RefinementFailure::EmptyTransition2s(state_pair)
        | RefinementFailure::NotEmptyResult(state_pair) => {
            Some(ProtobufResult::Refinement(RefinementResult {
                success: false,
                relation: vec![],
                state: Some(State {
                    federation: make_proto_zone(state_pair.ref_zone().minimal_constraints()),
                    location_tuple: Some(LocationTuple {
                        locations: make_location_vec(
                            state_pair.get_locations1(),
                            state_pair.get_locations2(),
                        ),
                    }),
                }),
                reason: failure.to_string(),
                action: "".to_string(),
            }))
        }
        RefinementFailure::ConsistencyFailure(location_id, action)
        | RefinementFailure::DeterminismFailure(location_id, action) => {
            Some(ProtobufResult::Refinement(RefinementResult {
                success: false,
                reason: failure.to_string(),
                state: Some(State {
                    location_tuple: Some(LocationTuple {
                        locations: vec![Location {
                            id: value_in_location(location_id),
                            specific_component: None,
                        }],
                    }),
                    federation: None,
                }),
                action: value_in_action(action),
                relation: vec![],
            }))
        }
    }
}

/// CAREFUL: sets specific_component to None
fn make_location_vec(
    locations1: &TransitionSystems::LocationTuple,
    locations2: &TransitionSystems::LocationTuple,
) -> Vec<Location> {
    let loc_vec: Vec<Location> = vec![
        Location {
            id: locations1.id.to_string(),
            specific_component: None,
        },
        Location {
            id: locations2.id.to_string(),
            specific_component: None,
        },
    ];
    loc_vec
}

/// CAREFUL: sets specific_component to None for ComponentClocks
fn make_proto_zone(disjunction: Disjunction) -> Option<Federation> {
    let mut conjunctions: Vec<ProtobufConjunction> = vec![];
    for conjunction in disjunction.conjunctions.iter() {
        let mut constraints: Vec<ProtobufConstraint> = vec![];
        for constraint in conjunction.constraints.iter() {
            constraints.push(ProtobufConstraint {
                x: Some(ProtobufComponentClock {
                    specific_component: None,
                    clock_name: constraint.i.to_string(),
                }),
                y: Some(ProtobufComponentClock {
                    specific_component: None,
                    clock_name: constraint.j.to_string(),
                }),
                strict: constraint.ineq().is_strict(),
                c: constraint.ineq().bound(),
            });
        }
        conjunctions.push(ProtobufConjunction { constraints })
    }
    Some(Federation {
        disjunction: Some(ProtobufDisjunction { conjunctions }),
    })
}

fn value_in_location(maybe_location: &Option<LocationID>) -> String {
    match maybe_location {
        Some(location_id) => location_id.to_string(),
        None => "".to_string(),
    }
}

fn value_in_action(maybe_action: &Option<String>) -> String {
    match maybe_action {
        Some(action) => action.to_string(),
        None => "".to_string(),
    }
}
