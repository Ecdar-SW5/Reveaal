use std::collections::HashMap;

use edbm::zones::OwnedFederation;

use crate::component::Declarations;
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
                match &**location {
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
    let locations_system = system.get_all_locations();
    if let Some(locationtuple) = locations_system
        .iter()
        .find(|loc| loc.id.compare_partial_locations(&location_id))
    {
        if !location_id.is_partial_location() {
            Ok(locationtuple.clone())
        } else {
            Ok(LocationTuple::create_partial_location(location_id))
        }
    } else {
        Err(format!(
            "{} is not a location in the transition system ",
            location_id
        ))
    }
}

fn get_location_id(locations: &mut Iter<&str>, machine: &SystemRecipe) -> LocationID {
    match machine {
        SystemRecipe::Composition(left, right) => LocationID::Composition(
            Box::new(get_location_id(locations, left)),
            Box::new(get_location_id(locations, right)),
        ),
        SystemRecipe::Conjunction(left, right) => LocationID::Conjunction(
            Box::new(get_location_id(locations, left)),
            Box::new(get_location_id(locations, right)),
        ),
        SystemRecipe::Quotient(left, right, _clock_index) => LocationID::Quotient(
            Box::new(get_location_id(locations, left)),
            Box::new(get_location_id(locations, right)),
        ),
        SystemRecipe::Component(..) => match locations.next().unwrap().trim() {
            // It is ensured .next() will not give a None, since the number of location is same as number of component. This is also being checked in validate_reachability_input function, that is called before get_state
            "_" => LocationID::AnyLocation(),
            str => LocationID::Simple(str.to_string()),
        },
    }
}
