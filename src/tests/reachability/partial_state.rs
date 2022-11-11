#[cfg(test)]
mod reachability_partial_states_test {
    use crate::TransitionSystems::LocationID;
    use test_case::test_case;

    #[test_case(LocationID::AnyLocation(),
                LocationID::Simple { location_id: "L9".to_string(), component_id: None };
                "_ == L9")]
    #[test_case(LocationID::Simple { location_id: "L7".to_string(), component_id: None }, 
                LocationID::AnyLocation();
                "L7 == _")]
    #[test_case(LocationID::Simple { location_id: "L5".to_string(), component_id: None }, 
                LocationID::Simple { location_id: "L5".to_string(), component_id: None };
                "L5 == L5")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple { location_id: "L5".to_string(), component_id: None }),Box::new(LocationID::AnyLocation())), 
                LocationID::Quotient(Box::new(LocationID::Simple { location_id: "L5".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }));
                "L5//_ == L5//L1")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Simple { location_id: "L5".to_string(), component_id: None }),Box::new(LocationID::AnyLocation())), 
                LocationID::Conjunction(Box::new(LocationID::AnyLocation()),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }));
                "L5&&_ == _&&L1")]
    #[test_case(LocationID::Composition(Box::new(LocationID::Simple { location_id: "L7".to_string(), component_id: None }),Box::new(LocationID::AnyLocation())), 
                LocationID::Composition(Box::new(LocationID::Simple { location_id: "L7".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }));
                "L7||_ == L7||L1")]
    #[test_case(LocationID::Composition(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation())),
                LocationID::Composition(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }));
                "_||_ == L2||L1")]
    #[test_case(LocationID::Composition(Box::new(LocationID::Composition(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()))),Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None })), 
                LocationID::Composition(Box::new(LocationID::Composition(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }))),Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }));
                "_||_||L2 == L2||L1||L2")]
    #[test_case(LocationID::Simple { location_id: "L_35".to_string(), component_id: None }, 
                LocationID::Simple { location_id: "L_35".to_string(), component_id: None };
                "L_35 == L_35")]
    fn checks_cmp_locations_returns_true(loc1: LocationID, loc2: LocationID) {
        assert!(loc1.compare_partial_locations(&loc2));
    }

    #[test_case(LocationID::Composition(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L5".to_string(), component_id: None })), 
                LocationID::Composition(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }));
                "L2||L5 != L2||L1")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L6".to_string(), component_id: None })), 
                LocationID::Composition(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }));
                "L2//L6 != L2||L1")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple { location_id: "L7".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L6".to_string(), component_id: None })), 
                LocationID::Conjunction(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }));
                "L7//L6 != L2&&L1")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::Simple { location_id: "L8".to_string(), component_id: None }),Box::new(LocationID::AnyLocation())), 
                LocationID::Conjunction(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }));
                "L8//_ != L2&&L1")]
    #[test_case(LocationID::AnyLocation(),
                LocationID::Conjunction(Box::new(LocationID::Simple { location_id: "L6".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }));
                "_ != L6&&L1")]
    #[test_case(LocationID::AnyLocation(),
                LocationID::Conjunction(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()));
                "anylocation _ != _&&_")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L4".to_string(), component_id: None })), 
                LocationID::Quotient(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L4".to_string(), component_id: None }));
                "L2&&L4 != L2\\L4")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Composition(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()))),Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None })), 
                LocationID::Composition(Box::new(LocationID::Composition(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L1".to_string(), component_id: None }))),Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }));
                "_||_&&L2 == L2||L1||L2")]
    #[test_case(LocationID::Conjunction(Box::new(LocationID::Composition(Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None }),Box::new(LocationID::AnyLocation()))),Box::new(LocationID::Simple { location_id: "L2".to_string(), component_id: None })), 
                LocationID::Conjunction(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation()));
                "L2||_&&L2 == _&&_")]
    #[test_case(LocationID::Simple { location_id: "L7".to_string(), component_id: None }, 
                LocationID::Simple { location_id: "L5".to_string(), component_id: None };
                "L7 != L5")]
    #[test_case(LocationID::Quotient(Box::new(LocationID::AnyLocation()),Box::new(LocationID::AnyLocation())),
                LocationID::Conjunction(Box::new(LocationID::Simple { location_id: "L6".to_string(), component_id: None }),Box::new(LocationID::Simple { location_id: "L25".to_string(), component_id: None }));
                "_//_ != L6&&L25")]
    #[test_case(LocationID::Simple { location_id: "_L1".to_string(), component_id: None }, 
                LocationID::Simple { location_id: "L1".to_string(), component_id: None };
                "_L1 != L1")]
    #[test_case(LocationID::Simple { location_id: "__".to_string(), component_id: None }, 
                LocationID::Simple { location_id: "L7".to_string(), component_id: None };
                "__ != L7")]
    fn checks_cmp_locations_returns_false(loc1: LocationID, loc2: LocationID) {
        assert!(!loc1.compare_partial_locations(&loc2));
    }
}
