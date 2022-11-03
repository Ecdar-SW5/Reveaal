#[cfg(test)]
mod reachability_transition_id_test {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use crate::DataReader::parse_queries;
    use crate::TransitionSystems::TransitionID;
    use crate::{
        extract_system_rep::create_executable_query, JsonProjectLoader, System::executable_query,
    };
    use crate::{
        tests::reachability::helper_functions::reachability_test_helper_functions,
        ModelObjects::representations::QueryExpression, System,
    };
    use test_case::test_case;
    const FOLDER_PATH: &str = "samples/json/EcdarUniversity";

    #[test_case(FOLDER_PATH, QueryExpression::VarName("Machine".to_string()), 
    vec![
        TransitionID::Simple("E25".to_string()), 
        TransitionID::Simple("E26".to_string()), 
        TransitionID::Simple("E27".to_string()), 
        TransitionID::Simple("E28".to_string()), 
        TransitionID::Simple("E29".to_string())])]
    fn transition_id_test(path: &str, machineExpression: QueryExpression, transition_ids: Vec<TransitionID>) {
        let mock_model = Box::new(machineExpression);
        let mut expected_ids: HashSet<&TransitionID> = HashSet::from_iter(transition_ids.iter());
        let (_, system) =
            reachability_test_helper_functions::create_system_recipe_and_machine(*mock_model, path);
        for loc in system.get_all_locations() {
            for ac in system.get_actions() {
                for tran in system.next_transitions(&loc, &ac){
                    if expected_ids.contains(&tran.id){
                        expected_ids.remove(&tran.id);
                    }
                    else{
                        panic!("Found unexpected ID in transition system: {}", &tran.id)
                    }
                }
            }
        }
        assert_eq!(expected_ids.len(), 0);
    }
}
