#[cfg(test)]
pub mod test {
    use std::collections::{HashMap, HashSet};
    use std::fmt::format;
    use std::hash::Hash;
    use std::iter::FromIterator;
    use std::ops::Deref;
    use crate::component::{ClockReductionReason, Component, RedundantClock};
    use crate::DataReader::json_reader::read_json_component;
    use crate::JsonProjectLoader;
    use crate::ModelObjects::representations::{ArithExpression, BoolExpression};
    use crate::tests::ClockReduction::helper::test::{assert_locations_replaced_clocks, assert_removed_unused_clocks};

    #[test]
    pub fn test_test() {
        let mut component = read_json_component("samples/json/RedundantClocks", "Component1");
        let mut clocks = HashSet::from(["x","y","z"]);

        let redundant_clocks = component.find_redundant_clocks();
        assert!(redundant_clocks.len() == 2, "Expected only two redundant clocks, but got {}", redundant_clocks.len());
        let duplicate_clocks = HashSet::from([redundant_clocks[0].clock.as_str(), redundant_clocks[1].clock.as_str()]);

        let global_clock = Vec::from_iter(clocks.symmetric_difference(&duplicate_clocks));
        assert!(global_clock.len() == 1, "reduced only one clock, expected two");

        let mut expected_locations: HashSet<String> = HashSet::from(["L2-i".to_string(), format!("L1-{}",global_clock[0].to_string()), format!("L4-{}",global_clock[0].to_string()), "L3-".to_string(), "L0-".to_string()]);

        let mut expected_edges: HashSet<String> = HashSet::from(["L1-i->L0".to_string(), "L0-i->L2".to_string(),format!("L2-{}->L1",global_clock[0].to_string()),
                                                                format!("L0-{}->L2", global_clock[0].to_string()), format!("L0-{}->L4", global_clock[0].to_string()), format!("L4-{}->L2", global_clock[0].to_string())]);

        component.reduce_clocks();

        //assert_locations_replaced_clocks(&component, expected_locations);
        assert_removed_unused_clocks(&component, expected_edges);




        /*expected_edges.insert(redundant_clocks[0].clock.to_string(),HashSet::from([]));
        expected_edges.insert(redundant_clocks[1].clock.to_string(),HashSet::from([]));
        expected_edges.insert("i".to_string(), HashSet::from(["L1->L0".to_string(), "L0->L2".to_string()]));
        expected_edges.insert(global_clock[0].to_string(), HashSet::from(["L2->L1".to_string(), "L0->L2".to_string(),"L0->L4".to_string(),"L4->L2".to_string()]));
        */
        //Find location og edges:
        //Hardcode hvordan det skal ende
        //Lav functionalitet i still af joms der fjerne og tjekker at den er gone fra alle de locations
        //redundantclocks ved hvor de er brugt - ud fra dette ved vi hvor vi skal tjekke om de er gone


        //Find location og edges hvor clocken bruges tjek at efter reduce_clocks at der nu bruges det nye clock i stedet.


        //assert_duplicated_clock_detection(&redundant_clocks, 2, HashSet::from(["x", "y"]), false);
    }
}