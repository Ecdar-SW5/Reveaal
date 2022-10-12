#[cfg(test)]
mod test {
    use crate::{
        tests::grpc::grpc_helper::{create_initial_state, create_sample_json_component},
        ProtobufServer::{
            self,
            services::{
                component, ecdar_backend_server::EcdarBackend, Component, ComponentsInfo,
                SimulationStartRequest, SimulationStepResponse,
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
        create_malformed_request(),
        create_expected_response_to_malformed_request();
        "given a malformed request, respond with invalid argument"
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
        let component_json = create_sample_json_component();

        Request::new(SimulationStartRequest {
            component_composition: String::from("Machine"),
            components_info: Some(ComponentsInfo {
                components: vec![Component {
                    rep: Some(component::Rep::Json(component_json.clone())),
                }],
                components_hash: 0, // TODO: this is not correct, but will do for now
            }),
        })
    }

    fn create_expected_response_to_good_request() -> Result<Response<SimulationStepResponse>, Status>
    {
        Ok(Response::new(SimulationStepResponse {
            new_state: Some(create_initial_state()),
        }))
    }

    fn create_malformed_request() -> Request<SimulationStartRequest> {        let test_json: String = String::from("");

        tonic::Request::new(SimulationStartRequest {
            component_composition: String::from("Machine"),
            components_info: Some(ComponentsInfo {
                components: vec![Component {
                    rep: Some(component::Rep::Json(test_json.clone())),
                }],
                components_hash: 0, // TODO: this is not correct, but will do for now
            }),
        })
    }

    fn create_expected_response_to_malformed_request() -> Result<Response<SimulationStepResponse>, Status> {
        Err(tonic::Status::invalid_argument(
            "Malformed component",
        ))
    }
}