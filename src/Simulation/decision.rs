use crate::component::{Edge, State};
use crate::ProtobufServer::services::Decision as ProtoDecision;
use crate::TransitionSystems::LocationID;

#[derive(Debug)]
pub struct Decision {
    pub source: State,
    pub decided: Edge,
}

impl From<ProtoDecision> for Decision {
    fn from(proto_decision: ProtoDecision) -> Self {
        let proto_state = match proto_decision.source {
            None => panic!("Not found"),
            Some(source) => source,
        };

        let proto_location_tuple = match proto_state.location_tuple {
            None => panic!("No loc tuple"),
            Some(loc_tuple) => loc_tuple,
        };

        let _proto_location_ids: Vec<LocationID> = proto_location_tuple
            .locations
            .iter()
            .map(|loc| LocationID::from_string(loc.id.as_str()))
            .collect();

        todo!();
        // return Decision {
        //     source: todo!(),
        //     decided: todo!(),
        // };
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        component::Edge,
        tests::Simulation::helper::{
            create_EcdarUniversity_Machine_Decision, create_EcdarUniversity_Machine_system,
            initial_transition_decision_point_EcdarUniversity_Machine,
        },
        Simulation::decision::Decision,
    };

    // TODO this test is badly formatted
    #[test]
    fn Decision_from__ProtoDecision__returns_correct_Decision() {
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
