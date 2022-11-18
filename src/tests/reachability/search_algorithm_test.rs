#[cfg(test)]
mod reachability_search_algorithm_test {
    use crate::component::Transition;
    use crate::tests::refinement::Helper::json_run_query;
    use crate::QueryResult;
    use test_case::test_case;

    const PATH: &str = "samples/json/EcdarUniversity";
    const PATH2: &str = "samples/json/AutomatonTestReachability";

    #[test_case(PATH, "reachability: Machine -> [L5](y<6); [L4](y<=6)", true; "Existing states and with right clocks")]
    #[test_case(PATH, "reachability: Machine -> [L5](); [L4](y>7)", false; "Exisiting locations but not possible with the clocks")]
    #[test_case(PATH, "reachability: Machine -> [L4](y<=6); [L5](y>=4)", true; "Switched the two states and with right clocks")]
    #[test_case(PATH, "reachability: Machine -> [L5](y<1); [L5](y<2)", true; "Same location, different clocks")]
    #[test_case(PATH, "reachability: Machine -> [L5](); [L5]()", true; "Same location, no clocks")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, L6](); [L4, L9]()", true; "Composition between Machine & Researcher, with existing locations and not clocks")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, U0](); [L5, L7]()", false; "No valid path from the two states")]
    #[test_case(PATH, "reachability: Researcher -> [U0](); [L7]()", false; "No possible path between to locations, locations exists in Researcher")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, L6](); [L4, _]()", true; "Machine || Researcher with Partial end state")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, L6](); [_, L9]()", true; "Machine || Researcher with Partial end state 2")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, U0](); [L5, _]()", true; "Machine || Researcher reachable with partial end state")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, U0](); [L4, _]()", true; "Machine || Researcher reachable with partial end state 2")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, U0](); [_, L7]()", false; "Machine || Researcher not reachable with partial end state")]
    #[test_case(PATH, "reachability: Researcher && Researcher -> [L7, _]()", true; "Machine || Researcher with partial state reachable from intial")]

    fn search_algorithm_returns_result_university(path: &str, query: &str, expected: bool) {
        match json_run_query(path, query) {
            QueryResult::Reachability(path) => assert_eq!(path.was_reachable, expected),
            _ => panic!("Inconsistent query result, expected Reachability"),
        }
    }

    #[test_case(PATH2, "reachability: Component1 -> [L1](); [L3]()", false; "False due to invariants")]
    #[test_case(PATH2, "reachability: Component2 -> [L4](); [L5]()", false; "False due to invariants, like the other")]
    #[test_case(PATH2, "reachability: Component3 -> [L6](); [L8]()", false; "False due to guards on the last transition")]
    #[test_case(PATH2, "reachability: Component1 -> [L0](); [L2]()", true; "It is possible to travel from L0 to L2 without specifiying guards")]
    #[test_case(PATH2, "reachability: Component4 -> [L9](); [L10]()", false; "False due to start state invariant and guard")]
    #[test_case(PATH2, "reachability: Component3 -> [L6](); [L7]()", true; "It is possible to travel from L6 to L7 without specifiying guards")]
    #[test_case(PATH2, "reachability: Component3 -> [L7](); [L8]()", true; "It is possible to travel from L7 to L8 without specifiying guards")]
    #[test_case(PATH2, "reachability: Component3 -> [L6](); [L7](x<5)", false; "It is not possible to travel from L6 to L7 due to specified guards")]
    #[test_case(PATH2, "reachability: Component3 -> [L7](x>4); [L8]()", false; "It is not possible to travel from L7 to L8 due to specified guards")]
    #[test_case(PATH2, "reachability: Component5 -> [L11](); [L12]()", true; "It is possible to travel from L11 to L12 due to update")]
    #[test_case(PATH2, "reachability: Component6 -> [L13](); [L15]()", true; "It is possible to travel from L13 to L15 due to the updates at L14")]
    #[test_case(PATH2, "reachability: Component7 -> [L16](); [L19]()", true; "Overwrite state of location once to reach end state")]
    #[test_case(PATH2, "reachability: Component8 -> [L20](); [L22]()", true; "Reset clock to reach end state")]
    #[test_case(PATH2, "reachability: Component7 -> [L16](); [L19](y<2)", false; "Unreachable due to second clock")]
    #[test_case(PATH2, "reachability: Component3 && Component3 -> [L6, L6](); [L7, L7]()", true; "Simple conjunction")]
    fn search_algorithm_returns_result(path: &str, query: &str, expected: bool) {
        match json_run_query(path, query) {
            QueryResult::Reachability(path) => assert_eq!(path.was_reachable, expected),
            _ => panic!("Inconsistent query result, expected Reachability"),
        }
    }

    #[test_case(PATH2, "reachability: Component1 -> [L0](); [L2]()", Vec::from(["E3", "E2"]); "Path in Component1 from L0 to L2")]
    #[test_case(PATH2, "reachability: Component3 -> [L6](); [L7]()", Vec::from(["E5"]); "Path in Component3 from L6 to L7")]
    #[test_case(PATH2, "reachability: Component3 -> [L7](); [L8]()", Vec::from(["E6"]); "Path in Component3 from L7 to L8")]
    #[test_case(PATH2, "reachability: Component5 -> [L11](); [L12]()", Vec::from(["E8"]); "Path in Component5 from L11 to L12")]
    #[test_case(PATH2, "reachability: Component6 -> [L13](); [L15]()", Vec::from(["E12", "E11", "E9", "E10", "E13"]); "Path in Component6 from L13 to L15")]
    #[test_case(PATH2, "reachability: Component7 -> [L16](); [L19]()", Vec::from(["E11", "E12", "E10"]); "Path in Component7 from L16 to L19")]
    #[test_case(PATH2, "reachability: Component8 -> [L20](); [L22]()", Vec::from(["E13", "E15", "E14"]); "Path in Component8 from L20 to L22")]
    #[test_case(PATH2, "reachability: Component9 -> [L23](x>5); [L26]()", Vec::from(["E17", "E18"]); "Path in Component9 from L23 x gt 5 to L26")]
    #[test_case(PATH2, "reachability: Component9 -> [L23](x<5); [L26]()", Vec::from(["E16", "E19"]); "Path in Component9 from L23 x lt 5 to L26")]
    #[test_case(PATH2, "reachability: Component3 && Component3 -> [L6, L6](); [L7, L7]()", Vec::from(["E5&&E5"]); "Path in Component3 && Component3 from L6 && L6 to L7 && L7")]
    fn path_gen_test_correct_path(folder_path: &str, query: &str, expected_path: Vec<&str>) {
        match json_run_query(folder_path, query) {
            QueryResult::Reachability(actual_path) => {
                assert!(
                    actual_path.was_reachable,
                    "Query: {}\nEnd state is not reachable from start state \n",
                    query
                );
                let path: Vec<Transition> = actual_path.path.unwrap();
                assert!(expected_path.len() == path.len(), "Query: {}\nThe length of the actual and expected are not the same.\nexpected_path.len = {}\nactual_path.len = {} \n", query, expected_path.len(),path.len());
                for i in 0..path.len() {
                    assert!(
                        expected_path[i] == path[i].id.to_string(),
                        "Query: {}\nThe actual and expected is not the same \n",
                        query
                    );
                }
            }
            _ => panic!("Inconsistent query result, expected Reachability"),
        }
    }
}
