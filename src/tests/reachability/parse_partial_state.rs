#[cfg(test)]
mod reachability_parse_partial_state {
    use crate::{
        extract_system_rep, parse_queries,
        tests::reachability::helper_functions::reachability_test_helper_functions,
        JsonProjectLoader, ModelObjects::representations::QueryExpression, System,
    };
    use test_case::test_case;

    const FOLDER_PATH: &str = "samples/json/EcdarUniversity";

    #[test_case("_", true;
    "State gets parsed as partial")]
    #[test_case("L20", false;
    "State gets parsed as not partial")]
    fn query_parser_checks_invalid_locations(location_str: &str, expect_partial: bool) {
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

        let result = System::extract_state::get_state(&mock_state, &machine, &system);

        if let Ok(end_state) = result {
            assert_eq!(
                end_state.get_location().is_partial_location(),
                expect_partial
            );
        } else {
            panic!("Could not get end state");
        }
    }

    #[test_case("reachability: Adm2 -> [_](); [L20]()";
    "no partial start state and one component")]
    #[test_case("reachability: Adm2 && Adm2 -> [L21, _](); [L20, L21]()";
    "partial start state and two components")]
    #[test_case("reachability: Adm2 && Adm2 && Adm2 && Adm2 && Adm2 -> [L20, L20, _, L21, L20](); [L20, L21, L20, L21, L20]()";
    "partial start state and complex composition")]
    #[test_case("reachability: Adm2 -> [_](); [_]()";
    "Both end and start are partial")]
    fn query_parser_reject_partial_start(parser_input: &str) {
        let mut comp_loader = JsonProjectLoader::new(String::from(FOLDER_PATH)).to_comp_loader();
        // Make query:
        let q = parse_queries::parse_to_query(parser_input);
        let queries = q.first().unwrap();

        let result = extract_system_rep::create_executable_query(queries, &mut *comp_loader);
        if let Err(e) = result {
            assert_eq!(
                (*e).to_string(),
                "Start state is a partial state, which it must not be"
            );
        } else {
            panic!("No error was returned")
        }
    }
}
