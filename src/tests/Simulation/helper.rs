use crate::{
    DataReader::json_reader::read_json_component,
    TransitionSystems::{CompiledComponent, TransitionSystemPtr}, Simulation::transition_decision_point::TransitionDecisionPoint,
};

pub fn create_EcdarUniversity_Machine_system() -> TransitionSystemPtr {
    let component = read_json_component("samples/json/EcdarUniversity", "Machine");
    CompiledComponent::from(vec![component], "Machine")
}

pub fn create_EcdarUniversity_Machine4_system() -> TransitionSystemPtr {
    let component = read_json_component("samples/json/EcdarUniversity", "Machine4");
    CompiledComponent::from(vec![component], "Machine4")
}

pub fn initial_transition_decision_point_EcdarUniversity_Machine() -> TransitionDecisionPoint {
    let system = create_EcdarUniversity_Machine_system();
    TransitionDecisionPoint::initial(system).unwrap()
}