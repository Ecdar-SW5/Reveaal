#[cfg(test)]
pub mod test {
    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;
    use std::ops::Deref;
    use crate::component::{ClockReductionReason, Component, RedundantClock};
    use crate::DataReader::json_reader::read_json_component;
    use crate::JsonProjectLoader;
    use crate::ModelObjects::representations::{ArithExpression, BoolExpression};

    #[test]
    pub fn test_test() {
        let component = read_json_component("samples/json/ReplacedClocks", "Clock replaced simpel");

        let troll = component.find_redundant_clocks();
        print!("{:?}", troll);
        //assert_duplicated_clock_detection(&redundant_clocks, 2, HashSet::from(["x", "y"]), false);
    }
}