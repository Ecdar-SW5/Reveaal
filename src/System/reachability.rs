use crate::ModelObjects::component::State;
use crate::TransitionSystems::TransitionSystem;


struct Path{

}

// pub fn preliminary_check_succes(take some input) -> return a path{
//    It returns a path
// }

pub fn is_reachable(begin_state: Option<State>, end_state: State, system: &dyn TransitionSystem) -> Option<Path>
{
    // if preliminary_check_succes() { return a path }

    let start_state = begin_state.unwrap_or_else(system.get_initial_state());

    searchAlgorithm(start_state, end_state, system)
}


pub fn searchAlgorithm(start_state: State, end_state: State, system: &dyn TransitionSystem) -> Option<Path>{



}


