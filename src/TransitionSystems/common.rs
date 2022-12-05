use std::collections::HashSet;

use dyn_clone::{clone_trait_object, DynClone};
use edbm::{
    util::{bounds::Bounds, constraints::ClockIndex},
    zones::OwnedFederation,
};
use log::warn;

use crate::TransitionSystems::CompositionType;
use crate::{
    ModelObjects::component::{Declarations, State, Transition},
    System::local_consistency::{ConsistencyResult, DeterminismResult},
};

use super::{LocationTuple, TransitionSystem, TransitionSystemPtr};

pub trait ComposedTransitionSystem: DynClone {
    fn next_transitions(&self, location: &LocationTuple, action: &str) -> Vec<Transition>;

    fn is_locally_consistent(&self) -> ConsistencyResult;

    fn get_children(&self) -> (&TransitionSystemPtr, &TransitionSystemPtr);

    fn get_children_mut(&mut self) -> (&mut TransitionSystemPtr, &mut TransitionSystemPtr);

    fn get_composition_type(&self) -> CompositionType;

    fn get_dim(&self) -> ClockIndex;

    fn get_input_actions(&self) -> HashSet<String>;

    fn get_output_actions(&self) -> HashSet<String>;
}

clone_trait_object!(ComposedTransitionSystem);

impl<T: ComposedTransitionSystem> TransitionSystem for T {
    fn get_local_max_bounds(&self, loc: &LocationTuple) -> Bounds {
        if loc.is_universal() || loc.is_inconsistent() {
            Bounds::new(self.get_dim())
        } else {
            let (left, right) = self.get_children();
            let loc_l = loc.get_left();
            let loc_r = loc.get_right();
            let mut bounds_l = left.get_local_max_bounds(loc_l);
            let bounds_r = right.get_local_max_bounds(loc_r);
            bounds_l.add_bounds(&bounds_r);
            bounds_l
        }
    }

    fn get_dim(&self) -> ClockIndex {
        self.get_dim()
    }
    fn next_transitions(&self, location: &LocationTuple, action: &str) -> Vec<Transition> {
        self.next_transitions(location, action)
    }
    fn get_input_actions(&self) -> HashSet<String> {
        self.get_input_actions()
    }

    fn get_output_actions(&self) -> HashSet<String> {
        self.get_output_actions()
    }

    fn get_actions(&self) -> HashSet<String> {
        self.get_input_actions()
            .union(&self.get_output_actions())
            .map(|action| action.to_string())
            .collect()
    }

    fn get_initial_location(&self) -> Option<LocationTuple> {
        let (left, right) = self.get_children();
        let l = left.get_initial_location()?;
        let r = right.get_initial_location()?;

        Some(LocationTuple::compose(&l, &r, self.get_composition_type()))
    }

    fn get_all_locations(&self) -> Vec<LocationTuple> {
        let (left, right) = self.get_children();
        let mut location_tuples = vec![];
        let left = left.get_all_locations();
        let right = right.get_all_locations();
        for loc1 in &left {
            for loc2 in &right {
                location_tuples.push(LocationTuple::compose(
                    loc1,
                    loc2,
                    self.get_composition_type(),
                ));
            }
        }
        location_tuples
    }

    /// Returns the declarations of both children.
    fn get_decls(&self) -> Vec<&Declarations> {
        let (left, right) = self.get_children();

        let mut comps = left.get_decls();
        comps.extend(right.get_decls());
        comps
    }

    fn is_deterministic(&self) -> DeterminismResult {
        let (left, right) = self.get_children();
        if let DeterminismResult::Success = left.is_deterministic() {
            if let DeterminismResult::Success = right.is_deterministic() {
                DeterminismResult::Success
            } else {
                right.is_deterministic()
            }
        } else {
            left.is_deterministic()
        }
    }

    fn is_locally_consistent(&self) -> ConsistencyResult {
        self.is_locally_consistent()
    }

    fn get_initial_state(&self) -> Option<State> {
        let init_loc = self.get_initial_location().unwrap();
        let mut zone = OwnedFederation::init(self.get_dim());
        zone = init_loc.apply_invariants(zone);
        if zone.is_empty() {
            warn!("Empty initial state");
            return None;
        }

        Some(State::create(init_loc, zone))
    }

    fn get_children(&self) -> (&TransitionSystemPtr, &TransitionSystemPtr) {
        self.get_children()
    }

    fn get_composition_type(&self) -> CompositionType {
        self.get_composition_type()
    }
}

pub trait CollectionOperation {
    fn is_disjoint_action(&self, other: &HashSet<String>) -> Result<bool, Vec<String>>;
}
impl CollectionOperation for HashSet<String> {
    fn is_disjoint_action(&self, other: &HashSet<String>) -> Result<bool, Vec<String>> {
        let mut reason: Vec<String> = vec![];
        for action in other {
            if self.contains(action) {
                reason.push(String::from(action));
            }
        }
        if !reason.is_empty() {
            return Err(reason);
        }
        Ok(true)
    }
}
