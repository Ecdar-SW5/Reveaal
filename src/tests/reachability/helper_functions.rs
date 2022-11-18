pub mod reachability_test_helper_functions {
    use edbm::util::constraints::ClockIndex;

    use crate::extract_system_rep::get_system_recipe;
    use crate::extract_system_rep::SystemRecipe;
    use crate::xml_parser;
    use crate::DataReader::parse_invariant::parse;
    use crate::JsonProjectLoader;
    use crate::ModelObjects::representations::BoolExpression;
    use crate::ModelObjects::representations::QueryExpression;
    use crate::TransitionSystems::TransitionSystem;
    use crate::XmlProjectLoader;

    /// Helper function which converts a string to an option<box<BoolExpression>> by replacing ',' with "&&" and using the invariant parser.
    pub fn string_to_boolexpression(string: &str) -> Option<Box<BoolExpression>> {
        let string_in_invariant_format = &string.replace(',', "&&");
        if string_in_invariant_format.is_empty() {
            None
        } else {
            Some(Box::new(parse(string_in_invariant_format).unwrap()))
        }
    }
    /// Helper function which converts a string to a Vec<Box<QueryExpression::LocName("")>>>
    pub fn string_to_locations(string: &str) -> Vec<Box<QueryExpression>> {
        let mut v = vec![];
        let parsed_string = string.split(',').map(|s| s.trim());
        for s in parsed_string {
            v.push(Box::new(QueryExpression::LocName(s.to_string())));
        }
        v
    }

    /// Helper function to create the mock data
    pub fn create_mock_data_from_args(
        machine: &str,
        start_loc: &str,
        start_clocks: &str,
        end_loc: &str,
        end_clocks: &str,
    ) -> QueryExpression {
        let mock_model = Box::new(QueryExpression::VarName(machine.to_string()));
        let mock_start_state = if start_loc.is_empty() {
            Box::new(None)
        } else {
            Box::new(Some(QueryExpression::State(
                string_to_locations(start_loc),
                string_to_boolexpression(start_clocks),
            )))
        };
        let mock_end_state = Box::new(QueryExpression::State(
            string_to_locations(end_loc),
            string_to_boolexpression(end_clocks),
        ));
        QueryExpression::Reachability(mock_model, mock_start_state, mock_end_state)
    }

    /// Helper function to create a transition system and a machine (system recipe)
    pub fn create_system_recipe_and_machine(
        model: QueryExpression,
        folder_path: &str,
    ) -> (Box<SystemRecipe>, Box<dyn TransitionSystem>) {
        let mut comp_loader = if xml_parser::is_xml_project(folder_path) {
            XmlProjectLoader::new(folder_path.to_string())
        } else {
            JsonProjectLoader::new(folder_path.to_string())
        }
        .to_comp_loader();
        let mut dim: ClockIndex = 0;
        let mut quotient_index = None;
        let machine = get_system_recipe(&model, &mut (*comp_loader), &mut dim, &mut quotient_index);
        let system = machine.clone().compile(dim).unwrap();
        (machine, system)
    }
}
