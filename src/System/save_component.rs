use crate::ModelObjects::component::{Component, Declarations, Edge, Location, SyncType};
use crate::ModelObjects::representations::BoolExpression;
use crate::TransitionSystems::{LocationTuple, TransitionSystemPtr};
use std::collections::HashMap;

pub enum PruningStrategy {
    Reachable,
    NoPruning,
}

use edbm::util::constraints::ClockIndex;
use PruningStrategy::*;

pub fn combine_components(
    system: &TransitionSystemPtr,
    reachability: PruningStrategy,
) -> Component {
    let mut location_tuples = vec![];
    let mut edges = vec![];
    let clocks = get_clock_map(system);
    match reachability {
        Reachable => {
            collect_reachable_edges_and_locations(system, &mut location_tuples, &mut edges, &clocks)
        }
        NoPruning => {
            collect_all_edges_and_locations(system, &mut location_tuples, &mut edges, &clocks)
        }
    };

    let locations = get_locations_from_tuples(&location_tuples, &clocks);
    let mut comp = Component {
        name: "".to_string(),
        declarations: Declarations {
            ints: HashMap::new(),
            clocks,
        },
        locations,
        edges,
        input_edges: None,
        output_edges: None,
    };
    comp.create_edge_io_split();
    comp
}

fn get_locations_from_tuples(
    location_tuples: &[LocationTuple],
    clock_map: &HashMap<String, ClockIndex>,
) -> Vec<Location> {
    location_tuples
        .iter()
        .cloned()
        .map(|loc_vec| {
            let invariant: Option<BoolExpression> = loc_vec.get_invariants().and_then(|fed| {
                BoolExpression::from_disjunction(&fed.minimal_constraints(), clock_map)
            });

            Location {
                id: loc_vec.id.to_string(),
                invariant,
                location_type: loc_vec.loc_type,
                urgency: "NORMAL".to_string(), //TODO: Handle different urgencies eventually
            }
        })
        .collect()
}

fn get_clock_map(sysrep: &TransitionSystemPtr) -> HashMap<String, ClockIndex> {
    let mut clocks = HashMap::new();
    let decls = sysrep.get_decls();

    if decls.len() == 1 {
        return decls[0].clocks.clone();
    }
    for (comp_id, decl) in decls.into_iter().enumerate() {
        for (k, v) in &decl.clocks {
            if clocks.contains_key(k) {
                clocks.insert(format!("{}{}", k, comp_id), *v);
            } else {
                clocks.insert(k.clone(), *v);
            }
        }
    }

    clocks
}

fn collect_all_edges_and_locations<'a>(
    representation: &'a TransitionSystemPtr,
    locations: &mut Vec<LocationTuple>,
    edges: &mut Vec<Edge>,
    clock_map: &HashMap<String, ClockIndex>,
) {
    let l = representation.get_all_locations();
    locations.extend(l);
    for location in locations {
        collect_edges_from_location(location, representation, edges, clock_map);
    }
}

fn collect_reachable_edges_and_locations<'a>(
    representation: &'a TransitionSystemPtr,
    locations: &mut Vec<LocationTuple>,
    edges: &mut Vec<Edge>,
    clock_map: &HashMap<String, ClockIndex>,
) {
    let l = representation.get_initial_location();

    if l.is_none() {
        return;
    }
    let l = l.unwrap();

    locations.push(l.clone());

    collect_reachable_locations(&l, representation, locations);

    for loc in locations {
        collect_edges_from_location(loc, representation, edges, clock_map);
    }
}

fn collect_reachable_locations<'a>(
    location: &LocationTuple,
    representation: &'a TransitionSystemPtr,
    locations: &mut Vec<LocationTuple>,
) {
    for input in [true, false].iter() {
        for sync in if *input {
            representation.get_input_actions()
        } else {
            representation.get_output_actions()
        } {
            let transitions = representation.next_transitions(location, &sync);

            for transition in transitions {
                let mut target_location = location.clone();
                transition.move_locations(&mut target_location);

                if !locations.contains(&target_location) {
                    locations.push(target_location.clone());
                    collect_reachable_locations(&target_location, representation, locations);
                }
            }
        }
    }
}

fn collect_edges_from_location(
    location: &LocationTuple,
    representation: &TransitionSystemPtr,
    edges: &mut Vec<Edge>,
    clock_map: &HashMap<String, ClockIndex>,
) {
    collect_specific_edges_from_location(location, representation, edges, true, clock_map);
    collect_specific_edges_from_location(location, representation, edges, false, clock_map);
}

fn collect_specific_edges_from_location(
    location: &LocationTuple,
    representation: &TransitionSystemPtr,
    edges: &mut Vec<Edge>,
    input: bool,
    clock_map: &HashMap<String, ClockIndex>,
) {
    for sync in if input {
        representation.get_input_actions()
    } else {
        representation.get_output_actions()
    } {
        let transitions = representation.next_transitions(location, &sync);
        for transition in transitions {
            let mut target_location = location.clone();
            transition.move_locations(&mut target_location);

            let guard = transition.get_renamed_guard_expression(clock_map);
            if let Some(BoolExpression::Bool(false)) = guard {
                continue;
            }

            let edge = Edge {
                source_location: location.id.to_string(),
                target_location: target_location.id.to_string(),
                sync_type: if input {
                    SyncType::Input
                } else {
                    SyncType::Output
                },
                guard,
                update: transition.get_renamed_updates(clock_map),
                sync: sync.clone(),
                component_name: None,
            };
            edges.push(edge);
        }
    }
}
