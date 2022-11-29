use std::fs;

use tonic::{Request, Response, Status};

use crate::ProtobufServer::services::{
    Component as ProtoComponent, ComponentClock as ProtoComponentClock,
    Conjunction as ProtoConjunction, Constraint as ProtoConstraint, Decision as ProtoDecision,
    DecisionPoint as ProtoDecisionPoint, Disjunction as ProtoDisjunction, Edge as ProtoEdge,
    Federation as ProtoFederation, Location as ProtoLocation, LocationTuple as ProtoLocationTuple,
    SimulationStartRequest, SimulationStepRequest, SimulationStepResponse,
    SpecificComponent as ProtoSpecificComponent, State as ProtoState,
};
use crate::Simulation::transition_decision_point::TransitionDecisionPoint;
use crate::TransitionSystems::CompositionType;
use crate::{
    component::Component, DataReader::json_reader::read_json_component,
    TransitionSystems::TransitionSystemPtr,
};

use super::helper::{
    create_1tuple_state_with_single_constraint, create_components, create_composition_string,
    create_empty_edge, create_empty_state, create_simulation_info, create_simulation_info_from,
    create_simulation_start_request, create_simulation_step_request, create_system_from_path,
};

static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

pub fn create_EcdarUniversity_Machine_component() -> Component {
    let project_path = "samples/json/EcdarUniversity";
    read_json_component(project_path, "Machine")
}

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

    let conjunction_dp = ProtoConjunction {
        constraints: vec![],
    };

    let disjunction_dp = ProtoDisjunction {
        conjunctions: vec![conjunction_dp],
    };

    let federation_dp = ProtoFederation {
        disjunction: Some(disjunction_dp),
    };

    let location_dp1 = ProtoLocation {
        id: "L5".to_string(),
        specific_component: Some(specific_comp_dp.clone()),
    };

    let loc_tuple_dp = ProtoLocationTuple {
        locations: vec![location_dp1],
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

pub fn create_EcdarUniversity_Machine_with_nonempty_Federation_Decision() -> ProtoDecision {
    // kopieret fra create_EcdarUnversity_Machine_Initial_Decision_Point men ved ikke hvordan det kunne gøres til en funktion smart
    let specific_comp_dp = ProtoSpecificComponent {
        component_name: "Machine".to_string(),
        component_index: 1,
    };

    let componentclock_dp1 = ProtoComponentClock {
        specific_component: None,
        clock_name: "0".to_string(),
    };
    let componentclock_dp2 = ProtoComponentClock {
        specific_component: Some(specific_comp_dp.clone()),
        clock_name: "y".to_string(),
    };

    let constraint29_dp = ProtoConstraint {
        x: Some(componentclock_dp1),
        y: Some(componentclock_dp2),
        strict: false,
        c: -2,
    };

    let conjunction_dp = ProtoConjunction {
        constraints: vec![constraint29_dp],
    };

    let disjunction_dp = ProtoDisjunction {
        conjunctions: vec![conjunction_dp],
    };

    let federation_dp = ProtoFederation {
        disjunction: Some(disjunction_dp),
    };

    let location_dp1 = ProtoLocation {
        id: "L5".to_string(),
        specific_component: Some(specific_comp_dp.clone()),
    };

    let loc_tuple_dp = ProtoLocationTuple {
        locations: vec![location_dp1],
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

pub fn create_EcdarUniversity_Machine3and1_with_nonempty_Federation_Decision() -> ProtoDecision {
    // kopieret fra create_EcdarUnversity_Machine_Initial_Decision_Point men ved ikke hvordan det kunne gøres til en funktion smart
    let specific_comp_dp1 = ProtoSpecificComponent {
        component_name: "Machine".to_string(),
        component_index: 1,
    };

    let source_dp = ProtoState {
        location_tuple: Some(ProtoLocationTuple {
            locations: vec![
                ProtoLocation {
                    id: "L8".to_string(),
                    specific_component: Some(ProtoSpecificComponent {
                        component_name: "Machine3".to_string(),
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
            ],
        }),
        federation: Some(ProtoFederation {
            disjunction: Some(ProtoDisjunction {
                conjunctions: vec![ProtoConjunction {
                    constraints: vec![
                        ProtoConstraint {
                            x: Some(ProtoComponentClock {
                                specific_component: Some(ProtoSpecificComponent {
                                    component_name: "Machine3".to_string(),
                                    component_index: 0,
                                }),
                                clock_name: "y".to_string(),
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
                                    component_name: "Machine3".to_string(),
                                    component_index: 0,
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
    };

    let edge29 = ProtoEdge {
        id: "E29".to_string(),
        specific_component: Some(specific_comp_dp1),
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

pub fn get_state_after_Administration_Machine_Researcher_composition() -> ProtoState {
    ProtoState {
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
    }
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
        new_decision_points: vec![proto_decision_point],
    };

    Ok(Response::new(response))
}

pub fn get_composition_response_Administration_Machine_Researcher_after_E29(
) -> Result<Response<SimulationStepResponse>, Status> {
    let decisionpoint1 = ProtoDecisionPoint {
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
                        id: "L7".to_string(),
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
                                        component_name: "Administration".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "z".to_string(),
                                }),
                                y: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "Researcher".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "x".to_string(),
                                }),
                                strict: false,
                                c: 15,
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
                                        component_name: "Administration".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "z".to_string(),
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
                                    specific_component: None,
                                    clock_name: "0".to_string(),
                                }),
                                strict: false,
                                c: 8,
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
                                c: -2,
                            },
                        ],
                    }],
                }),
            }),
        }),
        edges: vec![
            ProtoEdge {
                id: "E13".to_string(),
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
            ProtoEdge {
                id: "E9".to_string(),
                specific_component: None,
            },
        ],
    };
    let decisionpoint2 = ProtoDecisionPoint {
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
                        id: "U0".to_string(),
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
                                    specific_component: None,
                                    clock_name: "0".to_string(),
                                }),
                                y: Some(ProtoComponentClock {
                                    specific_component: Some(ProtoSpecificComponent {
                                        component_name: "Administration".to_string(),
                                        component_index: 0,
                                    }),
                                    clock_name: "z".to_string(),
                                }),
                                strict: true,
                                c: -15,
                            },
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
        new_decision_points: vec![decisionpoint1, decisionpoint2],
    };

    Ok(Response::new(response))
}
//to here
pub fn get_state_after_HalfAdm1_HalfAdm2_conjunction() -> ProtoState {
    ProtoState {
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
    }
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
        new_decision_points: vec![proto_decision_point],
    };

    Ok(Response::new(response))
}

