#[cfg(test)]
mod reachability_parser_location_validation {
    use crate::{
        extract_system_rep::get_system_recipe,
        tests::reachability::helper_functions::reachability_test_helper_functions, xml_parser,
        JsonProjectLoader, ModelObjects::representations::QueryExpression, System,
        XmlProjectLoader,
    };
    use edbm::util::constraints::ClockIndex;
    use test_case::test_case;

    // These tests check that the parser only accepts location arguments with existing locations.
    // i.e. check that the locations exist in the model.
    // The model/sample used is samples/json/EcdarUniversity/adm2.json
    // This model/sample contains the locations "L20", "L21" ... "L23".
    #[test_case("L19";
    "The location L19 in the state does not exist in the model")]
    #[test_case("NOTCORRECTNAME";
    "The location NOTCORRECTNAME in the state does not exist in the model")]
    fn query_parser_checks_invalid_locations(location_str: &str) {
        let folder_path = "samples/json/EcdarUniversity".to_string();
        let mut comp_loader = if xml_parser::is_xml_project(&folder_path) {
            XmlProjectLoader::new(folder_path)
        } else {
            JsonProjectLoader::new(folder_path)
        }
        .to_comp_loader();
        let mock_model = Box::new(QueryExpression::VarName("Adm2".to_string()));
        let mut dim: ClockIndex = 0;
        let mut quotient_index = None;
        let machine = get_system_recipe(
            &mock_model,
            &mut (*comp_loader),
            &mut dim,
            &mut quotient_index,
        );
        let system = machine.clone().compile(dim).unwrap();
        let mock_state = Box::new(QueryExpression::State(
            reachability_test_helper_functions::string_to_locations(location_str),
            None,
        ));
        match System::extract_state::get_state(&mock_state, &machine, &system) {
            Err(_) => (),
            Ok(_) => panic!("Expected Err, recieved Ok"),
        };
    }

    #[test_case("L20";
    "The location L20 in the state exists in the model")]
    #[test_case("L23";
    "The location L23 in the state exists in the model")]
    fn query_parser_checks_valid_clock_variables(location_str: &str) {
        let folder_path = "samples/json/EcdarUniversity".to_string();
        let mut comp_loader = if xml_parser::is_xml_project(&folder_path) {
            XmlProjectLoader::new(folder_path)
        } else {
            JsonProjectLoader::new(folder_path)
        }
        .to_comp_loader();
        let mock_model = Box::new(QueryExpression::VarName("Adm2".to_string()));
        let mut dim: ClockIndex = 0;
        let mut quotient_index = None;
        let machine = get_system_recipe(
            &mock_model,
            &mut (*comp_loader),
            &mut dim,
            &mut quotient_index,
        );
        let system = machine.clone().compile(dim).unwrap();
        let mock_state = Box::new(QueryExpression::State(
            reachability_test_helper_functions::string_to_locations(location_str),
            None,
        ));
        match System::extract_state::get_state(&mock_state, &machine, &system) {
            Ok(_) => (),
            Err(_) => panic!("Expected Ok, recieved Err"),
        };
    }
}
