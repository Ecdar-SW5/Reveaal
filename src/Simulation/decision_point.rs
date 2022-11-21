use super::transition_decision_point::TransitionDecisionPoint;
use crate::component::State;

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
}

impl From<&TransitionDecisionPoint> for DecisionPoint {
    fn from(transition_decision_point: &TransitionDecisionPoint) -> Self {
        let possible_decisions = transition_decision_point
            .possible_decisions()
            .iter()
            .flat_map(|transition| transition.id.get_leaves().concat())
            .map(|transition_id| match transition_id {
                crate::TransitionSystems::TransitionID::Simple(v) => v,
                crate::TransitionSystems::TransitionID::None => "".to_string(),
                _ => panic!("transition_id should not be other than Simple(_) and None"),
            })
            .collect();

        DecisionPoint {
            source: transition_decision_point.source().clone(),
            possible_decisions,
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::DecisionPoint;
    use crate::tests::Simulation::helper::initial_transition_decision_point_EcdarUniversity_Machine;

    #[test]
    #[ignore]
    fn DecisionPoint_from__initial_EcdarUniversity_Machine__returns_correct_DecisionPoint() {
        // Arrange
        let transition_decision_point = initial_transition_decision_point_EcdarUniversity_Machine();

        // Act
        let actual = DecisionPoint::from(&transition_decision_point);
        let actual_edge_ids = actual.possible_decisions();

        // Assert
        assert_eq!(actual.possible_decisions.len(), 2);
        assert!(actual_edge_ids.contains(&"E5".to_string()));
        assert!(actual_edge_ids.contains(&"E3".to_string()));
    }
}
