pub mod test {
    use std::collections::{HashMap, HashSet};
    use std::iter::FromIterator;
    use std::ops::Deref;
    use crate::component::{ClockReductionReason, Component, RedundantClock};
    use crate::ModelObjects::representations::{ArithExpression, BoolExpression};

    pub fn assert_locations_replaced_clocks(
        component: &Component,
        expected_locations: HashSet<String>
    ){
        let mut actual_locations: HashSet<String> = HashSet::new();

        for location in &component.locations {
            let mut dependentClocks = HashSet::new();
            if let Some(invariant) = &location.invariant {
                get_dependent_clocks(&invariant, &mut dependentClocks);
            }

           let clock = sort_clocks_and_join(&dependentClocks);

            actual_locations.insert(format!("{}-{}",location.id.clone(), clock));
        }
            assert!(expected_locations.is_subset(&actual_locations) && expected_locations.len() == actual_locations.len(), "Expected these locations {:?}, but got {:?}", expected_locations, actual_locations);
    }

    pub fn sort_clocks_and_join(dependent_clocks: &HashSet<String>) -> String{
        let mut dependent_clocks_vec = Vec::from_iter(dependent_clocks.iter());
        let mut sorted_clocks = String::new();
        dependent_clocks_vec.sort();

        for clock in dependent_clocks_vec{
            sorted_clocks = sorted_clocks + clock;
        }
        sorted_clocks
    }

    pub fn assert_removed_unused_clocks(
        component: &Component,
        expected_edges: HashSet<String>
    ){
        let mut actual_edges: HashSet<String> = HashSet::new();

        for edge in &component.edges {
            let mut dependent_clocks = HashSet::new();
            if let Some(guard) = &edge.guard {
                get_dependent_clocks( &guard, &mut dependent_clocks);
            }

            let sorted_clocks = sort_clocks_and_join(&dependent_clocks);


            let edge_id = format!("{}-{}->{}", edge.source_location, sorted_clocks, edge.target_location);

            assert!(!actual_edges.contains(&edge_id), "Duplicate edge: {}", edge_id);

            actual_edges.insert(edge_id);
        }
        assert!(expected_edges.is_subset(&actual_edges) && expected_edges.len() == actual_edges.len(), "Expected these edges {:?} but got {:?}", expected_edges, actual_edges)
    }

    pub fn assert_duplicated_clock_detection(redundant_clocks: &Vec<RedundantClock>, expected_amount_to_reduce: u32, expected_duplicate_clocks: HashSet<&str>, unused_allowed: bool) {
        let mut global_clock: String = String::from("");

        let mut clocksReduced: HashSet<String> = HashSet::new();

        for redundancy in redundant_clocks {
            match &redundancy.reason {
                ClockReductionReason::Duplicate(replaced_by) => {
                    if global_clock == "" {
                        global_clock = replaced_by.clone();
                    }
                    assert!(expected_duplicate_clocks.contains(redundancy.clock.as_str()), "Clock ({}) was marked as duplicate unexpectedly", redundancy.clock);
                    assert!(!clocksReduced.contains(&redundancy.clock), "Clock ({}) was marked as duplicate multiple times", redundancy.clock);
                    assert_eq!(&global_clock, replaced_by, "Multiple clocks were chosen as global clock {} and {}", global_clock, replaced_by);
                    clocksReduced.insert(redundancy.clock.clone());
                }
                ClockReductionReason::Unused => {
                    assert!(unused_allowed, "Unexpected unused optimization");
                    assert!(expected_duplicate_clocks.contains(&redundancy.clock.as_str()), "Clock ({}) is not set as unused, but is not in expected", redundancy.clock
                        .as_str());
                    assert!(!clocksReduced.contains(&redundancy.clock), "Clock {} has been removed multiple times", redundancy.clock);
                    clocksReduced.insert(redundancy.clock.clone());
                }
            }
        }
        assert_eq!(clocksReduced.len(), expected_amount_to_reduce as usize, "Too many clocks were reduced, expected only {}, got {}",expected_amount_to_reduce, clocksReduced.len());
    }

    pub fn get_dependent_clocks(expr: &BoolExpression, out: &mut HashSet<String>) {
        match expr.deref() {
            BoolExpression::Bool(_) => {},
            BoolExpression::Parentheses(op) => get_dependent_clocks(op, out),
            BoolExpression::Arithmetic(op) => get_dependent_clocks_arithmetic(op, out),

            BoolExpression::AndOp(lhs, rhs)
            | BoolExpression::OrOp(lhs, rhs) => {
                get_dependent_clocks(lhs, out);
                get_dependent_clocks(rhs, out);
            },

            BoolExpression::LessEQ(lhs, rhs)
            | BoolExpression::GreatEQ(lhs, rhs)
            | BoolExpression::LessT(lhs, rhs)
            | BoolExpression::GreatT(lhs, rhs)
            | BoolExpression::EQ(lhs, rhs) => {
                get_dependent_clocks_arithmetic(lhs, out);
                get_dependent_clocks_arithmetic(rhs, out);
            }
        }
    }

    fn get_dependent_clocks_arithmetic(expr: &ArithExpression, out: &mut HashSet<String>) {
        match expr.deref() {
            ArithExpression::Parentheses(op) => get_dependent_clocks_arithmetic(op, out),

            ArithExpression::Difference(lhs, rhs)
            | ArithExpression::Addition(lhs, rhs)
            | ArithExpression::Multiplication(lhs, rhs)
            | ArithExpression::Division(lhs, rhs)
            | ArithExpression::Modulo(lhs, rhs) => {
                get_dependent_clocks_arithmetic(lhs, out);
                get_dependent_clocks_arithmetic(rhs, out);
            },

            ArithExpression::Clock(index) => panic!("aaaaa"),
            ArithExpression::VarName(name) => {out.insert(name.clone());},
            ArithExpression::Int(i) => {}

        }
    }

    pub fn assert_correct_edges_and_locations(
        component: &Component,
        expected_locations: HashMap<String, HashSet<String>>,
        expected_edges: HashMap<String, HashSet<String>>
    ) {
        let redundant_clocks = component.find_redundant_clocks();

        for redundancy in redundant_clocks {
            let mut found_location_names: HashSet<String> = HashSet::new();
            let clock_expected_locations = expected_locations.get(redundancy.clock.as_str()).unwrap();
            for index in redundancy.location_indices {
                assert!(!found_location_names.contains(component.locations[index].id.as_str()), "Duplicate location index found");
                found_location_names.insert(component.locations[index].id.clone());
            }

            assert!(
                found_location_names.is_subset(clock_expected_locations)
                    && found_location_names.len() == clock_expected_locations.len(),
                "Got unexpected locations for reduction of {}. Expected: {:?}, got: {:?}",
                redundancy.clock,
                clock_expected_locations,
                found_location_names,
            );

            let mut found_edge_names: HashSet<String> = HashSet::new();
            let clock_expected_edges = expected_edges.get(&redundancy.clock).unwrap();

            for index in redundancy.edge_indices {
                let edge = &component.edges[index];
                let edge_id = format!("{}->{}", edge.source_location, edge.target_location);
                assert!(!found_edge_names.contains(&edge_id));
                found_edge_names.insert(edge_id);
            }

            assert!(
                found_edge_names.is_subset(clock_expected_edges)
                    && found_edge_names.len() == clock_expected_edges.len(),
                "Got unexpected edges for reduction of {}. Expected: {:?}, got: {:?}",
                redundancy.clock,
                clock_expected_edges,
                found_edge_names,
            );
        }
    }
}