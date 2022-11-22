#[cfg(test)]
mod reachability_partial_states_test {
    use crate::component::{Declarations, Location};
    use crate::TransitionSystems::CompositionType;
    use crate::{
        component::LocationType,
        TransitionSystems::{LocationID, LocationTuple},
    };
    use test_case::test_case;

    fn build_location_tuple_helper(id: &str, location_type: LocationType) -> LocationTuple {
        LocationTuple::simple(
            &Location {
                id: id.to_string(),
                invariant: None,
                location_type,
                urgency: "".to_string(),
            },
            None,
            &Declarations::empty(),
            0,
        )
    }
    #[test_case(LocationTuple::build_any_location_tuple(),
                build_location_tuple_helper("L9", LocationType::Normal);
                "_ == L9")]
    #[test_case(build_location_tuple_helper("L0", LocationType::Initial),
                LocationTuple::build_any_location_tuple();
                "L0 == _")]
    #[test_case(build_location_tuple_helper("L5", LocationType::Normal),
                build_location_tuple_helper("L5", LocationType::Normal);
                "L5 == L5")]
    #[test_case(LocationTuple::merge_as_quotient(&build_location_tuple_helper("L5", LocationType::Normal), &LocationTuple::build_any_location_tuple()),
                LocationTuple::merge_as_quotient(&build_location_tuple_helper("L5", LocationType::Normal), &build_location_tuple_helper("L1", LocationType::Normal));
                "L5//_ == L5//L1")]
    #[test_case(LocationTuple::compose(&build_location_tuple_helper("L5", LocationType::Normal), &LocationTuple::build_any_location_tuple(), CompositionType::Conjunction),
                LocationTuple::compose(&LocationTuple::build_any_location_tuple(), &build_location_tuple_helper("L1", LocationType::Normal), CompositionType::Conjunction);
                "L5&&_ == _&&L1")]
    #[test_case(LocationTuple::compose(&build_location_tuple_helper("L7", LocationType::Normal), &LocationTuple::build_any_location_tuple(), CompositionType::Composition),
                LocationTuple::compose(&build_location_tuple_helper("L7", LocationType::Normal), &build_location_tuple_helper("L1", LocationType::Normal), CompositionType::Composition);
                "L7||_ == L7||L1")]
    #[test_case(LocationTuple::compose(&LocationTuple::build_any_location_tuple(), &LocationTuple::build_any_location_tuple(), CompositionType::Composition),
                LocationTuple::compose(&build_location_tuple_helper("L2", LocationType::Normal), &build_location_tuple_helper("L1", LocationType::Normal), CompositionType::Composition);
                "_||_ == L2||L1")]
    #[test_case(LocationTuple::compose(&LocationTuple::compose(&LocationTuple::build_any_location_tuple(), &LocationTuple::build_any_location_tuple(), CompositionType::Composition),&build_location_tuple_helper("L2", LocationType::Normal), CompositionType::Composition),
                LocationTuple::compose(&LocationTuple::compose(&build_location_tuple_helper("L2", LocationType::Normal), &build_location_tuple_helper("L1", LocationType::Normal), CompositionType::Composition),&build_location_tuple_helper("L2", LocationType::Normal), CompositionType::Composition);
                "_||_||L2 == L2||L1||L2")]
    #[test_case(build_location_tuple_helper("L_35", LocationType::Normal),
                build_location_tuple_helper("L_35", LocationType::Normal);
                "L_35 == L_35")]
    fn checks_cmp_locations_returns_true(loc1: LocationTuple, loc2: LocationTuple) {
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

#[macro_export]
    macro_rules! test1{
       ( ($($left:expr), +) (($op:"||"), +) (($right:expr), +)) == $($left1:expr)$($op:"||")$($right1:expr) ) =>{

        }
    }