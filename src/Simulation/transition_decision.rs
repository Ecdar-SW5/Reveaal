use edbm::zones::OwnedFederation;

use crate::{
    component::{State, Transition},
    TransitionSystems::TransitionSystemPtr,
};

use super::{decision::Decision, transition_decision_point::TransitionDecisionPoint};

// Represent a decision in a transition system, that has been taken: In the current `source` state I have `decided` to use this `Transition`.
#[derive(Debug)]
pub struct TransitionDecision {
    pub source: State,
    pub decided: Transition,
}

impl TransitionDecision {
    // Takes a decision and system as input, and transforms
    // the decision into a TransitionDecision
    pub fn from(decision: &Decision, system: &TransitionSystemPtr) -> Self {
        let source = decision.source.to_owned();
        let action = decision.decided.get_sync();
        let fed = OwnedFederation::init(system.get_dim());

        let transitions = system.next_transitions_if_available(source.get_location(), action);
        
        for transition in &transitions {
            transition.apply_guards(fed.clone());
        }
        
        let decided = match transitions.len() {
            0 => panic!("No transitions for {}", action),
            1 => transitions.first().unwrap().to_owned(),
            _ => panic!("Multiple transitions for {}", action),
        };

        TransitionDecision { source, decided }
    }

    /// Resolves a `TransitionDecision`: use the `decided: Transition` and return the `TransitionDecisionPoint` of the destination `State`  
    pub fn resolve(mut self, system: TransitionSystemPtr) -> TransitionDecisionPoint {
        self.decided.use_transition(&mut self.source);
        TransitionDecisionPoint::from(system, self.source)
    }
}

#[cfg(test)]
mod tests {
    use edbm::zones::OwnedFederation;

    use crate::{
        component::{Declarations, State},
        tests::Simulation::helper::{
            create_EcdarUniversity_Machine_system, create_Simulation_Machine_system,
        },
        DataReader::json_reader::read_json_component,
        Simulation::{
            decision::Decision, transition_decision::TransitionDecision,
            transition_decision_point::TransitionDecisionPoint,
        },
    };

    // Yes this test is stupid, no you will not remove it >:(
    #[allow(unused_must_use)]
    #[test]
    fn resolve__EcdarUniversity_Machine__correct_TransitionDecisionPoint() {
        // Arrange
        let system = create_EcdarUniversity_Machine_system();

        let initial = system.get_initial_state().unwrap();

        let transition = system
            .next_transitions_if_available(initial.clone().get_location(), "coin")
            .first()
            .unwrap()
            .to_owned();

        let decision = TransitionDecision {
            source: initial.clone(),
            decided: transition.clone(),
        };

        // Act
        let actual = decision.resolve(system.clone());

        // Assert
        let actual_source = format!("{:?}", actual.source);
        let actual_possible_decisions: Vec<String> = actual
            .possible_decisions
            .into_iter()
            .map(|x| format!("{:?}", x))
            .collect();

        let mut source = initial.clone();
        transition.use_transition(&mut source);
        let expected = TransitionDecisionPoint::from(system, source);
        let expected_source = format!("{:?}", expected.source);
        let expected_possible_decisions = expected
            .possible_decisions
            .into_iter()
            .map(|x| format!("{:?}", x));

        assert_eq!(actual_source, expected_source);
        assert_eq!(
            actual_possible_decisions.len(),
            expected_possible_decisions.len()
        );

        expected_possible_decisions.map(|x| assert!(actual_possible_decisions.contains(&x)));
    }

    #[test]
    fn from__edge_with_action_that_maps_to_single_transition__returns_correct_TransitionDecision() {
        // Arrange
        let system = create_EcdarUniversity_Machine_system();
        let component = read_json_component("samples/json/EcdarUniversity", "Machine");
        let initial = system.get_initial_state().unwrap();
        let edge = component.get_edges()[4].clone();

        let decision = Decision {
            source: initial.clone(),
            decided: edge.clone(),
        };

        let expected = TransitionDecision {
            source: initial.clone(),
            decided: system
                .next_transitions(initial.get_location(), "tea")
                .first()
                .unwrap()
                .to_owned(),
        };

        // Act
        let actual = TransitionDecision::from(&decision, &system);

        // Assert
        let actual = format!("{:?}", actual);
        let expected = format!("{:?}", expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn from__edge_with_action_that_maps_to_multiple_transitions__returns_correct_TransitionDecision(
    ) {
        // Arrange
        let system = create_Simulation_Machine_system();
        let component = read_json_component("samples/json/Simulation", "SimMachine");
        let initial = system.get_initial_state().unwrap();
        let edges = component.get_edges().clone();
        let fed = OwnedFederation::init(system.get_dim());

        let decision = Decision {
            source: initial.clone(),
            decided: edges[0].clone(),
        };

        let edge_action = edges[0].get_sync();

        let transitions = system.next_transitions_if_available(initial.get_location(), edge_action).first().unwrap().to_owned();
        transitions.apply_guards(fed.clone());
        
        let expected = TransitionDecision {
            source: initial.clone(),
            decided: transitions
        };

        // Act
        let actual = TransitionDecision::from(&decision, &system);

        // Assert
        let actual = format!("{:?}", actual);
        let expected = format!("{:?}", expected);

        assert_eq!(actual, expected);
    }
}
