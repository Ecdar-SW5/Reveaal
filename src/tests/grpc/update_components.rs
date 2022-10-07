#[cfg(test)]
mod refinements {
    use crate::ProtobufServer::services::component::Rep;
    use crate::ProtobufServer::services::ecdar_backend_server::EcdarBackend;

    use crate::ProtobufServer::services::Component;
    use crate::ProtobufServer::ConcreteEcdarBackend;
    use tonic::Request;

    static CONJUN: &str = "samples/xml/conjun.xml";
    static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

    #[tokio::test]
    async fn send_component() {
        // let backend = ConcreteEcdarBackend::default();
        // let json =
        //     std::fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap();
        // let request = Request::new(ComponentsUpdateRequest {
        //     components: vec![Component {
        //         rep: Some(Rep::Json(json)),
        //     }],
        //     etag: 0,
        // });

        // let response = backend.update_components(request).await;
        // assert!(response.is_ok());

        // let components = backend.components.lock().unwrap();
        // let component_count = components.borrow_mut().loaded_components.len();

        // assert_eq!(component_count, 1);
        assert!(false);
    }

    #[tokio::test]
    async fn send_two_components() {
        // let backend = ConcreteEcdarBackend::default();
        // let json1 =
        //     std::fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap();
        // let json2 = std::fs::read_to_string(format!("{}/Components/Adm2.json", ECDAR_UNI)).unwrap();
        // let request = Request::new(ComponentsUpdateRequest {
        //     components: vec![
        //         Component {
        //             rep: Some(Rep::Json(json1)),
        //         },
        //         Component {
        //             rep: Some(Rep::Json(json2)),
        //         },
        //     ],
        //     etag: 0,
        // });

        // let response = backend.update_components(request).await;
        // assert!(response.is_ok());

        // let components = backend.components.lock().unwrap();
        // let component_count = components.borrow_mut().loaded_components.len();

        // assert_eq!(component_count, 2);
        assert!(false);
    }

    #[tokio::test]
    async fn send_xml_components() {
        // let backend = ConcreteEcdarBackend::default();
        // let xml = std::fs::read_to_string(CONJUN).unwrap();
        // let request = Request::new(ComponentsUpdateRequest {
        //     components: vec![Component {
        //         rep: Some(Rep::Xml(xml)),
        //     }],
        //     etag: 0,
        // });

        // let response = backend.update_components(request).await;
        // assert!(response.is_ok());

        // let components = backend.components.lock().unwrap();
        // let component_count = components.borrow_mut().loaded_components.len();

        // assert_eq!(component_count, 14);
        assert!(false);
    }
}
