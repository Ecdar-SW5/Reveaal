#[cfg(test)]

mod test {
    use crate::System::refine::{RefinementFailure, RefinementResult};
    use crate::{tests::refinement::Helper::json_run_query, System::executable_query::QueryResult};

    const PATH: &str = "samples/json/RefinementTests";

    #[test]
    fn not_empty_result_test() {
        let actual = json_run_query(PATH, "refinement: A <= B");
        assert!(matches!(
            actual,
            QueryResult::Refinement(RefinementResult::Failure(
                RefinementFailure::NotEmptyResult(_)
            ))
        ));
    }

    #[test]
    fn empty_transition2s_test() {
        let actual = json_run_query(PATH, "refinement: A <= A2");
        assert!(matches!(
            actual,
            QueryResult::Refinement(RefinementResult::Failure(
                RefinementFailure::EmptyTransition2s(_)
            ))
        ));
    }

    #[test]
    fn cuts_delay_solutions_test() {
        let actual = json_run_query(PATH, "refinement: A2 <= B2");
        assert!(matches!(
            actual,
            QueryResult::Refinement(RefinementResult::Failure(
                RefinementFailure::CutsDelaySolutions(_)
            ))
        ));
    }

    #[test]
    fn initial_state_test() {
        let actual = json_run_query(PATH, "refinement: C <= D");
        assert!(matches!(
            actual,
            QueryResult::Refinement(RefinementResult::Failure(RefinementFailure::InitialState(
                _
            )))
        ));
    }

    #[test]
    fn not_disjoint_and_not_subset_test() {
        let actual = json_run_query(
            PATH,
            "refinement: notDisjointAndNotSubset1 <= notDisjointAndNotSubset2",
        );
        assert!(matches!(
            actual,
            QueryResult::Refinement(RefinementResult::Failure(
                RefinementFailure::NotDisjointAndNotSubset
            ))
        ));
    }

    #[test]
    fn not_subset_test() {
        let actual = json_run_query(PATH, "refinement: notSubset1 <= notSubset2");
        assert!(matches!(
            actual,
            QueryResult::Refinement(RefinementResult::Failure(RefinementFailure::NotSubset))
        ));
    }

    #[test]
    fn not_disjoint_test() {
        let actual = json_run_query(PATH, "refinement: disJoint2 <= disJoint1");
        assert!(matches!(
            actual,
            QueryResult::Refinement(RefinementResult::Failure(RefinementFailure::NotDisjoint))
        ));
    }
}
