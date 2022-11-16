#[cfg(test)]
mod reachability_search_algorithm_test {
    use crate::component::Transition;
    use crate::tests::refinement::Helper::json_run_query;
    use crate::QueryResult;
    use std::fs::{self, File, OpenOptions};
    use std::io::prelude::*;
    use std::path::Path as PPath;
    use test_case::test_case;

    const PATH: &str = "samples/json/EcdarUniversity";
    const PATH2: &str = "samples/json/AutomatonTestReachability";


    #[test_case(TransitionID::Conjunction(
      TransitionID::Simple("a".to_string),
      TransitionID::Simple("b".to_string)
    ),
    vec![TransitionID::Simple("a".to_string), TransitionID::Simple("b".to_string)];
    "State gets parsed as not partial")]
    fn get_leaves_returns_correct_vector(id: TransitionID, expected: Vec<Vec<TransitionID>>) {
      assert_eq(id.get_leaves(), expected);
      /*
      
      match json_run_query(path, query) {
          QueryResult::Reachability(path) => assert_eq!(path.was_reachable, expected),
          _ => panic!("Inconsistent query result, expected Reachability"),
      } */
  }

}
