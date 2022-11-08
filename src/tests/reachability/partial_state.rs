#[cfg(test)]
mod reachability_partial_states_test {
    use crate::TransitionSystems::location_id::SimpleID;
    use crate::TransitionSystems::LocationID;
    use test_case::test_case;

    #[test_case(LocationID::AnyLocation(),
                LocationID::Simple(SimpleID::new("L9".to_string(), None));
                "_ == L9")]
    #[test_case(LocationID::Simple(SimpleID::new("L7".to_string(), None)), 
                LocationID::AnyLocation();
                "L7 == _")]
    #[test_case(LocationID::Simple(SimpleID::new("L5".to_string(), None)), 
                LocationID::Simple(SimpleID::new("L5".to_string(), None));
                "L5 == L5")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple(SimpleID::new("L5".to_string(), None))),Box::new(LocationID::AnyLocation())), 
                LocationID::Quotient(Box::new(LocationID::Simple(SimpleID::new("L5".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))));
                "L5//_ == L5//L1")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Simple(SimpleID::new("L5".to_string(), None))),Box::new(LocationID::AnyLocation())), 
                LocationID::Conjunction(Box::new(LocationID::AnyLocation()),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))));
                "L5&&_ == _&&L1")]
    #[test_case(LocationID::Composition(Box::new(LocationID::Simple(SimpleID::new("L7".to_string(), None))),Box::new(LocationID::AnyLocation())), 
                LocationID::Composition(Box::new(LocationID::Simple(SimpleID::new("L7".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))));
                "L7||_ == L7||L1")]
    #[test_case(LocationID::Composition(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation())),
                LocationID::Composition(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))));
                "_||_ == L2||L1")]
    #[test_case(LocationID::Composition(Box::new(LocationID::Composition(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()))),Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None)))), 
                LocationID::Composition(Box::new(LocationID::Composition(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))))),Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))));
                "_||_||L2 == L2||L1||L2")]
    #[test_case(LocationID::Simple(SimpleID::new("L_35".to_string(), None)), 
                LocationID::Simple(SimpleID::new("L_35".to_string(), None));
                "L_35 == L_35")]
    fn checks_cmp_locations_returns_true(loc1: LocationID, loc2: LocationID) {
        assert!(loc1.compare_partial_locations(&loc2));
    }

    #[test_case(LocationID::Composition(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L5".to_string(), None)))), 
                LocationID::Composition(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))));
                "L2||L5 != L2||L1")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L6".to_string(), None)))), 
                LocationID::Composition(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))));
                "L2//L6 != L2||L1")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple(SimpleID::new("L7".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L6".to_string(), None)))), 
                LocationID::Conjunction(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))));
                "L7//L6 != L2&&L1")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple(SimpleID::new("L8".to_string(), None))),Box::new(LocationID::AnyLocation())), 
                LocationID::Conjunction(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))));
                "L8//_ != L2&&L1")]
    #[test_case(LocationID::AnyLocation(),
                LocationID::Conjunction(Box::new(LocationID::Simple(SimpleID::new("L6".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))));
                "_ != L6&&L1")]
    #[test_case(LocationID::AnyLocation(),
                LocationID::Conjunction(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()));
                "anylocation _ != _&&_")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L4".to_string(), None)))), 
                LocationID::Quotient(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L4".to_string(), None))));
                "L2&&L4 != L2\\L4")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Composition(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()))),Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None)))), 
                LocationID::Composition(Box::new(LocationID::Composition(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L1".to_string(), None))))),Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))));
                "_||_&&L2 == L2||L1||L2")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Composition(Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None))),Box::new(LocationID::AnyLocation()))),Box::new(LocationID::Simple(SimpleID::new("L2".to_string(), None)))), 
                LocationID::Conjunction(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()));
                "L2||_&&L2 == _&&_")]
    #[test_case(LocationID::Simple(SimpleID::new("L7".to_string(), None)), 
                LocationID::Simple(SimpleID::new("L5".to_string(), None));
                "L7 != L5")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation())),
                LocationID::Conjunction(Box::new(LocationID::Simple(SimpleID::new("L6".to_string(), None))),Box::new(LocationID::Simple(SimpleID::new("L25".to_string(), None))));
                "_//_ != L6&&L25")]
    #[test_case(LocationID::Simple(SimpleID::new("_L1".to_string(), None)), 
                LocationID::Simple(SimpleID::new("L1".to_string(), None));
                "_L1 != L1")]
    #[test_case(LocationID::Simple(SimpleID::new("__".to_string(), None)), 
                LocationID::Simple(SimpleID::new("L7".to_string(), None));
                "__ != L7")]
    fn checks_cmp_locations_returns_false(loc1: LocationID, loc2: LocationID) {
        assert!(!loc1.compare_partial_locations(&loc2));
    }
}
