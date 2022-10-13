// These tests ensure the parser/grammar can parse strings of the reachability syntax,
// which is the parse_queries::parse_to_expression_tree() function.
#[cfg(test)]
mod reachability_grammar_test {
    use crate::parse_queries;
    use test_case::test_case;

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
    #[test_case("reachability: Hi -> [L1](); [_]()"; "no location or clock values specified for end state")] // 1 location argument provided
    #[test_case("reachability: Hi -> [_](); [_]()"; "no location or clock values specified")] // 1 location argument provided
    fn query_grammar_test_valid_queries(parser_input: &str) {
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
    #[test_case("reachability: Hi -> [_](); []()"; "blank start loc and and no location or clock values specified for end")] // Missing location argument
    #[test_case("reachability: Hi -> [](); [_]()"; "no location or clock values specified for start, blank for end")] // Missing location argument
    #[should_panic]
    fn query_grammar_test_panic(parser_input: &str) {
        // This tests that the grammar does NOT accept this string and panics:
        parse_queries::parse_to_expression_tree(parser_input);
    }
}
