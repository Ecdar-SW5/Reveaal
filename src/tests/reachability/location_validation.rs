#[cfg(test)]
mod reachability_parser_location_validation {
    use crate::{
        tests::reachability::helper_functions::reachability_test_helper_functions,
        ModelObjects::representations::QueryExpression, System,
    };
    use test_case::test_case;
    const FOLDER_PATH: &str = "samples/json/EcdarUniversity";
    // These tests check that the parser only accepts location arguments with existing locations.
    // i.e. check that the locations exist in the model.
    // The model/sample used is samples/json/EcdarUniversity/adm2.json
    // This model/sample contains the locations "L20", "L21" ... "L23".
    #[test_case("L19";
    "The location L19 in the state does not exist in the model")]
    #[test_case("NOTCORRECTNAME";
    "The location NOTCORRECTNAME in the state does not exist in the model")]
    fn query_parser_checks_invalid_locations(location_str: &str) {
        let mock_model = Box::new(QueryExpression::VarName("Adm2".to_string()));
        let (machine, system) =
            reachability_test_helper_functions::create_system_recipe_and_machine(
                *mock_model,
                FOLDER_PATH,
            );

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
    fn query_parser_checks_valid_locations(location_str: &str) {
        let mock_model = Box::new(QueryExpression::VarName("Adm2".to_string()));
        let (machine, system) =
            reachability_test_helper_functions::create_system_recipe_and_machine(
                *mock_model,
                FOLDER_PATH,
            );

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
