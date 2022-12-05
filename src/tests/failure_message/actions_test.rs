#[cfg(test)]

mod test {

    use crate::tests::refinement::Helper::json_run_query;
    use crate::System::local_consistency::DeterminismFailure;
    use crate::System::{
        executable_query::QueryResult,
        local_consistency::{ConsistencyFailure, ConsistencyResult, DeterminismResult},
        refine::{RefinementFailure, RefinementResult},
    };
    use crate::TransitionSystems::LocationID;

    const PATH: &str = "samples/json/Actions";

    #[test]
    fn determinism_test() {
        let expected_action = String::from("1");
        let expected_location = LocationID::Simple {
            location_id: (String::from("L1")),
            component_id: Some(String::from("NonDeterminismCom")),
        };
        if let QueryResult::Determinism(DeterminismResult::Failure(
            DeterminismFailure::NotDeterministicFrom(actual_location, actual_action),
        )) = json_run_query(PATH, "determinism: NonDeterministic1")
        {
            assert_eq!(
                (expected_location, expected_action),
                (actual_location, actual_action)
            );
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
    }

    #[test]
    fn not_consistent_from_test() {
        let expected_action = String::from("");
        let expected_location = LocationID::Simple {
            location_id: (String::from("L17")),
            component_id: Some(String::from("notConsistent")),
        };
        if let QueryResult::Consistency(ConsistencyResult::Failure(
            ConsistencyFailure::NotConsistentFrom(actual_location, actual_action),
        )) = json_run_query(PATH, "consistency: NonConsistent")
        {
            assert_eq!(
                (expected_location, expected_action),
                (actual_location, actual_action)
            );
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
    }

    #[test]
    fn refinement_determinism_test() {
        let expected_action = String::from("1");
        let expected_location = LocationID::Simple {
            location_id: (String::from("L1")),
            component_id: Some(String::from("NonDeterminismCom")),
        };
        if let QueryResult::Refinement(RefinementResult::Failure(
            RefinementFailure::DeterminismFailure(actual_location, actual_action),
        )) = json_run_query(PATH, "refinement: NonDeterministic1 <= NonDeterministic2")
        {
            assert_eq!(
                (expected_location, expected_action),
                (actual_location.unwrap(), actual_action.unwrap())
            );
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
    }

    #[test]
    fn refinement_consistency_test() {
        let expected_action = String::from("");
        let expected_location = LocationID::Simple {
            location_id: (String::from("L17")),
            component_id: Some(String::from("notConsistent")),
        };

        if let QueryResult::Refinement(RefinementResult::Failure(
            RefinementFailure::ConsistencyFailure(actual_location, actual_action),
        )) = json_run_query(PATH, "refinement: NonConsistent <= CorrectComponent")
        {
            assert_eq!(
                (expected_location, expected_action),
                (actual_location.unwrap(), actual_action.unwrap())
            );
        } else {
            panic!("Models in saples/action have been changed, REVERT!");
        }
    }
}
