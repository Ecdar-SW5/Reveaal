#[cfg(test)]
mod refinements {
    use crate::ProtobufServer::{
        self,
        services::{
            component::Rep, ecdar_backend_server::EcdarBackend, state_tuple::LocationTuple,
            Component, SimulationStartRequest, SimulationStepResponse, StateTuple, DecisionPoint, State, Edge, state::Location, simulation_zone::DifferenceBound, SimulationZone,
        },
    };
    use tonic::{self, Request, Response};

    //static CONJUN: &str = "samples/xml/conjun.xml";
    static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

    #[tokio::test]
    async fn start_simulation__normal_json__respondes_correct_decision_points() {
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        let json =
            std::fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap();

        let request: Request<SimulationStartRequest> = Request::new(SimulationStartRequest {
            component: Some(Component {
                rep: Some(Rep::Json(json)),
            }),
        });

        let expected_response = Response::new(SimulationStepResponse {
            new_decision_points: vec![
                DecisionPoint {
                    source: Some(State {
                        location: Some(Location {
                            id: String::from("L5"),
                            component_name: String::from("Machine")
                        }),
                        zone: Some(SimulationZone {
                            dimensions: 2,
                            matrix: vec![
                                DifferenceBound {
                                    bound: 0,
                                    is_infinite: false
                                },
                                DifferenceBound {
                                    bound: 1,
                                    is_infinite: true
                                },
                                DifferenceBound {
                                    bound: 0,
                                    is_infinite: false
                                },
                                DifferenceBound {
                                    bound: 0,
                                    is_infinite: false
                                },
                            ]
                        })
                    }),
                    edges: vec![
                        Edge {
                            id: String::from("E3"),
                            component_name: String::from("Machine")
                        },
                        Edge {
                            id: String::from("E5"),
                            component_name: String::from("Machine")
                        }
                    ]
                }
            ] 
        });

        // Act
        let actual_response = backend.start_simulation(request).await.unwrap();

        // Assert
        assert_eq!(expected_response.into_inner(), actual_response.into_inner());
    }
}
