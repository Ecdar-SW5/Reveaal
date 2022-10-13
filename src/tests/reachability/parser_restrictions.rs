#[cfg(test)]
mod reachability_parser_restrictions_test {
    use crate::{
        extract_system_rep, parse_queries, xml_parser, JsonProjectLoader, XmlProjectLoader,
    };
    use test_case::test_case;

    //These tests check the parsers validity checks, like an equal amount of parameters
    #[test_case("reachability: Adm2 -> [L21, _](); [L20]()";
    "Amount of machine and amount of location args does not match: 1 machine, 2 start-loc-args")]
    #[test_case("reachability: Adm2 -> [L21](); [L20, _]()";
    "Amount of machine and amount of location args does not match: 1 machine, 2 end-loc-args")]
    #[test_case("reachability: Adm2 || Machine -> [L21](); [L20]()";
    "Amount of machine and amount of location args does not match: 2 machines, 1 loc-arg")]
    // The amount of locations given as parameters must be the same as the amount of machines.
    fn query_parser_checks_invalid_amount_of_location_and_machine_args(parser_input: &str) {
        let folder_path = "samples/json/EcdarUniversity".to_string();
        let mut comp_loader = if xml_parser::is_xml_project(&folder_path) {
            XmlProjectLoader::new(folder_path)
        } else {
            JsonProjectLoader::new(folder_path)
        }
        .to_comp_loader();
        // Make query:
        let q = parse_queries::parse_to_query(parser_input);
        let queries = q.first().unwrap();

        // Runs the "validate_reachability" function from extract_system_rep, which we wish to test.
        match extract_system_rep::create_executable_query(queries, &mut *comp_loader){
            Err(_) => (),
            Ok(_) => panic!("Expected Err, recieved Ok")
        };

    }
    #[test_case("reachability: Adm2 -> [L21](); [L20]()";
    "Matching amount of locations and machines: 1 machine, 1 loc")]
    #[test_case("reachability: Adm2 || Machine -> [L21, L4](); [L20, L5]()";
    "Matching amount of locations and machines: 2 machines, 2 loc args")]
    // The amount of locations given as parameters must be the same as the amount of machines.
    fn query_parser_checks_valid_amount_of_location_and_machine_args(parser_input: &str) {
        let folder_path = "samples/json/EcdarUniversity".to_string();
        let mut comp_loader = if xml_parser::is_xml_project(&folder_path) {
            XmlProjectLoader::new(folder_path)
        } else {
            JsonProjectLoader::new(folder_path)
        }
        .to_comp_loader();
        // Make query:
        let q = parse_queries::parse_to_query(parser_input);
        let queries = q.first().unwrap();

        // Runs the "validate_reachability" function from extract_system_rep, which we wish to test.
        match extract_system_rep::create_executable_query(queries, &mut *comp_loader){
            Ok(_) => (),
            Err(_) => panic!("Expected Ok, recieved Err")
        };
    }
}
