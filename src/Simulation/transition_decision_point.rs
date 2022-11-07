use crate::{
    component::{State, Transition},
    TransitionSystems::{TransitionSystem, TransitionSystemPtr},
};

/// Represents a decision in a transition system: In the current `source` state there is a decision of using one of the `possible_decisions`.
#[derive(Debug, Clone)]
pub struct TransitionDecisionPoint {
    pub source: State,
    pub possible_decisions: Vec<Transition>,
}

impl TransitionDecisionPoint {
    /// Constructs the initial `TransitionDecisionPoint` for a given `TransitionSystemPtr`
    pub fn initial(system: TransitionSystemPtr) -> Option<Self> {
        system
            .get_initial_state()
            .map(|source| Self::from(system, source))
    }

    /// Constructs the `TransitionDecisionPoint` from a `source: State` and a given `TransitionSystemPtr`
    pub fn from(system: TransitionSystemPtr, source: State) -> TransitionDecisionPoint {
        let transitions = from_action_to_transitions(system, &source);

        TransitionDecisionPoint {
            source,
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

#[cfg(test)]
pub(crate) mod tests {
    use super::TransitionDecisionPoint;
    use crate::tests::Simulation::helper::{
        create_EcdarUniversity_Machine4_system, create_EcdarUniversity_Machine_system,
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
}
