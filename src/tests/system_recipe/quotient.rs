#[cfg(test)]

mod test {
    use crate::{tests::refinement::Helper::json_run_query, QueryResult, System::local_consistency::{ConsistencyResult, ConsistencyFailure}};

    const PATH: &str = "samples/json/SystemRecipe/Quotient";

    #[test]
    fn quotient1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftQuotient1 / RightQuotient1");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(..)))
        ))
    }

    #[test]
    fn left_quotient_fails_correctly() {
        let actual = json_run_query(
            PATH,
            "consistency: NotDeterministicQuotientComp / DeterministicQuotientComp",
        );
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(..)))
        ))
    }

    #[test]
    fn right_quotient_fails_correctly() {
        let actual = json_run_query(
            PATH,
            "consistency: DeterministicQuotientComp / NotDeterministicQuotientComp",
        );
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(..)))
        ))
    }
}
