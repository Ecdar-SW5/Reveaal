#[cfg(test)]
mod unused_clocks_tests {
    use crate::component::Component;
    use crate::tests::ClockReduction::helper::test::assert_clock_reason;
    use crate::DataReader::json_reader::read_json_component;
    use std::collections::HashSet;

    fn unused_clocks_with_cycles(component_name: &str, unused_clock: &str) {
        let component = read_json_component(
            "samples/json/ClockReductionTest/UnusedClockWithCycle",
            component_name,
        );

        unused_clocks_are_found(&component, HashSet::from([unused_clock]));
    }

    fn unused_clock(component_name: &str, unused_clock: &str) {
        let component = read_json_component(
            "samples/json/ClockReductionTest/UnusedClock",
            component_name,
        );

        unused_clocks_are_found(&component, HashSet::from([unused_clock]));
    }

    fn unused_clocks_are_found(component: &Component, unused_clocks: HashSet<&str>) {
        let redundant_clocks = component.find_redundant_clocks();
        assert_clock_reason(&redundant_clocks, 1, unused_clocks, true)
    }

    #[test]
    fn test_unused_clock_test() {
        unused_clocks_with_cycles("Component1", "x");
        unused_clocks_with_cycles("Component2", "z");
        unused_clocks_with_cycles("Component3", "j");
        unused_clock("Component1", "x");
        unused_clock("Component2", "i");
        unused_clock("Component3", "c");
    }
}
