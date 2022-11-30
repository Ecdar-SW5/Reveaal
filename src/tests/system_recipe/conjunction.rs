#[cfg(test)]

mod test {
    use crate::{
        tests::refinement::Helper::json_run_query,
        QueryResult,
        System::local_consistency::{ConsistencyFailure, ConsistencyResult},
    };

    const PATH: &str = "samples/json/SystemRecipe/Conjunction";

    #[test]
    fn conjunction1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftConjunction1 && RightConjunction1");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }

    #[test]
    fn conjunction2_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftConjunction2 && RightConjunction2");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }

    #[test]
    fn conjunction3_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftConjunction3 && RightConjunction3");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }

    #[test]
    fn empty_conjunction_fails_correctly() {
        let actual = json_run_query(
            PATH,
            "consistency: LeftConjunctionEmpty && RightConjunctionEmpty",
        );
        // TODO: Determine why this fails
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }
}
