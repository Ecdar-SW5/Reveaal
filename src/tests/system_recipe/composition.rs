#[cfg(test)]

mod test {
    use crate::{tests::refinement::Helper::json_run_query, System::local_consistency::{ConsistencyResult, ConsistencyFailure}, QueryResult};

    const PATH: &str = "samples/json/SystemRecipe/Composition";

    #[test]
    fn compostion1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftComposition1 || RightComposition1");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(..)))
        ));
    }

    #[test]
    fn compostion2_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftComposition2 || RightComposition2");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(..)))
        ));
    }

    #[test]
    fn compostion3_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftComposition3 || RightComposition3");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(..)))
        ));
    }
}
