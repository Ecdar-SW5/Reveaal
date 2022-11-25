#[cfg(test)]
pub mod clock_removal_tests {
    use crate::DataReader::json_reader::read_json_component;
    use crate::TransitionSystems::transition_system::Heights;
    use crate::TransitionSystems::{CompiledComponent, TransitionSystem};
    use std::collections::HashSet;

    /*
    // Tests that the clocks that are never used in any guards are removed.
    #[test] // TODO: How removal? This is no longer done
    fn test_removal_unused_clocks() {
        let mut component = read_json_component(
            "samples/json/ClockReductionTest/UnusedClockWithCycle",
            "Component1",
        );

        let mut dim = component.declarations.clocks.len() + 1;
        let transition_system = CompiledComponent::compile(component.clone(), dim).unwrap();
        let redundant_clocks = transition_system.find_redundant_clocks(Heights::empty());

        component.reduce_clocks(redundant_clocks);
    }
     */

    #[test]
    fn test_check_declarations_unused_clocks_are_removed(){
        check_declarations_unused_clocks_are_removed("Component1", "x");
        check_declarations_unused_clocks_are_removed("Component2", "i");
        check_declarations_unused_clocks_are_removed("Component3", "c");
    }

    fn check_declarations_unused_clocks_are_removed(component_name: &str, clock: &str){
        let mut component = read_json_component("samples/json/ClockReductionTest/UnusedClock",
                                            component_name);

        let compiled_component = CompiledComponent::compile(component.clone(), component.declarations.clocks.len() + 1)
            .unwrap();

        let clock_index = component
            .declarations
            .get_clock_index_by_name(clock)
            .unwrap();

        component.remove_clock(*clock_index);

        let clock_reduced_compiled_component = CompiledComponent::compile(component.clone(), component.declarations.clocks.len() + 1)
            .unwrap();

        let decls = clock_reduced_compiled_component.get_decls();

        assert!(!decls[0].clocks.contains_key(clock));
    }



    #[test]
    fn test_check_declarations_duplicated_clocks_are_removed(){
        let mut component = read_json_component(
            "samples/json/ClockReductionTest/RedundantClocks",
            "Component1",
        );

        let compiled_component = CompiledComponent::compile(component.clone(), component.declarations.clocks.len() + 1);

        let clock_1_index = component.declarations.get_clock_index_by_name("x").unwrap();
        let mut duplicate_clocks_index = HashSet::new();
        duplicate_clocks_index.insert(*component.declarations.get_clock_index_by_name("y").unwrap());
        duplicate_clocks_index.insert(*component.declarations.get_clock_index_by_name("z").unwrap());

        component.replace_clock(*clock_1_index, &duplicate_clocks_index);

        let clock_reduced_compiled_component = CompiledComponent::compile(component.clone(), component.declarations.clocks.len() + 1).unwrap();

        let decls = clock_reduced_compiled_component.get_decls();

        assert_eq!(*decls[0].clocks.get_key_value("x").unwrap().1, 1);
        assert_eq!(*decls[0].clocks.get_key_value("y").unwrap().1, 1);
        assert_eq!(*decls[0].clocks.get_key_value("z").unwrap().1, 1);
    }
}
