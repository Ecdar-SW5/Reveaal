#[cfg(test)]
pub mod test {
    use crate::component::{ClockReductionReason, Component, RedundantClock};
    use std::collections::{HashMap, HashSet};
    use std::iter::FromIterator;

    /// Asserts that component contains given locations and edges.
    pub fn assert_locations_and_edges_in_component(
        component: &Component,
        expected_locations: &HashSet<String>,
        expected_edges: &HashSet<String>,
    ) {
        assert_locations_in_component(component, expected_locations);
        assert_edges_in_component(component, expected_edges);
    }

    /// Asserts that component contains given locations.
    fn assert_locations_in_component(component: &Component, expected_locations: &HashSet<String>) {
        let mut actual_locations: HashSet<String> = HashSet::new();

        for location in &component.locations {
            let mut clocks_in_invariants = HashSet::new();
            if let Some(invariant) = &location.invariant {
                invariant.get_varnames().iter().for_each(|clock| {
                    clocks_in_invariants.insert((*clock).to_string());
                });
            }

            let clock = sort_clocks_and_join(&clocks_in_invariants);

            actual_locations.insert(format!("{}-{}", location.id.clone(), clock));
        }
        assert!(
            expected_locations.is_subset(&actual_locations)
                && expected_locations.len() == actual_locations.len(),
            "Expected these locations {:?}, but got {:?}",
            expected_locations,
            actual_locations
        );
    }

    /// Asserts that component contains given locations.
    pub(crate) fn assert_edges_in_component(
        component: &Component,
        expected_edges: &HashSet<String>,
    ) {
        let mut actual_edges: HashSet<String> = HashSet::new();

        for edge in &component.edges {
            let mut clocks_in_guards_and_updates = HashSet::new();
            if let Some(guard) = &edge.guard {
                guard.get_varnames().iter().for_each(|clock| {
                    clocks_in_guards_and_updates.insert((*clock).to_string());
                });
            }
            if let Some(updates) = &edge.update {
                for update in updates {
                    clocks_in_guards_and_updates.insert(update.variable.to_string());
                }
            }

            let sorted_clocks = sort_clocks_and_join(&clocks_in_guards_and_updates);

            let edge_id = format!(
                "{}-{}->{}",
                edge.source_location, sorted_clocks, edge.target_location
            );

            assert!(
                !actual_edges.contains(&edge_id),
                "Duplicate edge: {}",
                edge_id
            );

            actual_edges.insert(edge_id);
        }
        assert!(
            expected_edges.is_subset(&actual_edges) && expected_edges.len() == actual_edges.len(),
            "Expected these edges {:?} but got {:?}",
            expected_edges,
            actual_edges
        )
    }

    fn sort_clocks_and_join(dependent_clocks: &HashSet<String>) -> String {
        let mut dependent_clocks_vec = Vec::from_iter(dependent_clocks.iter());
        let mut sorted_clocks = String::new();
        dependent_clocks_vec.sort();

        for clock in dependent_clocks_vec {
            sorted_clocks = sorted_clocks + clock;
        }
        sorted_clocks
    }

    /// Assert that a redundant clock is redundant for the correct reason
    pub fn assert_clock_reason(
        redundant_clocks: &Vec<RedundantClock>,
        expected_amount_to_reduce: u32,
        expected_reducible_clocks: HashSet<&str>,
        unused_allowed: bool,
    ) {
        let mut global_clock: String = String::from("");

        let mut clocksReduced: HashSet<String> = HashSet::new();

        for redundancy in redundant_clocks {
            match &redundancy.reason {
                ClockReductionReason::Duplicate(replaced_by) => {
                    if global_clock == "" {
                        global_clock = replaced_by.clone();
                    }
                    assert!(
                        expected_reducible_clocks.contains(redundancy.clock.as_str()),
                        "Clock ({}) was marked as duplicate unexpectedly",
                        redundancy.clock
                    );
                    assert!(
                        !clocksReduced.contains(&redundancy.clock),
                        "Clock ({}) was marked as duplicate multiple times",
                        redundancy.clock
                    );
                    assert_eq!(
                        &global_clock, replaced_by,
                        "Multiple clocks were chosen as global clock {} and {}",
                        global_clock, replaced_by
                    );
                    clocksReduced.insert(redundancy.clock.clone());
                }
                ClockReductionReason::Unused => {
                    assert!(unused_allowed, "Unexpected unused optimization");
                    assert!(
                        expected_reducible_clocks.contains(&redundancy.clock.as_str()),
                        "Clock ({}) is not set as unused, but is not in expected",
                        redundancy.clock.as_str()
                    );
                    assert!(
                        !clocksReduced.contains(&redundancy.clock),
                        "Clock {} has been removed multiple times",
                        redundancy.clock
                    );
                    clocksReduced.insert(redundancy.clock.clone());
                }
            }
        }
        assert_eq!(
            clocksReduced.len(),
            expected_amount_to_reduce as usize,
            "Too many clocks were reduced, expected only {}, got {}",
            expected_amount_to_reduce,
            clocksReduced.len()
        );
    }

    /// Asserts that the specific clocks occur in the correct locations and edges
    pub fn assert_correct_edges_and_locations(
        component: &Component,
        expected_locations: HashMap<String, HashSet<String>>,
        expected_edges: HashMap<String, HashSet<String>>,
    ) {
        let redundant_clocks = component.find_redundant_clocks();

        for redundancy in redundant_clocks {
            let mut found_location_names: HashSet<String> = HashSet::new();
            let clock_expected_locations =
                expected_locations.get(redundancy.clock.as_str()).unwrap();
            for index in redundancy.location_indices {
                assert!(
                    !found_location_names.contains(component.locations[index].id.as_str()),
                    "Duplicate location index found"
                );
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
