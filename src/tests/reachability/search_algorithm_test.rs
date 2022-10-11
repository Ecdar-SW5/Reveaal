#[cfg(test)]
mod search_algorithm_test{
    use test_case::test_case;
    use crate::tests::refinement::Helper::json_run_query;
    use crate::QueryResult;
    const PATH: &str = "samples/json/EcdarUniversity";

    #[test_case(PATH, "reachability: Machine -> [L5](y<6); [L4](y<=6)", true; "T_Test1")]
    #[test_case(PATH, "reachability: Machine -> [L5](y>7); [L4](y<=6)", true; "T_Test2")]
    #[test_case(PATH, "reachability: Machine -> [L4](y<=6); [L5](y>=4)", true; "T_Test3")]
    #[test_case(PATH, "reachability: Machine -> [L4](y<=6); [L5](y<4)", true; "T_Test4")]
    #[test_case(PATH, "reachability: Machine -> [L5](y<1); [L5](y<2)", true; "T_Test5")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, L6](); [L4, L9]()", true; "T_Test6")]
    #[test_case(PATH, "reachability: Researcher -> [U0](); [L7]()", false; "T_Test7")]
    fn reachability_test_search_algorithm_returns_result(path: &str, query: &str, expected: bool) {
        match json_run_query(path, query) {
          QueryResult::Reachability(b, _) => assert_eq!(b, expected), 
          QueryResult::Error(e) => panic!("{}", e),
          _ => panic!("Inconsistent query result, expected Reachability")
        }
    }

    #[test_case(PATH, "reachability: Machine -> [L5](y<6); [L4](y>7)"; "P_Test1")]
    #[test_case(PATH, "reachability: Machine -> [L3](y<6); [L4](y<=6)"; "P_Test2")]
    #[test_case(PATH, "reachability: Machine -> [L5](y<6); [L3](y<=6)"; "P_Test3")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, L4](); [L6, L9]()"; "P_Test4")]
    #[should_panic]
    fn reachability_test_search_algorithm_panics(path: &str, query: &str) {
        match json_run_query(path, query) {
          QueryResult::Error(e) => panic!("{}", e),
          _ => println!("Expected error")
        }
    }
}
