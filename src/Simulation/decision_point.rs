use super::transition_decision_point::TransitionDecisionPoint;
use crate::component::{Edge, State, Transition};

#[derive(Clone, Debug)]
pub struct DecisionPoint {
    source: State,
    possible_decisions: Vec<Edge>,
}

impl From<&TransitionDecisionPoint> for DecisionPoint {
    fn from(transition_decision_point: &TransitionDecisionPoint) -> Self {
        let possible_decisions = transition_decision_point
            .possible_decisions()
            .iter()
            .flat_map(Vec::<Edge>::from)
            .collect();

        DecisionPoint {
            source: transition_decision_point.source().clone(),
            possible_decisions,
        }
    }
}

impl From<&Transition> for Vec<Edge> {
    fn from(_: &Transition) -> Self {
        vec![] //TODO finish this
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::DecisionPoint;
    use crate::tests::Simulation::helper::initial_transition_decision_point_EcdarUniversity_Machine;

    #[test]
    fn DecisionPoint_from__initial_EcdarUniversity_Machine__returns_correct_DecisionPoint() {
        // Arrange
        let transition_decision_point = initial_transition_decision_point_EcdarUniversity_Machine();

        // Act
        let actual = DecisionPoint::from(&transition_decision_point);
        let actual_edge_ids: Vec<&str> = actual
            .possible_decisions
            .iter()
            .map(|e| e.id.as_str())
            .collect();

        // Assert
        assert_eq!(actual.possible_decisions.len(), 2);
        assert!(actual_edge_ids.contains(&"E5"));
        assert!(actual_edge_ids.contains(&"E3"));
    }
}
