#[cfg(test)]
pub mod test {
    const ADVANCED_CLOCK_REDUCTION_PATH: &str = "samples/json/ClockReductionTest/AdvancedClockReduction";

    use std::path::Path;
    use crate::tests::ClockReduction::helper::test::{assert_duplicate_clock_in_clock_reduction_instruction_vec, assert_unused_clock_in_clock_reduction_instruction_vec, create_clock_name_to_index, get_composition_transition_system, get_conjunction_system_recipe, get_conjunction_transition_system, read_json_component_and_process};
    use crate::TransitionSystems::transition_system::{ClockReductionInstruction, Heights};
    use crate::TransitionSystems::TransitionSystem;
    use crate::extract_system_rep::SystemRecipe;
    use crate::ProtobufServer::services::query_request::Settings;
    use crate::ProtobufServer::services::query_request::settings::ReduceClocksLevel::All;

    #[test]
    fn test_advanced_clock_removal() {
        let (dimensions, mut system_recipe) = get_conjunction_system_recipe(
            &Path::new(ADVANCED_CLOCK_REDUCTION_PATH).join("Conjunction/Example1"),
            "Component1",
            "Component2",
        );

        let system_recipe_copy = system_recipe.clone();

        let clock_reduction_instruction = system_recipe_copy.find_redundant_clocks(Heights::empty());

        system_recipe.reduce_clocks(clock_reduction_instruction);

        system_recipe.compile(1);

        println!("{:?}", system_recipe.compile(dimensions).unwrap().get_analysis_graph());
        assert_eq!(clock_reduction_instruction.len(), 1, "Only one instruction needed");
        assert!(match &clock_reduction_instruction[0] {
            ClockReductionInstruction::RemoveClock {..} => false,
            ClockReductionInstruction::ReplaceClocks { clock_index, clock_indices } => {
                assert_eq!(clock_index, clock_name_to_index.get("component0:x").unwrap(), "Clocks get replaced by x");
                assert_eq!(clock_indices.len(), 1, "Only one clock should be replaced");
                assert!(clock_indices.contains(clock_name_to_index.get("component1:y").unwrap()), "Clock y can be replaced by x");
                true
            }
        }, "Clock reduction instruction is replace clocks");
    }
}