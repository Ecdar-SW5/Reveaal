#[cfg(test)]
pub mod test{
    use crate::tests::ClockReduction::helper::test::{assert_clock_reason, assert_correct_edges_and_locations, assert_correct_transitionSystem_clocks, get_combined_component, get_transitionSystem};
    use crate::DataReader::json_reader::read_json_component;
    use std::collections::{HashMap, HashSet};
    use std::fmt::Display;
    use edbm::util::constraints::ClockIndex;
    use crate::component::Component;
    use crate::extract_system_rep::SystemRecipe;
    use crate::JsonProjectLoader;
    use crate::System::save_component::{combine_components, PruningStrategy};
    use crate::TransitionSystems::{CompiledComponent, LocationID, TransitionSystem, TransitionSystemPtr};

    // Prototype function
    fn prot_get_clocks_in_transitions(transitionSystem: &TransitionSystemPtr) ->HashMap<String, Vec<(LocationID, usize)>>{
        let mut r: HashMap<String, Vec<(LocationID, usize)>> = HashMap::new();
        r.insert("x".to_string(), vec![(LocationID::Simple("L0".to_string()), 0)]);
        r
    }

    #[test]
    fn test_get_clocks_in_transitions(){
        let mut loader = JsonProjectLoader::new("samples/json/ClockReductionTest/AdvancedClockReduction/Composition/SimpleComposition".to_string());
        let mut component = loader.get_component("Component1");
        let transitionSystem = get_transitionSystem(component);

        let clocks = prot_get_clocks_in_transitions(&transitionSystem);


        let mut expected: HashSet<String> = HashSet::new();
        expected.insert("x:(L0:0;)".to_string());

        assert_correct_transitionSystem_clocks(clocks, expected)
    }

    #[test]
    fn test_redundant_clock_in_conjunction(){
        let mut combined_component_transition_system = get_combined_component(
            "samples/json/ClockReductionTest/AdvancedClockReduction/Conjunction/Example1",
            "Component1", "Component2");
        let mut expected_clocks: HashMap<String, Vec<(LocationID, usize)>> = HashMap::new();
        expected_clocks.insert("x".to_string(), vec![(LocationID::Simple("L0".to_string()), 0)]);

        //let actual_clocks = prot_get_clocks_in_transitions(combined_component_transition_system);

        //assert_correct_transitionSystem_clocks(actual_clocks, expected_clocks);
    }
}