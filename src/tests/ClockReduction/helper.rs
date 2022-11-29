pub mod test {
    use crate::component;
    use crate::component::{Edge, SyncType};
    use crate::DataReader::json_reader::read_json_component;
    use crate::TransitionSystems::transition_system::ClockReductionInstruction;
    use edbm::util::constraints::ClockIndex;
    use std::collections::HashSet;

    pub fn read_json_component_and_process(
        project_path: &str,
        component_name: &str,
    ) -> component::Component {
        let mut component = read_json_component(project_path, component_name);

        let input_edges: &mut Vec<Edge> = component.input_edges.insert(vec![]);
        let output_edges: &mut Vec<Edge> = component.output_edges.insert(vec![]);

        for edge in &component.edges {
            match edge.sync_type {
                SyncType::Input => input_edges.push(edge.clone()),
                SyncType::Output => output_edges.push(edge.clone()),
            };
        }

        component
    }

    /// Assert that a [`vec<&ClockReductionInstruction>`] contains an instruction that `clock` should
    /// be removed.
    pub(crate) fn assert_unused_clock_in_clock_reduction_instruction_vec(
        redundant_clocks: Vec<ClockReductionInstruction>,
        clock: ClockIndex,
    ) {
        assert!(redundant_clocks
            .iter()
            .any(|instruction| match instruction {
                ClockReductionInstruction::RemoveClock { clock_index } => {
                    println!("Found {}, searching for {}", clock_index, clock);
                    *clock_index == clock
                }
                _ => false,
            }));
    }
    /// Assert that a [`vec<&ClockReductionInstruction>`] contains an instruction that `clock` is a
    /// duplicate of the clocks in `clocks`.
    pub(crate) fn assert_duplicate_clock_in_clock_reduction_instruction_vec(
        redundant_clocks: Vec<ClockReductionInstruction>,
        clock: ClockIndex,
        clocks: &HashSet<ClockIndex>,
    ) {
        assert!(redundant_clocks
            .iter()
            .any(|instruction| match instruction {
                ClockReductionInstruction::RemoveClock { .. } => {
                    false
                }
                ClockReductionInstruction::ReplaceClocks {
                    clock_index,
                    clock_indices,
                } => {
                    *clock_index == clock && clock_indices == clocks
                }
            }));
    }
}
