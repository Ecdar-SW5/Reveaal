#[cfg(test)]
mod refinements {
    use crate::ProtobufServer::{
        self,
        services::{
            component::Rep, ecdar_backend_server::EcdarBackend,
            Component, SimulationStartRequest, SimulationStepResponse, DecisionPoint, State, Edge, state::Location, zone::DifferenceBound, Zone,
        },
    };
    use tonic::{self, Request};

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

        let expected_response = SimulationStepResponse {
            new_decision_points: vec![
                DecisionPoint {
                    source: Some(State {
                        location: Some(Location {
                            id: String::from("L5"),
                            component_name: String::from("Machine")
                        }),
                        zone: Some(Zone {
                            dimensions: 2,
                            matrix: vec![
                                DifferenceBound {
                                    bound: 0,
                                    is_infinite: false,
                                    is_strict: false,
                                },
                                DifferenceBound {
                                    bound: 1,
                                    is_infinite: true,
                                    is_strict: true
                                },
                                DifferenceBound {
                                    bound: 0,
                                    is_infinite: false,
                                    is_strict: false
                                },
                                DifferenceBound {
                                    bound: 0,
                                    is_infinite: false,
                                    is_strict: false
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
        };

        // Act
        let actual_response = 
            backend.start_simulation(request).await
                                             .unwrap()
                                             .into_inner();
        // Assert
        assert_eq!(actual_response, expected_response);
    }

}
