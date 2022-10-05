#[cfg(test)]
pub mod test {
    use std::collections::HashSet;
    use crate::component::{ClockReductionReason, Component, RedundantClock};
    use crate::JsonProjectLoader;

    fn assert_duplicated_clock_detection(redundant_clocks: &Vec<RedundantClock>, expected_amount_to_reduce: u32, expected_duplicate_clocks: HashSet<&str>, unused_allowed: bool) {
        let mut global_clock: String = String::from("");

        let mut clocksReduced: HashSet<String> = HashSet::new();

        for redundancy in redundant_clocks {
            match &redundancy.reason {
                ClockReductionReason::Duplicate(replaced_by) => {
                    if global_clock == "" {
                        global_clock = replaced_by.clone();
                    }
                    assert!(expected_duplicate_clocks.contains(redundancy.clock.as_str()), "Clock ({}) was marked as duplicate unexpectedly", redundancy.clock);
                    assert!(!clocksReduced.contains(&redundancy.clock), "Clock ({}) was marked as duplicate multiple times", redundancy.clock);
                    assert_eq!(&global_clock, replaced_by, "Multiple clocks were chosen as global clock {} and {}", global_clock, replaced_by);
                    clocksReduced.insert(redundancy.clock.clone());
                }
                ClockReductionReason::Unused => {
                    assert!(unused_allowed, "Unexpected unused optimization");
                }
            }
        }

        assert_eq!(clocksReduced.len(), 2, "Too many clocks were reduced, expected only 2, got {}", clocksReduced.len());
    }

    fn load_component(project_path: &str, component_name: &str) -> Component {
        let project_loader = JsonProjectLoader::new(String::from(project_path));

        let mut component_loader = project_loader.to_comp_loader();

        let component = component_loader.get_component(component_name);

        return component.clone();
    }

    #[test]
    fn test_three_synced_clocks() {
        let component = load_component("samples/RedundantClocks", "Component1");

        let redundant_clocks = component.find_redundant_clocks();

        assert_duplicated_clock_detection(&redundant_clocks, 2, HashSet::from(["x", "y", "z"]), false);
    }
}