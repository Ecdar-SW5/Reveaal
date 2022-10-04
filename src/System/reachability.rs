use edbm::zones::OwnedFederation;

use crate::ModelObjects::component::{State, Transition};
use crate::TransitionSystems::{TransitionSystem, LocationTuple};
use std::collections::HashMap;

pub struct SubPath {
    start_state: State,
    transition: Transition,
}

// pub fn preliminary_check_succes(take some input) -> return a path{
//    It returns a path
// }

pub fn is_reachable(
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

    search_algorithm(start_state, end_state, system)
}

pub fn search_algorithm(
    start_state: State,
    end_state: State,
    system: &dyn TransitionSystem,
) -> Option<Vec<SubPath>> {
    panic!("No implementation");

    // hashmap linking every location to all its current zones
    // TODO: figure out a way to hash a location
    let mut visited_states:HashMap<LocationTuple, Option<OwnedFederation>>= HashMap::new();

    // List of states that are to be visited
    let frontier_states: &mut Vec<State> = &mut Vec::new();

    frontier_states.push(start_state);
    loop{
        let next_state = frontier_states.pop();
        // All has been explored if no next state exist
        if next_state.is_none() {
            break;
        }
        let next_state = next_state.unwrap();
        if false/* next_state reaches end_state, possible check overlap of zones?*/{
            return None/* Return the path success? */
        }
        for input in system.get_input_actions(){
            for transition in &system.next_inputs(&next_state.decorated_locations, &input){
                let mut new_state = next_state.clone();
                if transition.use_transition(&mut new_state){
                    new_state.extrapolate_max_bounds(system); // Do we need to do this? consistency check does this
                    let mut existing_states: &mut Vec<State> = visited_states.entry(new_state.get_location()).or_insert(Vec::new());
                    if !state_subset_of_existing_state(&new_state, existing_states) {
                        remove_existing_subsets_of_state(&new_state, existing_states);
                        visited_states.insert(new_state.get_location(), new_state.zone_ref());
                        frontier_states.push(new_state);
                    }
                }
            }
        }
        // TODO: ADD OUTPUT ALSO
    };

    // If nothing has been found, it is not reachable
    return None;
}
fn state_subset_of_existing_state(
    state: &State,
    existing_states: & Vec<State>
) -> bool {
    panic!("No implementation"); // Check whether the state is new or just a subset 
}
fn remove_existing_subsets_of_state(
    state: &State,
    existing_states: &mut Vec<State>
) {
    panic!("No implementation"); // delete everything in existing_states that is a subset of state
}
