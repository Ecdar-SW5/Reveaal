#[cfg(test)]
mod refinements {
    use crate::ProtobufServer;
    use tonic;

    //static CONJUN: &str = "samples/xml/conjun.xml";
    static ECDAR_UNI: &str = "samples/json/EcdarUniversity";

    #[tokio::test]
    async fn start_simulation__normal__respondes_correct_states_and_transitions() {
        let backend = ProtobufServer::ConcreteEcdarBackend::default();
        let json = std::fs::read_to_string(format!("{}/Components/Machine.json", ECDAR_UNI)).unwrap();
        // let request = tonic::Request::new(

        // );
        

    }

}
