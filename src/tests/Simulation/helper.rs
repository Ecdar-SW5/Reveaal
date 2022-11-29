use std::vec;

use tonic::{Response, Status};

use crate::ProtobufServer::services::{
    Component as ProtoComponent, ComponentClock as ProtoComponentClock,
    ComponentsInfo as ProtoComponentsInfo, Conjunction as ProtoConjunction,
    Constraint as ProtoConstraint, Decision as ProtoDecision, DecisionPoint as ProtoDecisionPoint,
    Disjunction as ProtoDisjunction, Edge as ProtoEdge, Federation as ProtoFederation,
    Location as ProtoLocation, LocationTuple as ProtoLocationTuple,
    SimulationInfo as ProtoSimulationInfo, SpecificComponent as ProtoSpecificComponent,
    State as ProtoState,
};
use crate::{
    component::Component,
    tests::grpc::grpc_helper::create_json_component_as_string,
    DataReader::json_reader::read_json_component,
    ProtobufServer::services::{component::Rep, SimulationStepResponse},
    Simulation::transition_decision_point::TransitionDecisionPoint,
    TransitionSystems::{
        transition_system::components_to_transition_system, CompositionType, TransitionSystemPtr,
    },
};

use super::test_data::create_EcdarUniversity_Machine_system;

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
