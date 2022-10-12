#[cfg(test)]
mod test {
    use crate::tests::grpc::grpc_helper::{
        create_initial_state, create_sample_json_component,
        create_sample_state_component_decision_mismatch_1,
        create_sample_state_component_decision_mismatch_2, create_simulation_step_request,
        create_state_after_taking_step,
    };
    use crate::ProtobufServer::services::{
        self, Component, ComponentsInfo, SimulationStepRequest, SimulationStepResponse,
    };
    use crate::ProtobufServer::{self, services::ecdar_backend_server::EcdarBackend};
    use test_case::test_case;
    use tonic::{self, Request, Response, Status};

    #[test_case(
        create_good_request(),
        create_expected_response_to_good_request();
        "given a good request, responds with correct state"
    )]
    #[test_case(
        create_decision_not_in_decision_points_request(),
        create_expected_response_to_decision_not_in_decision_points_request();
        "given a request where decision not in decision points, responds with invalid argument"
    )]
    #[test_case(
        create_mismatched_request_1(),
        create_expected_response_to_mismatched_request_1();
        "given a request with component decision mismatch, decision referencing nonexistent location in component, responds with invalid argument"
    )]
    #[test_case(
        create_mismatched_request_2(),
        create_expected_response_to_mismatched_request_2();
        "given a request with component decision mismatch, decision taking edge that is not possible, responds with invalid argument"
    )]
    #[test_case(
        create_malformed_component_request(),
        create_response_to_malformed_component_request();
        "given a request with a malformed component, responds with invalid argument"
    )]
    #[test_case(
        create_malformed_composition_request(),
        create_response_to_malformed_composition_request();
        "given a request with a malformed composition, responds with invalid argument"
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

    // A request that Chooses the FAT EDGE:
    //
    //           ----coin? E3---->
    //          /
    // (L5,y>=0)=====tea! E5=====>
    //
    fn create_good_request() -> tonic::Request<SimulationStepRequest> {
        let current_state = create_initial_state();
        let chosen_source = current_state.decision_points[0].source.clone().unwrap();
        let chosen_edge = current_state.decision_points[0].edges[1].clone();
        tonic::Request::new(create_simulation_step_request(
            current_state,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_expected_response_to_good_request() -> Result<Response<SimulationStepResponse>, Status>
    {
        Ok(Response::new(SimulationStepResponse {
            new_state: Some(create_state_after_taking_step()),
        }))
    }

    fn create_decision_not_in_decision_points_request() -> Request<SimulationStepRequest> {
        let current_state = create_initial_state();

        let chosen_source = current_state.decision_points[0].source.clone().unwrap();
        // clearly "" is not in {"E3", "E5"}
        let chosen_edge = services::Edge {
            id: "".to_string(),
            specific_component: None,
        };

        Request::new(create_simulation_step_request(
            current_state,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_expected_response_to_decision_not_in_decision_points_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(tonic::Status::invalid_argument(
            "Decision not present in decision points",
        ))
    }

    fn create_mismatched_request_1() -> Request<SimulationStepRequest> {
        let mismatched_state = create_sample_state_component_decision_mismatch_1();
        let chosen_source = mismatched_state.decision_points[0].source.clone().unwrap();
        let chosen_edge = mismatched_state.decision_points[0].edges[1].clone();
        tonic::Request::new(create_simulation_step_request(
            mismatched_state,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_expected_response_to_mismatched_request_1(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(tonic::Status::invalid_argument("Mismatch between decision points and component, please don't modify the simulation state"))
    }

    fn create_mismatched_request_2() -> Request<SimulationStepRequest> {
        let current_state = create_sample_state_component_decision_mismatch_2();

        let chosen_source = current_state.decision_points[0].source.clone().unwrap();
        let chosen_edge = current_state.decision_points[0].edges[1].clone();
        Request::new(create_simulation_step_request(
            current_state,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_expected_response_to_mismatched_request_2(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(tonic::Status::invalid_argument("Mismatch between decision points and component, please don't modify the simulation state"))
    }

    fn create_malformed_component_request() -> Request<SimulationStepRequest> {
        let current_state = services::SimulationState {
            component_composition: String::from(""),
            components_info: Some(ComponentsInfo {
                components: vec![Component {
                    rep: Some(services::component::Rep::Json(String::from(""))),
                }],
                components_hash: 0, // TODO this is incorrect
            }),
            decision_points: vec![],
        };

        let chosen_source = services::State {
            location_tuple: None,
            zone: None,
        };
        let chosen_edge = services::Edge {
            id: "".to_string(),
            specific_component: None,
        };
        Request::new(create_simulation_step_request(
            current_state,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_response_to_malformed_component_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(Status::invalid_argument(
            "Malformed component, please don't modify the simulation state",
        ))
    }

    fn create_malformed_composition_request() -> Request<SimulationStepRequest> {
        let current_state = services::SimulationState {
            component_composition: String::from(""),
            components_info: Some(ComponentsInfo {
                components: vec![Component {
                    rep: Some(services::component::Rep::Json(
                        create_sample_json_component(),
                    )),
                }],
                components_hash: 0, // TODO this is incorrect
            }),
            decision_points: vec![],
        };

        let chosen_source = services::State {
            location_tuple: None,
            zone: None,
        };
        let chosen_edge = services::Edge {
            id: "".to_string(),
            specific_component: None,
        };
        Request::new(create_simulation_step_request(
            current_state,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_response_to_malformed_composition_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(Status::invalid_argument(
            "Malformed composition, please don't modify the simulation state",
        ))
    }
}
