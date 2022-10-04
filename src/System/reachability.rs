use crate::ModelObjects::component::{State, Transition};
use crate::TransitionSystems::TransitionSystem;

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

    searchAlgorithm(start_state, end_state, system)
}

pub fn searchAlgorithm(
    start_state: State,
    end_state: State,
    system: &dyn TransitionSystem,
) -> Option<Vec<SubPath>> {
    panic!("No implmentation");
}
