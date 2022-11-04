use crate::{
    DataReader::component_loader::ComponentContainer,
    ProtobufServer::services::SimulationInfo,
    TransitionSystems::{CompiledComponent, TransitionSystem},
};

pub fn simulation_info_to_transition_system(
    simulation_info: SimulationInfo,
) -> Box<dyn TransitionSystem> {
    let composition = simulation_info.component_composition;
    let component_info = simulation_info.components_info.unwrap();
    // Extract components from the request message

    let mut component_container = ComponentContainer::from(&component_info).unwrap();

    // Build transition_system as specified in the composition string
    return CompiledComponent::from_component_loader(&mut component_container, &composition);
}
