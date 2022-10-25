use crate::ModelObjects::component::{
    Component, Declarations, Edge, Location, LocationType, SyncType, State
};


pub struct SerializedDecisionPoint {}

impl SerializedDecisionPoint {}

#[cfg(test)]
mod tests {
    use crate::ModelObjects::component::{
        Component, Declarations, Edge, Location, LocationType, SyncType, State  
    };
    use crate::DataReader::component_loader::JsonProjectLoader;
    use crate::DataReader::parse_queries;
    use crate::ModelObjects::representations::QueryExpression;
    use crate::System::extract_system_rep;
    use crate::System::extract_system_rep::SystemRecipe;
    use crate::System::refine;
    use crate::System::save_component::combine_components;
    use crate::System::save_component::PruningStrategy;
    use edbm::util::constraints::ClockIndex;


    pub struct Setup {
        testSource: State,
        testEdges: Vec<Edge>,
    }
    
    pub fn setupHelper(input_path: &str, system: &str) -> Setup {
        let project_loader = JsonProjectLoader::new(String::from(input_path));

        //This query is not executed but simply used to extract an UncachedSystem so the tests can just give system expressions
        let str_query = format!("get-component: {} save-as test", system);
        let query = parse_queries::parse_to_expression_tree(str_query.as_str()).remove(0);

        let mut dim: ClockIndex = 0;
        let (base_system, new_system) = if let QueryExpression::GetComponent(expr) = &query {
            let mut comp_loader = project_loader.to_comp_loader();
            (
                extract_system_rep::get_system_recipe(
                    expr.as_ref(),
                    &mut *comp_loader,
                    &mut dim,
                    &mut None,
                ),
                extract_system_rep::get_system_recipe(
                    expr.as_ref(),
                    &mut *comp_loader,
                    &mut dim,
                    &mut None,
                ),
            )
        } else {
            panic!("Failed to create system")
        };

        let new_comp = new_system.compile(dim);
        if new_comp.is_err() {
            panic!("Error")
        }

        let new_comp = combine_components(&new_comp.unwrap(), PruningStrategy::NoPruning);

        let new_comp = SystemRecipe::Component(Box::new(new_comp))
            .compile(dim)
            .unwrap();
        let base_system = base_system.compile(dim).unwrap();
        Setup {
            testSource: match base_system.get_initial_state() {
                Some(source) => source,
                None => panic!("No initial state")
            },
            testEdges: vec![],
        }
    }

    #[test]
    fn given_state_return_serialized_state()
    {
        assert!(false);
    }
}
