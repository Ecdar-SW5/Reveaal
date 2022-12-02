#[cfg(test)]
mod refinements {
    use crate::ProtobufServer::services::component::Rep;
    use crate::ProtobufServer::services::ecdar_backend_server::EcdarBackend;
    use crate::ProtobufServer::services::query_response;
    use crate::ProtobufServer::services::Component;
    use crate::ProtobufServer::services::ComponentsInfo;
    use crate::ProtobufServer::services::QueryRequest;
    use crate::ProtobufServer::ConcreteEcdarBackend;
    use crate::TEST_SETTINGS;
    use tonic::Request;

    //static CONJUN: &str = "samples/xml/conjun.xml";
    static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

    #[tokio::test]
    async fn send_self_refinement_query() {
        let backend = ConcreteEcdarBackend::default();
        let query_request = create_query_request("refinement: Machine <= Machine");

        let query_response = backend.send_query(query_request).await;
        assert!(query_response.is_ok());

        let query_result = query_response.unwrap().into_inner();
        let result = query_result.result.unwrap();
        match result {
            query_response::Result::Refinement(refine) => assert!(refine.success),
            _ => panic!(),
        }
    }

    #[tokio::test]
    async fn send_consistency_query() {
        let backend = ConcreteEcdarBackend::default();
        let query_request = create_query_request("consistency: Machine");

        let query_response = backend.send_query(query_request).await;
        assert!(query_response.is_ok());

        let query_result = query_response.unwrap().into_inner();
        let result = query_result.result.unwrap();
        match result {
            query_response::Result::Consistency(consistent) => assert!(consistent.success),
            _ => panic!(),
        }
    }

    #[tokio::test]
    async fn send_determinism_query() {
        let backend = ConcreteEcdarBackend::default();
        let query_request = create_query_request("determinism: Machine");

        let query_response = backend.send_query(query_request).await;
        assert!(query_response.is_ok());

        let query_result = query_response.unwrap().into_inner();
        let result = query_result.result.unwrap();
        match result {
            query_response::Result::Determinism(determinism) => assert!(determinism.success),
            _ => panic!(),
        }
    }

    fn create_query_request(query: &str) -> Request<QueryRequest> {
        let json =
            std::fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap();

        Request::new(QueryRequest {
            user_id: 0,
            query_id: 0,
            query: String::from(query),
            components_info: Some(ComponentsInfo {
                components: vec![Component {
                    rep: Some(Rep::Json(json)),
                }],
                components_hash: 0,
            }),
            ignored_input_outputs: None,
            settings: Some(TEST_SETTINGS),
        })
    }
}
