
#[cfg(test)]
mod unused_clocks_tests {
    use std::collections::HashSet;
    use crate::component::{Component, RedundantClock};
    use crate::DataReader::component_loader::JsonProjectLoader;
    use crate::DataReader::json_reader::read_json_component;

    fn assert_unused_clocks_detection(unused_clocks: &Vec<RedundantClock>, expected_clocks: HashSet<&str> ,expected_amount: usize) {

        let mut clocks_reduced : HashSet<String> = HashSet::new();

        for unused_clock in unused_clocks {
            assert!(expected_clocks.contains(unused_clock.clock.as_str()), "Clock ({}) is not set as unused, but is not in expected", unused_clock.clock
                .as_str());
            assert!(!clocks_reduced.contains(&unused_clock.clock), "Clock {} has been removed multiple times", unused_clock.clock);

            clocks_reduced.insert(unused_clock.clock.clone());

        }
        assert_eq!(expected_amount, unused_clocks.len(), "{} clocks are unused, expected {} clocks", unused_clocks.len(), expected_amount);
    }


    #[test]
    fn unused_clocks_with_cycles() {
        let component = read_json_component("samples/json/ClockReductionTest/UnusedClockWithCycle", "Component1");

        let unused_clocks = component.find_redundant_clocks();

        assert_unused_clocks_detection(&unused_clocks, HashSet::from(["x"]), 1);
    }


    #[test]
    fn unused_clock() {
        let component = read_json_component("samples/json/ClockReductionTest/UnusedClock", "Component1");

        let unused_clocks = component.find_redundant_clocks();



        assert!(unused_clocks.get(0).clock, "x");
    }

}