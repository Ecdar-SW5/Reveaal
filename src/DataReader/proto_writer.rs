use edbm::{
    util::constraints::{Conjunction, Constraint, Disjunction},
    zones::OwnedFederation,
};

use crate::{
    component::State,
    ProtobufServer::services::{
        ComponentClock, Conjunction as ProtoConjunction, Constraint as ProtoConstraint,
        DecisionPoint as ProtoDecisionPoint, Disjunction as ProtoDisjunction, Edge as ProtoEdge,
        Federation as ProtoFederation, Location as ProtoLocation,
        LocationTuple as ProtoLocationTuple, SpecificComponent, State as ProtoState,
    },
    Simulation::{
        decision_point::DecisionPoint, transition_decision_point::TransitionDecisionPoint,
    },
    TransitionSystems::{LocationID, LocationTuple, TransitionSystemPtr},
};

pub fn transition_decision_point_to_proto_decision_point(
    decision_point: &TransitionDecisionPoint,
    system: &TransitionSystemPtr,
) -> ProtoDecisionPoint {
    let decision_point: DecisionPoint = DecisionPoint::from(decision_point);
    let decision_point: ProtoDecisionPoint =
        decision_point_to_proto_decision_point(&decision_point, system);
    decision_point
}

fn decision_point_to_proto_decision_point(
    decision_point: &DecisionPoint,
    system: &TransitionSystemPtr,
) -> ProtoDecisionPoint {
    let source = state_to_proto_state(decision_point.source(), system);

    let edges = decision_point
        .possible_decisions()
        .iter()
        .map(edge_id_to_proto_edge)
        .collect();

    ProtoDecisionPoint {
        source: Some(source),
        edges,
    }
}

fn state_to_proto_state(s: &State, system: &TransitionSystemPtr) -> ProtoState {
    let location_tuple = location_tuple_to_proto_location_tuple(s.get_location());
    let federation = federation_to_proto_federation(s.zone_ref(), system);

    ProtoState {
        location_tuple: Some(location_tuple),
        federation: Some(federation),
    }
}

fn location_tuple_to_proto_location_tuple(l: &LocationTuple) -> ProtoLocationTuple {
    ProtoLocationTuple {
        locations: location_id_to_proto_location_vec(&l.id),
    }
}

fn location_id_to_proto_location_vec(is: &LocationID) -> Vec<ProtoLocation> {
    match is {
        LocationID::Simple {
            location_id,
            component_id,
        } => vec![ProtoLocation {
            id: location_id.to_string(),
            specific_component: Some(SpecificComponent {
                component_name: component_id.as_ref().unwrap().to_string(), //TODO unwrap bad!
                component_index: 0,
            }),
        }],
        LocationID::Conjunction(l, r)
        | LocationID::Composition(l, r)
        | LocationID::Quotient(l, r) => location_id_to_proto_location_vec(l)
            .into_iter()
            .chain(location_id_to_proto_location_vec(r).into_iter())
            .collect(),
        LocationID::AnyLocation() => vec![],
    }
}

fn federation_to_proto_federation(
    f: &OwnedFederation,
    system: &TransitionSystemPtr,
) -> ProtoFederation {
    ProtoFederation {
        disjunction: Some(disjunction_to_proto_disjunction(
            &f.minimal_constraints(),
            system,
        )),
    }
}

fn disjunction_to_proto_disjunction(
    d: &Disjunction,
    system: &TransitionSystemPtr,
) -> ProtoDisjunction {
    ProtoDisjunction {
        conjunctions: d
            .conjunctions
            .iter()
            .map(|x| conjunction_to_proto_conjunction(x, system))
            .collect(),
    }
}

fn conjunction_to_proto_conjunction(
    c: &Conjunction,
    system: &TransitionSystemPtr,
) -> ProtoConjunction {
    ProtoConjunction {
        constraints: c
            .constraints
            .iter()
            .map(|x| constraint_to_proto_constraint(x, system))
            .collect(),
    }
}

