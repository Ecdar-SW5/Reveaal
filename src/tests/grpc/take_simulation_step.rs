#[cfg(test)]
mod test {
    use crate::tests::Simulation::test_data;
    use crate::ProtobufServer::services::{SimulationStepRequest, SimulationStepResponse};
    use crate::ProtobufServer::{self, services::ecdar_backend_server::EcdarBackend};
    use test_case::test_case;
    use tonic::{self, Request, Response, Status};

    #[test_case(
        test_data::create_good_request(),
        test_data::create_expected_response_to_good_request();
        "given a good request, responds with correct state"
    )]
    #[test_case(
        test_data::create_composition_request(),
        test_data::create_expected_response_to_composition_request();
        "given a composition request, responds with correct component"
    )]
    #[test_case(
        test_data::create_conjunction_request(),
        test_data::create_expected_response_to_conjunction_request();
        "given a good conjunction request, responds with correct component"
    )]
    #[tokio::test]
    async fn take_simulation_step__responds_as_expected(
        request: Request<SimulationStepRequest>,
        expected_response: Result<Response<SimulationStepResponse>, Status>,
    ) {
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        // Act
        let actual_response = backend.take_simulation_step(request).await;

        // Assert
        assert_eq!(
            format!("{:?}", expected_response),
            format!("{:?}", actual_response)
        );
    }

    #[ignore = "Server hangs on panic"]
    #[test_case(
        test_data::create_mismatched_request_1();
        "given a request with component decision mismatch, decision referencing source not in the set of states, responds with invalid argument"
    )]
    #[test_case(
        test_data::create_mismatched_request_2();
        "given a request with component decision mismatch, decision making transition that is not possible, responds with invalid argument"
    )]
    #[test_case(
        test_data::create_malformed_component_request();
        "given a request with a malformed component, responds with invalid argument"
    )]
    #[test_case(
        test_data::create_malformed_composition_request();
        "given a request with a malformed composition, responds with invalid argument"
    )]
    #[tokio::test]
    async fn start_simulation__bad_data__responds_with_error(
        request: Request<SimulationStepRequest>,
    ) {
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        // Act
        let actual_response = backend.take_simulation_step(request).await;

        // Assert
        assert!(actual_response.is_err());
    }
}
