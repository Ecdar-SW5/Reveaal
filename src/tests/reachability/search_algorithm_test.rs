#[cfg(test)]
mod search_algorithm_test{
    use test_case::test_case;
    use crate::tests::refinement::Helper::json_run_query;
    use crate::QueryResult;
    

    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y<6); [L4](y<=6)"; "Test1")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y>7); [L4](y<=6)"; "Test2")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L4](y<=6); [L5](y>=4)"; "Test3")]//This one makes an error but should work.
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L4](y<=6); [L5](y<4)"; "Test4")] //This one makes an error but should work.
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y<1); [L5](y<2)"; "Test5")]
    fn reachability_test_search_algorithme_return_true(path: &str, query: &str) {
        match json_run_query(path, query) {
          QueryResult::Reachability(b, _) => assert!(b), 
          QueryResult::Error(e) => panic!("{}", e),
          _ => panic!("Inconsistent query result, expected Reachability")
        }
    }
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y<6); [L4](y>7)"; "Test1")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L3](y<6); [L4](y<=6)"; "Test2")]
    #[test_case("samples/json/EcdarUniversity", "reachability: Machine -> [L5](y<6); [L3](y<=6)"; "Test3")]
    
    #[should_panic]
    fn reachability_test_search_algorithme_panics(path: &str, query: &str) {
        match json_run_query(path, query) {
          QueryResult::Error(e) => panic!("{}", e),
          _ => println!("Expected error")
        }
    }
}
