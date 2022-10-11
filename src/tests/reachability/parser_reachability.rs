// These tests ensure the parser/grammar can parse strings of the reachability syntax,
// which is the parse_queries::parse_to_expression_tree() function.
#[cfg(test)]
mod reachability_grammar_test {
    use test_case::test_case;
    use crate::parse_queries;

    #[test_case("reachability: Hi -> [L1](y<3); [L2](y<2)"; "only 1 machine, start/end location and clock restriction")]
    #[test_case("reachability: Hi -> [L1, L2](y<3); [L2, L3](y<2)"; "two locations")]
    #[test_case("reachability: Hi || M2 -> [L1](y<3); [L2](y<2)"; "Composition of to models")] // Grammar is ok, but 2 location args should be provided
    #[test_case("reachability: Hi || M2 -> [L1, L3](y<3); [_](y<2)"; "Only blank location argument in end state")] // Grammar is ok, but end location should contain 2 args
    #[test_case("reachability: Hi || M2 -> [L1, L3](y<3); [_, L2](y<2)"; "Blank location argument as first arg for end location")]
    #[test_case("reachability: Hi || M2 -> [L1, L3](y<3); [_, _](y<2)"; "Double blank location argument for end location")]
    #[test_case("reachability: Hi -> [L1, L2](y<3); [L2, L3]()"; "no clock restrictions for end state")]
    #[test_case("reachability: Hi -> [L1, L2](); [L2, L3](y<3)"; "no clock restrictions for start state")]
    #[test_case("reachability: Hi -> [L1, L2](); [L2, L3]()"; "no clock restrictions")]
    #[test_case("reachability: H -> [L1, L2](); [L2, L3]()"; "1 char model")]
    #[test_case("reachability: Hi -> [LX1, LQ2](); [Lorem, Ipsum]()"; "strange location names")]
    #[test_case("reachability: Hi -> [](); [L2, L3](y<3)"; "no location specified")] // Should this pass?
    #[test_case("reachability: Hi -> [](); []()"; "no location or clock values specified")] // Should this pass?
    fn reachability_query_grammar_test_valid_queries(parser_input: &str) {
        // This tests that the grammar accepts this string, and does not panic:
        parse_queries::parse_to_expression_tree(parser_input);
    }

    #[test_case("reachability: Hi -> (); []()"; "No [] to specify locations")]
    #[test_case("reachability: Hi -> [(); []()"; "Missing end ] to specify locations")]
    #[test_case("reachability: Hi -> (); []()"; "Missing start [ to specify locations")]
    #[test_case("reachability: Hi -> []; []()"; "Missing () to specify clocks for start state")]
    #[test_case("reachability: Hi -> [](); []"; "Missing () to specify clocks for end state")]
    #[test_case("reachability: Hi -> []() []()"; "Missing ; to seperate start and end states")]
    #[test_case("reachability: Hi []() []()"; "Missing -> to seperate model and start and end states")]
    #[test_case("reachability: Hi > []() []()"; "Missing greater then > to seperate model and start and end states")]
    #[test_case("reachability: Hi - []() []()"; "Missing dash - to seperate model and start and end states")]
    #[test_case("reachability:  -> []() []()"; "Missing model name")]
    #[test_case("reachability Hi -> []() []()"; "Missing : after query type")]
    #[test_case("ry: Hi -> []() []()"; "Misspelled reachability")]
    #[test_case("Hi -> []() []()"; "Query type omitted")]
    #[should_panic]
    fn reachability_query_grammar_test_panic(parser_input: &str) {
        // This tests that the grammar does NOT accept this string and panics:
        parse_queries::parse_to_expression_tree(parser_input);
    }
}

// These tests ensure the parser output is of the correct datatypes and that they contain the correct values.
#[cfg(test)]
mod reachability_parser_output_datatypes_test {
    use crate::parse_queries;
    use crate::DataReader::parse_invariant::parse;
    use crate::ModelObjects::representations::{BoolExpression, QueryExpression};
    use test_case::test_case;

