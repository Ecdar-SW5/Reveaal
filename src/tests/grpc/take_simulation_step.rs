#[cfg(test)]
mod test {
    use crate::tests::grpc::grpc_helper::{
        create_decision_point_after_taking_E5, create_edges_from_L5, create_empty_edge,
        create_empty_state, create_initial_decision_point, create_sample_json_component,
        create_simulation_info_from, create_simulation_step_request, create_state_not_in_machine,
        create_state_setup_for_mismatch,
    };
    use crate::ProtobufServer::services::{SimulationStepRequest, SimulationStepResponse};
    use crate::ProtobufServer::{self, services::ecdar_backend_server::EcdarBackend};
    use test_case::test_case;
    use tonic::{self, Request, Response, Status};

    #[ignore]
    #[test_case(
        create_good_request(),
        create_expected_response_to_good_request();
        "given a good request, responds with correct state"
    )]
    #[test_case(
        create_mismatched_request_1(),
        create_expected_response_to_mismatched_request_1();
        "given a request with component decision mismatch, decision referencing source not in the set of states, responds with invalid argument"
    )]
    #[test_case(
        create_mismatched_request_2(),
        create_expected_response_to_mismatched_request_2();
        "given a request with component decision mismatch, decision making transition that is not possible, responds with invalid argument"
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
        let simulation_info =
            create_simulation_info_from(String::from("Machine"), create_sample_json_component());
        let initial_decision_point = create_initial_decision_point();
        let chosen_source = initial_decision_point.source.clone().unwrap();
        let chosen_edge = initial_decision_point.edges[1].clone();

        tonic::Request::new(create_simulation_step_request(
            simulation_info,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_expected_response_to_good_request() -> Result<Response<SimulationStepResponse>, Status>
    {
        Ok(Response::new(SimulationStepResponse {
            new_decision_point: Some(create_decision_point_after_taking_E5()),
        }))
    }

    fn create_mismatched_request_1() -> Request<SimulationStepRequest> {
        let simulation_info =
            create_simulation_info_from(String::from("Machine"), create_sample_json_component());
        let chosen_source = create_state_not_in_machine();
        let chosen_edge = create_edges_from_L5()[0].clone();

        tonic::Request::new(create_simulation_step_request(
            simulation_info,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_expected_response_to_mismatched_request_1(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(tonic::Status::invalid_argument(
            "Mismatch between decision and system, state not in system",
        ))
    }

    fn create_mismatched_request_2() -> Request<SimulationStepRequest> {
        let simulation_info =
            create_simulation_info_from(String::from("Machine"), create_sample_json_component());

        let chosen_source = create_state_setup_for_mismatch();
        let chosen_edge = create_edges_from_L5()[1].clone(); // Should not be able to choose this edge
        Request::new(create_simulation_step_request(
            simulation_info,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_expected_response_to_mismatched_request_2(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(tonic::Status::invalid_argument(
            "Mismatch between decision and system, could not make transition",
        ))
    }

    fn create_malformed_component_request() -> Request<SimulationStepRequest> {
        let simulation_info = create_simulation_info_from(String::from(""), String::from(""));
        let chosen_source = create_empty_state();
        let chosen_edge = create_empty_edge();

        Request::new(create_simulation_step_request(
            simulation_info,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_response_to_malformed_component_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(Status::invalid_argument("Malformed component, bad json"))
    }

    fn create_malformed_composition_request() -> Request<SimulationStepRequest> {
        let simulation_info =
            create_simulation_info_from(String::from(""), create_sample_json_component());
        let chosen_source = create_empty_state();
        let chosen_edge = create_empty_edge();

        Request::new(create_simulation_step_request(
            simulation_info,
            chosen_source,
            chosen_edge,
        ))
    }

    fn create_response_to_malformed_composition_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        Err(Status::invalid_argument(
            "Malformed composition, bad expression",
        ))
    }
}
