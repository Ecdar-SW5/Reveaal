use crate::component::State;
use crate::DataReader::component_loader::ModelCache;
use crate::ProtobufServer::ecdar_requests::helpers;
use crate::ProtobufServer::services::{
    ComponentClock, Conjunction as ProtoConjunction, Constraint as ProtoConstraint,
    DecisionPoint as ProtoDecisionPoint, Disjunction as ProtoDisjunction, Edge as ProtoEdge,
    Federation as ProtoFederation, Location as ProtoLocation, LocationTuple as ProtoLocationTuple,
    SimulationStartRequest, SimulationStepResponse, SpecificComponent, State as ProtoState,
};
use crate::Simulation::decision_point::DecisionPoint;
use crate::Simulation::transition_decision_point::TransitionDecisionPoint;
use crate::TransitionSystems::{LocationID, LocationTuple, TransitionSystemPtr};
use edbm::util::constraints::{Conjunction, Constraint, Disjunction};
use edbm::zones::OwnedFederation;
use log::trace;

use tonic::Status;

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub fn handle_start_simulation(
        request: SimulationStartRequest,
        _cache: ModelCache, // TODO should be used...
    ) -> Result<SimulationStepResponse, Status> {
        trace!("Received query: {:?}", request);

        let simulation_info = match request.simulation_info {
            Some(v) => v,
            None => return Err(Status::invalid_argument("simulation_info was None")),
        };

        let transition_system = helpers::simulation_info_to_transition_system(simulation_info);

        let initial = TransitionDecisionPoint::initial(&transition_system)
            .map(|i| ProtoDecisionPoint::from_transition_decision_point(&i, &transition_system));

        Ok(SimulationStepResponse {
            new_decision_point: initial,
        })
    }
}

impl ProtoDecisionPoint {
    pub fn from_transition_decision_point(
        decision_point: &TransitionDecisionPoint,
        system: &TransitionSystemPtr,
    ) -> ProtoDecisionPoint {
        let decision_point: DecisionPoint = DecisionPoint::from(decision_point);
        let decision_point: ProtoDecisionPoint = ProtoDecisionPoint::from(&decision_point, system);
        decision_point
    }
}

impl ProtoDecisionPoint {
    fn from(decision_point: &DecisionPoint, system: &TransitionSystemPtr) -> Self {
        let source = ProtoState::from(decision_point.source(), system);

        let edges = decision_point
            .possible_decisions()
            .iter()
            .map(ProtoEdge::from)
            .collect();

        ProtoDecisionPoint {
            source: Some(source),
            edges,
        }
    }
}

impl ProtoState {
    fn from(s: &State, system: &TransitionSystemPtr) -> Self {
        let location_tuple = ProtoLocationTuple::from(s.get_location());
        let federation = ProtoFederation::from(s.zone_ref(), system);

        ProtoState {
            location_tuple: Some(location_tuple),
            federation: Some(federation),
        }
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

impl From<&LocationTuple> for ProtoLocationTuple {
    fn from(l: &LocationTuple) -> Self {
        ProtoLocationTuple {
            locations: location_id_to_proto_location_vec(&l.id),
        }
    }
}

impl ProtoFederation {
    fn from(f: &OwnedFederation, system: &TransitionSystemPtr) -> Self {
        ProtoFederation {
            disjunction: Some(ProtoDisjunction::from(&f.minimal_constraints(), system)),
        }
    }
}

impl ProtoDisjunction {
    fn from(d: &Disjunction, system: &TransitionSystemPtr) -> Self {
        ProtoDisjunction {
            conjunctions: d
                .conjunctions
                .iter()
                .map(|x| ProtoConjunction::from(x, system))
                .collect(),
        }
    }
}

impl ProtoConjunction {
    fn from(c: &Conjunction, system: &TransitionSystemPtr) -> Self {
        ProtoConjunction {
            constraints: c
                .constraints
                .iter()
                .map(|x| ProtoConstraint::from(x, system))
                .collect(),
        }
    }
}

// TODO finish this
#[allow(unused_must_use)]
impl ProtoConstraint {
    fn from(constraint: &Constraint, system: &TransitionSystemPtr) -> Self {
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
}

impl ProtoEdge {
    fn from(e: &String) -> Self {
        ProtoEdge {
            id: e.to_string(),
            specific_component: None, // Edge id's are unique thus this is not needed
        }
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
        ProtobufServer::services::{DecisionPoint as ProtoDecisionPoint, SimulationStepResponse},
        Simulation::decision_point::DecisionPoint,
        TransitionSystems::CompiledComponent,
    };
    use tonic::Response;

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
        let actual = ProtoDecisionPoint::from(&decision_point, &system);
        let actual = Response::new(SimulationStepResponse {
            new_decision_point: Some(actual),
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
        let actual = ProtoDecisionPoint::from(&decisionPoint, &system);

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
        let actual = ProtoDecisionPoint::from(&decisionPoint, &system);

        // Assert
        let expected = create_decision_point_after_taking_E5();

        assert_eq!(actual.source, expected.source);
        assert_eq!(actual.edges.len(), 2);
        assert!(actual.edges.contains(&expected.edges[0]));
        assert!(actual.edges.contains(&expected.edges[1]));
    }
}
