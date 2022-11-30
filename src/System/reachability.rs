use edbm::zones::OwnedFederation;

use crate::component::LocationType;
use crate::ModelObjects::component::{State, Transition};
use crate::TransitionSystems::{LocationID, TransitionSystem};
use std::collections::HashMap;
use std::rc::Rc;

/// This holds the result of a reachability query
pub struct Path {
    pub path: Option<Vec<Transition>>,
    pub was_reachable: bool,
}
// This holds which transition from which state (the destination_state of the previous_sub_path) we took to reach this state
struct SubPath {
    previous_sub_path: Option<Rc<SubPath>>,
    destination_state: State,
    transition: Option<Transition>,
}

fn is_trivially_unreachable(start_state: &State, end_state: &State) -> bool {
    // If the end location has invariants and these do not have an intersection (overlap) with the zone of the end state of the query
    if let Some(invariants) = end_state.get_location().get_invariants() {
        if !end_state.zone_ref().has_intersection(invariants) {
            return true;
        }
    }
    // If the start state is a universal or inconsistent location and the end state isn't
    // Since universal and inconsistent locations cannot reach other locations
    if matches!(
        start_state.decorated_locations.loc_type,
        LocationType::Universal | LocationType::Inconsistent
    ) && !start_state
        .decorated_locations
        .compare_partial_locations(&end_state.decorated_locations)
    {
        return true;
    }

    false
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
/// ```ignore
/// let is_reachable: bool = match find_path(Some(start_state), end_state, transition_system) {
///    Ok(result) => match result {
///        Some(path) => true,
///        None => false,
///    },
///    Err(string) => panic!(string),
/// };
/// ```
///
///## Omitting start state:
/// ```ignore
/// let is_reachable: bool = match find_path(None, end_state, transition_system) {
///    Ok(result) => match result {
///        Some(path) => true,
///        None => false,
///    },
///    Err(string) => panic!(string),
/// };
/// ```
pub fn find_path(
    start_state: State,
    end_state: State,
    system: &dyn TransitionSystem,
) -> Result<Path, String> {
    if is_trivially_unreachable(&start_state, &end_state) {
        return Ok(Path {
            path: None,
            was_reachable: false,
        });
    }

    Ok(search_algorithm(&start_state, &end_state, system))
}

fn search_algorithm(start_state: &State, end_state: &State, system: &dyn TransitionSystem) -> Path {
    // Apply the invariant of the start state to the start state
    let mut start_clone = start_state.clone();
    let start_zone = start_clone.take_zone();
    let zone = start_clone.decorated_locations.apply_invariants(start_zone);
    start_clone.set_zone(zone);

    // hashmap linking every location to all its current zones
    let mut visited_states: HashMap<LocationID, Vec<OwnedFederation>> = HashMap::new();

    // List of states that are to be visited
    let mut frontier_states: Vec<Rc<SubPath>> = Vec::new();

    let mut actions: Vec<String> = system.get_actions().into_iter().collect();
    actions.sort();

    // Push start state to visited state
    visited_states.insert(
        start_clone.get_location().id.clone(),
        vec![start_clone.zone_ref().clone()],
    );

    // Push state state to frontier
    frontier_states.push(Rc::new(SubPath {
        previous_sub_path: None,
        destination_state: start_clone,
        transition: None,
    }));

    // Take the first state from the frontier and exprore it
    while let Some(sub_path) = frontier_states.pop() {
        if reached_end_state(&sub_path.destination_state, end_state) {
            return make_path(sub_path);
        }

        for action in &actions {
            for transition in
                &system.next_transitions(&sub_path.destination_state.decorated_locations, action)
            {
                take_transition(
                    &sub_path,
                    transition,
                    &mut frontier_states,
                    &mut visited_states,
                    system,
                );
            }
        }
    }
    // If nothing has been found, it is not reachable
    Path {
        path: None,
        was_reachable: false,
    }
}

fn reached_end_state(cur_state: &State, end_state: &State) -> bool {
    cur_state
        .get_location()
        .compare_partial_locations(end_state.get_location())
        && cur_state.zone_ref().has_intersection(end_state.zone_ref())
}

fn take_transition(
    sub_path: &Rc<SubPath>,
    transition: &Transition,
    frontier_states: &mut Vec<Rc<SubPath>>,
    visited_states: &mut HashMap<LocationID, Vec<OwnedFederation>>,
    system: &dyn TransitionSystem,
) {
    let mut new_state = sub_path.destination_state.clone();
    if transition.use_transition(&mut new_state) {
        new_state.extrapolate_max_bounds(system); // Ensures the bounds cant grow infinitely, avoiding infinite loops in an edge case TODO: does not take end state zone into account, leading to a very rare edge case
        let new_location_id = &new_state.get_location().id;
        let existing_zones = visited_states.entry(new_location_id.clone()).or_default();
        // If this location has not already been reached (explored) with a larger zone
        if !zone_subset_of_existing_zones(new_state.zone_ref(), existing_zones) {
            // Remove the smaller zones for this location in visited_states
            remove_existing_subsets_of_zone(new_state.zone_ref(), existing_zones);
            // Add the new zone to the list of zones for this location in visited_states
            visited_states
                .get_mut(new_location_id)
                .unwrap()
                .push(new_state.zone_ref().clone());
            // Add the new state to the frontier
            frontier_states.push(Rc::new(SubPath {
                previous_sub_path: Some(Rc::clone(sub_path)),
                destination_state: new_state,
                transition: Some(transition.clone()),
            }));
        }
    }
}

/// Checks if this zone is redundant by being a subset of any other zone
fn zone_subset_of_existing_zones(
    new_state: &OwnedFederation,
    existing_states: &Vec<OwnedFederation>,
) -> bool {
    for existing_state in existing_states {
        if new_state.subset_eq(existing_state) {
            return true;
        }
    }
    false
}

/// Removes everything in existing_zones that is a subset of zone
fn remove_existing_subsets_of_zone(
    new_zone: &OwnedFederation,
    existing_zones: &mut Vec<OwnedFederation>,
) {
    existing_zones.retain(|existing_zone| !existing_zone.subset_eq(new_zone));
}
/// Makes the path from the last subpath
fn make_path(mut sub_path: Rc<SubPath>) -> Path {
    let mut path: Vec<Transition> = Vec::new();
    // Traverse the subpaths to make the path (from end location to start location)
    while sub_path.previous_sub_path.is_some() {
        path.push(sub_path.transition.clone().unwrap());
        sub_path = Rc::clone(sub_path.previous_sub_path.as_ref().unwrap());
    }
    // Revese the path since the transitions are in reverse order (now from start location to end location)
    path.reverse();

    Path {
        path: Some(path),
        was_reachable: true,
    }
}
