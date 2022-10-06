use edbm::zones::OwnedFederation;

use crate::ModelObjects::component::{State, Transition};
use crate::TransitionSystems::{TransitionSystem, LocationID};
use std::collections::HashMap;

pub struct SubPath {
    start_state: State,
    transition: Transition,
}

pub fn preliminary_check(
    start_state: &State,
    end_state: &State,
    system: &dyn TransitionSystem
) -> Result<bool, Box<dyn std::error::Error>> {
    if !system.get_all_locations().contains(start_state.get_location())
        {return Err("The transition system does not contain the start location".into())}
    if !system.get_all_locations().contains(end_state.get_location())
        {return Err("The transition system does not contain the end location".into())}
    if !&end_state.zone_ref().has_intersection(end_state.get_location().get_invariants().unwrap())
        {return Err("The desired end state is not allowed due to the invariant on this location".into())}

    return Ok(true);
}

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
// This is the main function for the reachability query.
pub fn find_path(
    begin_state: Option<State>,
    end_state: State,
    system: &dyn TransitionSystem,
) -> bool {
    let start_state: State;
    if begin_state.is_some() {
        start_state = begin_state.unwrap();
    } else if system.get_initial_state().is_some() {
        start_state = system.get_initial_state().unwrap();
    } else {
        panic!("No state to start with");
    }

    match preliminary_check(&start_state, &end_state, system) {
        Err(msg) => panic!("{}", msg),
        Ok(_b) => ()
    }

    search_algorithm(&start_state, &end_state, system)
}

pub fn search_algorithm(
    start_state: &State,
    end_state: &State,
    system: &dyn TransitionSystem,
) -> bool {

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
            return true/* TODO: Return the path success? */
        }

        for action in system.get_actions(){
            for transition in &system.next_transitions(&next_state.decorated_locations, &action){
                take_transition(&next_state, transition, frontier_states, &mut visited_states, system);

            }
        }
    };

    // If nothing has been found, it is not reachable
    false
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
    use crate::tests::refinement::Helper::json_run_query;
    use crate::System::executable_query::QueryResult;
    use crate::DataReader::component_loader::{JsonProjectLoader, XmlProjectLoader};

    #[test]
    #[ignore = "Cannot compile"]
    fn Reachability_Test_If_Location_Exists_In_TransitionSystem(){
        const PATH: &str = "samples/json/EcdarUniversity/Components/Machine.json";
        let project_loader = JsonProjectLoader::new(String::from(PATH));
        let mut comp_loader = project_loader.to_comp_loader();
        let component = comp_loader.get_component("Machine").to_owned();
        let mut result = false;
        let locationId = "L5";
        result = ExistLocation(component, locationId);
        assert!(result);
    }
}
