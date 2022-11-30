#[cfg(test)]

mod test {
    use crate::{
        tests::refinement::Helper::json_run_query,
        QueryResult,
        System::local_consistency::{ConsistencyFailure, ConsistencyResult},
    };

    const PATH: &str = "samples/json/SystemRecipe/CompiledComponent";

    #[test]
    fn compiled_component1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: CompiledComponent1");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }

    #[test]
    fn compiled_component2_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: CompiledComponent2");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }

    #[test]
    fn compiled_component3_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: CompiledComponent3");
        assert!(matches!(
            actual,
            QueryResult::Consistency(ConsistencyResult::Failure(ConsistencyFailure::NotDisjoint(
                ..
            )))
        ));
    }
}
