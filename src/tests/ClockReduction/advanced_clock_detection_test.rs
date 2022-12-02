#[cfg(test)]
pub mod test {
    use crate::component::Component;
    use crate::extract_system_rep::SystemRecipe;
    use crate::tests::ClockReduction::helper::test::{
        assert_duplicate_clock_in_clock_reduction_instruction_vec,
        assert_unused_clock_in_clock_reduction_instruction_vec,
        create_clock_name_to_index,
        get_composition_transition_system,
        get_conjunction_transition_system,
        read_json_component_and_process
    };
    use crate::DataReader::json_reader::read_json_component;
    use crate::System::save_component::{combine_components, PruningStrategy};
    use crate::TransitionSystems::{CompiledComponent, TransitionSystemPtr};
    use std::collections::{HashMap, HashSet};
    use std::path::Path;
    use edbm::util::constraints::ClockIndex;
    use crate::JsonProjectLoader;
    use crate::TransitionSystems::transition_system::{ClockReductionInstruction, Heights};
    use crate::ProtobufServer::services::query_request::Settings;
    use crate::System::executable_query::QueryResult;
    use crate::ProtobufServer::services::query_request::settings::ReduceClocksLevel::All;

    const ADVANCED_CLOCK_REDUCTION_PATH: &str = "samples/json/ClockReductionTest/AdvancedClockReduction";

    #[test]
    fn test_advanced_clock_detection() {
        let mut transition_system = get_conjunction_transition_system(
            &Path::new(ADVANCED_CLOCK_REDUCTION_PATH).join("Conjunction/Example1"),
            "Component1",
            "Component2",
        );

        let clock_name_to_index = create_clock_name_to_index(&transition_system);

        let clock_reduction_instruction = transition_system.find_redundant_clocks(Heights::empty());

        assert_eq!(clock_reduction_instruction.len(), 1, "Only one instruction needed");
        assert!(match (&clock_reduction_instruction[0]) {
            ClockReductionInstruction::RemoveClock {..} => false,
            ClockReductionInstruction::ReplaceClocks { clock_index, clock_indices } => {
                assert_eq!(clock_index, clock_name_to_index.get("component0:x").unwrap(), "Clocks get replaced by x");
                assert_eq!(clock_indices.len(), 1, "Only one clock should be replaced");
                assert!(clock_indices.contains(clock_name_to_index.get("component1:y").unwrap()), "Clock y can be replaced by x");
                true
            }
        }, "Clock reduction instruction is replace clocks");
    }

    #[test]
    fn test_advanced_same_component_clock_detection() {
        let mut transition_system = get_conjunction_transition_system(
            &Path::new(ADVANCED_CLOCK_REDUCTION_PATH).join("Conjunction/SameComponent"),
            "Component1",
            "Component1",
        );

        let clock_name_to_index = create_clock_name_to_index(&transition_system);

        let clock_reduction_instruction = transition_system.find_redundant_clocks(Heights::empty());

        assert_eq!(clock_reduction_instruction.len(), 1, "Only one instruction needed");
        assert!(match (&clock_reduction_instruction[0]) {
            ClockReductionInstruction::RemoveClock {..} => false,
            ClockReductionInstruction::ReplaceClocks { clock_index, clock_indices } => {
                assert_eq!(clock_index, clock_name_to_index.get("component0:x").unwrap(), "Clocks get replaced by component1:x");
                assert_eq!(clock_indices.len(), 1, "Only one clock should be replaced");
                assert!(clock_indices.contains(clock_name_to_index.get("component1:x").unwrap()), "Clock component2:x can be replaced by component1:x");
                true
            }
        }, "Clock reduction instruction is replace clocks");
    }

    #[test]
    fn test_conjunction_of_cyclical_component() {
        let mut transition_system = get_conjunction_transition_system(
            &Path::new(ADVANCED_CLOCK_REDUCTION_PATH).join("Conjunction/ConjunctionCyclic"),
            "Component1",
            "Component2"
        );

        let clock_name_to_index = create_clock_name_to_index(&transition_system);

        let clock_reduction_instruction = transition_system.find_redundant_clocks(Heights::empty());

        assert_eq!(clock_reduction_instruction.len(), 1, "Only one instruction needed");
        assert!(match (&clock_reduction_instruction[0]) {
            ClockReductionInstruction::RemoveClock {..} => false,
            ClockReductionInstruction::ReplaceClocks { clock_index, clock_indices } => {
                assert_eq!(clock_index, clock_name_to_index.get("component0:x").unwrap(), "Clocks get replaced by x");
                assert_eq!(clock_indices.len(), 1, "Only one clock should be replaced");
                assert!(clock_indices.contains(clock_name_to_index.get("component1:y").unwrap()), "Clock y can be replaced by x");
                true
            }
        }, "Clock reduction instruction is replace clocks");
    }

    #[test]
    fn test_composition_of_cyclical_component() {
        let mut transition_system = get_composition_transition_system(
            &Path::new(ADVANCED_CLOCK_REDUCTION_PATH).join("Composition/CyclicOnlyOutput"),
            "Component1",
            "Component2"
        );

        let clock_reduction_instruction = transition_system.find_redundant_clocks(Heights::empty());

        assert_eq!(clock_reduction_instruction.len(), 0, "No reduction is possible");
    }

    #[test]
    fn test_composition_of_component() {
        let mut transition_system = get_composition_transition_system(
            &Path::new(ADVANCED_CLOCK_REDUCTION_PATH).join("Composition/CyclicOnlyOutput"),
            "Component1",
            "Component2"
        );

        let clock_reduction_instruction = transition_system.find_redundant_clocks(Heights::empty());

        assert_eq!(clock_reduction_instruction.len(), 0, "No reduction is possible");
    }
}