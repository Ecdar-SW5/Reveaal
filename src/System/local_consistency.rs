use std::fmt;

use edbm::zones::OwnedFederation;
use log::warn;

use crate::ModelObjects::component::State;
use crate::TransitionSystems::location_id::SimpleID;
use crate::TransitionSystems::{LocationID, TransitionSystem};

/// The result of a consistency check.
/// If there was a failure, [ConsistencyFailure] will specify the failure.
pub enum ConsistencyResult {
    Success,
    Failure(ConsistencyFailure),
}

impl fmt::Display for ConsistencyResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsistencyResult::Success => write!(f, "Succes"),
            ConsistencyResult::Failure(_) => write!(f, "failure"),
        }
    }
}
/// The failure of a consistency check.
/// Variants with [LocationID] are specific locations that cause the failure.
#[derive(Debug)]
pub enum ConsistencyFailure {
    NoInitialLocation,
    EmptyInitialState,
    NotConsistentFrom(SimpleID, String),
    NotDeterministicFrom(SimpleID, String),
}

impl fmt::Display for ConsistencyFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsistencyFailure::NoInitialLocation => write!(f, "No Initial State"),
            ConsistencyFailure::EmptyInitialState => write!(f, "Empty Initial State"),
            ConsistencyFailure::NotConsistentFrom(location, action) => {
                write!(
                    f,
                    "Not Consistent From {}  Failing action {}",
                    location.location_id(), action
                )
            }
            ConsistencyFailure::NotDeterministicFrom(location, action) => {
                write!(
                    f,
                    "Not Deterministic From {} Failing action {}",
                    location.location_id(), action
                )
            }
        }
    }
}

/// The result of a determinism check.
/// Failure includes the [LocationID].
pub enum DeterminismResult {
    Success,
    Failure(SimpleID, String),
}

impl fmt::Display for DeterminismResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeterminismResult::Success => write!(f, "Success"),
            DeterminismResult::Failure(location, action) => {
                write!(
                    f,
                    "Not Deterministic From {} failing action {}",
                    location.location_id(), action
                )
            }
        }
    }
}

///Local consistency check WITH pruning.
pub fn is_least_consistent(system: &dyn TransitionSystem) -> ConsistencyResult {
    if system.get_initial_location().is_none() {
        return ConsistencyResult::Failure(ConsistencyFailure::NoInitialLocation);
        //TODO: figure out whether we want empty TS to be consistent
    }

    let mut passed = vec![];
    let state = system.get_initial_state();
    if state.is_none() {
        warn!("Empty initial state");
        return ConsistencyResult::Failure(ConsistencyFailure::EmptyInitialState);
    }
    let mut state = state.unwrap();
    state.extrapolate_max_bounds(system);
    consistency_least_helper(state, &mut passed, system)
}

///Checks if a [TransitionSystem] is deterministic.
pub fn is_deterministic(system: &dyn TransitionSystem) -> DeterminismResult {
    if system.get_initial_location().is_none() {
        return DeterminismResult::Success;
    }
    let mut passed = vec![];
    let state = system.get_initial_state();
    if state.is_none() {
        return DeterminismResult::Success;
    }
    let mut state = state.unwrap();
    state.set_zone(OwnedFederation::universe(system.get_dim()));
    is_deterministic_helper(state, &mut passed, system)
}

fn is_deterministic_helper(
    state: State,
    passed_list: &mut Vec<State>,
    system: &dyn TransitionSystem,
) -> DeterminismResult {
    if state.is_contained_in_list(passed_list) {
        return DeterminismResult::Success;
    }

    passed_list.push(state.clone());

    for action in system.get_actions() {
        let mut location_fed = OwnedFederation::empty(system.get_dim());
        for transition in &system.next_transitions(&state.decorated_locations, &action) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                let mut allowed_fed = transition.get_allowed_federation();
                allowed_fed = state.decorated_locations.apply_invariants(allowed_fed);
                if allowed_fed.has_intersection(&location_fed) {
                    warn!(
                        "Not deterministic from location {} failing action {}",
                        state.get_location().id,
                        action
                    );
                    return DeterminismResult::Failure(SimpleID::new(state.get_location().id.clone().to_string(), None), action);
                }
                location_fed += allowed_fed;
                new_state.extrapolate_max_bounds(system);

                if let DeterminismResult::Failure(location, action) =
                    is_deterministic_helper(new_state, passed_list, system)
                {
                    return DeterminismResult::Failure(location, action);
                }
            }
        }
    }
    DeterminismResult::Success
}

