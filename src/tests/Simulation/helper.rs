use tonic::{Response, Status};

use crate::{
    tests::grpc::grpc_helper::create_json_component_as_string,
    DataReader::json_reader::read_json_component,
    ProtobufServer::services::{component::Rep, SimulationStepResponse},
    Simulation::transition_decision_point::TransitionDecisionPoint,
    TransitionSystems::{CompiledComponent, CompositionType, TransitionSystemPtr},
};

use crate::ProtobufServer::services::{
    Component as ProtoComponent, ComponentClock as ProtoComponentClock,
    ComponentsInfo as ProtoComponentsInfo, Conjunction as ProtoConjunction,
    Constraint as ProtoConstraint, Decision as ProtoDecision, DecisionPoint as ProtoDecisionPoint,
    Disjunction as ProtoDisjunction, Edge as ProtoEdge, Federation as ProtoFederation,
    Location as ProtoLocation, LocationTuple as ProtoLocationTuple,
    SimulationInfo as ProtoSimulationInfo, SpecificComponent as ProtoSpecificComponent,
    State as ProtoState,
};

pub fn create_EcdarUniversity_Machine_system() -> TransitionSystemPtr {
    create_system_from_path("samples/json/EcdarUniversity", "Machine")
}

pub fn create_EcdarUniversity_HalfAdm1_system() -> TransitionSystemPtr {
    create_system_from_path("samples/json/EcdarUniversity", "HalfAdm1")
}

pub fn create_EcdarUniversity_HalfAdm2_system() -> TransitionSystemPtr {
    create_system_from_path("samples/json/EcdarUniversity", "HalfAdm2")
}

pub fn create_EcdarUniversity_Administration_system() -> TransitionSystemPtr {
    create_system_from_path("samples/json/EcdarUniversity", "Administration")
}

pub fn create_EcdarUniversity_Researcher_system() -> TransitionSystemPtr {
    create_system_from_path("samples/json/EcdarUniversity", "Researcher")
}

pub fn create_system_from_path(path: &str, name: &str) -> TransitionSystemPtr {
    let component = read_json_component(path, name);
    CompiledComponent::from(vec![component], name)
}

pub fn create_Simulation_Machine_system() -> TransitionSystemPtr {
    create_system_from_path("samples/json/Simulation", "SimMachine")
}

pub fn create_EcdarUniversity_Machine4_system() -> TransitionSystemPtr {
    create_system_from_path("samples/json/EcdarUniversity", "Machine4")
}

pub fn create_EcdarUniversity_Machine_Decision() -> ProtoDecision {
    // kopieret fra create_EcdarUnversity_Machine_Initial_Decision_Point men ved ikke hvordan det kunne gøres til en funktion smart
    let specific_comp_dp = ProtoSpecificComponent {
        component_name: "Machine".to_string(),
        component_index: 1,
    };

    let componentclock_dp1 = ProtoComponentClock {
        specific_component: Some(specific_comp_dp.clone()),
        clock_name: "0".to_string(),
    };
    let componentclock_dp2 = ProtoComponentClock {
        specific_component: Some(specific_comp_dp.clone()),
        clock_name: "y".to_string(),
    };

    let constraint26_dp = ProtoConstraint {
        x: Some(componentclock_dp1),
        y: Some(componentclock_dp2),
        strict: false,
        c: -2,
    };

    let conjunction_dp = ProtoConjunction {
        constraints: vec![constraint26_dp],
    };

    let disjunction_dp = ProtoDisjunction {
        conjunctions: vec![conjunction_dp],
    };

    let federation_dp = ProtoFederation {
        disjunction: Some(disjunction_dp),
    };

    let location_dp1 = ProtoLocation {
        id: "L4".to_string(),
        specific_component: Some(specific_comp_dp.clone()),
    };

    let location_dp2 = ProtoLocation {
        id: "L5".to_string(),
        specific_component: Some(specific_comp_dp.clone()),
    };

    let loc_tuple_dp = ProtoLocationTuple {
        locations: vec![location_dp1, location_dp2],
    };

    let source_dp = ProtoState {
        location_tuple: Some(loc_tuple_dp),
        federation: Some(federation_dp),
    };

    let edge29 = ProtoEdge {
        id: "E29".to_string(),
        specific_component: Some(specific_comp_dp),
    };

    ProtoDecision {
        source: Some(source_dp),
        edge: Some(edge29),
    }
}

pub fn initial_transition_decision_point_EcdarUniversity_Machine() -> TransitionDecisionPoint {
    let system = create_EcdarUniversity_Machine_system();
    TransitionDecisionPoint::initial(&system).unwrap()
}

