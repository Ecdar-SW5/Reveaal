#[cfg(test)]
pub mod test {
    use crate::tests::ClockReduction::helper::test::{
        assert_clock_reason
    };
    use crate::DataReader::json_reader::read_json_component;
    use crate::TransitionSystems::{CompiledComponent, TransitionSystem};
    use edbm::util::constraints::ClockIndex;
    use std::collections::{HashMap, HashSet};

    const REDUNDANT_CLOCKS_TEST_PROJECT: &str = "samples/json/ClockReductionTest/RedundantClocks";
    const DIM: ClockIndex = 5; // TODO: Dim

    /*TODO: FIX
    #[test]
    fn test_three_synced_clocks() {
        let component = CompiledComponent::compile(
            read_json_component(REDUNDANT_CLOCKS_TEST_PROJECT, "Component1"),
            DIM,
        )
        .unwrap();
        let redundant_clocks = component.find_redundant_clocks();

        assert_clock_reason(&redundant_clocks, 2, HashSet::from(["x", "y", "z"]), false);
    }
    */

    #[test]
    fn test_three_synced_clocks_correct_location_target() {
        let component = CompiledComponent::compile(
            read_json_component(REDUNDANT_CLOCKS_TEST_PROJECT, "Component1"),
            DIM,
        )
        .unwrap();

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

        //assert_correct_edges_and_locations(&component, expected_locations, expected_edges);
        //assert_correct_edges_and_locations(&component, vec![], ("".to_string(), 0));
        //TODO
    }
}
