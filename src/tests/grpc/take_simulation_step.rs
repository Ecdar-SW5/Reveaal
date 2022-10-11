#[allow(unused_imports)]
use crate::ProtobufServer::{self, services::{self, ecdar_backend_server::EcdarBackend}};
#[allow(unused_imports)]
use super::grpc_helper;

#[tokio::test]
async fn take_simulation_step__normal__respondes_with_correct_state() {
    // Arrange
    let backend = ProtobufServer::ConcreteEcdarBackend::default();

    let old_state = grpc_helper::create_sample_state_1();

    let expected_new_state = grpc_helper::create_sample_state_2();

    let expected_response = services::SimulationStepResponse {
        new_state: Some(expected_new_state)
    };

    // A request that Chooses the FAT EDGE:
    //
    //             ----coin?---->
    //            /
    // <L5,y>=0>=======TEA!=====>
    //
    //
    let request = tonic::Request::new(
services::SimulationStepRequest {
            current_state: Some(old_state.clone()), 
            chosen_decision: Some(services::Decision { 
                source: old_state.decision_points[0].source.clone(), 
                edge: Some(old_state.decision_points[0].edges[1].clone())
            })
        } 
    );

    // Act
    let actual_response = backend
        .take_simulation_step(request)
        .await
        .unwrap()
        .into_inner();

    
    // Assert
    assert_eq!(actual_response, expected_response);
}
