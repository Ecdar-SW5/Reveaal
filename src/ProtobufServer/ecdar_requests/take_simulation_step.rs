impl ConcreteEcdarBackend {

    async fn take_simulation_step(
        &self,
        request: Request<SimulationStepRequest>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        trace!("Recieved query: {:?}", request);
        let handle_step_simulation_step = request.0.into_inner();

        let components = self.get_components_lock()?;   
        let component_container = self.get_components_lock()?;


        let reply = {
            start_component= &handle_step_simulation_step.start_component
            sim_component = &handle_step_simulation_step.sim_component
        };

        Ok(Response::new(reply));
    }
}

fn parse_component_if_some(
    &self,
    proto_component: &ProtobufComponent,
) -> Result<Vec<Component>, tonic::Status> {
    if let Some(rep) = &proto_component.rep {
        match rep {
            Rep::Json(json) => parse_json_component(json),
            Rep::Xml(xml) => Ok(parse_xml_components(xml)),
        }
    } else {
        Ok(vec![])
    }
}
}
fn parse_json_component(json: &str) -> Result<Vec<Component>, tonic::Status> {
match json_to_component(json) {
    Ok(comp) => Ok(vec![comp]),
    Err(_) => Err(tonic::Status::invalid_argument(
        "Failed to parse json component",
    )),
}
}

fn parse_xml_components(xml: &str) -> Vec<Component> {
let (comps, _, _) = parse_xml_from_str(xml);
comps
}
