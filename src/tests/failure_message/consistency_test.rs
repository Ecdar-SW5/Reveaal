#[cfg(test)]

mod test {
    use crate::System::local_consistency::{ConsistencyFailure, ConsistencyResult};
    use crate::{tests::refinement::Helper::json_run_query, System::executable_query::QueryResult};

    const PATH: &str = "samples/json/ConsistencyTest";

    #[test]
    fn not_consistent_test() {
        let actual = json_run_query(PATH, "consistency: notConsistent");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(
                ConsistencyFailure::NotConsistentFrom(..)
            ))
        ));
    }
}
