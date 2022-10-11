#[cfg(test)]
mod start_simulation {
    use std::result;

    use crate::tests::grpc::grpc_helper;
    use crate::ProtobufServer::{
        self,
        services::{self, ecdar_backend_server::EcdarBackend},
    };
    use tonic;

    #[tokio::test]
    async fn start_simulation__normal_json__respondes_with_correct_state() {
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        let component_json = grpc_helper::create_sample_json_component();

        let request: tonic::Request<services::SimulationStartRequest> =
            tonic::Request::new(services::SimulationStartRequest {
                component_composition: String::from("Machine"),
                components_info: Some(services::ComponentsInfo {
                    components: vec![services::Component {
                        rep: Some(services::component::Rep::Json(component_json.clone())),
                    }],
                    components_hash: 0, // TODO: this is not correct, but will do for now
                }),
            });

        let expected_response = services::SimulationStepResponse {
            new_state: Some(grpc_helper::create_sample_state_1()),
        };

        // Act
        let actual_response = backend
            .start_simulation(request)
            .await
            .unwrap()
            .into_inner();
        // Assert
        assert_eq!(actual_response, expected_response);
    }

    #[tokio::test]
    async fn start_simulation_bad_json_responds_with_error() {
        
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();
        let test_json: String = String::from("this is an invalid argument");

        let request: tonic::Request<services::SimulationStartRequest> =
        tonic::Request::new(services::SimulationStartRequest {
            component_composition: String::from("Machine"),
            components_info: Some(services::ComponentsInfo {
                components: vec![services::Component {
                    rep: Some(services::component::Rep::Json(test_json.clone())),
                }],
                components_hash: 0, // TODO: this is not correct, but will do for now
            }),
        });

        let expected_response = tonic::Status::invalid_argument("Bad JSON");

        // Act
        let actual_response = backend
            .start_simulation(request)
            .await
            .err()
            .unwrap()
            .code();

        // Assert
        
        assert_eq!(
            actual_response,
            expected_response.code()
        )
    }
}
