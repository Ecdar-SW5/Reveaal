#[cfg(test)]

mod test {
    use crate::System::refine::{RefinementFailure, RefinementResult};
    use crate::{
        tests::refinement::Helper::json_run_query, System::executable_query::QueryResult,
        System::local_consistency::DeterminismResult,
    };

    const PATH: &str = "samples/json/Determinism";

    #[test]
    fn determinism_failure_test() {
        let actual = json_run_query(PATH, "determinism: NonDeterminismCom");

        assert!(matches!(
            actual,
            QueryResult::Determinism(DeterminismResult::Failure(..))
        ));
    }

    #[test]
    fn determinism_failure_in_refinement_test() {
        let actual = json_run_query(PATH, "refinement: NonDeterminismCom <= Component2");
        assert!(matches!(
            actual,
            QueryResult::Refinement(RefinementResult::Failure(
                RefinementFailure::DeterminismFailure(..)
            ))
        ));
    }
}
