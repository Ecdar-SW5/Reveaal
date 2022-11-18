use crate::{
    component::{State, Transition},
    TransitionSystems::{TransitionID, TransitionSystemPtr},
};

use super::{decision::Decision, transition_decision_point::TransitionDecisionPoint};

/// Represent a decision in a transition system, that has been taken: In the current `source` state I have `decided` to use this `Transition`.
#[derive(Debug)]
pub struct TransitionDecision {
    source: State,
    decided: Transition,
}

impl TransitionDecision {
    /// Returns a `TransitionDecision` equivalent to the given `&Decision` in relation to the given `&TransitionSystemPtr`
    pub fn from(decision: &Decision, system: &TransitionSystemPtr) -> Result<Self, String> {
        fn contains(transition: &Transition, edge_id: &String) -> bool {
            transition
                .id
                .get_leaves()
                .concat()
                .iter()
                .map(|x| match x {
                    TransitionID::Simple(x) => &x,
                    _ => "",
                })
                .any(|x| x == edge_id)
        }
        let source = decision.source().to_owned();
        let action = decision.decided().get_sync();
        let edge_id = &decision.decided().id;

        // Choose transitions that correspond to a given edge.
        let transitions = system
            .next_transitions_if_available(source.get_location(), action)
            .into_iter()
            .filter(|t| contains(t, edge_id))
            .collect::<Vec<_>>()
            .to_owned();

        let decided = match transitions.len() {
            // If no transitions are left we have nothing to step along... Something has gone wrong
            0 => return Err("No valid transitions for {action}".to_string()),
            // If 1 transitions is left we choose that transition as our decided
            1 => transitions.first().unwrap().to_owned(), // If transitions.len() == 1 then transitions.first() == Some(...) always
            // Otherwise the result is non-deterministic, this is currently not supported by the simulation API
            // This might never happen, but i'm unsure.
            _ => {
                return Err("Non determinism not currently supported by Simulation API".to_string())
            }
        };

        Ok(TransitionDecision { source, decided })
    }

    /// Resolves a `TransitionDecision`: use the `decided: Transition` and return the `TransitionDecisionPoint` of the destination `State`  
    pub fn resolve(mut self, system: TransitionSystemPtr) -> TransitionDecisionPoint {
        self.decided.use_transition(&mut self.source);
        TransitionDecisionPoint::from(&system, &self.source)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tests::Simulation::helper::{
            create_EcdarUniversity_Machine_system, create_Simulation_Machine_system,
            create_system_from_path,
        },
        DataReader::json_reader::read_json_component,
        Simulation::{
            decision::Decision, transition_decision::TransitionDecision,
            transition_decision_point::TransitionDecisionPoint,
        },
        TransitionSystems::TransitionSystemPtr,
    };

    fn assert__from__good_Decision__returns_correct_TransitionDecision(
        system: TransitionSystemPtr,
        decision: Decision,
        expected: TransitionDecision,
    ) {
        // Act
        let actual = TransitionDecision::from(&decision, &system).unwrap();

        // Assert
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected))
    }

    #[test]
    fn from__Determinism_NonDeterminismCom__returns_ok() {
        // Arrange
        let path = "samples/json/Determinism";
        let component = "NonDeterminismCom";
        let system = create_system_from_path(path, component);
        let component = read_json_component(path, component);

        let decision = Decision::new(
            system.get_initial_state().unwrap(),
            component.get_edges().first().unwrap().to_owned(),
        );

        // Act
        let actual = TransitionDecision::from(&decision, &system);

        // Assert
        assert!(actual.is_ok());
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
        let actual_source = format!("{:?}", actual.source());
        let actual_possible_decisions: Vec<String> = actual
            .possible_decisions()
            .into_iter()
            .map(|x| format!("{:?}", x))
            .collect();

        let mut source = initial.clone();
        transition.use_transition(&mut source);
        let expected = TransitionDecisionPoint::from(&system, &source);
        let expected_source = format!("{:?}", expected.source());
        let expected_possible_decisions = expected
            .possible_decisions()
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

        let decision = Decision::new(initial.clone(), edge.clone());

        let expected = TransitionDecision {
            source: initial.clone(),
            decided: system
                .next_transitions(initial.get_location(), "tea")
                .first()
                .unwrap()
                .to_owned(),
        };

        assert__from__good_Decision__returns_correct_TransitionDecision(system, decision, expected);
    }

    #[test]
    fn from__edge_with_action_that_maps_to_multiple_transitions__returns_correct_TransitionDecision(
    ) {
        // Arrange
        let system = create_Simulation_Machine_system();
        let component = read_json_component("samples/json/Simulation", "SimMachine");
        let initial = system.get_initial_state().unwrap();
        let edges = component.get_edges().clone();

        let decision = Decision::new(initial.clone(), edges[0].clone());

        let edge_action = edges[0].get_sync();

        let expected = TransitionDecision {
            source: initial.clone(),
            decided: system.next_transitions(initial.get_location(), &edge_action)[0].clone(),
        };

        assert__from__good_Decision__returns_correct_TransitionDecision(system, decision, expected);
    }
}
