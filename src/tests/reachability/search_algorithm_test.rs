#[cfg(test)]
mod reachability_search_algorithm_test{
    use test_case::test_case;
    use crate::tests::refinement::Helper::json_run_query;
    use crate::QueryResult;
    const PATH: &str = "samples/json/EcdarUniversity";
    const PATH2: &str = "samples/json/AutomatonTestReachability";
	use std::fs::{File, self, OpenOptions};
    use std::io::prelude::*;

    #[test_case(PATH, "reachability: Machine -> [L5](y<6); [L4](y<=6)", true; "Existing states and with right clocks")]
    #[test_case(PATH, "reachability: Machine -> [L5](); [L4](y>7)", false; "Exisiting locations but not possible with the clocks")]
    #[test_case(PATH, "reachability: Machine -> [L4](y<=6); [L5](y>=4)", true; "Switched the two states and with right clocks")]
    #[test_case(PATH, "reachability: Machine -> [L5](y<1); [L5](y<2)", true; "Same location, different clocks")]
    #[test_case(PATH, "reachability: Machine -> [L5](); [L5]()", true; "Same location, no clocks")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, L6](); [L4, L9]()", true; "Composition between Machine & Researcher, with existing locations and not clocks")]
    #[test_case(PATH, "reachability: Machine || Researcher -> [L5, U0](); [L5, L7]()", false; "No valid path from the two states")]
    #[test_case(PATH, "reachability: Researcher -> [U0](); [L7]()", false; "No possible path between to locations, locations exists in Researcher")]
    fn search_algorithm_returns_result_university(path: &str, query: &str, expected: bool) {
        match json_run_query(path, query) {
          QueryResult::Reachability(b, _) => assert_eq!(b, expected),
          _ => panic!("Inconsistent query result, expected Reachability")
        }
    }

    #[test_case(PATH2, "reachability: Component1 -> [L1](); [L3]()", false; "False due to invariants")]
    #[test_case(PATH2, "reachability: Component2 -> [L4](); [L5]()", false; "False due to invariants, like the other")]
    #[test_case(PATH2, "reachability: Component3 -> [L6](); [L8]()", false; "False due to guards on the last transition")]
	fn search_algorithm_returns_result(path: &str, query: &str, expected: bool) {

		TEMPORARY_MISSING_DECLERATIONS_HACK(path);

        match json_run_query(path, query) {
          QueryResult::Reachability(b, _) => assert_eq!(b, expected),
          _ => panic!("Inconsistent query result, expected Reachability")
        }
    }


    fn TEMPORARY_MISSING_DECLERATIONS_HACK(path: &str) {
		// Add system declarations
		let mut declarations = String::new();
		let componentNames = fs::read_dir(path.to_string() + "/Components").unwrap();
		declarations += "{\n\"name\": \"System Declarations\",\n\"declarations\": \"system ";
		let mut first = true;
		for filename in componentNames{
			if !first{
				declarations += ", ";
			}
			first = false;
			declarations = declarations + &filename.unwrap().file_name().into_string().unwrap().replace(".json", "");
		}
		declarations += "\"\n}";

		let mut file = File::create(path.to_owned() + "/SystemDeclarations.json").unwrap();
		file.write_all(declarations.as_bytes()).unwrap();

		// Set declarations in file
		let componentNames = fs::read_dir(path.to_string() + "/Components").unwrap();
		for filename in componentNames{
			let filenamestring = &filename.unwrap().file_name().into_string().unwrap();
			let contents = fs::read_to_string(path.to_string() + "/Components/" + filenamestring).unwrap();
			let new = contents.replace("declarations\": \"\",", "declarations\": \"clock x, y, z;\",");
		
			let mut file = OpenOptions::new().write(true).truncate(true).open(path.to_string() + "/Components/" + filenamestring).unwrap();
			file.write(new.as_bytes());		
		}
    }
}
