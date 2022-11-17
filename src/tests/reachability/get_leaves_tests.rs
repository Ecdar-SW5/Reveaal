#[cfg(test)]
mod reachability_search_algorithm_test {
    use crate::component::Transition;
    use crate::tests::refinement::Helper::json_run_query;
    use crate::QueryResult;
    use crate::TransitionSystems::TransitionID;
    use std::fs::{self, File, OpenOptions};
    use std::io::prelude::*;
    use std::path::Path as PPath;
    use test_case::test_case;

    const PATH: &str = "samples/json/EcdarUniversity";
    const PATH2: &str = "samples/json/AutomatonTestReachability";

    #[test_case(TransitionID::Conjunction(
      Box::new(TransitionID::Simple("a".to_string())),
      Box::new(TransitionID::Simple("b".to_string()))
    ),
    vec![vec!(TransitionID::Simple("a".to_string())), vec!(TransitionID::Simple("b".to_string()))];
    "Simple conjunction")]
    #[test_case(TransitionID::Composition(
      Box::new(TransitionID::Simple("a".to_string())),
      Box::new(TransitionID::Simple("b".to_string()))
    ),
    vec![vec!(TransitionID::Simple("a".to_string())), vec!(TransitionID::Simple("b".to_string()))];
    "Simple composition")]
    #[test_case(TransitionID::Conjunction(
      Box::new(TransitionID::Conjunction(
        Box::new(TransitionID::Simple("a".to_string())),
        Box::new(TransitionID::Simple("b".to_string()))
      )),
      Box::new(TransitionID::Simple("c".to_string()))
    ),
    vec![vec!(TransitionID::Simple("a".to_string())), vec!(TransitionID::Simple("b".to_string())), vec!(TransitionID::Simple("c".to_string()))];
    "Simple nesting")]
    #[test_case(TransitionID::Composition(
      Box::new(TransitionID::Conjunction(
        Box::new(TransitionID::Simple("a".to_string())),
        Box::new(TransitionID::Composition(
          Box::new(TransitionID::Simple("b".to_string())),
          Box::new(TransitionID::Simple("c".to_string()))
        ))
      )),
      Box::new(TransitionID::Composition(
        Box::new(TransitionID::Simple("d".to_string())),
        Box::new(TransitionID::Simple("e".to_string()))
      ))
    ),
    vec![
      vec!(TransitionID::Simple("a".to_string())), 
      vec!(TransitionID::Simple("b".to_string())), 
      vec!(TransitionID::Simple("c".to_string())), 
      vec!(TransitionID::Simple("d".to_string())), 
      vec!(TransitionID::Simple("e".to_string()))];
    "Multiple conjunction and composition")]
    #[test_case(TransitionID::Quotient(
      1,
      vec!(TransitionID::Simple("a".to_string())),
      vec!(TransitionID::Simple("b".to_string()))
    ),
    vec![vec!(TransitionID::Simple("a".to_string())), vec!(TransitionID::Simple("b".to_string()))];
    "simple quotient")]
    #[test_case(TransitionID::Quotient(
      1,
      vec!(TransitionID::Simple("a".to_string()), TransitionID::Simple("b".to_string())),
      vec!(TransitionID::Simple("c".to_string()), TransitionID::Simple("d".to_string()), TransitionID::Simple("e".to_string()))
    ),
    vec![
      vec!(TransitionID::Simple("a".to_string()), TransitionID::Simple("b".to_string())), 
      vec!(TransitionID::Simple("c".to_string()), TransitionID::Simple("d".to_string()), TransitionID::Simple("e".to_string()))];
    "quotient with vec")]
    #[test_case(
        TransitionID::Conjunction(
            Box::new(
                TransitionID::Quotient(
                    1,
                    vec![
                        TransitionID::Conjunction(
                            Box::new(TransitionID::Simple("a".to_string())),
                            Box::new(TransitionID::Simple("b".to_string())), 
                        ),
                        TransitionID::Conjunction(
                            Box::new(TransitionID::Simple("c".to_string())),
                            Box::new(TransitionID::Simple("d".to_string())), 
                        )
                    ],
                    vec![TransitionID::Simple("e".to_string()), TransitionID::Simple("f".to_string())]
                )
            ),
            Box::new(TransitionID::Simple("g".to_string()))
        ),
        vec![
            vec!(TransitionID::Simple("a".to_string()), TransitionID::Simple("c".to_string())), 
            vec!(TransitionID::Simple("b".to_string()), TransitionID::Simple("d".to_string())),
            vec!(TransitionID::Simple("e".to_string()), TransitionID::Simple("f".to_string())),
            vec!(TransitionID::Simple("g".to_string()))];
        "Complex quotient")]
    fn get_leaves_returns_correct_vector(id: TransitionID, expected: Vec<Vec<TransitionID>>) {
        assert_eq!(id.get_leaves(), expected);
        /*

        match json_run_query(path, query) {
            QueryResult::Reachability(path) => assert_eq!(path.was_reachable, expected),
            _ => panic!("Inconsistent query result, expected Reachability"),
        } */
    }
}
