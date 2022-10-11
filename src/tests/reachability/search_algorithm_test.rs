#[cfg(test)]
mod search_algorithm_test{
    use test_case::test_case;
    use crate::tests::refinement::Helper::json_run_query;
    use crate::QueryResult;
    

    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y<6); [L4](y<=6)"; "T_Test1")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y>7); [L4](y<=6)"; "T_Test2")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L4](y<=6); [L5](y>=4)"; "T_Test3")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L4](y<=6); [L5](y<4)"; "T_Test4")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y<1); [L5](y<2)"; "T_Test5")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine || Researcher -> [L5, L6](); [L4, L9]()"; "T_Test6")]
    fn reachability_test_search_algorithm_return_true(path: &str, query: &str) {
        match json_run_query(path, query) {
          QueryResult::Reachability(b, _) => assert!(b), 
          QueryResult::Error(e) => panic!("{}", e),
          _ => panic!("Inconsistent query result, expected Reachability")
        }
    }
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y<6); [L4](y>7)"; "P_Test1")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L3](y<6); [L4](y<=6)"; "P_Test2")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y<6); [L3](y<=6)"; "P_Test3")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Reseacher -> [L9](x<2); [L7](x<=15)"; "P_Test4")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Reseacher -> [U0](); [L7](x<=15)"; "P_Test5")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine || Researcher -> [L5, L4](); [L6, L9]()"; "P_Test6")]
    #[should_panic]
    fn reachability_test_search_algorithm_panics(path: &str, query: &str) {
        match json_run_query(path, query) {
          QueryResult::Error(e) => panic!("{}", e),
          _ => println!("Expected error")
        }
    }
}
