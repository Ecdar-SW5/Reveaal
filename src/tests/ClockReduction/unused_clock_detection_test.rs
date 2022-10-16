
#[cfg(test)]
mod unused_clocks_tests {
    use std::collections::HashSet;
    use crate::DataReader::json_reader::read_json_component;
    use crate::tests::ClockReduction::helper::test::{assert_duplicated_clock_detection};

    fn unused_clocks_with_cycles(component_name: &str, unused_clock: &str) {
        let component = read_json_component("samples/json/ClockReductionTest/UnusedClockWithCycle", component_name);

        let unused_clocks = component.find_redundant_clocks();

        assert_duplicated_clock_detection(&unused_clocks, 1, HashSet::from([unused_clock]), true);
    }

    fn unused_clock(component_name: &str, unused_clock: &str) {
        let component = read_json_component("samples/json/ClockReductionTest/UnusedClock", component_name);

        let unused_clocks = component.find_redundant_clocks();

        assert_duplicated_clock_detection(&unused_clocks,1, HashSet::from([unused_clock]), true);
    }

    #[test]
    fn unused_clock_test(){
        unused_clocks_with_cycles("Component1","x");
        unused_clocks_with_cycles("Component2","z");
        unused_clocks_with_cycles("Component3","j");
        unused_clock("Component1", "x");
        unused_clock("Component2", "i");
        unused_clock("Component3", "c");
    }
}