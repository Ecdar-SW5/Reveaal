use tonic::Request;

use crate::ProtobufServer::services::{
    self, Component, ComponentClock, ComponentsInfo, Conjunction, Constraint, DecisionPoint,
    Disjunction, Edge, Federation, Location, LocationTuple, SimulationInfo, SimulationStartRequest,
    SimulationStepRequest, SpecificComponent, State,
};
use std::fs;

static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

pub fn create_edges_from_L5() -> Vec<Edge> {
    vec![
        Edge {
            id: "E27".to_string(),
            specific_component: None,
        },
        Edge {
            id: "E29".to_string(),
            specific_component: None,
        },
    ]
}

pub fn create_1tuple_state_with_single_constraint(
    id: &str,
    component_name: &str,
    component_index: u32,
    clock_x_name: &str,
    clock_y_name: &str,
    clock_constraint: i32,
    is_constrain_strict: bool,
) -> State {
    State {
        location_tuple: Some(LocationTuple {
            locations: vec![Location {
                id: String::from(id),
                specific_component: Some(SpecificComponent {
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
                                specific_component: Some(SpecificComponent {
                                    component_name: String::from(component_name),
                                    component_index,
                                }),
                                clock_name: String::from(clock_x_name),
                            }),
                            y: Some(services::ComponentClock {
                                specific_component: Some(SpecificComponent {
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

// Create the decision point drawn below:
//
//           -----coin? E3----->
//          /
// (L5, universe)-------tea! E5----->
//
pub fn create_initial_decision_point() -> DecisionPoint {
    DecisionPoint {
        source: Some(State {
            location_tuple: Some(LocationTuple {
                locations: vec![Location {
                    id: "L5".to_string(),
                    specific_component: Some(SpecificComponent {
                        component_name: "Machine".to_string(),
                        component_index: 0,
                    }),
                }],
            }),
            federation: Some(Federation {
                disjunction: Some(Disjunction {
                    conjunctions: vec![Conjunction {
                        constraints: vec![],
                    }],
                }),
            }),
        }),
        edges: create_edges_from_L5(),
    }
}

// Returns the Machine component as a String, in the .json format
pub fn create_sample_json_component() -> String {
    fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap()
}

pub fn create_json_component_as_string(path: String) -> String {
    fs::read_to_string(format!("{}", path)).unwrap()
}

pub fn convert_json_component_to_string(path: String) -> String {
    let component = fs::read_to_string(path);
    match component {
        Ok(component) => component,
        Err(_) => panic!("Could not read component"),
    }
}

// Create the decision point drawn below:
//
//           -----coin? E3----->
//          /
// (L5,y>=2)-------tea! E5----->
//
pub fn create_decision_point_after_taking_E5() -> DecisionPoint {
    DecisionPoint {
        source: Some(State {
            location_tuple: Some(LocationTuple {
                locations: vec![Location {
                    id: "L5".to_string(),
                    specific_component: Some(SpecificComponent {
                        component_name: "Machine".to_string(),
                        component_index: 0,
                    }),
                }],
            }),
            federation: Some(Federation {
                disjunction: Some(Disjunction {
                    conjunctions: vec![Conjunction {
                        constraints: vec![Constraint {
                            x: Some(ComponentClock {
                                specific_component: None,
                                clock_name: "0".to_string(),
                            }),
                            y: Some(ComponentClock {
                                specific_component: Some(SpecificComponent {
                                    component_name: "Machine".to_string(),
                                    component_index: 0,
                                }),
                                clock_name: "y".to_string(),
                            }),
                            strict: false,
                            c: -2,
                        }],
                    }],
                }),
            }),
        }),
        edges: create_edges_from_L5(),
    }
}

// Create a simulation state with the Machine component and the decision point drawn below:
//
//          -----coin? E3----->
//         /
// (ε,y>=0)-------tea! E5----->
//
pub fn create_state_not_in_machine() -> State {
    create_1tuple_state_with_single_constraint("", "Machine", 0, "0", "y", 0, false)
}

pub fn create_simulation_step_request(
    simulation_info: SimulationInfo,
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

// create a state such that can't transition via E5
pub fn create_state_setup_for_mismatch() -> State {
    create_1tuple_state_with_single_constraint("L5", "Machine", 0, "y", "0", 2, true)
}

pub fn create_empty_state() -> State {
    State {
        location_tuple: None,
        federation: None,
    }
}

pub fn create_empty_edge() -> Edge {
    Edge {
        id: String::from(""),
        specific_component: None,
    }
}

pub fn create_simulation_info_from(composition: String, component_json: String) -> SimulationInfo {
    SimulationInfo {
        component_composition: composition,
        components_info: Some(ComponentsInfo {
            components: vec![Component {
                rep: Some(services::component::Rep::Json(component_json)),
            }],
            components_hash: 0, // TODO this is incorrect
        }),
    }
}