pub fn get_conjunction_response_HalfAdm1_HalfAdm2_after_E37(
) -> Result<Response<SimulationStepResponse>, Status> {
    let new_decision_points = ProtoDecisionPoint {
        source: Some(ProtoState {
            location_tuple: Some(ProtoLocationTuple {
                locations: vec![
                    ProtoLocation {
                        id: "L13".to_string(),
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
                                    specific_component: None,
                                    clock_name: "0".to_string(),
                                }),
                                strict: false,
                                c: 2,
                            },
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
                id: "E36".to_string(),
                specific_component: None,
            },
            ProtoEdge {
                id: "E38".to_string(),
                specific_component: None,
            },
            ProtoEdge {
                id: "E40".to_string(),
                specific_component: None,
            },
            ProtoEdge {
                id: "E41".to_string(),
                specific_component: None,
            },
        ],
    };

    let response = SimulationStepResponse {
        new_decision_points: vec![new_decision_points],
    };
    Ok(Response::new(response))
}

// A request that Chooses the FAT EDGE:
//
//           ----coin? E3---->
//          /
// (L5,y>=0)=====tea! E5=====>
//
pub fn create_good_request() -> tonic::Request<SimulationStepRequest> {
    let simulation_info =
        create_simulation_info_from(String::from("Machine"), create_sample_json_component());
    let initial_decision_point = create_initialdecision_point();
    let chosen_source = initial_decision_point.source.clone().unwrap();
    let chosen_edge = initial_decision_point.edges[1].clone();

    tonic::Request::new(create_simulation_step_request(
        simulation_info,
        chosen_source,
        chosen_edge,
    ))
}

pub fn create_expected_response_to_good_request() -> Result<Response<SimulationStepResponse>, Status>
{
    Ok(Response::new(SimulationStepResponse {
        new_decision_points: vec![create_decision_point_after_taking_E5()],
    }))
}

pub fn create_mismatched_request_1() -> Request<SimulationStepRequest> {
    let simulation_info =
        create_simulation_info_from(String::from("Machine"), create_sample_json_component());
    let chosen_source = create_state_not_in_machine();
    let chosen_edge = create_edges_from_L5()[0].clone();

    tonic::Request::new(create_simulation_step_request(
        simulation_info,
        chosen_source,
        chosen_edge,
    ))
}

pub fn create_expected_response_to_mismatched_request_1(
) -> Result<Response<SimulationStepResponse>, Status> {
    Err(tonic::Status::invalid_argument(
        "Mismatch between decision and system, state not in system",
    ))
}

pub fn create_mismatched_request_2() -> Request<SimulationStepRequest> {
    let simulation_info =
        create_simulation_info_from(String::from("Machine"), create_sample_json_component());

    let chosen_source = create_state_setup_for_mismatch();
    let chosen_edge = create_edges_from_L5()[1].clone(); // Should not be able to choose this edge
    Request::new(create_simulation_step_request(
        simulation_info,
        chosen_source,
        chosen_edge,
    ))
}

pub fn create_expected_response_to_mismatched_request_2(
) -> Result<Response<SimulationStepResponse>, Status> {
    Err(tonic::Status::invalid_argument(
        "Mismatch between decision and system, could not make transition",
    ))
}

pub fn create_malformed_component_request() -> Request<SimulationStepRequest> {
    let simulation_info = create_simulation_info_from(String::from(""), String::from(""));
    let chosen_source = create_empty_state();
    let chosen_edge = create_empty_edge();

    Request::new(create_simulation_step_request(
        simulation_info,
        chosen_source,
        chosen_edge,
    ))
}

pub fn create_response_to_malformed_component_request(
) -> Result<Response<SimulationStepResponse>, Status> {
    Err(Status::invalid_argument("Malformed component, bad json"))
}

pub fn create_malformed_composition_request() -> Request<SimulationStepRequest> {
    let simulation_info =
        create_simulation_info_from(String::from(""), create_sample_json_component());
    let chosen_source = create_empty_state();
    let chosen_edge = create_empty_edge();

    Request::new(create_simulation_step_request(
        simulation_info,
        chosen_source,
        chosen_edge,
    ))
}

pub fn create_response_to_malformed_compostion_request(
) -> Result<Response<SimulationStepResponse>, Status> {
    Err(Status::invalid_argument(
        "Malformed composition, bad expression",
    ))
}

// A || B || C
pub fn create_composition_request() -> Request<SimulationStepRequest> {
    let comp_names = vec!["Administration", "Machine", "Researcher"];
    let sample_name = "EcdarUniversity".to_string();
    let composition_string = "Administration || Machine || Researcher".to_string();

    let components: Vec<ProtoComponent> = create_components(&comp_names, sample_name);
    let simulation_info = create_simulation_info(composition_string, components);

    let edge = ProtoEdge {
        id: "E29".to_string(),
        specific_component: None,
    };

    let source = get_state_after_Administration_Machine_Researcher_composition();

    let simulation_step_request = create_simulation_step_request(simulation_info, source, edge);

    Request::new(simulation_step_request)
}

pub fn create_expected_response_to_composition_request(
) -> Result<Response<SimulationStepResponse>, Status> {
    get_composition_response_Administration_Machine_Researcher_after_E29()
}

// A && B
pub fn create_conjunction_request() -> Request<SimulationStepRequest> {
    let comp_names = vec!["HalfAdm1", "HalfAdm2"];
    let sample_name = "EcdarUniversity".to_string();
    let composition_string = "HalfAdm1 && HalfAdm2".to_string();
    create_composition_string(&comp_names, CompositionType::Conjunction);

    let components: Vec<ProtoComponent> = create_components(&comp_names, sample_name);
    let simulation_info = create_simulation_info(composition_string, components);

    let edge = ProtoEdge {
        id: "E37".to_string(),
        specific_component: None,
    };

    let source = get_state_after_HalfAdm1_HalfAdm2_conjunction();

    let simulation_step_request = create_simulation_step_request(simulation_info, source, edge);

    Request::new(simulation_step_request)
}

pub fn create_expected_response_to_conjunction_request(
) -> Result<Response<SimulationStepResponse>, Status> {
    get_conjunction_response_HalfAdm1_HalfAdm2_after_E37()
}

pub fn create_good_start_request() -> Request<SimulationStartRequest> {
    create_simulation_start_request(String::from("Machine"), create_sample_json_component())
}

pub fn create_expected_response_to_good_start_request(
) -> Result<Response<SimulationStepResponse>, Status> {
    Ok(Response::new(SimulationStepResponse {
        new_decision_points: vec![create_initialdecision_point()],
    }))
}

pub fn create_malformed_component_start_request() -> Request<SimulationStartRequest> {
    create_simulation_start_request(String::from(""), String::from(""))
}

pub fn create_malformed_composition_start_request() -> Request<SimulationStartRequest> {
    create_simulation_start_request(String::from(""), create_sample_json_component())
}

// A || B || C
pub fn create_composition_start_request() -> Request<SimulationStartRequest> {
    let comp_names = vec!["Administration", "Machine", "Researcher"];
    let sample_name = "EcdarUniversity".to_string();

    let composition = create_composition_string(&comp_names, CompositionType::Composition);
    let components: Vec<ProtoComponent> = create_components(&comp_names, sample_name);

    let simulation_info = create_simulation_info(composition, components);

    let simulation_start_request = Request::new(SimulationStartRequest {
        simulation_info: Some(simulation_info),
    });

    simulation_start_request
}

pub fn create_expected_response_to_composition_start_request(
) -> Result<Response<SimulationStepResponse>, Status> {
    get_composition_response_Administration_Machine_Researcher()
}

// A && B
pub fn create_conjunction_start_request() -> Request<SimulationStartRequest> {
    let comp_names = vec!["HalfAdm1", "HalfAdm2"];
    let sample_name = "EcdarUniversity".to_string();
    let composition_string = create_composition_string(&comp_names, CompositionType::Conjunction);

    let components: Vec<ProtoComponent> = create_components(&comp_names, sample_name);
    let simulation_info = create_simulation_info(composition_string, components);

    Request::new(SimulationStartRequest {
        simulation_info: Some(simulation_info),
    })
}

pub fn create_expected_response_to_conjunction_start_request(
) -> Result<Response<SimulationStepResponse>, Status> {
    get_conjunction_response_HalfAdm1_HalfAdm2()
}

pub fn create_edges_from_L5() -> Vec<ProtoEdge> {
    vec![
        ProtoEdge {
            id: "E27".to_string(),
            specific_component: None,
        },
        ProtoEdge {
            id: "E29".to_string(),
            specific_component: None,
        },
    ]
}

// Create the decision point drawn below:
//
//           -----coin? E3----->
//          /
// (L5, universe)-------tea! E5----->
//
pub fn create_initialdecision_point() -> ProtoDecisionPoint {
    ProtoDecisionPoint {
        source: Some(ProtoState {
            location_tuple: Some(ProtoLocationTuple {
                locations: vec![ProtoLocation {
                    id: "L5".to_string(),
                    specific_component: Some(ProtoSpecificComponent {
                        component_name: "Machine".to_string(),
                        component_index: 0,
                    }),
                }],
            }),
            federation: Some(ProtoFederation {
                disjunction: Some(ProtoDisjunction {
                    conjunctions: vec![ProtoConjunction {
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

// Create the decision point drawn below:
//
//           -----coin? E3----->
//          /
// (L5,y>=2)-------tea! E5----->
//
pub fn create_decision_point_after_taking_E5() -> ProtoDecisionPoint {
    ProtoDecisionPoint {
        source: Some(ProtoState {
            location_tuple: Some(ProtoLocationTuple {
                locations: vec![ProtoLocation {
                    id: "L5".to_string(),
                    specific_component: Some(ProtoSpecificComponent {
                        component_name: "Machine".to_string(),
                        component_index: 0,
                    }),
                }],
            }),
            federation: Some(ProtoFederation {
                disjunction: Some(ProtoDisjunction {
                    conjunctions: vec![ProtoConjunction {
                        constraints: vec![ProtoConstraint {
                            x: Some(ProtoComponentClock {
                                specific_component: None,
                                clock_name: "0".to_string(),
                            }),
                            y: Some(ProtoComponentClock {
                                specific_component: Some(ProtoSpecificComponent {
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
pub fn create_state_not_in_machine() -> ProtoState {
    create_1tuple_state_with_single_constraint("", "Machine", 0, "0", "y", 0, false)
}

// create a state such that can't transition via E5
pub fn create_state_setup_for_mismatch() -> ProtoState {
    create_1tuple_state_with_single_constraint("L5", "Machine", 0, "y", "0", 2, true)
}
