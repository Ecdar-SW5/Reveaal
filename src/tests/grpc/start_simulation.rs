#[cfg(test)]
mod test {
    use crate::{
        tests::{
            grpc::grpc_helper::{
                create_initial_decision_point, create_json_component_as_string,
                create_sample_json_component, create_simulation_start_request,
            },
            Simulation::helper,
        },
        ProtobufServer::{
            self,
            services::{
                ecdar_backend_server::EcdarBackend, Component, ComponentsInfo,
                SimulationStartRequest, SimulationStepResponse,
            },
        },
        TransitionSystems::CompositionType,
    };
    use test_case::test_case;
    use tonic::{Request, Response, Status};

    #[ignore]
    #[test_case(
        create_good_request(),
        create_expected_response_to_good_request();
        "given a good request, responds with correct state"
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

    #[ignore]
    #[test_case(
        create_malformed_component_request();
        "given a request with a malformed component, respond with error"
    )]
    #[test_case(
        create_malformed_composition_request();
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

    fn create_good_request() -> Request<SimulationStartRequest> {
        create_simulation_start_request(String::from("Machine"), create_sample_json_component())
    }

    fn create_expected_response_to_good_request() -> Result<Response<SimulationStepResponse>, Status>
    {
        Ok(Response::new(SimulationStepResponse {
            new_decision_point: Some(create_initial_decision_point()),
        }))
    }

    fn create_malformed_component_request() -> Request<SimulationStartRequest> {
        create_simulation_start_request(String::from(""), String::from(""))
    }

    // fn create_expected_response_to_malformed_component_request(
    // ) -> Result<Response<SimulationStepResponse>, Status> {
    //     Err(tonic::Status::invalid_argument(
    //         "Malformed component, bad json",
    //     ))
    // }

    fn create_malformed_composition_request() -> Request<SimulationStartRequest> {
        create_simulation_start_request(String::from(""), create_sample_json_component())
    }

    // fn create_expected_response_to_malformed_composition_request(
    // ) -> Result<Response<SimulationStepResponse>, Status> {
    //     Err(tonic::Status::invalid_argument(
    //         "Malformed composition, bad expression",
    //     ))
    // }

    #[test_case(
        create_composition_request(),
        create_expected_response_to_composition_request();
        "given a composition request, responds with correct component"
    )]
    #[test_case(
        create_conjunction_request(),
        create_expected_response_to_conjunction_request();
        "given a good conjunction request, responds with correct component"
    )]
    #[tokio::test]
    async fn start_simulation_step__get_composit_component__should_return_component(
        request: Request<SimulationStartRequest>,
        expected_response: Result<Response<SimulationStepResponse>, Status>,
    ) {
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        let actual_response = backend.start_simulation(request).await;

        // Assert
        assert_eq!(
            format!("{:?}", expected_response),
            format!("{:?}", actual_response)
        );
    }

    // A || B || C
    fn create_composition_request() -> Request<SimulationStartRequest> {
        let comp_names = vec!["Administration", "Machine", "Researcher"];
        let sample_name = "EcdarUniversity".to_string();

        let composition =
            helper::create_composition_string(&comp_names, CompositionType::Composition);
        let components: Vec<Component> = helper::create_components(&comp_names, sample_name);

        let simulation_info = helper::create_simulation_info(composition, components);

        let simulation_start_request = Request::new(SimulationStartRequest {
            simulation_info: Some(simulation_info),
        });

        return simulation_start_request;
    }

    fn create_expected_response_to_composition_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        let expected = helper::get_composition_response_Administration_Machine_Researcher();

        expected
    }

    // A && B
    fn create_conjunction_request() -> Request<SimulationStartRequest> {
        let comp_names = vec!["HalfAdm1", "HalfAdm2"];
        let sample_name = "EcdarUniversity".to_string();
        let composition_string =
            helper::create_composition_string(&comp_names, CompositionType::Conjunction);

        let components: Vec<Component> = helper::create_components(&comp_names, sample_name);
        let simulation_info = helper::create_simulation_info(composition_string, components);

        let simulation_start_request = Request::new(SimulationStartRequest {
            simulation_info: Some(simulation_info),
        });

        simulation_start_request
    }

    fn create_expected_response_to_conjunction_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        let expected = helper::get_conjunction_response_HalfAdm1_HalfAdm2();

        expected
    }
}
