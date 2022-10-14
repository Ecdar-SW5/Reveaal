
#[cfg(test)]
pub mod clock_removal_tests{
    use std::collections::HashSet;
    use crate::DataReader::json_reader::read_json_component;
    use crate::component::RedundantClock;
    use crate::tests::ClockReduction::helper::test::assert_removed_unused_clocks;

    #[test]
    fn test_removal_unused_clocks(){
        let mut component = read_json_component("samples/json/ClockReductionTest/UnusedClockWithCycle", "Component1");

        component.reduce_clocks();

        assert_removed_unused_clocks(&component, HashSet::from(["L0-y->L1".to_string(), "L1-y->L0".to_string(), "L0-->L1".to_string(), "L1-y->L3".to_string()]))
    }
}