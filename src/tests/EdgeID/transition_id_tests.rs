#[cfg(test)]
mod reachability_transition_id_test {
    use crate::TransitionSystems::TransitionID;
    use crate::{
        tests::reachability::helper_functions::reachability_test_helper_functions,
        ModelObjects::representations::QueryExpression,
    };
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use test_case::test_case;
    const FOLDER_PATH: &str = "samples/json/EcdarUniversity";

    #[test_case(FOLDER_PATH,
        QueryExpression::VarName("Machine".to_string()),
    vec![
        TransitionID::Simple("E25".to_string()),
        TransitionID::Simple("E26".to_string()),
        TransitionID::Simple("E27".to_string()),
        TransitionID::Simple("E28".to_string()),
        TransitionID::Simple("E29".to_string())]; "Simple transition id test")]
    #[test_case(FOLDER_PATH,
        QueryExpression::Conjunction(
            Box::new(QueryExpression::VarName("HalfAdm1".to_string())),
            Box::new(QueryExpression::VarName("HalfAdm2".to_string()))),
        vec![
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E43".to_string())),
                Box::new(TransitionID::Simple("E31".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E37".to_string())),
                Box::new(TransitionID::Simple("E34".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E42".to_string())),
                Box::new(TransitionID::Simple("E33".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E37".to_string())),
                Box::new(TransitionID::Simple("E35".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E42".to_string())),
                Box::new(TransitionID::Simple("E30".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E39".to_string())),
                Box::new(TransitionID::Simple("E31".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E38".to_string())),
                Box::new(TransitionID::Simple("E32".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E41".to_string())),
                Box::new(TransitionID::Simple("E34".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E40".to_string())),
                Box::new(TransitionID::Simple("E33".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E40".to_string())),
                Box::new(TransitionID::Simple("E30".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E38".to_string())),
                Box::new(TransitionID::Simple("E36".to_string()))
            ),
            TransitionID::Conjunction(
                Box::new(TransitionID::Simple("E41".to_string())),
                Box::new(TransitionID::Simple("E35".to_string()))
            )
            ]; "Conjunction HalfAdm1 and HalfAdm2")]
    #[test_case(FOLDER_PATH, QueryExpression::Quotient(
                Box::new(QueryExpression::VarName("HalfAdm1".to_string())),
                Box::new(QueryExpression::VarName("HalfAdm2".to_string()))
            ),
            vec![
              TransitionID::Simple("E25".to_string()),
              TransitionID::Simple("E26".to_string()),
              TransitionID::Simple("E27".to_string()),
              TransitionID::Simple("E28".to_string()),
              TransitionID::Simple("E29".to_string())]; "Quotient HalfAdm1 and HalfAdm2")]
    fn transition_id_test(
        path: &str,
        machineExpression: QueryExpression,
        transition_ids: Vec<TransitionID>,
    ) {
        let mock_model = Box::new(machineExpression);
        let mut expected_ids: HashSet<&TransitionID> = HashSet::from_iter(transition_ids.iter());
        let (_, system) =
            reachability_test_helper_functions::create_system_recipe_and_machine(*mock_model, path);
        for loc in system.get_all_locations() {
            for ac in system.get_actions() {
                for tran in system.next_transitions(&loc, &ac) {
                    println!("{}", &tran.id);
                }
            }
        }

        for loc in system.get_all_locations() {
            for ac in system.get_actions() {
                for tran in system.next_transitions(&loc, &ac) {
                    if expected_ids.contains(&tran.id) {
                        expected_ids.remove(&tran.id);
                    } else {
                        panic!("Found unexpected ID in transition system: {}", &tran.id)
                    }
                }
            }
        }
        assert_eq!(expected_ids.len(), 0);
    }
}
