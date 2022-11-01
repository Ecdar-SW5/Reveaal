use crate::{TransitionSystems::{TransitionSystemPtr, CompiledComponent}, DataReader::json_reader::read_json_component};

pub fn create_EcdarUniversity_Machine_system() -> TransitionSystemPtr {
    let component = read_json_component("samples/json/EcdarUniversity", "Machine");
    CompiledComponent::from(vec![component], "Machine")
}

pub fn create_EcdarUniversity_Machine4_system() -> TransitionSystemPtr {
    let component = read_json_component("samples/json/EcdarUniversity", "Machine4");
    CompiledComponent::from(vec![component], "Machine4")
}