// These tests ensure the parser output is of the correct datatypes and that they contain the correct values.
#[cfg(test)]
mod reachability_parser_output_datatypes_test {
    use crate::parse_queries;
    use crate::tests::reachability::helper_functions::reachability_test_helper_functions;
    use crate::ModelObjects::representations::QueryExpression;
    use test_case::test_case;
    // Tests whether the parser converts string input with acceptable grammar into the correct datatypes with the corresponding composition
    // Only works with one model as argument!
    // This test does not support m1 || m2 etc. because of the test helper function: create_mock_data_from_args
    #[test_case("reachability: Hi -> [L1](y<3); [L2](y<2)", "Hi", "L1", "y<3", "L2", "y<2";
    "1 machine, start/end location and clock restriction")]
    #[test_case("reachability: Hi -> [L1, L2](y<3); [L3, L4](y<2)", "Hi", "L1, L2", "y<3", "L3, L4", "y<2";
    "Multiple locations")]
    #[test_case("reachability: Hi -> [L1](y<2, y>3); [L2](y<2)", "Hi", "L1", "y<2, y>3", "L2", "y<2";
    "Multiple clock restrictions on start state")]
    #[test_case("reachability: Hi -> [L1](y>3); [L2](y<2, y>5)", "Hi", "L1", "y>3", "L2", "y<2, y>5";
    "Multiple clock restrictions on end state")]
    #[test_case("reachability: Hi -> [L1](); [L2](y<2, y>5)", "Hi", "L1", "", "L2", "y<2, y>5";
    "No clock restrictions on start state")]
    #[test_case("reachability: Hi -> [L1](y<1); [L2]()", "Hi", "L1", "y<1", "L2", "";
    "No clock restrictions on end state")]
    #[test_case("reachability: Hi -> [L1](y<1); [_]()", "Hi", "L1", "y<1", "_", "";
    "No clock restrictions on end state and match-all location argument")]
    // These tests check the shorthand where the start state is omitted:
    #[test_case("reachability: Hi -> [L2](y<1)", "Hi", "", "", "L2", "y<1";
    "No start state")]
    #[test_case("reachability: Hi -> [L2]()", "Hi", "", "", "L2", "";
    "No start state with no end state clock restrictions")]
    #[test_case("reachability: Hi -> [_]()", "Hi", "", "", "_", "";
    "No start state with no end state clock restrictions and match-all location argument")]
    fn query_parser_output_valid(
        parser_input: &str,
        machine: &str,
        start_loc: &str,
        start_clocks: &str,
        end_loc: &str,
        end_clocks: &str,
    ) {
        // Functionality to be tested:
        let parser_result: QueryExpression = parse_queries::parse_to_expression_tree(parser_input)
            .unwrap()
            .first()
            .unwrap()
            .to_owned();
        // Mock version:
        let mock: QueryExpression = reachability_test_helper_functions::create_mock_data_from_args(
            machine,
            start_loc,
            start_clocks,
            end_loc,
            end_clocks,
        );
        // Assert they are equal:
        assert_eq!(format!("{:?}", mock), format!("{:?}", parser_result));
    }

    #[test_case("reachability: Hi -> [L1](y<3); [L2](y<2)", "H", "L1", "y<3", "L2", "y<2";
    "Wrong machine name")]
    #[test_case("reachability: Hi -> [L1, L2](y<3); [L3, L4](y<2)", "Hi", "L3", "y<3", "L3, L4", "y<2";
    "Wrong start location")]
    #[test_case("reachability: Hi -> [L1](y<2, y>3); [L2](y<2)", "Hi", "L1", "y<22222, y>3", "L2", "y<2";
    "Wrong clock restrictions")]
    // These tests check the shorthand where the start state is omitted:
    #[test_case("reachability: Hi -> [L3](y<1)", "Hi", "", "", "L2", "y<1";
    "No start state, L3 and L2 -> not the same location argument")]
    #[test_case("reachability: Hi -> [L2](y<1)", "Hi", "", "", "L2", "y<2";
    "No start state, y<1 and y<2 -> not the same clock restrictions")]
    fn query_parser_output_invalid_values(
        parser_input: &str,
        machine: &str,
        start_loc: &str,
        start_clocks: &str,
        end_loc: &str,
        end_clocks: &str,
    ) {
        // Functionality to be tested:
        let parser_result: QueryExpression = parse_queries::parse_to_expression_tree(parser_input)
            .unwrap()
            .first()
            .unwrap()
            .to_owned();
        // Mock version:
        let mock: QueryExpression = reachability_test_helper_functions::create_mock_data_from_args(
            machine,
            start_loc,
            start_clocks,
            end_loc,
            end_clocks,
        );
        // Assert they are equal:
        assert_ne!(format!("{:?}", mock), format!("{:?}", parser_result));
    }

    #[test]
    fn query_parser_output_invalid_data_type() {
        let parser_result: QueryExpression = parse_queries::parse_to_expression_tree(
            "reachability: HalfAdm1 -> [L1](y<3); [L2](z<2)",
        )
        .unwrap()
        .first()
        .unwrap()
        .to_owned();

        // Mock data:
        let mock_model = Box::new(QueryExpression::VarName("HalfAdm1".to_string()));

        // This should be QueryExpression::LocName instead of QueryExpression::VarName
        let mock_start_state = Box::new(Some(QueryExpression::State(
            Vec::from([Box::new(QueryExpression::VarName("L1".to_string()))]),
            reachability_test_helper_functions::string_to_boolexpression("y<3"),
        )));
        let mock_end_state = Box::new(QueryExpression::State(
            reachability_test_helper_functions::string_to_locations("L2"),
            reachability_test_helper_functions::string_to_boolexpression("z<2"),
        ));
        let mock: QueryExpression =
            QueryExpression::Reachability(mock_model, mock_start_state, mock_end_state);

        assert_ne!(format!("{:?}", mock), format!("{:?}", parser_result));
    }

    #[test]
    fn query_parser_output_invalid_types_for_model() {
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
            "reachability: HalfAdm1 || HalfAdm2 -> [L1, L2](y<3, z>1); [L3, L4](y<4, z<2)",
        )
        .unwrap()
        .first()
        .unwrap()
        .to_owned();

        // Mock data:
        let mock_model = Box::new(QueryExpression::Composition(
            Box::new(QueryExpression::VarName("HalfAdm1".to_string())),
            // This should be VarName type:
            Box::new(QueryExpression::LocName("HalfAdm2".to_string())),
        ));
        let mock_start_state = Box::new(Some(QueryExpression::State(
            reachability_test_helper_functions::string_to_locations("L1, L2"),
            reachability_test_helper_functions::string_to_boolexpression("y<3, z>1"),
        )));
        let mock_end_state = Box::new(QueryExpression::State(
            reachability_test_helper_functions::string_to_locations("L3, L4"),
            reachability_test_helper_functions::string_to_boolexpression("y<4, z<2"),
        ));
        let mock: QueryExpression =
            QueryExpression::Reachability(mock_model, mock_start_state, mock_end_state);

        assert_ne!(format!("{:?}", mock), format!("{:?}", parserResult));
    }
}
