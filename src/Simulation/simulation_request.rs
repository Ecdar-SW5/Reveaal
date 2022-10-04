use crate::ModelObjects::component::{
    Component, DeclarationProvider, Declarations, State, Transition,
};
use crate::json_reader::json_reader;
use crate::compiled_component::compiled_component;

// Tager en path & component navn, og returner en CompiledComponent til næste step.
pub fn start_simulation(project_path: &str, component_name: &str) -> SimulationComponent {
    let startComponent: Component = json_reader(project_path, component_name);

    // convert component til CompiledComponent og returner
    startComponent
}

pub fn continue_simulation(component: Component) {

}

pub struct SimulationComponent {
    component: component::Component,
    actions: Vec<Channel>,
    // actions: get_actions(component);
}

impl SimulationComponent {
    fn new(component: component::Component, actions: Vec<Channel>) -> start_simulation {
        component: component,
        actions: component.get_actions();

    }
 }
