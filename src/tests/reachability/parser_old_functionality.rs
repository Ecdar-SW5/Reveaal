// All tests in this file are not related to reachability.
// They are here to ensure we didn't break anything in the parser/grammar.
#[cfg(test)]
mod parser_old_functionality_tests {
    use crate::parse_queries;
    use crate::ModelObjects::representations::QueryExpression;

    #[test]
    fn reachability_string_to_query_expression_pass() {
        let parserResult: Vec<QueryExpression> =
            parse_queries::parse_to_expression_tree("consistency: HalfAdm2");
        let unwrappedParserResult: QueryExpression = parserResult.first().unwrap().to_owned();

        let mock: QueryExpression = QueryExpression::Consistency(Box::new(
            QueryExpression::VarName("HalfAdm2".to_string()),
        ));

        assert_eq!(unwrappedParserResult.pretty_string(), mock.pretty_string());
    }

    #[test]
    fn reachability_string_to_query_expression_fail_wrong_varname() {
        let parserResult: Vec<QueryExpression> =
            parse_queries::parse_to_expression_tree("consistency: HalfAdm2");
        let unwrappedParserResult: QueryExpression = parserResult.first().unwrap().to_owned();

        let mock: QueryExpression = QueryExpression::Consistency(Box::new(
            QueryExpression::VarName("HalfAdm69-420".to_string()),
        ));

        assert_ne!(unwrappedParserResult.pretty_string(), mock.pretty_string());
    }

    #[test]
    fn reachability_string_to_query_expression_fail_wrong_queryexpression_enum() {
        let parserResult: Vec<QueryExpression> =
            parse_queries::parse_to_expression_tree("consistency: HalfAdm2");
        let unwrappedParserResult: QueryExpression = parserResult.first().unwrap().to_owned();

        let mock: QueryExpression = QueryExpression::GetComponent(Box::new(
            QueryExpression::VarName("HalfAdm2".to_string()),
        ));

        // Not equal since QueryExpression::GetComponent != QueryExpression::Consistency
        assert_ne!(
            (unwrappedParserResult.pretty_string()),
            (mock.pretty_string())
        );
    }
}
