use crate::{TransitionSystems::{TransitionSystemPtr}, component::{State, Transition}};
use mockall::*;
use mockall::predicate::*;

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
    pub fn initial_transition_decision(system: TransitionSystemPtr) -> Self {
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

        TransitionDecision { source: source, transitions: transitions }
    }
}

#[cfg(test)]
mod tests {
    fn from__source_with_no_transitions__returns_source_with_no_transitions() {
        assert!(false);
    }
}

