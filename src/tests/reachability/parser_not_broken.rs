// The test in this file is not related to reachability, but to ensure we didn't break anything in the parser/grammar.
#[cfg(test)]
mod parser_not_broken_test {
    use crate::parse_queries;
    use crate::ModelObjects::representations::QueryExpression;

    #[test]
    fn reachability_parser_not_broken() {
        let parserResult: QueryExpression =
            parse_queries::parse_to_expression_tree("consistency: HalfAdm2")
                .first()
                .unwrap()
                .to_owned();

        let mock: QueryExpression = QueryExpression::Consistency(Box::new(
            QueryExpression::VarName("HalfAdm2".to_string()),
        ));

        assert_eq!(format!("{:?}", mock), format!("{:?}", parserResult));
    }
}
