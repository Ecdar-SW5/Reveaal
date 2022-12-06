use itertools::Itertools;
use regex::Regex;

use crate::{
    component::State,
    TransitionSystems::{TransitionID, TransitionSystemPtr},
};

use super::transition_decision_point::TransitionDecisionPoint;

/// Represents a decision in any composition of components: In the current `source` state there is a decision of using one of the `possible_decisions`.
#[derive(Clone, Debug)]
pub struct DecisionPoint {
    source: State,
    possible_decisions: Vec<String>,
}

impl DecisionPoint {
    pub fn new(source: State, possible_decisions: Vec<String>) -> Self {
        Self {
            source,
            possible_decisions,
        }
    }

    pub fn source(&self) -> &State {
        &self.source
    }

    pub fn possible_decisions(&self) -> &[String] {
        self.possible_decisions.as_ref()
    }

    /// Returns the initial [`DecisionPoint`] in the given [`TransitionSystemPrt`].
    pub fn initial(system: &TransitionSystemPtr) -> Option<Self> {
        TransitionDecisionPoint::initial(system).map(|initial| DecisionPoint::from(&initial))
    }
}

impl From<&TransitionDecisionPoint> for DecisionPoint {
    fn from(transition_decision_point: &TransitionDecisionPoint) -> Self {
        fn is_edge(x: &str) -> bool {
            let is_not_edge_regex = Regex::new("(input_).*").unwrap(); // `.unwrap()` always return `Some(...)` here
            !is_not_edge_regex.is_match(x)
        }
        let possible_decisions = transition_decision_point
            .possible_decisions()
            .iter()
            .flat_map(|transition| transition.id.get_leaves().concat())
            .filter_map(|transition_id| match transition_id {
                TransitionID::Simple(v) => Some(v),
                TransitionID::None => None,
                _ => panic!("transition_id should not be other than Simple(_) and None"),
            })
            .unique()
            .filter(|x| is_edge(x))
            .sorted()
            .collect();

        DecisionPoint {
            source: transition_decision_point.source().clone(),
            possible_decisions,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tests::Simulation::test_data::initial_transition_decision_point_EcdarUniversity_Machine;

    use super::DecisionPoint;

    #[test]
    fn from__initial_EcdarUniversity_Machine__returns_correct_DecisionPoint() {
        // Arrange
        let transition_decision_point = initial_transition_decision_point_EcdarUniversity_Machine();

        // Act
        let actual = DecisionPoint::from(&transition_decision_point);

        // Assert
        assert_eq!(actual.possible_decisions.len(), 2);
        assert!(actual.possible_decisions().contains(&"E27".to_string()));
        assert!(actual.possible_decisions().contains(&"E29".to_string()));
    }
}
