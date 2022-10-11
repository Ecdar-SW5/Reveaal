use crate::ProtobufServer::services;
use std::fs;

static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

// Create a simulation state with the Machine component and the decision point drawn below:
//
//           -----coin?----->
//          /
// (L5,y>=0)-------tea!----->
//
pub fn create_initial_state() -> services::SimulationState {
    let component_json = create_sample_json_component();

    services::SimulationState {
        component: Some(services::Component {
            rep: Some(services::component::Rep::Json(component_json.clone())),
        }),
        decision_points: vec![services::DecisionPoint {
            source: Some(services::State {
                location_id: "L5".to_string(),
                zone: Some(services::Zone {
                    disjunction: Some(services::Disjunction {
                        conjunctions: vec![services::Conjunction {
                            constraints: vec![
                                // constraint (0 - y <= 0) <= (y >= 0)
                                services::Constraint {
                                    x: Some(services::ComponentClock {
                                        specific_component: None,
                                        clock_name: "0".to_string(),
                                    }),
                                    y: Some(services::ComponentClock {
                                        specific_component: None,
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
                    specific_component: None,
                },
                services::Edge {
                    id: "E5".to_string(),
                    specific_component: None,
                },
            ],
        }],
    }
}

// Create a simulation state with the Machine component and the decision point drawn below:
//
//           -----coin?----->
//          /
// (L5,y>=0)-------tea!----->
//
//           -----coin?----->
//          /
// (L5,y>=2)-------tea!----->
//
pub fn create_state_after_taking_step() -> services::SimulationState {
    let mut new_state = create_initial_state();
    new_state.decision_points.push(services::DecisionPoint {
        source: Some(services::State {
            location_id: "L5".to_string(),
            zone: Some(services::Zone {
                disjunction: Some(services::Disjunction {
                    conjunctions: vec![services::Conjunction {
                        constraints: vec![
                            // constraint (0 - y <= -2) <= (y >= 2)
                            services::Constraint {
                                x: Some(services::ComponentClock {
                                    specific_component: None,
                                    clock_name: "0".to_string(),
                                }),
                                y: Some(services::ComponentClock {
                                    specific_component: None,
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
        edges: new_state.decision_points[0].edges.clone(),
    });
    new_state
}

// Create a simulation state with the Machine component and the decision point drawn below:
//
//              -----coin?----->
//             /
// (Wrong,y>=0)-------tea!----->
//
pub fn create_sample_state_component_decision_mismatch() -> services::SimulationState {
    let mut new_state = create_initial_state();
    new_state.decision_points.push(services::DecisionPoint {
        source: Some(services::State {
            location_id: "Wrong".to_string(),
            zone: Some(services::Zone {
                disjunction: Some(services::Disjunction {
                    conjunctions: vec![services::Conjunction {
                        constraints: vec![
                            // constraint (0 - y <= 0) <= (y >= 0)
                            services::Constraint {
                                x: Some(services::ComponentClock {
                                    specific_component: None,
                                    clock_name: "0".to_string(),
                                }),
                                y: Some(services::ComponentClock {
                                    specific_component: None,
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
        edges: new_state.decision_points[0].edges.clone(),
    });
    new_state
}

// Returns the Machine component as a String, in the .json format
pub fn create_sample_json_component() -> String {
    fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap()
}
