#[cfg(test)]

mod test {
    use log::info;

    use crate::{
        extract_system_rep::SystemRecipeFailure,
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
    fn conjunction1_fails_with_correct_actions() {
        let expected_actions = vec!["Input1".to_string(), "Output1".to_string()];
        if let QueryResult::Consistency(ConsistencyResult::Failure(
            ConsistencyFailure::NotDisjoint(SystemRecipeFailure { actions, .. }),
        )) = json_run_query(PATH, "consistency: LeftConjunction1 && RightConjunction1")
        {
            assert_eq!(actions, expected_actions);
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
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
    fn conjunction2_fails_with_correct_actions() {
        let expected_actions = vec!["Input1".to_string()];
        if let QueryResult::Consistency(ConsistencyResult::Failure(
            ConsistencyFailure::NotDisjoint(SystemRecipeFailure { actions, .. }),
        )) = json_run_query(PATH, "consistency: LeftConjunction2 && RightConjunction2")
        {
            assert_eq!(actions, expected_actions);
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
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
    fn conjunction3_fails_with_correct_actions() {
        let expected_actions = vec!["Output1".to_string()];
        if let QueryResult::Consistency(ConsistencyResult::Failure(
            ConsistencyFailure::NotDisjoint(SystemRecipeFailure { actions, .. }),
        )) = json_run_query(PATH, "consistency: LeftConjunction3 && RightConjunction3")
        {
            assert_eq!(actions, expected_actions);
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
    }
}
