#[allow(unused_imports)]
use super::grpc_helper;
#[allow(unused_imports)]
use crate::ProtobufServer::services::SimulationStepResponse;
#[allow(unused_imports)]
use crate::ProtobufServer::{
    self,
    services::{self, ecdar_backend_server::EcdarBackend},
};
#[allow(unused_imports)]
use prost::encoding::message;
#[allow(unused_imports)]
use tonic;

#[tokio::test]
async fn take_simulation_step__normal__respondes_with_correct_state() {
    // Arrange
    let backend = ProtobufServer::ConcreteEcdarBackend::default();

    let initial_state = grpc_helper::create_initial_state();

    let expected_new_state = grpc_helper::create_state_after_taking_step();

    let expected_response = services::SimulationStepResponse {
        new_state: Some(expected_new_state),
    };

    // A request that Chooses the FAT EDGE:
    //
    //           ----coin? E3---->
    //          /
    // (L5,y>=0)=====tea! E5=====>
    //
    let chosen_source = initial_state.decision_points[0].source.clone().unwrap();
    let chosen_edge = initial_state.decision_points[0].edges[1].clone();
    let request = tonic::Request::new(grpc_helper::create_simulation_step_request(initial_state, chosen_source, chosen_edge));

    // Act
    let actual_response = backend
        .take_simulation_step(request)
        .await
        .unwrap()
        .into_inner();

    // Assert
    assert_eq!(actual_response, expected_response);
}

#[tokio::test]
async fn take_simulation_step__decision_not_in_decision_points__respondes_with_invalid_argument() {
    // Arrange
    let backend = ProtobufServer::ConcreteEcdarBackend::default();

    let initial_state = grpc_helper::create_initial_state();

    let chosen_source = initial_state.decision_points[0].source.clone().unwrap();
    // clearly "" not in {"E3", "E5"}
    let chosen_edge = services::Edge { id: "".to_string(), specific_component: None};
    let request = tonic::Request::new(
        grpc_helper::create_simulation_step_request(
            initial_state, 
            chosen_source, 
            chosen_edge
        )
    );

    let expected_response: Result<tonic::Response<SimulationStepResponse>, tonic::Status> = Err(
        tonic::Status::invalid_argument("Decision not in decision points"),
    );

    // Act
    let actual_response = backend.take_simulation_step(request).await;

    // Assert
    assert_eq!(
        actual_response.as_ref().err().unwrap().code(),
        expected_response.as_ref().err().unwrap().code()
    );

    assert_eq!(
        actual_response.err().unwrap().message(),
        expected_response.err().unwrap().message()
    );
}

#[tokio::test]
async fn take_simulation_step__decision_points_component_mismatch__respondes_with_invalid_argument()
{
    // Arrange
    let backend = ProtobufServer::ConcreteEcdarBackend::default();

    let mismatched_state = grpc_helper::create_sample_state_component_decision_mismatch();

    let chosen_source = mismatched_state.decision_points[0].source.clone().unwrap();
    let chosen_edge = mismatched_state.decision_points[0].edges[1].clone();
    let request = tonic::Request::new(
        grpc_helper::create_simulation_step_request(
            mismatched_state, 
            chosen_source, 
            chosen_edge
        )
    );

    let expected_response: Result<tonic::Response<SimulationStepResponse>, tonic::Status> = Err(tonic::Status::invalid_argument("Mismatch between decision points and component, please don't modify the simulation state"));

    // Act
    let actual_response = backend.take_simulation_step(request).await;

    // Assert
    assert_eq!(
        actual_response.as_ref().err().unwrap().code(),
        expected_response.as_ref().err().unwrap().code()
    );

    assert_eq!(
        actual_response.err().unwrap().message(),
        expected_response.err().unwrap().message()
    );
}