fn constraint_to_proto_constraint(
    constraint: &Constraint,
    system: &TransitionSystemPtr,
) -> ProtoConstraint {
    fn clock_name(clock_name_and_component: Option<&(String, String)>) -> String {
        let ZERO_CLOCK_NAME = "0";
        match clock_name_and_component {
            Some((clock_name, _)) => clock_name.to_string(),
            // If an index does not correspond to an index we assume it's the zero clock
            None => ZERO_CLOCK_NAME.to_string(),
        }
    }

    fn clock_component(
        clock_name_and_component: Option<&(String, String)>,
    ) -> Option<SpecificComponent> {
        clock_name_and_component.map(|x| SpecificComponent {
            component_name: x.1.to_string(),
            component_index: 0,
        })
    }

    let x = system.index_to_clock_name_and_component(&constraint.i);
    let y = system.index_to_clock_name_and_component(&constraint.j);

    ProtoConstraint {
        x: Some(ComponentClock {
            specific_component: clock_component(x.as_ref()),
            clock_name: clock_name(x.as_ref()),
        }),
        y: Some(ComponentClock {
            specific_component: clock_component(y.as_ref()),
            clock_name: clock_name(y.as_ref()),
        }),
        strict: constraint.ineq().is_strict(),
        c: constraint.ineq().bound(),
    }
}

fn edge_id_to_proto_edge(e: &String) -> ProtoEdge {
    ProtoEdge {
        id: e.to_string(),
        specific_component: None, // Edge id's are unique thus this is not needed
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::Simulation::helper::get_composition_response_Administration_Machine_Researcher;
    use crate::{
        tests::{
            grpc::grpc_helper::{
                create_decision_point_after_taking_E5, create_initial_decision_point,
            },
            Simulation::helper::{
                create_EcdarUniversity_Machine_system,
                initial_transition_decision_point_EcdarUniversity_Machine,
            },
        },
        DataReader::json_reader::read_json_component,
        ProtobufServer::services::SimulationStepResponse,
        Simulation::decision_point::DecisionPoint,
        TransitionSystems::CompiledComponent,
    };
    use tonic::Response;

    use super::decision_point_to_proto_decision_point;

    #[test]
    fn from__initial_DecisionPoint_EcdarUniversity_Administration_par_Machine_par_Researcher__returns_correct_ProtoDecisionPoint(
    ) {
        // Arrange
        let project_path = "samples/json/EcdarUniversity";

        let administration = read_json_component(project_path, "Administration");
        let machine = read_json_component(project_path, "Machine");
        let researcher = read_json_component(project_path, "Researcher");

        let combined = vec![administration, machine, researcher];
        let composition = "(Administration || Machine || Researcher)";

        let system = CompiledComponent::from(combined, composition);

        let decision_point = DecisionPoint::new(
            system.get_initial_state().unwrap(),
            vec![
                "E11".to_string(),
                "E16".to_string(),
                "E29".to_string(),
                "E44".to_string(),
            ],
        );

        // Act
        let actual = decision_point_to_proto_decision_point(&decision_point, &system);
        let actual = Response::new(SimulationStepResponse {
            new_decision_points: vec![actual],
        });

        // Assert
        let expected = get_composition_response_Administration_Machine_Researcher().unwrap();

        assert_eq!(format!("{:?}", actual), format!("{:?}", expected))
    }

    #[test]
    fn from__initial_DecisionPoint_EcdarUniversity_Machine__returns_correct_ProtoDecisionPoint() {
        // Arrange
        let transitionDecisionPoint = initial_transition_decision_point_EcdarUniversity_Machine();

        let system = create_EcdarUniversity_Machine_system();

        let decisionPoint = DecisionPoint::new(
            transitionDecisionPoint.source().to_owned(),
            vec!["E27".to_string(), "E29".to_string()],
        );

        // Act
        let actual = decision_point_to_proto_decision_point(&decisionPoint, &system);

        // Assert
        let expected = create_initial_decision_point();

        assert_eq!(actual.source, expected.source);
        assert_eq!(actual.edges.len(), 2);
        assert!(actual.edges.contains(&expected.edges[0]));
        assert!(actual.edges.contains(&expected.edges[1]));
    }

    #[test]
    fn from__initial_DecisionPoint_EcdarUniversity_Machine_after_tea__returns_correct_ProtoDecisionPoint(
    ) {
        // Arrange
        let system = create_EcdarUniversity_Machine_system();
        let mut after_tea = system.get_initial_state().unwrap();
        let action = "tea";
        let binding = system.next_transitions_if_available(after_tea.get_location(), action);
        let tea_transition = binding.first().unwrap();
        tea_transition.use_transition(&mut after_tea);

        let decisionPoint =
            DecisionPoint::new(after_tea, vec!["E27".to_string(), "E29".to_string()]);

        // Act
        let actual = decision_point_to_proto_decision_point(&decisionPoint, &system);

        // Assert
        let expected = create_decision_point_after_taking_E5();

        assert_eq!(actual.source, expected.source);
        assert_eq!(actual.edges.len(), 2);
        assert!(actual.edges.contains(&expected.edges[0]));
        assert!(actual.edges.contains(&expected.edges[1]));
    }
}
