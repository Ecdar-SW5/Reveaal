#[cfg(test)]
mod tests {
    use std::fs;

    use test_case::test_case;
    use tonic::{Request, Response, Status};

    use crate::ProtobufServer::{
        self,
        services::{
            component::Rep, ecdar_backend_server::EcdarBackend, Component, ComponentsInfo,
            Decision, SimulationInfo, SimulationStartRequest, SimulationStepRequest,
            SimulationStepResponse,
        },
    };

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
        let request = create_start_request(component_names, components_path, composition);

        // Act
        let response = backend.start_simulation(request).await;

        // Arrange
        let request = create_step_request(component_names, components_path, composition, response);

        // Act
        let response = backend.take_simulation_step(request).await;

        // Assert
        assert!(response.is_ok())
    }

    fn create_start_request(
        component_names: &[&str],
        components_path: &str,
        composition: &str,
    ) -> Request<SimulationStartRequest> {
        let simulation_info = create_simulation_info(component_names, components_path, composition);
        Request::new(SimulationStartRequest {
            simulation_info: Some(simulation_info),
        })
    }

    fn create_step_request(
        component_names: &[&str],
        components_path: &str,
        composition: &str,
        last_response: Result<Response<SimulationStepResponse>, Status>,
    ) -> Request<SimulationStepRequest> {
        let simulation_info = create_simulation_info(component_names, components_path, composition);
        let last_response = last_response.unwrap().into_inner();
        let source = last_response
            .clone()
            .new_decision_points
            .first()
            .unwrap()
            .source
            .to_owned();
        let decision = last_response
            .clone()
            .new_decision_points
            .first()
            .unwrap()
            .edges
            .first()
            .unwrap()
            .to_owned();

        Request::new(SimulationStepRequest {
            simulation_info: Some(simulation_info),
            chosen_decision: Some(Decision {
                source: source,
                edge: Some(decision),
            }),
        })
    }

    fn create_simulation_info(
        component_names: &[&str],
        components_path: &str,
        composition: &str,
    ) -> SimulationInfo {
        let json_components: Vec<_> = component_names
            .into_iter()
            .map(|component_name| Component {
                rep: Some(Rep::Json(
                    fs::read_to_string(format!(
                        "{}/Components/{}.json",
                        components_path, component_name
                    ))
                    .unwrap(),
                )),
            })
            .collect();

        SimulationInfo {
            user_id: 0,
            component_composition: composition.to_string(),
            components_info: Some(ComponentsInfo {
                components: json_components,
                components_hash: 0,
            }),
        }
    }
}
