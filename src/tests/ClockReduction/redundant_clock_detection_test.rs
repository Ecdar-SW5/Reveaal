#[cfg(test)]
pub mod test {
    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;
    use std::ops::Deref;
    use crate::component::{ClockReductionReason, Component, RedundantClock};
    use crate::DataReader::json_reader::read_json_component;
    use crate::JsonProjectLoader;
    use crate::ModelObjects::representations::{ArithExpression, BoolExpression};

    #[test]
    fn test_three_synced_clocks() {
        let component = read_json_component("samples/json/RedundantClocks", "Component1");

        let redundant_clocks = component.find_redundant_clocks();

        assert_duplicated_clock_detection(&redundant_clocks, 2, HashSet::from(["x", "y", "z"]), false);
    }

    #[test]
    fn test_three_synced_clocks_correct_targeting() {
        let component = read_json_component("samples/json/RedundantClocks", "Component1");

        let mut expected_locations: HashMap<String, HashSet<usize>> = HashMap::new();
        for clock in ["x", "y", "z", "i"] {
            expected_locations.insert(String::from(clock), HashSet::new());
        }


        let redundant_clocks = component.find_redundant_clocks();


        for (i, location) in component.locations.into_iter().enumerate() {
            if let Some(invariant) = location.invariant {
                let mut dependent_clocks: HashSet<String> = HashSet::new();
                let dependent_clocks = get_dependent_clocks(&invariant, &mut dependent_clocks);
                println!("{}", dependent_clocks);
            }
        }

        for redundancy in redundant_clocks {

        }
    }
}