
#[cfg(test)]
pub mod clock_removal_tests{
    use crate::DataReader::json_reader::read_json_component;
    use crate::component::RedundantClock;

    #[test]
    fn test_removal_unused_clocks(){
        let mut component = read_json_component("samples/json/ClockReductionTest/UnusedClockWithCycle", "Component1");

        component.reduce_clocks();
        let r = component.find_redundant_clocks();

        assert!(r.is_empty())
    }
}