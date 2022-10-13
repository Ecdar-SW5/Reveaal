pub mod reachability_test_helper_functions {
    use crate::DataReader::parse_invariant::parse;
    use crate::ModelObjects::representations::BoolExpression;
    use crate::ModelObjects::representations::QueryExpression;

    /// Helper function which converts a string to an option<box<BoolExpression>> by replacing ',' with "&&" and using the invariant parser.
    pub fn string_to_boolexpression(string: &str) -> Option<Box<BoolExpression>> {
        let string_in_invariant_format = &string.replace(',', "&&");
        if string_in_invariant_format.is_empty() {
            None
        } else {
            Some(Box::new(parse(string_in_invariant_format).unwrap()))
        }
    }
    /// Helper function which converts a string to a Vec<Box<QueryExpression::LocName("")>>>
    pub fn string_to_locations(string: &str) -> Vec<Box<QueryExpression>> {
        let mut v = vec![];
        let parsed_string = string.split(',').map(|s| s.trim());
        for s in parsed_string {
            v.push(Box::new(QueryExpression::LocName(s.to_string())));
        }
        v
    }

    /// Helper function to create the mock data
    pub fn create_mock_data_from_args(
        machine: &str,
        start_loc: &str,
        start_clocks: &str,
        end_loc: &str,
        end_clocks: &str,
    ) -> QueryExpression {
        let mock_l = Box::new(QueryExpression::VarName(machine.to_string()));
        let mock_m = Box::new(QueryExpression::State(
            string_to_locations(start_loc),
            string_to_boolexpression(start_clocks),
        ));
        let mock_r = Box::new(QueryExpression::State(
            string_to_locations(end_loc),
            string_to_boolexpression(end_clocks),
        ));
        QueryExpression::Reachability(mock_l, mock_m, mock_r)
    }
}
