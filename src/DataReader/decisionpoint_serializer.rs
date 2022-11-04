pub struct SerializedDecisionPoint {}

impl SerializedDecisionPoint {}

#[cfg(test)]
mod tests {
    use std::process::id;

    use crate::component::Component;
    use crate::tests::Simulation::helper::create_EcdarUniversity_Machine_system;
    use crate::DataReader::json_reader::read_json_component;
    use crate::DataReader::parse_edge::EdgeParser;
    use crate::ProtobufServer::services::component;
    use crate::ProtobufServer::services::ComponentClock as ProtoComponentClock;
    use crate::ProtobufServer::services::Conjunction as ProtoConjunction;
    use crate::ProtobufServer::services::Constraint as ProtoConstraint;
    use crate::ProtobufServer::services::Decision;
    use crate::ProtobufServer::services::Decision as ProtoDecision;
    use crate::ProtobufServer::services::DecisionPoint as ProtoDecisionPoint;
    use crate::ProtobufServer::services::Disjunction as ProtoDisjunction;
    use crate::ProtobufServer::services::Edge as ProtoEdge;
    use crate::ProtobufServer::services::Federation as ProtoFederation;
    use crate::ProtobufServer::services::Location as ProtoLocation;
    use crate::ProtobufServer::services::LocationTuple as ProtoLocationTuple;
    use crate::ProtobufServer::services::SpecificComponent as ProtoSpecificComponent;
    use crate::ProtobufServer::services::State as ProtoState;
    use crate::ProtobufServer::ConcreteEcdarBackend;
    use crate::Simulation::decision_point;
    use crate::Simulation::decision_point::DecisionPoint;
    use crate::Simulation::transition_decision_point::TransitionDecisionPoint;
    use crate::TransitionSystems::TransitionSystem;

    pub fn setupHelper(_input_path: &str, _system: &str) -> Box<dyn TransitionSystem> {
        todo!();
    }

    fn create_EcdarUniversity_Machine_System_Component() -> Component {
        let component = read_json_component("samples/json/EcdarUniversity", "machine");

        return component;
    }

    fn create_EcdarUnversity_Machine_Initial_Decision_Point() -> ProtoDecisionPoint {
        //
        // LET STATEMENT PURGATORY
        // ENJOY
        //

        let specific_comp_dp = ProtoSpecificComponent {
            component_name: "Machine".to_string(),
            component_index: 1,
        };

        let componentclock_dp1 = ProtoComponentClock {
            specific_component: Some(specific_comp_dp.clone()),
            clock_name: "5".to_string(),
        };
        let componentclock_dp2 = ProtoComponentClock {
            specific_component: Some(specific_comp_dp.clone()),
            clock_name: "6".to_string(),
        };

        let constraint_dp = ProtoConstraint {
            x: Some(componentclock_dp1),
            y: Some(componentclock_dp2),
            strict: false,
            c: 1,
        };

        let conjunction_dp = ProtoConjunction {
            constraints: vec![constraint_dp],
        };

        let disjunction_dp = ProtoDisjunction {
            conjunctions: vec![conjunction_dp],
        };

        let federation_dp = ProtoFederation {
            disjunction: Some(disjunction_dp),
        };

        let location_dp1 = ProtoLocation {
            id: "L4".to_string(),
            specific_component: Some(specific_comp_dp.clone()),
        };

        let location_dp2 = ProtoLocation {
            id: "L5".to_string(),
            specific_component: Some(specific_comp_dp.clone()),
        };

        let loc_tuple_dp = ProtoLocationTuple {
            locations: vec![location_dp1, location_dp2],
        };

        let source_dp = ProtoState {
            location_tuple: Some(loc_tuple_dp),
            federation: Some(federation_dp),
        };

        let edge_dp1 = ProtoEdge {
            id: "E3".to_string(),
            specific_component: Some(specific_comp_dp.clone()),
        };

        let edge_dp2 = ProtoEdge {
            id: "E5".to_string(),
            specific_component: Some(specific_comp_dp.clone()),
        };

        let initial_dp = ProtoDecisionPoint {
            source: Some(source_dp),
            edges: vec![edge_dp1, edge_dp2],
        };
        return initial_dp;
    }

    #[test]
    fn given_state_return_serialized_state() {
        static PATH: &str = "samples/json/Conjunction";

        let _transition_system: Box<dyn TransitionSystem> = setupHelper(PATH, "Test1 && Test1");

        assert!(false);
    }

    #[test]
    fn from_decisionpoint_to_protoDecisionPoint__correctProtoDecisionPoint__returnsProtoDecisionPoint(
    ) {
        // Arrange
        let system = create_EcdarUniversity_Machine_system();
        let transitionDecisionPoint = decision_point::test::initial_transition_decision_point();
        let decisionPoint = DecisionPoint::from(transitionDecisionPoint);

        let expected = create_EcdarUnversity_Machine_Initial_Decision_Point();

        // fn protodecision_from_decisionpoint(input: DecisionPoint) -> ProtoDecisionPoint {
        //     let source = input.source;
        //     let edges = input.possible_decisions.
        //     iter().
        //     map(|e| Vec::<ProtoEdge>:).
        //     collect();

        // };
        // Act
        // let actual = protodecision_from_decisionpoint(decisionPoint);

        // Assert
        // assert_eq!(actual.edges.len(), 2);
        // assert!(actual.edges.contains(&expected.edges[1]));
        // assert!(actual.edges.contains(&expected.edges[2]));
        // assert_eq!(actual.source, expected.source);
        // assert_eq!(expected, actual);

        assert!(true);
    }
}
