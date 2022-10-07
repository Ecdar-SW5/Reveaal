#[cfg(test)]
mod refinements {
    use crate::ProtobufServer::services::component::Rep;
    use crate::ProtobufServer::services::ecdar_backend_server::EcdarBackend;
    use crate::ProtobufServer::services::query_response;
    use crate::ProtobufServer::services::Component;
    use crate::ProtobufServer::ConcreteEcdarBackend;
    use tonic::Request;

    //static CONJUN: &str = "samples/xml/conjun.xml";
    static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

    #[tokio::test]
    async fn send_query_fails_with_multiple_queries() {
        // let backend = ConcreteEcdarBackend::default();
        // let request = Request::new(Query {
        //     id: 0,
        //     query: String::from("refinement: A <= A; refinement: B <= B"),
        //     ignored_input_outputs: None,
        // });

        // let response = backend.send_query(request).await;

        // assert!(response.is_err());
        assert!(false);
    }

    #[tokio::test]
    async fn send_query_fails_with_no_queries() {
        // let backend = ConcreteEcdarBackend::default();
        // let request = Request::new(Query {
        //     id: 0,
        //     query: String::from(""),
        //     ignored_input_outputs: None,
        // });

        // let response = backend.send_query(request).await;

        // assert!(response.is_err());
        assert!(false);
    }

    #[tokio::test]
    async fn send_empty_update_components() {
        // let backend = ConcreteEcdarBackend::default();
        // let request = Request::new(ComponentsUpdateRequest {
        //     components: vec![],
        //     etag: 0,
        // });

        // let response = backend.update_components(request).await;
        // assert!(response.is_ok());
        assert!(false);
    }

    #[tokio::test]
    async fn send_self_refinement_query() {
        // let backend = ConcreteEcdarBackend::default();
        // let json =
        //     std::fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap();
        // let update_request = Request::new(ComponentsUpdateRequest {
        //     components: vec![Component {
        //         rep: Some(Rep::Json(json)),
        //     }],
        //     etag: 0,
        // });

        // let update_response = backend.update_components(update_request).await;
        // assert!(update_response.is_ok());

        // let query = String::from("refinement: Machine <= Machine");
        // let query_request = Request::new(Query {
        //     id: 0,
        //     query,
        //     ignored_input_outputs: None,
        // });

        // let query_response = backend.send_query(query_request).await;
        // assert!(query_response.is_ok());

        // let query_result = query_response.unwrap().into_inner();

        // if let Some(result) = query_result.result {
        //     match result {
        //         query_response::Result::Refinement(refine) => assert!(refine.success),
        //         _ => panic!(),
        //     }
        // } else {
        //     panic!();
        // }
        assert!(false)
    }

    #[tokio::test]
    async fn send_consistency_query() {
        // let backend = ConcreteEcdarBackend::default();
        // let json =
        //     std::fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap();
        // let update_request = Request::new(ComponentsUpdateRequest {
        //     components: vec![Component {
        //         rep: Some(Rep::Json(json)),
        //     }],
        //     etag: 0,
        // });

        // let update_response = backend.update_components(update_request).await;
        // assert!(update_response.is_ok());

        // let query = String::from("consistency: Machine");
        // let query_request = Request::new(Query {
        //     id: 0,
        //     query,
        //     ignored_input_outputs: None,
        // });

        // let query_response = backend.send_query(query_request).await;
        // assert!(query_response.is_ok());

        // let query_result = query_response.unwrap().into_inner();

        // if let Some(result) = query_result.result {
        //     match result {
        //         query_response::Result::Consistency(consistent) => assert!(consistent.success),
        //         _ => panic!(),
        //     }
        // } else {
        //     panic!();
        // }
        assert!(false)
    }

    #[tokio::test]
    async fn send_determinism_query() {
        // let backend = ConcreteEcdarBackend::default();
        // let json =
        //     std::fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap();
        // let update_request = Request::new(ComponentsUpdateRequest {
        //     components: vec![Component {
        //         rep: Some(Rep::Json(json)),
        //     }],
        //     etag: 0,
        // });

        // let update_response = backend.update_components(update_request).await;
        // assert!(update_response.is_ok());

        // let query = String::from("determinism: Machine");
        // let query_request = Request::new(Query {
        //     id: 0,
        //     query,
        //     ignored_input_outputs: None,
        // });

        // let query_response = backend.send_query(query_request).await;
        // assert!(query_response.is_ok());

        // let query_result = query_response.unwrap().into_inner();

        // if let Some(result) = query_result.result {
        //     match result {
        //         query_response::Result::Determinism(determinsm) => assert!(determinsm.success),
        //         _ => panic!(),
        //     }
        // } else {
        //     panic!();
        // }
        assert!(false)
    }
}
