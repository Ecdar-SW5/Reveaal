use edbm::zones::OwnedFederation;

use crate::ModelObjects::component::{State, Transition};
use crate::TransitionSystems::{TransitionSystem, LocationID};
use std::collections::HashMap;

pub struct SubPath {
    start_state: State,
    transition: Transition,
}

// pub fn preliminary_check_succes(take some input) -> return a path{
//    It returns a path
// }

///# Find path
/// 
/// Returns a path from a start state to an end state in a transition system.
/// 
/// If it is reachable, it returns a path.
/// 
/// If it is not reachable, it returns None.
/// 
/// The start state can be omitted with None to use the start state of the transition system.
/// 
///## Checking if a state can reach another:
/// 
/// let is_reachable: bool = match find_path(Some(start_state), end_state, transition_system) {
///     Some(path) => true,
///     None => false
/// };
/// 
///## Omitting start state:
/// 
/// let is_reachable: bool = match find_path(None, end_state, transition_system) {
///     Some(path) => true,
///     None => false
/// };
/// 
pub fn find_path(
    begin_state: Option<State>,
    end_state: State,
    system: &dyn TransitionSystem,
) -> Option<Vec<SubPath>> {
    // if preliminary_check_succes() { return a path }

    let start_state: State;

    if begin_state.is_some() {
        start_state = begin_state.unwrap();
    } else if system.get_initial_state().is_some() {
        start_state = system.get_initial_state().unwrap();
    } else {
        panic!("No state to start with");
    }

    search_algorithm(&start_state, &end_state, system)
}

pub fn search_algorithm(
    start_state: &State,
    end_state: &State,
    system: &dyn TransitionSystem,
) -> Option<Vec<SubPath>> {

    // hashmap linking every location to all its current zones
    let mut visited_states:HashMap<LocationID, Vec<OwnedFederation>> = HashMap::new();

    // List of states that are to be visited
    let frontier_states: &mut Vec<State> = &mut Vec::new();

    frontier_states.push(start_state.clone());
    loop{
        let next_state = frontier_states.pop();
        // All has been explored if no next state exist
        if next_state.is_none() {
            break;
        }
        let next_state = next_state.unwrap();
        // If there is a overlap with the end state, it has been reached.
        if next_state.zone_ref().has_intersection(end_state.zone_ref()){
            return None/* TODO: Return the path success? */
        }

        // Take all input transitions
        for input in system.get_input_actions(){
            for transition in &system.next_inputs(&next_state.decorated_locations, &input){
                take_transition(&next_state, transition, frontier_states, &mut visited_states, system);
            
            }
        }

        // Take all output transitions
        for output in system.get_output_actions(){
            for transition in &system.next_outputs(&next_state.decorated_locations, &output){
                take_transition(&next_state, transition, frontier_states, &mut visited_states, system);
            }
        }
    };

    // If nothing has been found, it is not reachable
    None
}

fn take_transition(
    next_state:  &State, 
    transition: &Transition, 
    frontier_states: &mut Vec<State>, 
    visited_states: &mut HashMap<LocationID, Vec<OwnedFederation>>, 
    system: &dyn TransitionSystem) {
    let mut new_state = next_state.clone();
    if transition.use_transition(&mut new_state){
        new_state.extrapolate_max_bounds(system); // Do we need to do this? consistency check does this
        let existing_zones: &mut Vec<OwnedFederation> = visited_states.entry(new_state.get_location().id.clone()).or_insert(Vec::new());
        if !zone_subset_of_existing_zones(new_state.zone_ref(), existing_zones) {
            remove_existing_subsets_of_zone(new_state.zone_ref(), existing_zones);
            visited_states.get_mut(&new_state.get_location().id).unwrap().push(new_state.zone_ref().clone());
            frontier_states.push(new_state);
        }
    }
}

/// Checks if this zone is redundant by being a subset of any other zone
fn zone_subset_of_existing_zones(
    new_state: &OwnedFederation,
    existing_states: & Vec<OwnedFederation>
) -> bool {
    for existing_state in existing_states {
        if new_state.subset_eq(existing_state) {
            return true
        }
    }
    false
}

/// Removes everything in existing_zones that is a subset of zone
fn remove_existing_subsets_of_zone(
    new_zone: &OwnedFederation,
    existing_zones: &mut Vec<OwnedFederation>
) {
    existing_zones
        .retain(|existing_zone| !existing_zone.subset_eq(new_zone));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::System::reachability::search_algorithm;
    use crate::DataReader::component_loader::JsonProjectLoader;
    use crate::ModelObjects::component::State;
    use edbm::util::constraints::ClockIndex;
    use log::debug;
    use crate::System::extract_system_rep::SystemRecipe;
    use crate::TransitionSystems::LocationTuple;
    use crate::component::Component;

    #[test]
    fn Reachability_Test_If_Location_Exists_In_TransitionSystem() {
        const PATH: &str = "samples/json/EcdarUniversity/Components/Machine.json";
        let project_loader = JsonProjectLoader::new(String::from(PATH));
        let mut comp_loader = project_loader.to_comp_loader();
        let component = comp_loader.get_component("Machine").to_owned();
        let mut result = false;
        let locationId = "L5";
        result = ExistLocation(component, locationId.to_string());
        assert!(result);
    }

    #[test]
    fn Reachability_TestIfSearchAlgorithmeReturnsTrueForReachableState() {
        const PATH: &str = "samples/json/EcdarUniversity/Components/Machine.json";
        let project_loader = JsonProjectLoader::new(String::from(PATH));
        let mut comp_loader = project_loader.to_comp_loader();
        
        let mut component = comp_loader.get_component("Machine").to_owned();
        
        let mut dim1: ClockIndex = 0;
        let dim2: ClockIndex = 0;
        component.set_clock_indices(&mut dim1);
        debug!("{} Clocks: {:?}", "Machine", component.declarations.clocks);

        
        let sr = Box::new(SystemRecipe::Component(Box::new(component.clone()))).compile(dim2).unwrap();

        let locations: Vec<LocationTuple> = sr.get_all_locations();
        let state0: State = State::create(locations[0].to_owned(), locations[0].get_invariants().unwrap().to_owned());
        let state1: State = State::create(locations[1].to_owned(), locations[1].get_invariants().unwrap().to_owned());


        let path: Option<Vec<SubPath>> = search_algorithm(&state0, &state1, &component);

        match path {
            Some(_) => assert!(true),
            None => assert!(false),
        }

        //let start_state = State::create(component.get_locations, /* Federation<DBM<Valid>> */);
    }

    fn ExistLocation(component: Component, locationId: String) -> bool {
        true
    }
}

//pub fn search_algorithm(
//    start_state: &State,
//    end_state: &State,
//    system: &dyn TransitionSystem,
//) -> Option<Vec<SubPath>>