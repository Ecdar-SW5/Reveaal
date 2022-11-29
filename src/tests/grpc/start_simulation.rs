#[cfg(test)]
mod test {
    use crate::{
        tests::Simulation::{
            helper,
            test_data::{
                get_composition_response_Administration_Machine_Researcher,
                get_conjunction_response_HalfAdm1_HalfAdm2,
            },
        },
        ProtobufServer::{
            self,
            services::{
                ecdar_backend_server::EcdarBackend, Component, SimulationStartRequest,
                SimulationStepResponse,
            },
        },
        TransitionSystems::CompositionType,
    };
    use test_case::test_case;
    use tonic::{Request, Response, Status};

    #[test_case(
        crate::tests::Simulation::test_data::create_good_start_request(),
        crate::tests::Simulation::test_data::create_expected_response_to_good_start_request();
        "given a good request, responds with correct state"
    )]
    #[test_case(
        crate::tests::Simulation::test_data::create_composition_start_request(),
        crate::tests::Simulation::test_data::create_expected_response_to_composition_start_request();
        "given a composition request, responds with correct component"
    )]
    #[test_case(
        crate::tests::Simulation::test_data::create_conjunction_start_request(),
        crate::tests::Simulation::test_data::create_expected_response_to_conjunction_start_request();
        "given a good conjunction request, responds with correct component"
    )]
    #[tokio::test]
    async fn start_simulation__responds_as_expected(
        request: Request<SimulationStartRequest>,
        expected_response: Result<Response<SimulationStepResponse>, Status>,
    ) {
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        // Act
        let actual_response = backend.start_simulation(request).await;

        // Assert
        assert_eq!(
            format!("{:?}", expected_response),
            format!("{:?}", actual_response)
        );
    }

    #[ignore = "Server hangs on panic"]
    #[test_case(
        crate::tests::Simulation::test_data::create_malformed_component_start_request();
        "given a request with a malformed component, respond with error"
    )]
    #[test_case(
        crate::tests::Simulation::test_data::create_malformed_composition_start_request();
        "given a request with a malformed composition, respond with error"
    )]
    #[tokio::test]
    async fn start_simulation__bad_data__responds_with_error(
        request: Request<SimulationStartRequest>,
    ) {
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        // Act
        let actual_response = backend.start_simulation(request).await;

        // Assert
        assert!(actual_response.is_err());
    }
}
