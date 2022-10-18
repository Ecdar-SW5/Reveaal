#[cfg(test)]
pub mod test {
    use crate::tests::ClockReduction::helper::test::{
        assert_correct_edges_and_locations, assert_duplicated_clock_detection,
    };
    use crate::DataReader::json_reader::read_json_component;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_three_synced_clocks() {
        let component = read_json_component("samples/json/RedundantClocks", "Component1");

        let redundant_clocks = component.find_redundant_clocks();

        assert_duplicated_clock_detection(
            &redundant_clocks,
            2,
            HashSet::from(["x", "y", "z"]),
            false,
        );
    }

    #[test]
    fn test_three_synced_clocks_correct_location_target() {
        let component = read_json_component("samples/json/RedundantClocks", "Component1");

        let mut expected_locations: HashMap<String, HashSet<String>> = HashMap::new();

        expected_locations.insert("i".to_string(), HashSet::from(["L2".to_string()]));
        expected_locations.insert("x".to_string(), HashSet::from(["L1".to_string()]));
        expected_locations.insert("y".to_string(), HashSet::from(["L4".to_string()]));
        expected_locations.insert("z".to_string(), HashSet::from([]));

        let mut expected_edges: HashMap<String, HashSet<String>> = HashMap::new();
        expected_edges.insert(
            "i".to_string(),
            HashSet::from(["L1->L0".to_string(), "L0->L2".to_string()]),
        );
        expected_edges.insert(
            "x".to_string(),
            HashSet::from(["L2->L1".to_string(), "L0->L2".to_string()]),
        );
        expected_edges.insert("y".to_string(), HashSet::from(["L0->L4".to_string()]));
        expected_edges.insert("z".to_string(), HashSet::from(["L4->L2".to_string()]));

        assert_correct_edges_and_locations(&component, expected_locations, expected_edges);
    }
}
