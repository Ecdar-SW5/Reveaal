// These tests ensure the parser/grammar can parse strings of the reachability syntax
#[cfg(test)]
mod parser_new_functionality_tests {

    use crate::parse_queries;
    use crate::DataReader::parse_invariant::parse;
    use crate::ModelObjects::representations::{BoolExpression, QueryExpression};

    /// Helper function which converts a string to a BoolExpression by replacing ',' with "&&" and using invariant parser.
    fn string_to_boolexpression(string: &str) -> BoolExpression {
        parse(&string.replace(",", "&&")).unwrap()
    }

    #[test]
    fn parser_reachability_query_string_to_queryexpressions_and_boolexpressions() {
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
            "reachability: HalfAdm1 -> [L12](y<3); [L13](z<2)",
        )
        .first()
        .unwrap()
        .to_owned();

        // Mock data:
        let mock_l = Box::new(QueryExpression::VarName("HalfAdm1".to_string()));

        let mock_m = Box::new(QueryExpression::State(
            Box::new(QueryExpression::LocName("L12".to_string())),
            Box::new(string_to_boolexpression("y<3")),
        ));

        let mock_r = Box::new(QueryExpression::State(
            Box::new(QueryExpression::LocName("L13".to_string())),
            Box::new(string_to_boolexpression("z<2")),
        ));

        let mock: QueryExpression = QueryExpression::Reachability(mock_l, mock_m, mock_r);

        assert_eq!(parserResult.pretty_string(), mock.pretty_string());
    }

    #[test]
    fn reachability_query_multiple_clock_restrictions() {
        // This clocks are parsed by the inveriant parser, which we havn't touched. So this should pass.
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
            "reachability: HalfAdm1 -> [L12](y<3, z>1); [L13](y<4, z<2)",
        )
        .first()
        .unwrap()
        .to_owned();

        // Mock data:
        let mock_l = Box::new(QueryExpression::VarName("HalfAdm1".to_string()));

        let mock_m = Box::new(QueryExpression::State(
            Box::new(QueryExpression::LocName("L12".to_string())),
            Box::new(string_to_boolexpression("y<3, z>1")),
        ));

        let mock_r = Box::new(QueryExpression::State(
            Box::new(QueryExpression::LocName("L13".to_string())),
            Box::new(string_to_boolexpression("y<4, z<2")),
        ));

        let mock: QueryExpression = QueryExpression::Reachability(mock_l, mock_m, mock_r);

        assert_eq!(parserResult.pretty_string(), mock.pretty_string());
    }

    #[test]
    fn reachability_query_multiple_components() {
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
            "reachability: HalfAdm1 || HalfAdm2 -> [L12, L4](y<3, z>1); [L13, L5](y<4, z<2)",
        )
        .first()
        .unwrap()
        .to_owned();

        // Mock data:
        let mock_l = Box::new(QueryExpression::Composition(
            Box::new(QueryExpression::VarName("HalfAdm1".to_string())),
            Box::new(QueryExpression::VarName("HalfAdm2".to_string())),
        ));

        let mock_m = Box::new(QueryExpression::State(
            Box::new(QueryExpression::LocName("L12, L4".to_string())),
            Box::new(string_to_boolexpression("y<3, z>1")),
        ));

        let mock_r = Box::new(QueryExpression::State(
            Box::new(QueryExpression::LocName("L13, L5".to_string())),
            Box::new(string_to_boolexpression("y<4, z<2")),
        ));

        let mock: QueryExpression = QueryExpression::Reachability(mock_l, mock_m, mock_r);

        assert_eq!(parserResult.pretty_string(), mock.pretty_string());
    }

    #[test]
    fn reachability_query_no_clockvalues() {
        // Providing clock values/constraints should be optional.
        let parserResult: QueryExpression = parse_queries::parse_to_expression_tree(
            "reachability: HalfAdm1 || HalfAdm2 -> [L12, L4](); [L13, L5]()",
        )
        .first()
        .unwrap()
        .to_owned();

        // Mock data:
        let mock_l = Box::new(QueryExpression::Composition(
            Box::new(QueryExpression::VarName("HalfAdm1".to_string())),
            Box::new(QueryExpression::VarName("HalfAdm2".to_string())),
        ));

        let mock_m = Box::new(QueryExpression::State(
            Box::new(QueryExpression::LocName("L12, L4".to_string())),
            Box::new(string_to_boolexpression("")),
        ));

        let mock_r = Box::new(QueryExpression::State(
            Box::new(QueryExpression::LocName("L13, L5".to_string())),
            Box::new(string_to_boolexpression("")),
        ));

        let mock: QueryExpression = QueryExpression::Reachability(mock_l, mock_m, mock_r);

        assert_eq!(parserResult.pretty_string(), mock.pretty_string());
    }
}
