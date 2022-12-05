#[cfg(test)]
pub mod test {
    use crate::tests::ClockReduction::helper::test::{
        assert_duplicate_clock_in_clock_reduction_instruction_vec, read_json_component_and_process,
    };
    use crate::TransitionSystems::{CompiledComponent, TransitionSystem};
    use edbm::util::constraints::ClockIndex;
    use std::collections::HashSet;

    const REDUNDANT_CLOCKS_TEST_PROJECT: &str = "samples/json/ClockReductionTest/RedundantClocks";
    const DIM: ClockIndex = 5; // TODO: Dim

    #[test]
    fn test_three_synced_clocks() {
        let expected_clocks = ["x".to_string(), "y".to_string(), "z".to_string()];
        let component =
            read_json_component_and_process(REDUNDANT_CLOCKS_TEST_PROJECT, "Component1");
        let compiled_component = CompiledComponent::compile(component.clone(), DIM).unwrap();
        let clock_index_x = component
            .declarations
            .get_clock_index_by_name(&expected_clocks[0])
            .unwrap();
        let clock_index_y = component
            .declarations
            .get_clock_index_by_name(&expected_clocks[1])
            .unwrap();
        let clock_index_z = component
            .declarations
            .get_clock_index_by_name(&expected_clocks[2])
            .unwrap();

        let instructions = compiled_component.find_redundant_clocks();

        assert_duplicate_clock_in_clock_reduction_instruction_vec(
            instructions,
            *clock_index_x,
            &HashSet::from([*clock_index_y, *clock_index_z]),
        );
    }
}
