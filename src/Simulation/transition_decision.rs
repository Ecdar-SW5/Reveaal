use crate::{
    component::{State, Transition},
    TransitionSystems::TransitionSystemPtr,
};

#[allow(dead_code)]
pub struct TransitionDecision {
    source: State,
    transitions: Vec<Transition>,
}

impl TransitionDecision {
    /// Constructs the inital TransitionDecision for a given TransitionSystemPtr
    ///
    /// # Panics
    /// If the system has no inital state
    pub fn initial(system: TransitionSystemPtr) -> Self {
        let source = system.get_initial_state().unwrap();
        Self::from(system, source)
    }

    /// Constructs the TransitionDecision from a source State and a given TransitionSystemPtr
    pub fn from(system: TransitionSystemPtr, source: State) -> TransitionDecision {
        let mut transitions = vec![];
        let actions = system.get_actions();

        // get all transitions
        for action in actions {
            let transition = system.next_transitions_if_available(source.get_location(), &action);
            transitions.append(&mut transition.clone());
        }

        // prune transitions that can not be taken
        for (index, transition) in transitions.clone().iter().enumerate() {
            if !transition.use_transition(&mut source.clone()) {
                transitions.remove(index);
            }
        }

        TransitionDecision {
            source: source,
            transitions: transitions,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use edbm::util::{bounds::Bounds, constraints::ClockIndex};
    use mockall::mock;
    use crate::{TransitionSystems::{TransitionSystem, LocationTuple, TransitionSystemPtr, CompositionType}, component::{Transition, Declarations, State}};

    use super::TransitionDecision;



    mock! {
        TransitionSystem { }
        impl Clone for TransitionSystem {
            fn clone(&self) -> Self;
        }
        impl TransitionSystem for TransitionSystem {
            fn get_local_max_bounds(&self, loc: &LocationTuple) -> Bounds;
            fn get_dim(&self) -> ClockIndex;
            fn next_transitions_if_available(
                &self,
                location: &LocationTuple,
                action: &str,
            ) -> Vec<Transition>;
            fn next_transitions(&self, location: &LocationTuple, action: &str) -> Vec<Transition>;
            fn next_outputs(&self, location: &LocationTuple, action: &str) -> Vec<Transition>;
            fn next_inputs(&self, location: &LocationTuple, action: &str) -> Vec<Transition>;
            fn get_input_actions(&self) -> HashSet<String>;
            fn inputs_contain(&self, action: &str) -> bool;
            fn get_output_actions(&self) -> HashSet<String>;
            fn outputs_contain(&self, action: &str) -> bool;
            fn get_actions(&self) -> HashSet<String>;
            fn actions_contain(&self, action: &str) -> bool; 
            fn get_initial_location(&self) -> Option<LocationTuple>;
            fn get_all_locations(&self) -> Vec<LocationTuple>;
            fn get_decls(&self) -> Vec<&'static Declarations>;
            fn precheck_sys_rep(&self) -> bool;
            fn is_deterministic(&self) -> bool;
            fn is_locally_consistent(&self) -> bool;
            fn get_initial_state(&self) -> Option<State>;
            fn get_children<'a>(&self) -> (&'static TransitionSystemPtr, &'static TransitionSystemPtr);
            fn get_composition_type(&self) -> CompositionType;
        }
    }

    #[test]
    #[should_panic]
    fn initial__no_initial_state__panics() {
        // Arrange
        let mut system = Box::new(MockTransitionSystem::new());
        system.expect_get_initial_state().return_const(None);
        
        // Act
        TransitionDecision::initial(system);
    }

}
