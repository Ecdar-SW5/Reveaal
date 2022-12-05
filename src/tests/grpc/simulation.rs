#[cfg(test)]
mod tests {
    use crate::{
        tests::Simulation::helper::{create_start_request, create_step_request},
        ProtobufServer::{self, services::ecdar_backend_server::EcdarBackend},
    };
    use test_case::test_case;
    use tonic::Request;

    #[test_case(
        &["Machine"],
        "samples/json/EcdarUniversity",
        "(Machine)"
    )]
    #[test_case(
        &["HalfAdm1", "HalfAdm2"],
        "samples/json/EcdarUniversity",
        "(HalfAdm1 && HalfAdm2)"
    )]
    #[test_case(
        &["Administration", "Machine", "Researcher"],
        "samples/json/EcdarUniversity",
        "(Administration || Machine || Researcher)"
    )]
    #[test_case(
        &["HalfAdm1", "HalfAdm2", "Machine", "Researcher"],
        "samples/json/EcdarUniversity",
        "((HalfAdm1 && HalfAdm2) || Machine || Researcher)"
    )]
    #[tokio::test]
    async fn start_simulation_then_take_simulation_step(
        component_names: &[&str],
        components_path: &str,
        composition: &str,
    ) {
        // Arrange
        let backend = ProtobufServer::ConcreteEcdarBackend::default();
        let request = Request::new(create_start_request(
            component_names,
            components_path,
            composition,
        ));

        // Act
        let response = backend.start_simulation(request).await;

        // Arrange
        let request = Request::new(create_step_request(
            component_names,
            components_path,
            composition,
            response,
        ));

        // Act
        let response = backend.take_simulation_step(request).await;

        // Assert
        assert!(response.is_ok())
    }
}
