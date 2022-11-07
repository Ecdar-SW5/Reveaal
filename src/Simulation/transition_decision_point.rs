use crate::{
    component::{State, Transition},
    TransitionSystems::{TransitionSystem, TransitionSystemPtr},
};

use super::decision_point::Decision;

/// Represents a decision in a transition system: In the current `source` state there is a decision of using one of the `possible_decisions`.
#[derive(Debug, Clone)]
pub struct TransitionDecisionPoint {
    pub source: State,
    pub possible_decisions: Vec<Transition>,
}

/// Represent a decision in a transition system, that has been taken: In the current `source` state I have `decided` to use this `Transition`.  
#[derive(Debug)]
pub struct TransitionDecision {
    pub source: State,
    pub decided: Transition,
}

impl TransitionDecision {
    pub fn from(decision: &Decision, system: &TransitionSystemPtr) -> Self {
        let source = decision.source.to_owned();
        let action = decision.decided.get_sync();

        let transitions = system.next_transitions_if_available(source.get_location(), action);

        let decided = match transitions.len() {
            0 => panic!("No transitions for {}", action),
            1 => transitions.first().unwrap().to_owned(),
            _ => todo!(),
        };

        TransitionDecision { source, decided }
    }
}

impl TransitionDecisionPoint {
    /// Constructs the initial `TransitionDecisionPoint` for a given `TransitionSystemPtr`
    pub fn initial(system: TransitionSystemPtr) -> Option<Self> {
        match system.get_initial_state() {
            Some(source) => Some(Self::from(system, source)),
            None => None,
        }
    }

    /// Constructs the `TransitionDecisionPoint` from a `source: State` and a given `TransitionSystemPtr`
    pub fn from(system: TransitionSystemPtr, source: State) -> TransitionDecisionPoint {
        let transitions = from_action_to_transitions(system, &source);

        TransitionDecisionPoint {
            source: source,
            possible_decisions: transitions,
        }
    }
}

pub fn from_action_to_transitions(
    system: Box<dyn TransitionSystem>,
    source: &State,
) -> Vec<Transition> {
    let actions = system.get_actions();
    let transitions: Vec<Transition> = actions
        .into_iter()
        // Map actions to transitions. An action can map to multiple actions thus flatten
        .flat_map(|action| system.next_transitions_if_available(source.get_location(), &action))
        // Filter transitions that can be used
        .filter(|transition| transition.use_transition(&mut source.clone()))
        .collect();
    transitions
}

impl TransitionDecision {
    /// Resolves a `TransitionDecision`: use the `decided: Transition` and return the `TransitionDecisionPoint` of the destination `State`  
    pub fn resolve(mut self, system: TransitionSystemPtr) -> TransitionDecisionPoint {
        self.decided.use_transition(&mut self.source);
        TransitionDecisionPoint::from(system, self.source)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::{from_action_to_transitions, TransitionDecision, TransitionDecisionPoint};
    use crate::component::{Edge, Transition};
    use crate::tests::Simulation::helper::*;
    use crate::ProtobufServer::services::Decision as ProtoDecision;
    use crate::{
        tests::Simulation::helper::{
            create_EcdarUniversity_Machine4_system, create_EcdarUniversity_Machine_system,
        },
        DataReader::json_reader::read_json_component,
        Simulation::decision_point::{test::initial_transition_decision_point, Decision},
        TransitionSystems::{CompiledComponent, TransitionSystemPtr},
    };

    #[test]
    fn initial__EcdarUniversity_Machine__return_correct_state() {
        // Arrange
        let system = create_EcdarUniversity_Machine_system();

        // Act
        let actual = format!(
            "{:?}",
            TransitionDecisionPoint::initial(system.clone())
                .unwrap()
                .source
        );

        // Assert
        let expected = format!("{:?}", system.get_initial_state().unwrap());
        assert_eq!(actual, expected)
    }

    #[test]
    fn initial__EcdarUniversity_Machine__correct_transitions() {
        // Arrange
        let system = create_EcdarUniversity_Machine_system();

        // Act
        let actual: Vec<String> = TransitionDecisionPoint::initial(system.clone())
            .unwrap()
            .possible_decisions
            .into_iter()
            .map(|x| format!("{:?}", x)) // shhhhhh, close your eyes, this is not logic
            .collect();

        // Assert
        let expected_len = 2;
        assert_eq!(actual.len(), expected_len);

        let expected_tea_transition = &format!(
            "{:?}",
            system.next_transitions_if_available(&system.get_initial_location().unwrap(), "tea")[0]
        );
        assert!(actual.contains(expected_tea_transition));

        let expected_coin_transition = &format!(
            "{:?}",
            system.next_transitions_if_available(&system.get_initial_location().unwrap(), "coin")
                [0]
        );
        assert!(actual.contains(expected_coin_transition));
    }

    #[test]
    fn initial__EcdarUniversity_Machine4__correct_transitions() {
        // Arrange
        let system = create_EcdarUniversity_Machine4_system();

        // Act
        let actual: Vec<String> = TransitionDecisionPoint::initial(system.clone())
            .unwrap()
            .possible_decisions
            .into_iter()
            .map(|x| format!("{:?}", x)) // still no logic to be found here
            .collect();

        // Assert
        let expected_len = 1;
        assert_eq!(actual.len(), expected_len);

        let expected_coin_transition = &format!(
            "{:?}",
            system.next_transitions_if_available(&system.get_initial_location().unwrap(), "coin")
                [0]
        );
        assert!(actual.contains(expected_coin_transition));
    }

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
    fn TransitionDecision_from__Decision__returns_correct_TransitionDecision() {
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
    fn from__edge_with_action_that_maps_to_multiple_transitions__returns_correct_transitions() {
        // Arrange
        // let transition = Transition::from(comp, edge, 0);

        // Act

        // Assert

        assert!(false);
    }
}
