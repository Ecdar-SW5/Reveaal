#[cfg(test)]
pub mod test {
    const ADVANCED_CLOCK_REDUCTION_PATH: &str =
        "samples/json/ClockReductionTest/AdvancedClockReduction";

    use crate::extract_system_rep::SystemRecipe;
    use crate::tests::ClockReduction::helper::test::{
        assert_duplicate_clock_in_clock_reduction_instruction_vec,
        assert_unused_clock_in_clock_reduction_instruction_vec, create_clock_name_to_index,
        get_composition_transition_system, get_conjunction_system_recipe,
        get_conjunction_transition_system, read_json_component_and_process,
    };
    use crate::ProtobufServer::services::query_request::settings::ReduceClocksLevel::All;
    use crate::ProtobufServer::services::query_request::Settings;
    use crate::TransitionSystems::transition_system::{ClockReductionInstruction, Heights};
    use crate::TransitionSystems::TransitionSystem;
    use std::collections::HashSet;
    use std::path::Path;

    #[test]
    fn test_advanced_clock_removal() {
        let (dimensions, mut system_recipe) = get_conjunction_system_recipe(
            &Path::new(ADVANCED_CLOCK_REDUCTION_PATH).join("Conjunction/Example1"),
            "Component1",
            "Component2",
        );

        let system_recipe_copy = system_recipe.clone();

        let clock_reduction_instruction = system_recipe_copy
            .compile(dimensions)
            .unwrap()
            .find_redundant_clocks(Heights::empty());

        system_recipe.reduce_clocks(clock_reduction_instruction);

        //We let it use the unreduced amount of dimensions so we can catch the error
        //If a clock is not reduced
        let compiled = system_recipe.compile(dimensions).unwrap();

        let clock_name_to_index = create_clock_name_to_index(&compiled);

        for location in compiled.get_all_locations() {
            assert!(location.invariant.is_none(), "Should contain no invariants")
        }

        let graph = compiled.get_analysis_graph();
        for edge in &graph.edges {
            match format!("{}->{}", edge.from, edge.to).as_str() {
                "(Component1.L0&&Component2.L4)->(Component1.L1&&Component2.L5)" => {
                    assert_eq!(edge.guard_dependencies.len(), 2, "edge (Component1.L0&&Component2.L4)->(Component1.L1&&Component2.L5) should only have 1 guard dependency");
                    assert!(edge.guard_dependencies.is_subset(&HashSet::from([0, 1])));
                    assert_eq!(edge.updates.len(), 0, "(Component1.L0&&Component2.L4)->(Component1.L1&&Component2.L5) should have no updates");
                }
                "(Component1.L1&&Component2.L5)->(Component1.L2&&Component2.L6)" => {
                    assert_eq!(edge.guard_dependencies.len(), 0, "edge (Component1.L0&&Component2.L4)->(Component1.L1&&Component2.L5) should only have 2 guard dependency");
                    for update in &edge.updates {
                        assert_eq!(update.clock_index, 1, "edge (Component1.L0&&Component2.L4)->(Component1.L1&&Component2.L5) should only update clock 1");
                    }
                }
                "(Component1.L2&&Component2.L6)->(Component1.L3&&Component2.L7)" => {
                    assert_eq!(edge.guard_dependencies.len(), 0, "edge (Component1.L0&&Component2.L4)->(Component1.L1&&Component2.L5) should only have 1 guard dependency");
                    assert_eq!(edge.updates.len(), 0, "(Component1.L2&&Component2.L6)->(Component1.L3&&Component2.L7) should have no updates");
                }
                _ => assert!(false, "unknown edge"),
            }
        }
    }
}
