#[cfg(test)]
pub mod clock_removal_tests {
    use crate::tests::ClockReduction::helper::test::assert_edges_in_component;
    use crate::DataReader::json_reader::read_json_component;
    use crate::TransitionSystems::CompiledComponent;
    use std::collections::HashSet;

    /* TODO
    // Tests that the clocks that are never used in any guards are removed.
    #[test]
    fn test_removal_unused_clocks() {
        let mut component = read_json_component(
            "samples/json/ClockReductionTest/UnusedClockWithCycle",
            "Component1",
        );
        let redundant_clocks = component.find_redundant_clocks();

        component.reduce_clocks(redundant_clocks);

        assert_edges_in_component(
            &component,
            &HashSet::from([
                "L0-y->L1".to_string(),
                "L1-y->L0".to_string(),
                "L0-->L1".to_string(),
                "L1-y->L3".to_string(),
            ]),
        )
    }
     */
}
