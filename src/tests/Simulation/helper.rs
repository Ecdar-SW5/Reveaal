use std::{fs, vec};

use tonic::Request;

use crate::ProtobufServer::services::{
    self, Component as ProtoComponent, ComponentsInfo as ProtoComponentsInfo, Edge as ProtoEdge,
    SimulationInfo as ProtoSimulationInfo, SimulationStartRequest, SimulationStepRequest,
    State as ProtoState,
};
use crate::{
    DataReader::json_reader::read_json_component,
    ProtobufServer::services::component::Rep,
    TransitionSystems::{
        transition_system::components_to_transition_system, CompositionType, TransitionSystemPtr,
    },
};

pub fn create_system_from_path(path: &str, name: &str) -> TransitionSystemPtr {
    let component = read_json_component(path, name);
    components_to_transition_system(vec![component], name)
}

pub fn create_simulation_info(
    composition: String,
    components: Vec<ProtoComponent>,
) -> ProtoSimulationInfo {
    ProtoSimulationInfo {
        component_composition: composition,
        components_info: Some(ProtoComponentsInfo {
            components,
            components_hash: 0,
        }),
        user_id: 0,
    }
}

pub fn create_composition_string(comp_names: &Vec<&str>, comp_type: CompositionType) -> String {
    let mut composition = String::new();
    for (i, name) in comp_names.iter().enumerate() {
        composition.push_str(name);
        if i < comp_names.len() - 1 {
            match comp_type {
                CompositionType::Conjunction => composition.push_str(" && "),
                CompositionType::Composition => composition.push_str(" || "),
                CompositionType::Quotient => {
                    unimplemented!("Quotient composition not implemented")
                }
                CompositionType::Simple => unimplemented!("Simple composition not implemented"),
            }
        }
    }
    composition
}

pub fn create_components(comp_names: &[&str], sample_name: String) -> Vec<ProtoComponent> {
    let components: Vec<String> = comp_names
        .iter()
        .map(|name| {
            create_json_component_as_string(format!(
                "samples/json/{}/Components/{}.json",
                sample_name, name
            ))
        })
        .collect();

    let components: Vec<ProtoComponent> = components
        .iter()
        .map(|string| ProtoComponent {
            rep: Some(Rep::Json(string.clone())),
        })
        .collect();

    components
}

pub fn create_1tuple_state_with_single_constraint(
    id: &str,
    component_name: &str,
    component_index: u32,
    clock_x_name: &str,
    clock_y_name: &str,
    clock_constraint: i32,
    is_constrain_strict: bool,
) -> services::State {
    services::State {
        location_tuple: Some(services::LocationTuple {
            locations: vec![services::Location {
                id: String::from(id),
                specific_component: Some(services::SpecificComponent {
                    component_name: String::from(component_name),
                    component_index,
                }),
            }],
        }),
        federation: Some(services::Federation {
            disjunction: Some(services::Disjunction {
                conjunctions: vec![services::Conjunction {
                    constraints: vec![
                        // constraint (x - y <= c)
                        services::Constraint {
                            x: Some(services::ComponentClock {
                                specific_component: Some(services::SpecificComponent {
                                    component_name: String::from(component_name),
                                    component_index,
                                }),
                                clock_name: String::from(clock_x_name),
                            }),
                            y: Some(services::ComponentClock {
                                specific_component: Some(services::SpecificComponent {
                                    component_name: String::from(component_name),
                                    component_index,
                                }),
                                clock_name: String::from(clock_y_name),
                            }),
                            strict: is_constrain_strict,
                            c: clock_constraint,
                        },
                    ],
                }],
            }),
        }),
    }
}

pub fn create_json_component_as_string(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn create_simulation_step_request(
    simulation_info: ProtoSimulationInfo,
    source: services::State,
    edge: services::Edge,
) -> SimulationStepRequest {
    SimulationStepRequest {
        simulation_info: Some(simulation_info),
        chosen_decision: Some(services::Decision {
            source: Some(source),
            edge: Some(edge),
        }),
    }
}

pub fn create_simulation_start_request(
    composition: String,
    component_json: String,
) -> Request<SimulationStartRequest> {
    Request::new(SimulationStartRequest {
        simulation_info: Some(create_simulation_info_from(composition, component_json)),
    })
}

pub fn create_empty_state() -> ProtoState {
    ProtoState {
        location_tuple: None,
        federation: None,
    }
}

pub fn create_empty_edge() -> ProtoEdge {
    ProtoEdge {
        id: String::from(""),
        specific_component: None,
    }
}

pub fn create_simulation_info_from(
    composition: String,
    component_json: String,
) -> ProtoSimulationInfo {
    ProtoSimulationInfo {
        user_id: 0,
        component_composition: composition,
        components_info: Some(ProtoComponentsInfo {
            components: vec![ProtoComponent {
                rep: Some(services::component::Rep::Json(component_json)),
            }],
            components_hash: 0,
        }),
    }
}
