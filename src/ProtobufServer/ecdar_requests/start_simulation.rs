use std::collections::HashMap;
use std::hash::Hash;
use std::iter::zip;

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
        fn invert<T1, T2>(hash_map: HashMap<T1, T2>) -> HashMap<T2, T1>
        where
            T2: Hash + Eq,
        {
            hash_map.into_iter().map(|x| (x.1, x.0)).collect()
        }

        fn clock_name(
            index_to_identifier: &HashMap<usize, (String, String)>,
            index: usize,
        ) -> String {
            let ZERO_CLOCK_NAME = "0";
            match index_to_identifier.get(&index) {
                Some((clock_name, _)) => clock_name.to_string(),
                // If an index does not correspond to an index we assume it's the zero clock
                None => ZERO_CLOCK_NAME.to_string(),
            }
        }

        fn clock_component(
            index_to_identifier: &HashMap<usize, (String, String)>,
            index: usize,
        ) -> Option<SpecificComponent> {
            index_to_identifier.get(&index).map(|x| SpecificComponent {
                component_name: x.1.to_string(),
                component_index: 0,
            })
        }

        let binding = system.component_names();
        let component_names = binding.into_iter();
        let binding = system.get_decls();
        let clock_to_index = binding.into_iter().map(|decl| decl.clocks.to_owned());

        let index_to_name_and_component = zip(component_names, clock_to_index)
            .map(|x| {
                x.1.iter()
                    .map(|y| ((y.0.to_owned(), x.0.to_string()), y.1.to_owned()))
                    .collect::<HashMap<(String, String), usize>>()
            })
            .fold(HashMap::new(), |accumulator, head| {
                accumulator.into_iter().chain(head).collect()
            });
        let index_to_name_and_component = invert(index_to_name_and_component);

        ProtoConstraint {
            x: Some(ComponentClock {
                specific_component: clock_component(&index_to_name_and_component, constraint.i),
                clock_name: clock_name(&index_to_name_and_component, constraint.i),
            }),
            y: Some(ComponentClock {
                specific_component: clock_component(&index_to_name_and_component, constraint.j),
                clock_name: clock_name(&index_to_name_and_component, constraint.j),
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
    use crate::tests::grpc::grpc_helper::create_json_component_as_string;
    use crate::tests::Simulation::helper::get_composition_response_Administration_Machine_Researcher;
    use crate::ProtobufServer::services::{
        ComponentClock, Conjunction as ProtoConjunction, Constraint as ProtoConstraint,
        Disjunction as ProtoDisjunction, Edge as ProtoEdge, Federation as ProtoFederation,
        Location as ProtoLocation, LocationTuple as ProtoLocationTuple, SpecificComponent,
        State as ProtoState,
    };
    use crate::{
        tests::{
            grpc::grpc_helper::{
                convert_json_component_to_string, create_decision_point_after_taking_E5,
                create_initial_decision_point,
            },
            Simulation::helper::{
                create_EcdarUniversity_Machine_system,
                initial_transition_decision_point_EcdarUniversity_Machine,
            },
        },
        DataReader::json_reader::read_json_component,
        ProtobufServer::{
            self,
            services::{
                component::Rep, ecdar_backend_server::EcdarBackend, Component, ComponentsInfo,
                DecisionPoint as ProtoDecisionPoint, SimulationInfo, SimulationStartRequest,
                SimulationStepResponse,
            },
        },
        Simulation::decision_point::DecisionPoint,
        TransitionSystems::CompiledComponent,
    };
    use test_case::test_case;
    use tonic::{Request, Response, Status};

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
                "E29".to_string(),
                "E11".to_string(),
                "E16".to_string(),
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

    #[test_case(
        create_composition_request(),
        create_expected_response_to_composition_request();
        "given a composition request, responds with correct component"
    )]
    // #[test_case(
    //     create_conjunction_request(),
    //     create_expected_response_to_conjunction_request();
    //     "given a good conjunction request, responds with correct component"
    // )]
    // #[test_case(
    //     create_quotient_request(),
    //     create_expected_response_to_quotient_request();
    //     "given a good quotient request, responds with correct component"
    // )]
    #[tokio::test]
    async fn start_simulation_step__get_composit_component__should_return_component(
        request: Request<SimulationStartRequest>,
        expected_response: Result<Response<SimulationStepResponse>, Status>,
    ) {
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        let actual_response = backend.start_simulation(request).await;

        // Assert
        assert_eq!(
            format!("{:?}", expected_response),
            format!("{:?}", actual_response)
        );
    }

    // Helpers
    fn create_composition_request() -> Request<SimulationStartRequest> {
        let composition = "(Administration || Machine || Researcher)".to_string();

        let administration_component = create_json_component_as_string(
            "samples/json/EcdarUniversity/Components/Administration.json".to_string(),
        );
        let machine_component = create_json_component_as_string(
            "samples/json/EcdarUniversity/Components/Machine.json".to_string(),
        );
        let researcher_component = create_json_component_as_string(
            "samples/json/EcdarUniversity/Components/Researcher.json".to_string(),
        );

        let components: Vec<String> = vec![
            administration_component,
            machine_component,
            researcher_component,
        ];
        let components = components
            .iter()
            .map(|string| Component {
                rep: Some(Rep::Json(string.clone())),
            })
            .collect();
        let simulation_info = SimulationInfo {
            component_composition: composition,
            components_info: Some(ComponentsInfo {
                components,
                components_hash: 0,
            }),
        };

        let simulation_start_request = Request::new(SimulationStartRequest {
            simulation_info: Some(simulation_info),
        });

        return simulation_start_request;
    }

    fn create_expected_response_to_composition_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        let expected = get_composition_response_Administration_Machine_Researcher();

        expected
    }

    // fn create_conjunction_request() -> Request<SimulationStartRequest> {
    //     todo!()
    // }

    // fn create_expected_response_to_conjunction_request() -> Result<Response<SimulationStepResponse>, Status> {
    //     todo!()
    // }

    // fn create_quotient_request() -> Request<SimulationStartRequest> {
    //     todo!()
    // }

    // fn create_expected_response_to_quotient_request() -> Result<Response<SimulationStepResponse>, Status> {
    //     todo!()
    // }
    // fn create_good_request() -> Request<SimulationStartRequest> {
    //     create_simulation_start_request(String::from("Machine"), create_sample_json_component())
    // }

    // fn create_expected_response_to_good_request() -> Result<Response<SimulationStepResponse>, Status>
    // {
    //     Ok(Response::new(SimulationStepResponse {
    //         new_decision_point: Some(create_initial_decision_point()),
    //     }))
    // }
}
