#[cfg(test)]
pub mod test {
    use crate::tests::ClockReduction::helper::test::assert_locations_and_edges_in_component;
    use crate::DataReader::json_reader::read_json_component;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    /// If clocks are never reset, a global clock should be used.
    /// Checks that clocks are replaced with a global clock in these cases and that other clocks
    /// are not removed.
    #[test]
    pub fn test_replace_clocks() {
        let mut component = read_json_component(
            "samples/json/ClockReductionTest/RedundantClocks",
            "Component1",
        );
        let clocks = HashSet::from(["x", "y", "z"]);

        let redundant_clocks = component.find_redundant_clocks();
        assert_eq!(
            redundant_clocks.len(),
            2,
            "Expected only two redundant clocks, but got {}",
            redundant_clocks.len()
        );
        let duplicate_clocks = HashSet::from([
            redundant_clocks[0].clock.as_str(),
            redundant_clocks[1].clock.as_str(),
        ]);

        let global_clock = Vec::from_iter(clocks.symmetric_difference(&duplicate_clocks));
        assert_eq!(
            global_clock.len(),
            1,
            "reduced only one clock, expected two"
        );

        let expected_locations: HashSet<String> = HashSet::from([
            "L2-i".to_string(),
            format!("L1-{}", global_clock[0].to_string()),
            format!("L4-{}", global_clock[0].to_string()),
            "L3-".to_string(),
            "L0-".to_string(),
        ]);

        let expected_edges: HashSet<String> = HashSet::from([
            "L1-i->L0".to_string(),
            format!("L2-{}->L1", global_clock[0].to_string()),
            format!("L0-i{}->L2", global_clock[0].to_string()),
            format!("L0-{}->L4", global_clock[0].to_string()),
            format!("L4-{}->L2", global_clock[0].to_string()),
            "L2-->L3".to_string(),
        ]);

        component.reduce_clocks(redundant_clocks);

        assert_locations_and_edges_in_component(&component, &expected_locations, &expected_edges);
    }
}
