#[cfg(test)]
mod refinements {
    use crate::ProtobufServer::{self, services::{self, ecdar_backend_server::EcdarBackend}};
    use std::fs;
    use tonic;

    //static CONJUN: &str = "samples/xml/conjun.xml";
    static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

    #[tokio::test]
    async fn start_simulation__normal_json__respondes_correct_decision_points() {
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        let component_json =
            fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap();

        let request: tonic::Request<services::SimulationStartRequest> = tonic::Request::new(
            services::SimulationStartRequest {
                component_composition: String::from("Machine"),
                components_info: Some(services::ComponentsInfo {
                    components: vec![
                        services::Component {
                            rep: Some(
                                services::component::Rep::Json(component_json.clone())
                            )
                        }
                    ],
                    components_hash: 0 // TODO: this is not correct, but will do for now
                })
            }
        );

        // the expected respones is the same component (no composition takes place), and the
        // decision point drawn below: 
        //
        //          ----coin?---->
        //         /
        // <L5,Ø>-------tea!----->
        //
        //
        let expected_response = services::SimulationStepResponse {
            new_state: Some(services::SimulationState {
                component: Some(services::Component {
                    rep: Some(services::component::Rep::Json(component_json.clone()))
                }),
                decision_points: vec![
                    services::DecisionPoint {
                        source: Some(services::State {
                            location_id: "L5".to_string(),
                            zone: Some(services::Zone {
                                disjunction: Some(services::Disjunction {
                                    conjunctions: vec![
                                        services::Conjunction {
                                            constraints: vec![]
                                        }
                                    ]
                                })
                            })
                        }),
                        edges: vec![
                            services::Edge {
                                id: "E3".to_string(),
                                specific_component: None
                            },
                            services::Edge {
                                id: "E5".to_string(),
                                specific_component: None
                            }
                        ]
                    }
                ]
            })
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
