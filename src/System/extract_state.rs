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
    let out = is_universal_or_inconsistent_input(locations, machine);

    let location_id = get_location_id(&mut locations.iter(), machine);
    let locations = system.get_all_locations();

    //If the location is universal
    if out.0 {
        locations
            .iter()
            .find(|loc| matches!(loc.loc_type, LocationType::Universal))
            .map(|loc| loc.to_owned())
            .ok_or_else(|| "Could not find universal location in the transition system".to_string())
    // If the location is inconsistent
    } else if out.1 {
        locations
            .iter()
            .find(|loc| matches!(loc.loc_type, LocationType::Inconsistent))
            .map(|loc| loc.to_owned())
            .ok_or_else(|| {
                "Could not find inconsistent location in the transition system".to_string()
            })
    // If the location is normal
    } else {
        locations
            .iter()
            .find(|loc| loc.id.compare_partial_locations(&location_id))
            .ok_or_else(|| {
                format!(
                    "{} is not a location in the transition system ",
                    location_id
                )
            })
            .map(|location_tuple| {
                if !location_id.is_partial_location() {
                    location_tuple.clone()
                } else {
                    LocationTuple::create_partial_location(location_id)
                }
            })
    }
}

/// Checks if the input [LocationTuple] is of type `LocationType::Universal` or `LocationType::Inconsistent`.
/// Returns (`bool 0: is_universal`, `bool 1: is_inconsistent`)
fn is_universal_or_inconsistent_input(locations: &[&str], machine: &SystemRecipe) -> (bool, bool) {
    let mut components: Vec<Component> = Vec::new();
    machine.get_components(&mut components);

    let mut is_inconsistent = true;
    let mut is_universal = true;

    components
        .iter()
        .enumerate()
        .map(|(i, comp)| {
            let loc = comp
                .get_locations()
                .iter()
                .find(|loc| match loc.get_location_type() {
                    LocationType::Universal => {
                        is_inconsistent = false;
                        true
                    }
                    LocationType::Inconsistent => {
                        is_universal = false;
                        true
                    }
                    _ => false,
                });
            match loc {
                Some(l) => {
                    is_universal = is_universal && (l.id == locations[i]);
                    is_inconsistent = is_inconsistent && (l.id == locations[i]);
                }
                None => {
                    is_universal = false;
                    is_inconsistent = false
                }
            }
        })
        .for_each(drop);

    (is_universal, is_inconsistent)
}

fn get_location_id(locations: &mut Iter<&str>, machine: &SystemRecipe) -> LocationID {
    match machine {
        SystemRecipe::Composition(left, right) => LocationID::Composition(
            box_location_id(locations, left),
            box_location_id(locations, right),
        ),
        SystemRecipe::Conjunction(left, right) => LocationID::Conjunction(
            box_location_id(locations, left),
            box_location_id(locations, right),
        ),
        SystemRecipe::Quotient(left, right, ..) => LocationID::Quotient(
            box_location_id(locations, left),
            box_location_id(locations, right),
        ),
        SystemRecipe::Component(..) => match locations.next().unwrap().trim() {
            // It is ensured .next() will not give a None, since the number of location is same as number of component. This is also being checked in validate_reachability_input function, that is called before get_state
            "_" => LocationID::AnyLocation(),
            str => LocationID::Simple(str.to_string()),
        },
    }
}

fn box_location_id(left: &mut Iter<&str>, right: &SystemRecipe) -> Box<LocationID> {
    Box::new(get_location_id(left, right))
}
