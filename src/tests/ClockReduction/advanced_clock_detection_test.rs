#[cfg(test)]
pub mod test {
    use crate::component::Component;
    use crate::extract_system_rep::SystemRecipe;
    use crate::tests::ClockReduction::helper::test::{assert_duplicate_clock_in_clock_reduction_instruction_vec, assert_unused_clock_in_clock_reduction_instruction_vec, read_json_component_and_process};
    use crate::DataReader::json_reader::read_json_component;
    use crate::System::save_component::{combine_components, PruningStrategy};
    use crate::TransitionSystems::{CompiledComponent, TransitionSystemPtr};
    use std::collections::{HashMap, HashSet};
    use edbm::util::constraints::ClockIndex;
    use crate::JsonProjectLoader;
    use crate::TransitionSystems::transition_system::{ClockReductionInstruction, Heights};
    use crate::ProtobufServer::services::query_request::Settings;
    use crate::System::executable_query::QueryResult;
    use crate::ProtobufServer::services::query_request::settings::ReduceClocksLevel::All;
    use crate::TransitionSystems::transition_system::ClockReductionInstruction::ReplaceClocks;

    fn get_conjunction_transition_system(path: &str, comp1: &str, comp2: &str) -> TransitionSystemPtr {
        let project_loader = JsonProjectLoader::new(path.to_string(), Settings {
            reduce_clocks_level: Some(All(true)),
        });

        let mut component_loader = project_loader.to_comp_loader();

        let mut next_clock_index: usize = 0;
        let mut component1 = component_loader.get_component(comp1).clone();
        let mut component2 = component_loader.get_component(comp2).clone();


        component1.set_clock_indices(&mut next_clock_index);
        component2.set_clock_indices(&mut next_clock_index);

        let dimensions = component1.declarations.clocks.len() + component2.declarations.clocks.len();

        let sr_component1 = Box::new(SystemRecipe::Component(Box::new(component1)));
        let sr_component2 = Box::new(SystemRecipe::Component(Box::new(component2)));

        let conjunction = SystemRecipe::Conjunction(sr_component1, sr_component2);
        conjunction
            .compile(dimensions)
            .unwrap()
    }

    #[test]
    fn test_advanced_clock_detection() {
        let mut transition_system = get_conjunction_transition_system(
            "samples/json/ClockReductionTest/AdvancedClockReduction/Conjunction/SameComponent",
            "Component1",
            "Component1",
        );

        let mut clock_name_to_index: HashMap<String, ClockIndex> = HashMap::new();

        for (i, declaration) in (&transition_system.get_decls()).iter().enumerate() {
            for (clock_name, clock_index) in &declaration.clocks {
                clock_name_to_index.insert(format!("component{}:{}", i, clock_name), i);
            }
        }


        println!("{:?}", clock_name_to_index);

        let clock_reduction_instruction = transition_system.find_redundant_clocks(Heights::empty());
        assert_eq!(clock_reduction_instruction.len(), 1);
        //assert_eq!(ClockReductionInstruction::ReplaceClocks(clock_reduction_instruction[0].))
        assert_duplicate_clock_in_clock_reduction_instruction_vec(
            clock_reduction_instruction,
            *clock_name_to_index.get("component0:x").unwrap(),
            &HashSet::from([
                clock_name_to_index.get("component1:y").unwrap().clone()
            ])
        );

        //assert_clock_reason(&redundantClocks, 3, HashSet::from(["x", "y"]), true);
    }

    fn test_advanced_clock_removal() {
        let mut combinedComponent = get_conjunction_transition_system(
            "samples/json/ClockReduction/AdvancedClockReduction",
            "Component1",
            "Component2",
        );

        /*let redundantClocks = combinedComponent.find_redundant_clocks();

        combinedComponent.reduce_clocks(&redundantClocks);

        assert_correct_edges_and_locations(combinedComponent);*/
    }
}