    /// Helper function which converts a string to an option<box<BoolExpression>> by replacing ',' with "&&" and using the invariant parser.
    fn string_to_boolexpression(string: &str) -> Option<Box<BoolExpression>> {
        let string_in_invariant_format = &string.replace(",", "&&");
        if string_in_invariant_format.is_empty(){
            return None;
        } else {
            return Some(Box::new(parse(&string.replace(",", "&&")).unwrap()));
        }
    }
    /// Helper function which converts a string to a Vec<Box<QueryExpression::LocName("")>>>
    fn string_to_locations(string: &str)->Vec<Box<QueryExpression>>{
        let mut v = vec![];
        let parsed_string = string.split(",").map(|s| s.trim());
        for s in parsed_string {
            v.push(Box::new(QueryExpression::LocName(s.to_string())));
        }
        return v;
    }

    /// Helper function to create the mock data
    fn create_mock_data_from_args(machine: &str, start_loc: &str, start_clocks: &str, end_loc: &str, end_clocks: &str) -> QueryExpression{
        let mock_l = Box::new(QueryExpression::VarName(machine.to_string()));
        let mock_m = Box::new(QueryExpression::State(
            string_to_locations(start_loc),
            string_to_boolexpression(start_clocks),
        ));
        let mock_r = Box::new(QueryExpression::State(
            string_to_locations(end_loc),
            string_to_boolexpression(end_clocks),
        ));
        QueryExpression::Reachability(mock_l, mock_m, mock_r)
    }

    //reachability: HalfAdm1 -> [L12](y<3, z>1); [L13](y<4, z<2)

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
    fn reachability_query_parser_output_valid(parser_input: &str, machine: &str, start_loc: &str, start_clocks: &str, end_loc: &str, end_clocks: &str) {
        // Functionality to be tested:
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
            parser_input,
        )
        .first()
        .unwrap()
        .to_owned();
        // Mock version:
        let mock: QueryExpression = create_mock_data_from_args(machine, start_loc, start_clocks, end_loc, end_clocks);
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
    fn reachability_query_parser_output_invalid_values(parser_input: &str, machine: &str, start_loc: &str, start_clocks: &str, end_loc: &str, end_clocks: &str) {
        // Functionality to be tested:
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
            parser_input,
        )
        .first()
        .unwrap()
        .to_owned();
        // Mock version:
        let mock: QueryExpression = create_mock_data_from_args(machine, start_loc, start_clocks, end_loc, end_clocks);
        // Assert they are equal:
        assert_ne!(format!("{:?}", mock), format!("{:?}", parserResult));
    }

    #[test]
    fn reachability_query_parser_output_invalid_data_type() {
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
            string_to_boolexpression("y<3"),
        ));
        let mock_r = Box::new(QueryExpression::State(
            string_to_locations("L2"),
            string_to_boolexpression("z<2"),
        ));
        let mock: QueryExpression = QueryExpression::Reachability(mock_l, mock_m, mock_r);

        assert_ne!(format!("{:?}", mock), format!("{:?}", parserResult));
    }

    #[test]
    fn reachability_query_parser_output_invalid_types_for_model() {
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
            string_to_locations("L1, L2"),
            string_to_boolexpression("y<3, z>1"),
        ));
        let mock_r = Box::new(QueryExpression::State(
            string_to_locations("L3, L4"),
            string_to_boolexpression("y<4, z<2"),
        ));
        let mock: QueryExpression = QueryExpression::Reachability(mock_l, mock_m, mock_r);

        assert_ne!(format!("{:?}", mock), format!("{:?}", parserResult));
    }

    // These tests check the parsers validity checks, like an equal amount of parameters
    // #[test]
    // #[should_panic]
    // fn reachability_query_amount_of_locations_in_end_state_does_not_match_amount_of_machines() {
    //     // The amount of locations given as parameters must be the same as the amount of machines.
    //     let _parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
    //         "reachability: HalfAdm1 || HalfAdm2 -> [L1, L2](c>2); [L3](c>1)",
    //     )
    //     .first()
    //     .unwrap()
    //     .to_owned();
    // }
    // #[test]
    // #[should_panic]
    // fn reachability_query_amount_of_locations_in_start_state_does_not_match_amount_of_machines() {
    //     // The amount of locations given as parameters must be the same as the amount of machines.
    //     let _parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
    //         "reachability: HalfAdm1 || HalfAdm2 -> [L1](c>2); [L3, L4](c>1)",
    //     )
    //     .first()
    //     .unwrap()
    //     .to_owned();
    // }
}