/// Local consistency check WITHOUT pruning
#[allow(dead_code)]
pub fn is_fully_consistent(system: &dyn TransitionSystem) -> ConsistencyResult {
    if system.get_initial_location().is_none() {
        return ConsistencyResult::Failure(ConsistencyFailure::NoInitialLocation);
    }

    let mut passed = vec![];
    let state = system.get_initial_state();
    if state.is_none() {
        warn!("Empty initial state");
        return ConsistencyResult::Failure(ConsistencyFailure::EmptyInitialState);
    }
    consistency_fully_helper(state.unwrap(), &mut passed, system)
}

pub fn consistency_least_helper(
    state: State,
    passed_list: &mut Vec<State>,
    system: &dyn TransitionSystem,
) -> ConsistencyResult {
    let mut failing_action = String::new();

    if state.is_contained_in_list(passed_list) {
        return ConsistencyResult::Success;
    }
    if state.decorated_locations.is_universal() {
        return ConsistencyResult::Success;
    }
    if state.decorated_locations.is_inconsistent() {
        return ConsistencyResult::Failure(ConsistencyFailure::NotConsistentFrom(
            SimpleID::new(state.get_location().id.clone().to_string(), None),
            failing_action,
        ));
    }

    passed_list.push(state.clone());

    for input in system.get_input_actions() {
        for transition in &system.next_inputs(&state.decorated_locations, &input) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                new_state.extrapolate_max_bounds(system);
                if let ConsistencyResult::Failure(failure) =
                    consistency_least_helper(new_state, passed_list, system)
                {
                    warn!(
                        "Input \"{input}\" not consistent from {}",
                        state.get_location().id
                    );
                    return ConsistencyResult::Failure(failure);
                }
            } else {
                failing_action = input.clone();
            }
        }
    }

    if state.zone_ref().can_delay_indefinitely() {
        return ConsistencyResult::Success;
    }

    for output in system.get_output_actions() {
        for transition in system.next_outputs(&state.decorated_locations, &output) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                new_state.extrapolate_max_bounds(system);
                if let ConsistencyResult::Success =
                    consistency_least_helper(new_state, passed_list, system)
                {
                    return ConsistencyResult::Success;
                }
            } else {
                failing_action = output.clone();
            }
        }
    }
    warn!("No saving outputs from {}", state.get_location().id);
    ConsistencyResult::Failure(ConsistencyFailure::NotConsistentFrom(
        SimpleID::new(state.get_location().id.clone().to_string(), None),
        failing_action,
    ))
}

#[allow(dead_code)]
fn consistency_fully_helper(
    state: State,
    passed_list: &mut Vec<State>,
    system: &dyn TransitionSystem,
) -> ConsistencyResult {
    let mut failing_action = String::new();

    if state.is_contained_in_list(passed_list) {
        return ConsistencyResult::Success;
    }
    passed_list.push(state.clone());

    for input in system.get_input_actions() {
        for transition in system.next_inputs(&state.decorated_locations, &input) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                new_state.extrapolate_max_bounds(system);
                if new_state.is_subset_of(&state) {
                    continue;
                }
                if let ConsistencyResult::Failure(failure) =
                    consistency_fully_helper(new_state, passed_list, system)
                {
                    return ConsistencyResult::Failure(failure);
                }
            } else {
                failing_action = input.clone();
            }
        }
    }

    let mut output_existed = false;
    for output in system.get_output_actions() {
        for transition in system.next_outputs(&state.decorated_locations, &output) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                new_state.extrapolate_max_bounds(system);
                if new_state.is_subset_of(&state) {
                    continue;
                }

                output_existed = true;

                if let ConsistencyResult::Failure(failure) =
                    consistency_fully_helper(new_state, passed_list, system)
                {
                    return ConsistencyResult::Failure(failure);
                }
            } else {
                failing_action = output.clone();
            }
        }
    }
    if output_existed {
        ConsistencyResult::Success
    } else {
        let last_state = passed_list.last().unwrap();
        match last_state.zone_ref().can_delay_indefinitely() {
            false => ConsistencyResult::Failure(ConsistencyFailure::NotConsistentFrom(
                SimpleID::new(state.get_location().id.clone().to_string(), None),
                failing_action,
            )),
            true => ConsistencyResult::Success,
        }
    }
}
