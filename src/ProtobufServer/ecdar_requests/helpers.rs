use crate::ProtobufServer::services::{
    self, Component, ComponentsInfo, DecisionPoint, Edge, Location, LocationTuple, SimulationInfo,
    SimulationStartRequest, SimulationStepRequest, SpecificComponent, State,
};

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


pub fn create_edges_from_L5() -> Vec<Edge> {
    vec![
        Edge {
            id: "E27".to_string(),
            specific_component: Some(SpecificComponent {
                component_name: String::from("Machine"),
                component_index: 0,
            }),
        },
        Edge {
            id: "E29".to_string(),
            specific_component: Some(SpecificComponent {
                component_name: String::from("Machine"),
                component_index: 0,
            }),
        },
    ]
}