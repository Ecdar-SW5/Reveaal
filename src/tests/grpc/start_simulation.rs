#[cfg(test)]
mod test {
    use crate::{
        tests::{
            grpc::grpc_helper::{
                create_initial_decision_point, create_json_component_as_string,
                create_sample_json_component, create_simulation_start_request,
            },
            Simulation::helper::get_composition_response_Administration_Machine_Researcher,
        },
        ProtobufServer::{
            self,
            services::{
                component::Rep, ecdar_backend_server::EcdarBackend, Component, ComponentsInfo,
                SimulationInfo, SimulationStartRequest, SimulationStepResponse,
            },
        },
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
    // #[test_case(
    //     create_conjunction_request(),
    //     create_expected_response_to_conjunction_request();
    //     "given a good conjunction request, responds with correct component"
    // )]
    // #[test_case(
    //     create_quotient_request(),
    //     create_expected_response_to_quotient_request();
    //     "given a good quotient request, responds with correct component"
    // )]
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

    // Helpers
    fn create_composition_request() -> Request<SimulationStartRequest> {
        let composition = "(Administration || Machine || Researcher)".to_string();

        let administration_component = create_json_component_as_string(
            "samples/json/EcdarUniversity/Components/Administration.json".to_string(),
        );
        let machine_component = create_json_component_as_string(
            "samples/json/EcdarUniversity/Components/Machine.json".to_string(),
        );
        let researcher_component = create_json_component_as_string(
            "samples/json/EcdarUniversity/Components/Researcher.json".to_string(),
        );

        let components: Vec<String> = vec![
            administration_component,
            machine_component,
            researcher_component,
        ];
        let components = components
            .iter()
            .map(|string| Component {
                rep: Some(Rep::Json(string.clone())),
            })
            .collect();
        let simulation_info = SimulationInfo {
            component_composition: composition,
            components_info: Some(ComponentsInfo {
                components,
                components_hash: 0,
            }),
        };

        let simulation_start_request = Request::new(SimulationStartRequest {
            simulation_info: Some(simulation_info),
        });

        return simulation_start_request;
    }

    fn create_expected_response_to_composition_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        let expected = get_composition_response_Administration_Machine_Researcher();

        expected
    }

    // fn create_conjunction_request() -> Request<SimulationStartRequest> {
    //     todo!()
    // }

    // fn create_expected_response_to_conjunction_request() -> Result<Response<SimulationStepResponse>, Status> {
    //     todo!()
    // }

    // fn create_quotient_request() -> Request<SimulationStartRequest> {
    //     todo!()
    // }

    // fn create_expected_response_to_quotient_request() -> Result<Response<SimulationStepResponse>, Status> {
    //     todo!()
    // }
    // fn create_good_request() -> Request<SimulationStartRequest> {
    //     create_simulation_start_request(String::from("Machine"), create_sample_json_component())
    // }

    // fn create_expected_response_to_good_request() -> Result<Response<SimulationStepResponse>, Status>
    // {
    //     Ok(Response::new(SimulationStepResponse {
    //         new_decision_point: Some(create_initial_decision_point()),
    //     }))
    // }
}
