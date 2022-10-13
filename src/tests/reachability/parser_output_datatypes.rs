// These tests ensure the parser output is of the correct datatypes and that they contain the correct values.
#[cfg(test)]
mod reachability_parser_output_datatypes_test {
    use crate::parse_queries;
    use crate::tests::reachability::helper_functions::reachability_test_helper_functions;
    use crate::ModelObjects::representations::QueryExpression;
    use test_case::test_case;

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
    // Only works with one model as argument! This test does not support m1 || m2 etc.
    fn query_parser_output_valid(
        parser_input: &str,
        machine: &str,
        start_loc: &str,
        start_clocks: &str,
        end_loc: &str,
        end_clocks: &str,
    ) {
        // Functionality to be tested:
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(parser_input)
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
        assert_eq!(format!("{:?}", mock), format!("{:?}", parserResult));
    }

    #[test_case("reachability: Hi -> [L1](y<3); [L2](y<2)", "H", "L1", "y<3", "L2", "y<2";
    "Wrong machine name")]
    #[test_case("reachability: Hi -> [L1, L2](y<3); [L3, L4](y<2)", "Hi", "L3", "y<3", "L3, L4", "y<2";
    "Wrong start location")]
    #[test_case("reachability: Hi -> [L1](y<2, y>3); [L2](y<2)", "Hi", "L1", "y<22222, y>3", "L2", "y<2";
    "Wrong clock restrictions")]
    // Only works with one model as argument! This test does not support m1 || m2 etc.
    fn query_parser_output_invalid_values(
        parser_input: &str,
        machine: &str,
        start_loc: &str,
        start_clocks: &str,
        end_loc: &str,
        end_clocks: &str,
    ) {
        // Functionality to be tested:
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(parser_input)
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
        assert_ne!(format!("{:?}", mock), format!("{:?}", parserResult));
    }

    #[test]
    fn query_parser_output_invalid_data_type() {
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
            "reachability: HalfAdm1 -> [L1](y<3); [L2](z<2)",
        )
        .first()
        .unwrap()
        .to_owned();

        // Mock data:
        let mock_l = Box::new(QueryExpression::VarName("HalfAdm1".to_string()));

        // This should be QueryExpression::LocName instead of QueryExpression::VarName
        let mock_m = Box::new(QueryExpression::State(
            Vec::from([Box::new(QueryExpression::VarName("L1".to_string()))]),
            reachability_test_helper_functions::string_to_boolexpression("y<3"),
        ));
        let mock_r = Box::new(QueryExpression::State(
            reachability_test_helper_functions::string_to_locations("L2"),
            reachability_test_helper_functions::string_to_boolexpression("z<2"),
        ));
        let mock: QueryExpression = QueryExpression::Reachability(mock_l, mock_m, mock_r);

        assert_ne!(format!("{:?}", mock), format!("{:?}", parserResult));
    }

    #[test]
    fn query_parser_output_invalid_types_for_model() {
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
            "reachability: HalfAdm1 || HalfAdm2 -> [L1, L2](y<3, z>1); [L3, L4](y<4, z<2)",
        )
        .first()
        .unwrap()
        .to_owned();

        // Mock data:
        let mock_l = Box::new(QueryExpression::Composition(
            Box::new(QueryExpression::VarName("HalfAdm1".to_string())),
            // This should be VarName type:
            Box::new(QueryExpression::LocName("HalfAdm2".to_string())),
        ));
        let mock_m = Box::new(QueryExpression::State(
            reachability_test_helper_functions::string_to_locations("L1, L2"),
            reachability_test_helper_functions::string_to_boolexpression("y<3, z>1"),
        ));
        let mock_r = Box::new(QueryExpression::State(
            reachability_test_helper_functions::string_to_locations("L3, L4"),
            reachability_test_helper_functions::string_to_boolexpression("y<4, z<2"),
        ));
        let mock: QueryExpression = QueryExpression::Reachability(mock_l, mock_m, mock_r);

        assert_ne!(format!("{:?}", mock), format!("{:?}", parserResult));
    }
}
