#[allow(unused_imports)]
use crate::tests::grpc::grpc_helper;
#[allow(unused_imports)]
use crate::ProtobufServer::{
    self,
    services::{self, ecdar_backend_server::EcdarBackend},
};

#[tokio::test]
async fn start_simulation__normal_json__respondes_with_correct_state() {
    // Arrange
    let backend = ProtobufServer::ConcreteEcdarBackend::default();

    let component_json = grpc_helper::create_sample_json_component();

    let request =
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
