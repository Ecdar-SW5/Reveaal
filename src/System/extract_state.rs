use std::collections::HashMap;

use edbm::zones::OwnedFederation;

use crate::component::{Declarations, LocationType};
use crate::extract_system_rep::SystemRecipe;
use crate::EdgeEval::constraint_applyer::apply_constraints_to_state;
use crate::ModelObjects::component::State;
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

    if machine
        .get_components()
        .iter()
        .enumerate()
        .map(|(i, comp)| -> bool {
            comp.clone().location_exists(locations[i]) || locations[i] == "_"
        })
        .any(|x| x == false)
    {
        return Err(format!(
            "Location {} does not exist in the system",
            location_id
        ));
    }

    let out = if partial {
        LocationType::Normal
    } else {
        is_universal_or_inconsistent_input(locations, machine)
    };

    match out {
        LocationType::Universal => find_location_and_then(
            system_locations,
            &|loc: &&LocationTuple| matches!(loc.loc_type, LocationType::Universal),
            &|loc: &LocationTuple| Some(loc.to_owned()),
            None,
        ),
        LocationType::Inconsistent => find_location_and_then(
            system_locations,
            &|loc| matches!(loc.loc_type, LocationType::Inconsistent),
            &|loc| Some(loc.to_owned()),
            None,
        ),
        LocationType::Normal => find_location_and_then(
            system_locations,
            &|loc| loc.id.compare_partial_locations(&location_id),
            &|location_tuple| {
                Some(if !partial {
                    location_tuple.to_owned()
                } else {
                    LocationTuple::create_partial_location(location_id.clone())
                })
            },
            None,
        ),
        LocationType::Initial => unreachable!(),
    }
}

/// Checks if the input [LocationTuple] is of type `LocationType::Universal` or `LocationType::Inconsistent`.
/// Returns (`bool 0: is_universal`, `bool 1: is_inconsistent`)
fn is_universal_or_inconsistent_input(locations: &[&str], machine: &SystemRecipe) -> LocationType {
    let mut is_inconsistent = true;
    let mut is_universal = true;

    machine
        .get_components()
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
        LocationType::Universal
    } else if is_inconsistent {
        LocationType::Inconsistent
    } else {
        LocationType::Normal
    }
}

fn find_location_and_then(
    locations: Vec<LocationTuple>,
    predicate: &dyn Fn(&&LocationTuple) -> bool,
    op: &dyn Fn(&LocationTuple) -> Option<LocationTuple>,
    err: Option<&str>,
) -> Result<LocationTuple, String> {
    locations
        .iter()
        .find(predicate)
        .and_then(op)
        .ok_or_else(|| {
            if let Some(msg) = err {
                msg.to_string()
            } else {
                "Unexpected error happened".to_string()
            }
        })
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
