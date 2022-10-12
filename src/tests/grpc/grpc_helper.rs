use tonic::Request;

use crate::ProtobufServer::services::{self, SimulationStepRequest, component, SimulationStartRequest, ComponentsInfo, Component, LocationTuple, location_tuple::Location, SpecificComponent};
use std::fs;

static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

// Create a simulation state with the Machine component and the decision point drawn below:
//
//           -----coin? E3----->
//          /
// (L5,y>=0)-------tea! E5----->
//
pub fn create_initial_state() -> services::SimulationState {
    let component_json = create_sample_json_component();

    services::SimulationState {
        component_composition: String::from("Machine"),
        components_info: Some(ComponentsInfo {
            components: vec![
                Component {
                    rep: Some(services::component::Rep::Json(component_json.clone()))
                }
            ],
            components_hash: 0 // TODO this is incorrect
        }), 
        decision_points: vec![services::DecisionPoint {
            source: Some(services::State {
                location_tuple: Some(LocationTuple {
                    locations: vec![
                        Location {
                            id: String::from("L5"),
                            component_name: String::from("Machine")
                        }
                    ]
                }),
                zone: Some(services::Zone {
                    disjunction: Some(services::Disjunction {
                        conjunctions: vec![services::Conjunction {
                            constraints: vec![
                                // constraint (0 - y <= 0)
                                services::Constraint {
                                    x: Some(services::ComponentClock {
                                        specific_component: Some(SpecificComponent {
                                            component_name: String::from("Machine"),
                                            component_index: 0 
                                        }),
                                        clock_name: "0".to_string(),
                                    }),
                                    y: Some(services::ComponentClock {
                                        specific_component: Some(SpecificComponent {
                                            component_name: String::from("Machine"),
                                            component_index: 0 
                                        }),
                                        clock_name: "y".to_string(),
                                    }),
                                    strict: false,
                                    c: 0,
                                },
                            ],
                        }],
                    }),
                }),
            }),
            edges: vec![
                services::Edge {
                    id: "E3".to_string(),
                    specific_component: Some(SpecificComponent {
                        component_name: String::from("Machine"),
                        component_index: 0 
                    }),
                },
                services::Edge {
                    id: "E5".to_string(),
                    specific_component: Some(SpecificComponent {
                        component_name: String::from("Machine"),
                        component_index: 0 
                    }),
                },
            ],
        }],
    }
}

// Create a simulation state that has the Machine component and the decision point drawn below:
//
//           -----coin? E3----->
//          /
// (L5,y>=0)-------tea! E5----->
//
//           -----coin? E3----->
//          /
// (L5,y>=2)-------tea! E5----->
//
pub fn create_state_after_taking_step() -> services::SimulationState {
    let mut initial_state = create_initial_state();

    initial_state.decision_points.push(services::DecisionPoint {
        source: Some(services::State {
            location_tuple: Some(LocationTuple {
                locations: vec![
                    Location {
                        id: String::from("L5"),
                        component_name: String::from("Machine")
                    }
                ]
            }),
            zone: Some(services::Zone {
                disjunction: Some(services::Disjunction {
                    conjunctions: vec![services::Conjunction {
                        constraints: vec![
                            // constraint (0 - y <= -2)
                            services::Constraint {
                                x: Some(services::ComponentClock {
                                    specific_component: Some(SpecificComponent {
                                        component_name: String::from("Machine"),
                                        component_index: 0 
                                    }),
                                    clock_name: "0".to_string(),
                                }),
                                y: Some(services::ComponentClock {
                                    specific_component: Some(SpecificComponent {
                                        component_name: String::from("Machine"),
                                        component_index: 0 
                                    }),
                                    clock_name: "y".to_string(),
                                }),
                                strict: false,
                                c: -2,
                            },
                        ],
                    }],
                }),
            }),
        }),
        edges: initial_state.decision_points[0].edges.clone(),
    });
    initial_state
}

