use std::collections::HashMap;

use edbm::zones::OwnedFederation;

use crate::component::{Declarations, LocationType};
use crate::extract_system_rep::SystemRecipe;
use crate::EdgeEval::constraint_applyer::apply_constraints_to_state;
use crate::ModelObjects::component::{Component, State};
use crate::ModelObjects::representations::QueryExpression;
use crate::TransitionSystems::{LocationID, LocationTuple, TransitionSystemPtr};
use std::slice::Iter;

/// This function takes a [`QueryExpression`], the system recipe, and the transitionsystem -
/// to define a state from the [`QueryExpression`] which has clocks and locations.
/// The [`QueryExpression`] looks like this: `State(Vec<LocName>, Option<BoolExpression>)`.
/// `state_query` is the part of the query that describes the location and the clock constraints of the state.
/// `machine` defines which operators is used to define the transistion system.
/// `system` is the transition system.
pub fn get_state(
    state_query: &QueryExpression,
    machine: &SystemRecipe,
    system: &TransitionSystemPtr,
) -> Result<State, String> {
    match state_query {
        QueryExpression::State(loc, clock) => {
            let mut locations: Vec<&str> = Vec::new();

            for location in loc {
                match location.as_ref() {
                    QueryExpression::LocName(name) => locations.push(name),
                    _ => panic!(),
                };
            }

            let locationtuple = build_location_tuple(&locations, machine, system)?;

            let zone = if let Some(clock_constraints) = clock {
                let mut clocks = HashMap::new();
                for decl in system.get_decls() {
                    clocks.extend(decl.clocks.clone());
                }

                let declarations = Declarations {
                    ints: HashMap::new(),
                    clocks,
                };

                match apply_constraints_to_state(
                    clock_constraints,
                    &declarations,
                    OwnedFederation::universe(system.get_dim()),
                ) {
                    Ok(zone) => zone,
                    Err(wrong_clock) => {
                        return Err(format!(
                            "Clock {} does not exist in the transition system",
                            wrong_clock
                        ))
                    }
                }
            } else {
                OwnedFederation::universe(system.get_dim())
            };

            Ok(State::create(locationtuple, zone))
        }
        _ => panic!("Expected QueryExpression::State, but got {:?}", state_query),
    }
}

fn build_location_tuple(
    locations: &[&str],
    machine: &SystemRecipe,
    system: &TransitionSystemPtr,
) -> Result<LocationTuple, String> {
    let location_id = get_location_id(&mut locations.iter(), machine);
    let partial = location_id.is_partial_location();
    let system_locations = system.get_all_locations();

    let out = if partial {
        EtEllerAndet::Normal
    } else {
        is_universal_or_inconsistent_input(locations, machine)
    };

    match out {
        EtEllerAndet::Universal => loc_find_and_then(
            system_locations,
            &|loc: &&LocationTuple| matches!(loc.loc_type, LocationType::Universal),
            &|loc: &LocationTuple| Some(loc.to_owned()),
            "Could not find universal location in the transition system",
        ),
        EtEllerAndet::Inconsistent => loc_find_and_then(
            system_locations,
            &|loc| matches!(loc.loc_type, LocationType::Inconsistent),
            &|loc| Some(loc.to_owned()),
            "Could not find inconsistent location in the transition system",
        ),
        EtEllerAndet::Normal => loc_find_and_then(
            system_locations,
            &|loc| loc.id.compare_partial_locations(&location_id),
            &|location_tuple| {
                Some(if !partial {
                    location_tuple.clone()
                } else {
                    LocationTuple::create_partial_location(location_id.clone())
                })
            },
            format!("{} is not a location in the transition system", location_id).as_str(),
        ),
    }
}

enum EtEllerAndet {
    Inconsistent,
    Universal,
    Normal,
}

/// Checks if the input [LocationTuple] is of type `LocationType::Universal` or `LocationType::Inconsistent`.
/// Returns (`bool 0: is_universal`, `bool 1: is_inconsistent`)
fn is_universal_or_inconsistent_input(locations: &[&str], machine: &SystemRecipe) -> EtEllerAndet {
    let mut components: Vec<Component> = Vec::new();
    machine.get_components(&mut components);

    let mut is_inconsistent = true;
    let mut is_universal = true;

    components
        .iter()
        .enumerate()
        .map(
            |(i, comp)| match comp.get_location_by_name(locations[i]).location_type {
                LocationType::Universal => is_inconsistent = false,
                LocationType::Inconsistent => is_universal = false,
                _ => {
                    is_universal = false;
                    is_inconsistent = false;
                }
            },
        )
        .for_each(drop);

    if is_universal {
        EtEllerAndet::Universal
    } else if is_inconsistent {
        EtEllerAndet::Inconsistent
    } else {
        EtEllerAndet::Normal
    }
}

fn loc_find_and_then(
    locations: Vec<LocationTuple>,
    predicate: &dyn Fn(&&LocationTuple) -> bool,
    op: &dyn Fn(&LocationTuple) -> Option<LocationTuple>,
    err: &str,
) -> Result<LocationTuple, String> {
    locations
        .iter()
        .find(predicate)
        .and_then(op)
        .ok_or_else(|| err.to_string())
}

fn get_location_id(locations: &mut Iter<&str>, machine: &SystemRecipe) -> LocationID {
    match machine {
        SystemRecipe::Composition(left, right) => {
            LocationID::Composition(box_loc_id(locations, left), box_loc_id(locations, right))
        }
        SystemRecipe::Conjunction(left, right) => {
            LocationID::Conjunction(box_loc_id(locations, left), box_loc_id(locations, right))
        }
        SystemRecipe::Quotient(left, right, ..) => {
            LocationID::Quotient(box_loc_id(locations, left), box_loc_id(locations, right))
        }
        SystemRecipe::Component(..) => match locations.next().unwrap().trim() {
            // It is ensured .next() will not give a None, since the number of location is same as number of component. This is also being checked in validate_reachability_input function, that is called before get_state
            "_" => LocationID::AnyLocation(),
            str => LocationID::Simple(str.to_string()),
        },
    }
}

fn box_loc_id(left: &mut Iter<&str>, right: &SystemRecipe) -> Box<LocationID> {
    Box::new(get_location_id(left, right))
}
