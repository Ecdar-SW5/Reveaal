use crate::{TransitionSystems::{TransitionSystemPtr}, component::{State, Transition}};

#[allow(dead_code)]
pub struct TransitionDecision {
    source: State,
    transitions: Vec<Transition>,
}

impl TransitionDecision {
    pub fn initial_transition_decision(system: TransitionSystemPtr) -> Self {
        let source = system.get_initial_state().unwrap();
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
