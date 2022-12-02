#[cfg(test)]

mod test {
    use crate::{
        extract_system_rep::SystemRecipeFailure,
        tests::refinement::Helper::json_run_query,
        QueryResult,
        System::local_consistency::{ConsistencyFailure, ConsistencyResult},
    };

    const PATH: &str = "samples/json/SystemRecipe/Quotient";

    #[test]
    fn quotient1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftQuotient1 // RightQuotient1");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ))
    }

    #[test]
    fn quotient1_fails_with_correct_actions() {
        let expected_actions = vec!["Output1".to_string()];
        if let QueryResult::Consistency(ConsistencyResult::Failure(
            ConsistencyFailure::NotDisjoint(SystemRecipeFailure { actions, .. }),
        )) = json_run_query(PATH, "consistency: LeftQuotient1 // RightQuotient1")
        {
            assert_eq!(actions, expected_actions);
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
    }

    #[test]
    fn left_quotient_fails_correctly() {
        let actual = json_run_query(
            PATH,
            "consistency: NotDeterministicQuotientComp // DeterministicQuotientComp",
        );
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ))
    }

    #[test]
    fn right_quotient_fails_correctly() {
        let actual = json_run_query(
            PATH,
            "consistency: DeterministicQuotientComp // NotDeterministicQuotientComp",
        );
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ))
    }
}
