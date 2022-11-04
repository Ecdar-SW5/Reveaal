use super::transition_decision_point::TransitionDecisionPoint;
use crate::component::{Edge, State, Transition};
use crate::ProtobufServer::services::Decision as ProtoDecision;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DecisionPoint {
    pub(crate) source: State,
    pub(crate) possible_decisions: Vec<Edge>,
}

impl From<TransitionDecisionPoint> for DecisionPoint {
    fn from(transition_decision_point: TransitionDecisionPoint) -> Self {
        let possible_decisions = transition_decision_point
            .possible_decisions
            .iter()
            .flat_map(|t| Vec::<Edge>::from(t))
            .collect();

        DecisionPoint {
            source: transition_decision_point.source,
            possible_decisions,
        }
    }
}

impl From<&Transition> for Vec<Edge> {
    fn from(_: &Transition) -> Self {
        todo!()
    }
}
#[derive(Debug)]
pub struct Decision {

    source: State,
    decided: Edge,
}

impl From<ProtoDecision> for Decision {
    fn from(proto_decision: ProtoDecision) -> Self {
        todo!();
        // let serialized_source: ProtoDecision = proto_decision;
        // Decision {
        //     source: todo!(),
        //     decided: todo!(),
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::{DecisionPoint, Decision};
    use crate::{
        component::Edge,
        tests::Simulation::helper::{
            create_EcdarUniversity_Machine_system,
            initial_transition_decision_point_EcdarUniversity_Machine, create_EcdarUniversity_Machine_Decision,
        },
        Simulation::transition_decision_point::TransitionDecisionPoint,
    };
    use crate::ProtobufServer::services::Decision as ProtoDecision;

    #[test]
    fn DecisionPoint_from__initial_EcdarUniversity_Machine__returns_correct_DecisionPoint() {
        // Arrange
        let transition_decision_point = initial_transition_decision_point_EcdarUniversity_Machine();

        // Act
        let actual = DecisionPoint::from(transition_decision_point);
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

    pub fn initial_transition_decision_point() -> TransitionDecisionPoint {
        let system = create_EcdarUniversity_Machine_system();
        TransitionDecisionPoint::initial(system).unwrap()
    }

    #[test]
    fn Decision_from__ProtoDecision__returns_correct_Decision()
    {
        // Arrange
        let proto_decision = create_EcdarUniversity_Machine_Decision();

        let transition_decisions = initial_transition_decision_point_EcdarUniversity_Machine();
        let possible_decisions: Vec<Edge> = transition_decisions
            .possible_decisions
            .iter()
            .flat_map(|t| Vec::<Edge>::from(t))
            .collect();

        let expected_decision = match possible_decisions.into_iter().next() {
            None => panic!("No edges found"),
            Some(edge) => edge,
        };

        
        let actual_decision = Decision::from(proto_decision);
    
        let system = create_EcdarUniversity_Machine_system();
        let expected_source = match system.get_initial_state() {
            None => panic!("No inital state found"),
            Some(expected_source) => expected_source,
        };

        let expected_decision = Decision {
            source: expected_source,
            decided: expected_decision,
        };

        // Act
        let actual_decision = format!("{:?}", actual_decision);
        let expected_decision = format!("{:?}", expected_decision);

        // Assert
        assert_eq!(actual_decision, expected_decision);
    }
}
