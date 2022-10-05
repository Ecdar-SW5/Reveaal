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
    let mut frontier_states: &mut Vec<State> = &mut Vec::new();

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
                take_transition(&next_state, &transition, &mut frontier_states, &mut visited_states, system);
            
            }
        }

        // Take all output transitions
        for output in system.get_output_actions(){
            for transition in &system.next_outputs(&next_state.decorated_locations, &output){
                take_transition(&next_state, &transition, &mut frontier_states, &mut visited_states, system);
            }
        }
    };

    // If nothing has been found, it is not reachable
    return None;
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
        let existing_states: &mut Vec<OwnedFederation> = visited_states.entry(new_state.get_location().id.clone()).or_insert(Vec::new());
        if !state_subset_of_existing_state(&new_state, existing_states) {
            remove_existing_subsets_of_state(&new_state, existing_states);
            visited_states.get_mut(&new_state.get_location().id).unwrap().push(new_state.zone_ref().clone());
            frontier_states.push(new_state);
        }
    }
}

fn state_subset_of_existing_state(
    new_state: &State,
    existing_states: & Vec<OwnedFederation>
) -> bool {
    for existing_state in existing_states {
        if new_state.zone_ref().subset_eq(existing_state) {
            return true
        }
    }
    false
}

/// Removes everything in existing_states that is a subset of state
fn remove_existing_subsets_of_state(
    new_state: &State,
    existing_states: &mut Vec<OwnedFederation>
) {
    existing_states
        .retain(|existing_state| !existing_state.subset_eq(new_state.zone_ref()));
}