pub fn get_composition_response_Administration_Machine_Researcher(
) -> Result<Response<SimulationStepResponse>, Status> {
    let proto_decision_point = ProtoDecisionPoint {
        source: Some(ProtoState {
            location_tuple: Some(ProtoLocationTuple {
                locations: vec![
                    ProtoLocation {
                        id: "L0".to_string(),
                        specific_component: Some(ProtoSpecificComponent {
                            component_name: "Administration".to_string(),
                            component_index: 0,
                        }),
                    },
                    ProtoLocation {
                        id: "L5".to_string(),
                        specific_component: Some(ProtoSpecificComponent {
                            component_name: "Machine".to_string(),
                            component_index: 0,
                        }),
                    },
                    ProtoLocation {
                        id: "L6".to_string(),
                        specific_component: Some(ProtoSpecificComponent {
                            component_name: "Researcher".to_string(),
                            component_index: 0,
                        }),
                    },
                ],
            }),
            federation: Some(ProtoFederation {
                disjunction: Some(ProtoDisjunction {
                    conjunctions: vec![ProtoConjunction {
                        constraints: vec![
                            ProtoConstraint {
                                x: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "Administration".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "z".to_string(),
                                }),
                                y: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "Machine".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "y".to_string(),
                                }),
                                strict: false,
                                c: 0,
                            },
                            ProtoConstraint {
                                x: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "Machine".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "y".to_string(),
                                }),
                                y: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "Researcher".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "x".to_string(),
                                }),
                                strict: false,
                                c: 0,
                            },
                            ProtoConstraint {
                                x: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "Researcher".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "x".to_string(),
                                }),
                                y: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "Administration".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "z".to_string(),
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
            ProtoEdge {
                id: "E11".to_string(),
                specific_component: None,
            },
            ProtoEdge {
                id: "E16".to_string(),
                specific_component: None,
            },
            ProtoEdge {
                id: "E29".to_string(),
                specific_component: None,
            },
            ProtoEdge {
                id: "E44".to_string(),
                specific_component: None,
            },
        ],
    };

    let response = SimulationStepResponse {
        new_decision_point: Some(proto_decision_point),
    };

    return Ok(Response::new(response));
}

pub fn get_conjunction_response_HalfAdm1_HalfAdm2(
) -> Result<Response<SimulationStepResponse>, Status> {
    let proto_decision_point = ProtoDecisionPoint {
        source: Some(ProtoState {
            location_tuple: Some(ProtoLocationTuple {
                locations: vec![
                    ProtoLocation {
                        id: "L12".to_string(),
                        specific_component: Some(ProtoSpecificComponent {
                            component_name: "HalfAdm1".to_string(),
                            component_index: 0,
                        }),
                    },
                    ProtoLocation {
                        id: "L14".to_string(),
                        specific_component: Some(ProtoSpecificComponent {
                            component_name: "HalfAdm2".to_string(),
                            component_index: 0,
                        }),
                    },
                ],
            }),
            federation: Some(ProtoFederation {
                disjunction: Some(ProtoDisjunction {
                    conjunctions: vec![ProtoConjunction {
                        constraints: vec![
                            ProtoConstraint {
                                x: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "HalfAdm1".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "x".to_string(),
                                }),
                                y: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "HalfAdm2".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "y".to_string(),
                                }),
                                strict: false,
                                c: 0,
                            },
                            ProtoConstraint {
                                x: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "HalfAdm2".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "y".to_string(),
                                }),
                                y: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "HalfAdm1".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "x".to_string(),
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
            ProtoEdge {
                id: "E30".to_string(),
                specific_component: None,
            },
            ProtoEdge {
                id: "E35".to_string(),
                specific_component: None,
            },
            ProtoEdge {
                id: "E37".to_string(),
                specific_component: None,
            },
            ProtoEdge {
                id: "E42".to_string(),
                specific_component: None,
            },
        ],
    };

    let response = SimulationStepResponse {
        new_decision_point: Some(proto_decision_point),
    };

    return Ok(Response::new(response));
}

pub fn create_simulation_info(
    composition: String,
    components: Vec<ProtoComponent>,
) -> ProtoSimulationInfo {
    let simulation_info = ProtoSimulationInfo {
        component_composition: composition,
        components_info: Some(ProtoComponentsInfo {
            components,
            components_hash: 0,
        }),
    };
    simulation_info
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

pub fn create_components(comp_names: &Vec<&str>, sample_name: String) -> Vec<ProtoComponent> {
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
