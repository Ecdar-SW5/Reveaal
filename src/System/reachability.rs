use edbm::zones::OwnedFederation;

use crate::ModelObjects::component::{State, Transition};
use crate::TransitionSystems::{LocationID, TransitionSystem};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Path {
    pub path: Option<Vec<Transition>>,
    pub was_reachable: bool,
}

#[derive(Clone)]
struct SubPath {
    previous_link: Option<Rc<SubPath>>,
    destination_state: State,
    transition: Option<Transition>,
}

impl SubPath {
    fn new(
        previous_link: Option<Rc<SubPath>>,
        destination_state: State,
        transition: Option<Transition>,
    ) -> Self {
        SubPath {
            previous_link,
            destination_state,
            transition,
        }
    }
}

// fn validate_input(
//     start_state: &State,
//     end_state: &State,
//     system: &dyn TransitionSystem,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let locations = system.get_all_locations();
//     if !locations.contains(start_state.get_location()) {
//         return Err("The transition system does not contain the start location".into());
//     }
//     if !locations.contains(end_state.get_location()) {
//         return Err("The transition system does not contain the end location".into());
//     }

//     Ok(())
// }

fn is_trivially_uncreachable(
    _start_state: &State,
    end_state: &State,
    _system: &dyn TransitionSystem,
) -> bool {
    if let Some(invariants) = end_state.get_location().get_invariants() {
        if !&end_state.zone_ref().has_intersection(invariants) {
            return true;
        }
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
    // if let Err(err) = validate_input(&start_state, &end_state, system) {
    //     return Err(err.to_string());
    // }

    if is_trivially_uncreachable(&start_state, &end_state, system) {
        return Ok(Path {
            path: None,
            was_reachable: false,
        });
    }

    search_algorithm(&start_state, &end_state, system)
}

fn search_algorithm(
    start_state: &State,
    end_state: &State,
    system: &dyn TransitionSystem,
) -> Result<Path, String> {
    // Apply the invariant of the start state to the start state
    let mut start_clone = start_state.clone();
    let start_zone = start_clone.take_zone();
    let zone = start_clone.decorated_locations.apply_invariants(start_zone);
    start_clone.set_zone(zone);

    // hashmap linking every location to all its current zones
    let mut visited_states: HashMap<LocationID, Vec<OwnedFederation>> = HashMap::new();
    //let mut made_transitions: Vec<SubPath> = Vec::new();

    // List of states that are to be visited
    let mut frontier_states: Vec<Rc<SubPath>> = Vec::new();

    let mut actions: Vec<String> = system.get_actions().into_iter().collect();
    actions.sort();

    visited_states.insert(
        start_clone.get_location().id.clone(),
        vec![start_clone.zone_ref().clone()],
    );

    frontier_states.push(Rc::new(SubPath::new(None, start_clone, None)));

    while let Some(sub_path) = frontier_states.pop() {
        if reached_end_state(&sub_path.destination_state, end_state) {
            return make_path(sub_path);
        }

        for action in &actions {
            for transition in &system.next_transitions(&sub_path.destination_state.decorated_locations, action) {
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
    Ok(Path {
        path: None,
        was_reachable: false,
    })
}

fn reached_end_state(cur_state: &State, end_state: &State) -> bool {
    cur_state.get_location().id == end_state.get_location().id
        && cur_state.zone_ref().has_intersection(end_state.zone_ref())
}

fn take_transition(
    sub_path: &Rc<SubPath>,
    transition: &Transition,
    frontier_states: &mut Vec<Rc<SubPath>>,
    visited_states: &mut HashMap<LocationID, Vec<OwnedFederation>>,
    system: &dyn TransitionSystem,
){
    let mut new_state = sub_path.destination_state.clone();
    if transition.use_transition(&mut new_state) {
        new_state.extrapolate_max_bounds(system); // Do we need to do this? consistency check does this
        let existing_zones = visited_states
            .entry(new_state.get_location().id.clone())
            .or_insert(Vec::new());
        if !zone_subset_of_existing_zones(new_state.zone_ref(), existing_zones) {
            remove_existing_subsets_of_zone(new_state.zone_ref(), existing_zones);
            visited_states
                .get_mut(&new_state.get_location().id)
                .unwrap()
                .push(new_state.zone_ref().clone());
            frontier_states.push(Rc::new(SubPath::new(
                Some(Rc::clone(sub_path)),
                new_state,
                Some(transition.clone()),
            )));
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

fn make_path(sub_path: Rc<SubPath>) -> Result<Path, String> {
    let mut path: Vec<Transition> = Vec::new();

    let mut sub_path = sub_path;
    while sub_path.previous_link.is_some() {
        path.push(sub_path.transition.clone().unwrap());
        sub_path = Rc::clone(sub_path.previous_link.as_ref().unwrap());
    }

    path.reverse();

    for e in &path {
        println!("Id: {}", e.id);
    }

    Ok(Path {
        path: Some(path),
        was_reachable: true,
    })
    // if made_transitions.len() > 1 {
    //     made_transitions.reverse();
    //     let mut prev_state: State = made_transitions[0].source_state.clone();

    //     for sub_path in &made_transitions[1..] {
    //         if prev_state.get_location().id != sub_path.source_state.get_location().id
    //             || !prev_state
    //                 .zone_ref()
    //                 .equals(sub_path.source_state.zone_ref())
    //         {
    //             if sub_path.source_state.get_location().id == start_state.get_location().id {
    //                 //Cannot unwrap None since made_transistion from > 0 will provide a SubPath with a transition.
    //                 path.push(sub_path.transition.clone().unwrap());
    //                 break;
    //             }
    //             path.push(sub_path.transition.clone().unwrap());
    //             prev_state = sub_path.source_state.clone();
    //         }
    //     }

    //     path.reverse();
    // }
}

/// Removes everything in existing_zones that is a subset of zone
fn remove_existing_subsets_of_zone(
    new_zone: &OwnedFederation,
    existing_zones: &mut Vec<OwnedFederation>,
) {
    existing_zones.retain(|existing_zone| !existing_zone.subset_eq(new_zone));
}
