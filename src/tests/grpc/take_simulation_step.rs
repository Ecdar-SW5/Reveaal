#[cfg(test)]
mod take_simulation_step {
    use crate::tests::grpc::grpc_helper;
    use crate::ProtobufServer::{
        self,
        services::{self, ecdar_backend_server::EcdarBackend},
    };
    use std::fs;
    use tonic;

    //static CONJUN: &str = "samples/xml/conjun.xml";
    static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

    #[tokio::test]
    async fn take_simulation_step__normal__respondes_with_correct_state() {
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        let old_simulation_state = grpc_helper::create_state_after_start_simulation();

        let expected_new_simulation_state = old_simulation_state.clone();
        expected_new_simulation_state.decision_points.push();

        // Act
        let actual_response = backend
            .take_simulation_step(request)
            .await
            .unwrap()
            .into_inner();
        // Assert
        assert_eq!(actual_response, expected_response);
    }
}
