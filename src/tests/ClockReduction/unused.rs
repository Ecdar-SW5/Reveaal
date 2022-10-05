
#[cfg(test)]
mod unused_clocks_tests {
    use generic_array::typenum::assert_type_eq;
    use crate::DataReader::component_loader::JsonProjectLoader;

    static UNUSED_WITH_CYCLE: &str = "samples/json/ClockReductionTest/UnusedClockWithCycle";

    #[test]
    fn found_unused_clocks_with_cycles() {
        let project_loader = JsonProjectLoader::new(String::from(UNUSED_WITH_CYCLE));
        let component = project_loader.to_comp_loader();

        let unused_clocks = check_for_unused_clocks(component);

        assert!(unused_clocks.get(0).clock, "x");
    }

    #[test]
    fn found_unused_clock() {
        let project_loader = JsonProjectLoader::new(String::from(UNUSED_WITH_CYCLE));
        let component = project_loader.to_comp_loader();

        let unused_clocks = check_cycles(component);

        assert!(unused_clocks.get(0).clock, "x");
    }
}