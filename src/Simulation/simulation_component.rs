use crate::ModelObjects::component::{
    Component, DeclarationProvider, Declarations, State, Transition, Channel, Location,
};
use crate::DataReader::json_reader;

// Takes a path string & component name, and returns a SimulationComponent.
pub fn start_simulation(project_path: &str, component_name: &str) -> SimulationComponent {
    let start_component: Component = json_reader::read_json_component(project_path, component_name);
    let sim_component: SimulationComponent = build_simulation_component(start_component);

    sim_component

}

pub fn continue_simulation(simulation_component: SimulationComponent) {

}

pub struct SimulationComponent {
    component: Component,
    actions: Vec<Channel>,
    location: Location,
}

fn build_simulation_component(component: Component) -> SimulationComponent {
    let t_actions: Vec<Channel> = component.get_actions();
    let t_location: Location = match component.get_initial_location()
    {
        None => panic!("no initial location"),
        Some(x) => x.clone(),
    };
    SimulationComponent {
        component,
        actions: t_actions,
        location: t_location,
    }
}
