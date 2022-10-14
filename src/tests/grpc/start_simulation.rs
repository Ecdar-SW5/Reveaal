#[cfg(test)]
mod test {
    use crate::{
        tests::grpc::grpc_helper::{
            create_sample_json_component, create_simulation_start_request, create_initial_decision_point,
        },
        ProtobufServer::{
            self,
            services::{
                ecdar_backend_server::EcdarBackend, SimulationStartRequest, SimulationStepResponse,
            },
        },
    };
    use test_case::test_case;
    use tonic::{Request, Response, Status};

    #[test_case(
        create_good_request(),
        create_expected_response_to_good_request();
        "given a good request, responds with correct state"
    )]
    #[test_case(
        create_malformed_component_request(),
        create_expected_response_to_malformed_component_request();
        "given a request with a malformed component, respond with invalid argument"
    )]
    #[test_case(
        create_malformed_composition_request(),
        create_expected_response_to_malformed_composition_request();
        "given a request with a malformed composition, respond with invalid argument"
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

    fn create_good_request() -> Request<SimulationStartRequest> {
        create_simulation_start_request(String::from("Machine"), create_sample_json_component())
    }

    fn create_expected_response_to_good_request() -> Result<Response<SimulationStepResponse>, Status>
    {
        Ok(Response::new(SimulationStepResponse {
            new_decision_point: Some(create_initial_decision_point())
        }))
    }

    fn create_malformed_component_request() -> Request<SimulationStartRequest> {
        create_simulation_start_request(String::from(""), String::from(""))
    }

    fn create_expected_response_to_malformed_component_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(tonic::Status::invalid_argument("Malformed component"))
    }

    fn create_malformed_composition_request() -> Request<SimulationStartRequest> {
        create_simulation_start_request(String::from(""), create_sample_json_component())
    }

    fn create_expected_response_to_malformed_composition_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(tonic::Status::invalid_argument("Malformed composition"))
    }
}