// Create a simulation state with the Machine component and the decision point drawn below:
//
//              -----coin? E3----->
//             /
// (Wrong,y>=0)-------tea! E5----->
//
pub fn create_sample_state_component_decision_mismatch_1() -> services::SimulationState {
    let mut initial_state = create_initial_state();

    initial_state.decision_points.push(services::DecisionPoint {
        source: Some(services::State {
            location_tuple: Some(LocationTuple {
                locations: vec![
                    Location {
                        id: String::from("Wrong"),
                        component_name: String::from("Machine")
                    }
                ]
            }),
            zone: Some(services::Zone {
                disjunction: Some(services::Disjunction {
                    conjunctions: vec![services::Conjunction {
                        constraints: vec![
                            // constraint (0 - y <= 0)
                            services::Constraint {
                                x: Some(services::ComponentClock {
                                    specific_component: Some(SpecificComponent {
                                        component_name: String::from("Machine"),
                                        component_index: 0 
                                    }),
                                    clock_name: "0".to_string(),
                                }),
                                y: Some(services::ComponentClock {
                                    specific_component: Some(SpecificComponent {
                                        component_name: String::from("Machine"),
                                        component_index: 0 
                                    }),
                                    clock_name: "y".to_string(),
                                }),
                                strict: false,
                                c: 0,
                            },
                        ],
                    }],
                }),
            }),
        }),
        edges: initial_state.decision_points[0].edges.clone(),
    });
    initial_state
}

pub fn create_simulation_step_request(
    current_state: services::SimulationState,
    source: services::State,
    edge: services::Edge,
) -> SimulationStepRequest {
    services::SimulationStepRequest {
        current_state: Some(current_state.clone()),
        chosen_decision: Some(services::Decision {
            source: Some(source),
            edge: Some(edge),
        }),
    }
}

pub fn create_simulation_start_request(
    composition: String,
    component_json: String
) -> Request<SimulationStartRequest> {
    Request::new(SimulationStartRequest {
        component_composition: composition,
        components_info: Some(ComponentsInfo {
            components: vec![Component {
                rep: Some(component::Rep::Json(component_json)),
            }],
            components_hash: 0, // TODO: this is not correct, but will do for now
        }),
    })
}

// Returns the Machine component as a String, in the .json format
pub fn create_sample_json_component() -> String {
    fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap()
}

pub fn create_sample_state_component_decision_mismatch_2() -> services::SimulationState {
    let component_json = create_sample_json_component();

    services::SimulationState {
        component_composition: String::from("Machine"),
        components_info: Some(ComponentsInfo {
            components: vec![
                Component {
                    rep: Some(services::component::Rep::Json(component_json.clone()))
                }
            ],
            components_hash: 0 // TODO this is incorrect
        }), 
        decision_points: vec![services::DecisionPoint {
            source: Some(services::State {
                location_tuple: Some(LocationTuple {
                    locations: vec![
                        Location {
                            id: String::from("L5"),
                            component_name: String::from("Machine")
                        }
                    ]
                }),
                zone: Some(services::Zone {
                    disjunction: Some(services::Disjunction {
                        conjunctions: vec![services::Conjunction {
                            constraints: vec![
                                // constraint (y - 0 < 2) <= (y < 22)
                                services::Constraint {
                                    x: Some(services::ComponentClock {
                                        specific_component: None,
                                        clock_name: "y".to_string(),
                                    }),
                                    y: Some(services::ComponentClock {
                                        specific_component: None,
                                        clock_name: "0".to_string(),
                                    }),
                                    strict: true,
                                    c: 2,
                                },
                            ],
                        }],
                    }),
                }),
            }),
            edges: vec![
                services::Edge {
                    id: "E3".to_string(),
                    specific_component: None,
                },
                // Should not be able to take this edge, but somehow the gui group made it happen
                services::Edge {
                    id: "E5".to_string(),
                    specific_component: None,
                },
            ],
        }],
    }
}
