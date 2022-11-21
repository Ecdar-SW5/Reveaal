use crate::tests::grpc::grpc_helper::*;
use crate::ProtobufServer::services::{
    self, DecisionPoint, Edge, Location, LocationTuple, SpecificComponent, State,
};
use crate::{
    DataReader::component_loader::ComponentContainer,
    ProtobufServer::services::SimulationInfo,
    TransitionSystems::{CompiledComponent, TransitionSystem},
};

pub fn create_edges_from_L4() -> Vec<Edge> {
    vec![
        Edge {
            id: "E25".to_string(),
            specific_component: Some(SpecificComponent {
                component_name: String::from("Machine"),
                component_index: 0,
            }),
        },
        Edge {
            id: "E26".to_string(),
            specific_component: Some(SpecificComponent {
                component_name: String::from("Machine"),
                component_index: 0,
            }),
        },
        Edge {
            id: "E28".to_string(),
            specific_component: Some(SpecificComponent {
                component_name: String::from("Machine"),
                component_index: 0,
            }),
        },
    ]
}

pub fn create_decision_point_from_L4() -> DecisionPoint {
    let source = create_1tuple_state_with_single_constraint("L4", "Machine", 0, "0", "y", 0, false);
    let edges = create_edges_from_L4();
    let new_decision_point: DecisionPoint = DecisionPoint {
        source: Some(source),
        edges,
    };
    new_decision_point
}

pub fn simulation_info_to_transition_system(
    simulation_info: SimulationInfo,
) -> Box<dyn TransitionSystem> {
    let composition = simulation_info.component_composition;
    let component_info = simulation_info.components_info.unwrap();
    // Extract components from the request message

    let mut component_container = ComponentContainer::from(&component_info).unwrap();

    // Build transition_system as specified in the composition string
    CompiledComponent::from_component_loader(&mut component_container, &composition)
}
