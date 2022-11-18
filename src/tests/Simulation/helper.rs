use crate::{
    DataReader::json_reader::read_json_component,
    Simulation::transition_decision_point::TransitionDecisionPoint,
    TransitionSystems::{CompiledComponent, TransitionSystemPtr},
};

use crate::ProtobufServer::services::ComponentClock as ProtoComponentClock;
use crate::ProtobufServer::services::Conjunction as ProtoConjunction;
use crate::ProtobufServer::services::Constraint as ProtoConstraint;
use crate::ProtobufServer::services::Decision as ProtoDecision;
use crate::ProtobufServer::services::Disjunction as ProtoDisjunction;
use crate::ProtobufServer::services::Edge as ProtoEdge;
use crate::ProtobufServer::services::Federation as ProtoFederation;
use crate::ProtobufServer::services::Location as ProtoLocation;
use crate::ProtobufServer::services::LocationTuple as ProtoLocationTuple;
use crate::ProtobufServer::services::SpecificComponent as ProtoSpecificComponent;
use crate::ProtobufServer::services::State as ProtoState;

pub fn create_EcdarUniversity_Machine_system() -> TransitionSystemPtr {
    let component = read_json_component("samples/json/EcdarUniversity", "Machine");
    CompiledComponent::from(vec![component], "Machine")
}

pub fn create_Simulation_Machine_system() -> TransitionSystemPtr {
    let component = read_json_component("samples/json/Simulation", "SimMachine");
    CompiledComponent::from(vec![component], "SimMachine")
}

pub fn create_EcdarUniversity_Machine4_system() -> TransitionSystemPtr {
    let component = read_json_component("samples/json/EcdarUniversity", "Machine4");
    CompiledComponent::from(vec![component], "Machine4")
}

pub fn create_EcdarUniversity_Machine_Decision() -> ProtoDecision {
    // kopieret fra create_EcdarUnversity_Machine_Initial_Decision_Point men ved ikke hvordan det kunne gøres til en funktion smart
    let specific_comp_dp = ProtoSpecificComponent {
        component_name: "Machine".to_string(),
        component_index: 1,
    };

    let componentclock_dp1 = ProtoComponentClock {
        specific_component: Some(specific_comp_dp.clone()),
        clock_name: "5".to_string(),
    };
    let componentclock_dp2 = ProtoComponentClock {
        specific_component: Some(specific_comp_dp.clone()),
        clock_name: "6".to_string(),
    };

    let constraint_dp = ProtoConstraint {
        x: Some(componentclock_dp1),
        y: Some(componentclock_dp2),
        strict: false,
        c: 1,
    };

    let conjunction_dp = ProtoConjunction {
        constraints: vec![constraint_dp],
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

    let edge = ProtoEdge {
        id: "E3".to_string(),
        specific_component: Some(specific_comp_dp),
    };

    ProtoDecision {
        source: Some(source_dp),
        edge: Some(edge),
    }
}
pub fn initial_transition_decision_point_EcdarUniversity_Machine() -> TransitionDecisionPoint {
    let system = create_EcdarUniversity_Machine_system();
    TransitionDecisionPoint::initial(&system).unwrap()
}
