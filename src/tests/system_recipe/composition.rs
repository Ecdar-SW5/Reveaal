#[cfg(test)]

mod test {
    use crate::{
        extract_system_rep::SystemRecipeFailure,
        tests::refinement::Helper::json_run_query,
        QueryResult,
        System::local_consistency::{ConsistencyFailure, ConsistencyResult},
    };

    const PATH: &str = "samples/json/SystemRecipe/Composition";

    #[test]
    fn compostion1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftComposition1 || RightComposition1");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }

    #[test]
    fn composition1_fails_with_correct_actions() {
        let expected_actions = vec!["Output1".to_string()];
        if let QueryResult::Consistency(ConsistencyResult::Failure(
            ConsistencyFailure::NotDisjoint(SystemRecipeFailure { actions, .. }),
        )) = json_run_query(PATH, "consistency: LeftComposition1 || RightComposition1")
        {
            assert_eq!(actions, expected_actions);
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
    }

    #[test]
    fn compostion2_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftComposition2 || RightComposition2");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }

    #[test]
    fn composition2_fails_with_correct_actions() {
        let expected_actions = vec!["Output1".to_string(), "Output2".to_string()];
        if let QueryResult::Consistency(ConsistencyResult::Failure(
            ConsistencyFailure::NotDisjoint(SystemRecipeFailure { mut actions, .. }),
        )) = json_run_query(PATH, "consistency: LeftComposition2 || RightComposition2")
        {
            actions.sort();
            assert_eq!(actions, expected_actions);
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
    }

    #[test]
    fn compostion3_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftComposition3 || RightComposition3");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }

    #[test]
    fn composition3_fails_with_correct_actions() {
        let expected_actions = vec!["Output2".to_string()];
        if let QueryResult::Consistency(ConsistencyResult::Failure(
            ConsistencyFailure::NotDisjoint(SystemRecipeFailure { actions, .. }),
        )) = json_run_query(PATH, "consistency: LeftComposition3 || RightComposition3")
        {
            assert_eq!(actions, expected_actions);
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
    }
}
