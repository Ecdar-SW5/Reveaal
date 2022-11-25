#[cfg(test)]
mod test {
    use crate::tests::grpc::grpc_helper::{
        create_decision_point_after_taking_E5, create_edges_from_L5, create_empty_edge,
        create_empty_state, create_initial_decision_point, create_sample_json_component,
        create_simulation_info_from, create_simulation_step_request, create_state_not_in_machine,
        create_state_setup_for_mismatch,
    };
    use crate::tests::Simulation::helper::{
        self, get_state_after_Administration_Machine_Researcher_composition,
        get_state_after_HalfAdm1_HalfAdm2_conjunction,
    };
    use crate::ProtobufServer::services::{
        Component as ProtoComponent, Edge as ProtoEdge, SimulationStepRequest,
        SimulationStepResponse,
    };
    use crate::ProtobufServer::{self, services::ecdar_backend_server::EcdarBackend};
    use crate::TransitionSystems::CompositionType;
    use test_case::test_case;
    use tonic::{self, Request, Response, Status};

    #[ignore = "Server hangs on panic"]
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
            new_decision_points: vec![create_decision_point_after_taking_E5()],
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
    async fn take_simulation_step__get_composit_component__should_return_component(
        request: Request<SimulationStepRequest>,
        expected_response: Result<Response<SimulationStepResponse>, Status>,
    ) {
        let backend = ProtobufServer::ConcreteEcdarBackend::default();

        let actual_response = backend.take_simulation_step(request).await;

        // Assert
        assert_eq!(
            format!("{:?}", expected_response),
            format!("{:?}", actual_response)
        );
    }

    // A || B || C
    fn create_composition_request() -> Request<SimulationStepRequest> {
        let comp_names = vec!["Administration", "Machine", "Researcher"];
        let sample_name = "EcdarUniversity".to_string();
        let composition_string = "Administration || Machine || Researcher".to_string();

        let components: Vec<ProtoComponent> = helper::create_components(&comp_names, sample_name);
        let simulation_info = helper::create_simulation_info(composition_string, components);

        let edge = ProtoEdge {
            id: "E29".to_string(),
            specific_component: None,
        };

        let source = get_state_after_Administration_Machine_Researcher_composition();

        let simulation_step_request = create_simulation_step_request(simulation_info, source, edge);

        Request::new(simulation_step_request)
    }

    fn create_expected_response_to_composition_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        helper::get_composition_response_Administration_Machine_Researcher_after_E29()
    }

    // A && B
    fn create_conjunction_request() -> Request<SimulationStepRequest> {
        let comp_names = vec!["HalfAdm1", "HalfAdm2"];
        let sample_name = "EcdarUniversity".to_string();
        let composition_string = "HalfAdm1 && HalfAdm2".to_string();
        helper::create_composition_string(&comp_names, CompositionType::Conjunction);

        let components: Vec<ProtoComponent> = helper::create_components(&comp_names, sample_name);
        let simulation_info = helper::create_simulation_info(composition_string, components);

        let edge = ProtoEdge {
            id: "E37".to_string(),
            specific_component: None,
        };

        let source = get_state_after_HalfAdm1_HalfAdm2_conjunction();

        let simulation_step_request = create_simulation_step_request(simulation_info, source, edge);

        Request::new(simulation_step_request)
    }

    fn create_expected_response_to_conjunction_request(
    ) -> Result<Response<SimulationStepResponse>, Status> {
        helper::get_conjunction_response_HalfAdm1_HalfAdm2_after_E37()
    }
}
