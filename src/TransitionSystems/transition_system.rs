use super::{CompositionType, LocationTuple};
use crate::ModelObjects::component::{Declarations, State, Transition};
use dyn_clone::{clone_trait_object, DynClone};
use edbm::util::{bounds::Bounds, constraints::ClockIndex};
use std::collections::hash_set::HashSet;

pub type TransitionSystemPtr = Box<dyn TransitionSystem>;

pub trait TransitionSystem: DynClone {
    fn get_local_max_bounds(&self, loc: &LocationTuple) -> Bounds;

    fn get_dim(&self) -> ClockIndex;

    fn next_transitions_if_available(
        &self,
        location: &LocationTuple,
        action: &str,
    ) -> Vec<Transition> {
        if self.actions_contain(action) {
            self.next_transitions(location, action)
        } else {
            vec![]
        }
    }

    fn next_transitions(&self, location: &LocationTuple, action: &str) -> Vec<Transition>;

    fn next_outputs(&self, location: &LocationTuple, action: &str) -> Vec<Transition> {
        debug_assert!(self.get_output_actions().contains(action));
        self.next_transitions(location, action)
    }

    fn next_inputs(&self, location: &LocationTuple, action: &str) -> Vec<Transition> {
        debug_assert!(self.get_input_actions().contains(action));
        self.next_transitions(location, action)
    }

    fn get_input_actions(&self) -> HashSet<String>;

    fn inputs_contain(&self, action: &str) -> bool {
        self.get_input_actions().contains(action)
    }

    fn get_output_actions(&self) -> HashSet<String>;

    fn outputs_contain(&self, action: &str) -> bool {
        self.get_output_actions().contains(action)
    }

    fn get_actions(&self) -> HashSet<String>;

    fn actions_contain(&self, action: &str) -> bool {
        self.get_actions().contains(action)
    }

    fn get_initial_location(&self) -> Option<LocationTuple>;

    fn get_all_locations(&self) -> Vec<LocationTuple>;

    fn get_decls(&self) -> Vec<&Declarations>;

    fn precheck_sys_rep(&self) -> bool;

    fn is_deterministic(&self) -> bool;

    fn is_locally_consistent(&self) -> bool;

    fn get_initial_state(&self) -> Option<State>;

    fn get_children(&self) -> (&TransitionSystemPtr, &TransitionSystemPtr);

    fn get_composition_type(&self) -> CompositionType;
}

clone_trait_object!(TransitionSystem);
