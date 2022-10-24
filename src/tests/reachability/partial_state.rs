#[cfg(test)]
mod reachability_partial_states_test {
    use crate::TransitionSystems::LocationID;
    use test_case::test_case;

    #[test_case(LocationID::AnyLocation(),
                LocationID::Simple("L9".to_string());
                "_ == L9")]
    #[test_case(LocationID::Simple("L7".to_string()), 
                LocationID::AnyLocation();
                "L7 == _")]
    #[test_case(LocationID::Simple("L5".to_string()), 
                LocationID::Simple("L5".to_string());
                "L5 == L5")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple("L5".to_string())),Box::new(LocationID::AnyLocation())), 
                LocationID::Quotient(Box::new(LocationID::Simple("L5".to_string())),Box::new(LocationID::Simple("L1".to_string())));
                "L5//_ == L5//L1")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Simple("L5".to_string())),Box::new(LocationID::AnyLocation())), 
                LocationID::Conjunction(Box::new(LocationID::AnyLocation()),Box::new(LocationID::Simple("L1".to_string())));
                "L5&&_ == _&&L1")]
    #[test_case(LocationID::Composition(Box::new(LocationID::Simple("L7".to_string())),Box::new(LocationID::AnyLocation())), 
                LocationID::Composition(Box::new(LocationID::Simple("L7".to_string())),Box::new(LocationID::Simple("L1".to_string())));
                "L7||_ == L7||L1")]
    #[test_case(LocationID::Composition(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation())),
                LocationID::Composition(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L1".to_string())));
                "_||_ == L2||L1")]
    #[test_case(LocationID::Composition(Box::new(LocationID::Composition(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()))),Box::new(LocationID::Simple("L2".to_string()))), 
                LocationID::Composition(Box::new(LocationID::Composition(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L1".to_string())))),Box::new(LocationID::Simple("L2".to_string())));
                "_||_||L2 == L2||L1||L2")]
    #[test_case(LocationID::Simple("L_35".to_string()), 
                LocationID::Simple("L_35".to_string());
                "L_35 == L_35")]
    fn checks_cmp_locations_returns_true(loc1: LocationID, loc2: LocationID) {
        assert_eq!(loc1.cmp_locations(&loc2), true);
    }

    #[test_case(LocationID::Composition(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L5".to_string()))), 
                LocationID::Composition(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L1".to_string())));
                "L2||L5 != L2||L1")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L6".to_string()))), 
                LocationID::Composition(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L1".to_string())));
                "L2//L6 != L2||L1")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple("L7".to_string())),Box::new(LocationID::Simple("L6".to_string()))), 
                LocationID::Conjunction(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L1".to_string())));
                "L7//L6 != L2&&L1")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple("L8".to_string())),Box::new(LocationID::AnyLocation())), 
                LocationID::Conjunction(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L1".to_string())));
                "L8//_ != L2&&L1")]
    #[test_case(LocationID::AnyLocation(),
                LocationID::Conjunction(Box::new(LocationID::Simple("L6".to_string())),Box::new(LocationID::Simple("L1".to_string())));
                "_ != L6&&L1")]
    #[test_case(LocationID::AnyLocation(),
                LocationID::Conjunction(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()));
                "anylocation _ != _&&_")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L4".to_string()))), 
                LocationID::Quotient(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L4".to_string())));
                "L2&&L4 != L2\\L4")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Composition(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()))),Box::new(LocationID::Simple("L2".to_string()))), 
                LocationID::Composition(Box::new(LocationID::Composition(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::Simple("L1".to_string())))),Box::new(LocationID::Simple("L2".to_string())));
                "_||_&&L2 == L2||L1||L2")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Composition(Box::new(LocationID::Simple("L2".to_string())),Box::new(LocationID::AnyLocation()))),Box::new(LocationID::Simple("L2".to_string()))), 
                LocationID::Conjunction(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()));
                "L2||_&&L2 == _&&_")]
    #[test_case(LocationID::Simple("L7".to_string()), 
                LocationID::Simple("L5".to_string());
                "L7 != L5")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation())),
                LocationID::Conjunction(Box::new(LocationID::Simple("L6".to_string())),Box::new(LocationID::Simple("L25".to_string())));
                "_//_ != L6&&L25")]
    #[test_case(LocationID::Simple("_L1".to_string()), 
                LocationID::Simple("L1".to_string());
                "_L1 != L1")]
    #[test_case(LocationID::Simple("__".to_string()), 
                LocationID::Simple("L7".to_string());
                "__ != L7")]
    fn checks_cmp_locations_returns_false(loc1: LocationID, loc2: LocationID) {
        assert_ne!(loc1.cmp_locations(&loc2), true);
    }
}
