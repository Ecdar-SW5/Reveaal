use super::{CompositionType, LocationID, LocationTuple};
use crate::{
    ModelObjects::component::{Declarations, State, Transition},
    System::local_consistency::DeterminismResult,
    System::local_consistency::{ConsistencyFailure, ConsistencyResult},
};
use dyn_clone::{clone_trait_object, DynClone};
use edbm::util::{bounds::Bounds, constraints::ClockIndex};
use std::collections::{hash_set::HashSet, HashMap};

pub type TransitionSystemPtr = Box<dyn TransitionSystem>;

/// Precheck can fail because of either consistency or determinism.
pub enum PrecheckResult {
    Success,
    NotDeterministic(LocationID, String),
    NotConsistent(ConsistencyFailure),
}

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

    fn get_location(&self, id: &LocationID) -> Option<LocationTuple> {
        self.get_all_locations()
            .iter()
            .find(|loc| loc.id == *id)
            .cloned()
    }

    fn get_decls(&self) -> Vec<&Declarations>;

    fn get_combined_decls(&self) -> Declarations {
        let (left, right) = self.get_children();
        let mut clocks = HashMap::new();
        let mut ints = HashMap::new();
        for decl in [left.get_combined_decls(), right.get_combined_decls()] {
            clocks.extend(decl.clocks);
            ints.extend(decl.ints)
        }

        Declarations { ints, clocks }
    }

    fn precheck_sys_rep(&self) -> PrecheckResult;

    fn is_deterministic(&self) -> DeterminismResult;

    fn is_locally_consistent(&self) -> ConsistencyResult;

    fn get_initial_state(&self) -> Option<State>;

    fn get_children(&self) -> (&TransitionSystemPtr, &TransitionSystemPtr);

    fn get_composition_type(&self) -> CompositionType;
}

clone_trait_object!(TransitionSystem);
