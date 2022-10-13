#[cfg(test)]
mod search_algorithm_test{
    use test_case::test_case;
    use crate::tests::refinement::Helper::json_run_query;
    use crate::QueryResult;
    const PATH: &str = "samples/json/EcdarUniversity";

    #[test_case(PATH, "reachability: Machine -> [L5](y<6); [L4](y<=6)", true; "Existing states and with right clocks")]
    #[test_case(PATH, "reachability: Machine -> [L5](); [L4](y>7)", false; "Exisiting locations but not possible with the clocks")] //This one fails because it panics
    #[test_case(PATH, "reachability: Machine -> [L4](y<=6); [L5](y>=4)", true; "Switched the two states and with right clocks")]
    #[test_case(PATH, "reachability: Machine -> [L5](y<1); [L5](y<2)", true; "Same location, different clocks")]
    #[test_case(PATH, "reachability: Machine -> [L5](); [L5]()", true; "Same location, no clocks")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, L6](); [L4, L9]()", true; "Composition between Machine & Researcher, with existing locations and not clocks")]
    #[test_case(PATH, "reachability: Researcher -> [U0](); [L7]()", false; "No possible path between to locations, locations exists in Researcher")]
    fn reachability_test_search_algorithm_returns_result(path: &str, query: &str, expected: bool) {
        match json_run_query(path, query) {
          QueryResult::Reachability(b, _) => assert_eq!(b, expected),
          _ => panic!("Inconsistent query result, expected Reachability")
        }
    }
}
