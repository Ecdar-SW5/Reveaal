pub struct SerializedDecisionPoint {}

impl SerializedDecisionPoint {}

#[cfg(test)]
mod tests {
    use crate::DataReader::json_reader::read_json_component;
    use crate::DataReader::parse_edge::EdgeParser;
    use crate::ProtobufServer::services::component;
    use crate::TransitionSystems::TransitionSystem;
    use crate::Simulation::decision_point::DecisionPoint;
    use crate::component::Component;
    use crate::ProtobufServer::services::DecisionPoint as ProtoDecisionPoint;
    use crate::ProtobufServer::services::Edge as ProtoEdge;
    use crate::ProtobufServer::services::State as ProtoState;

    pub fn setupHelper(_input_path: &str, _system: &str) -> Box<dyn TransitionSystem> {
        todo!();
        // let project_loader = JsonProjectLoader::new(String::from(input_path));

        // //This query is not executed but simply used to extract an UncachedSystem so the tests can just give system expressions
        // let str_query = format!("get-component: {} save-as test", system);
        // let query = parse_queries::parse_to_expression_tree(str_query.as_str()).remove(0);

        // let mut dim: ClockIndex = 0;
        // let (base_system, new_system) = if let QueryExpression::GetComponent(expr) = &query {
        //     let mut comp_loader = project_loader.to_comp_loader();
        //     (
        //         extract_system_rep::get_system_recipe(
        //             expr.as_ref(),
        //             &mut *comp_loader,
        //             &mut dim,
        //             &mut None,
        //         ),
        //         extract_system_rep::get_system_recipe(
        //             expr.as_ref(),
        //             &mut *comp_loader,
        //             &mut dim,
        //             &mut None,
        //         ),
        //     )
        // } else {
        //     panic!("Failed to create system")
        // };

        // let new_comp = new_system.compile(dim);
        // if new_comp.is_err() {
        //     panic!("Error")
        // }

        // let new_comp = combine_components(&new_comp.unwrap(), PruningStrategy::NoPruning);

        // let new_comp = SystemRecipe::Component(Box::new(new_comp))
        //     .compile(dim)
        //     .unwrap();
        // let base_system = base_system.compile(dim).unwrap();
        // base_system
    }

    // pub fn setupStructInitialize(transition_system: Box<dyn TransitionSystem>) -> Setup {
    //     Setup {
    //         testSource: match transition_system.get_initial_state() {
    //             Some(source) => source,
    //             None => panic!("No initial state"),
    //         },
    //         //testSource: base_system.get_initial_state(),
    //         testEdges: vec![],
    //     }
    // }

    // pub fn protoStateSetup(transition_system: Box<dyn TransitionSystem>) -> ProtoState {
    //     let location_tuple: LocationTuple = match transition_system.get_initial_location()
    //     {
    //         Some(x) => x,
    //         None => panic!("No initial location")
    //     };

    //     SpecificComponent {

    //     };
    //     ProtoLocation {

    //     };
    //     LocationTuple {

    //     };
    //     Federation {

    //     };
    //     ProtoState {
    //         location_tuple: location_tuple,

    //     }
    // }
    fn create_EcdarUniversity_Machine_System_Component() -> Component {
        let component = read_json_component("samples/json/EcdarUniversity", "machine");

        return component;
    }
        
    #[test]
    fn given_state_return_serialized_state() {
        static PATH: &str = "samples/json/Conjunction";

        let _transition_system: Box<dyn TransitionSystem> = setupHelper(PATH, "Test1 && Test1");

        assert!(false);
    }


     #[test]
     fn from_decisionpoint_to_protoDecisionPoint__correctProtoDecisionPoint__returnsProtoDecisionPoint() {
    //     // Arrange
    //    let system = create_EcdarUniversity_Machine_system();
    //    let component = create_EcdarUniversity_Machine_System_Component().clone();
    //    let edges = component.get_edges().clone();
    //    let initial_state = system.get_initial_state().unwrap();

    //     let decisionPoint = DecisionPoint {
    //         source: initial_state.clone(),
    //         possible_decisions: edges,
    // };
        
        
        

        // Assert
        //let expectedProtoDecisionPoint = ProtoDecisionPoint {
        //    source: initial_state as ProtoState,
        //   edges: ,
        //};
        
        // let actual = DecisionPoint::From();
        // let expected: ProtoDecisionPoint = todo!();

        // Act
    }
}
