#[cfg(test)]
mod reachability_search_algorithm_test {
    use crate::component::Transition;
    use crate::tests::refinement::Helper::json_run_query;
    use crate::QueryResult;
    use std::fs::{self, File, OpenOptions};
    use std::io::prelude::*;
    use std::path::Path as PPath;
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
    #[test_case(PATH, "reachability: Researcher && Researcher -> [U0, U0](); [U0, U0]()", true; "Trivially reachable")]
    #[test_case(PATH, "reachability: Researcher && Researcher -> [U0, U0](); [U0, U0](x>5)", true; "Trivially reachable but with clocks")]
    #[test_case(PATH, "reachability: Researcher && Researcher -> [U0, U0](); [L6, U0]()", false; "Trivially unreachable")]
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
        //TEMPORARY_MISSING_DECLERATIONS_HACK(path);

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
        //TEMPORARY_MISSING_DECLERATIONS_HACK(folder_path);

        match json_run_query(folder_path, query) {
            QueryResult::Reachability(actual_path) => {
                if actual_path.was_reachable {
                    let path: Vec<Transition> = actual_path.path.unwrap().clone();
                    if expected_path.len() != path.len() {
                        assert!(false);
                    }
                    for i in 0..path.len() {
                        if expected_path[i] != path[i].id.to_string() {
                            assert!(false);
                        }
                    }
                    assert!(true);
                } else {
                    assert!(true);
                }
            }
            _ => panic!("Inconsistent query result, expected Reachability"),
        }
    }

    fn TEMPORARY_MISSING_DECLERATIONS_HACK(path: &str) {
        if !PPath::new(&(path.to_owned() + "/SystemDeclarations.json")).exists() {
            // Add system declarations
            let mut declarations = String::new();
            let componentNames = fs::read_dir(path.to_string() + "/Components").unwrap();
            declarations += "{\n\"name\": \"System Declarations\",\n\"declarations\": \"system ";
            let mut first = true;
            for filename in componentNames {
                if !first {
                    declarations += ", ";
                }
                first = false;
                declarations = declarations
                    + &filename
                        .unwrap()
                        .file_name()
                        .into_string()
                        .unwrap()
                        .replace(".json", "");
            }
            declarations += "\"\n}";

            let mut file = File::create(path.to_owned() + "/SystemDeclarations.json").unwrap();
            file.write_all(declarations.as_bytes()).unwrap();

            // Set declarations in file
            let componentNames = fs::read_dir(path.to_string() + "/Components").unwrap();
            for filename in componentNames {
                let filenamestring = &filename.unwrap().file_name().into_string().unwrap();
                let contents =
                    fs::read_to_string(path.to_string() + "/Components/" + filenamestring).unwrap();
                let new = contents.replace(
                    "declarations\": \"\",",
                    "declarations\": \"clock x, y, z;\",",
                );

                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(path.to_string() + "/Components/" + filenamestring)
                    .unwrap();
                file.write(new.as_bytes());
            }
        }
    }
}
