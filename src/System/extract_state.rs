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

            let locationtuple = build_location_tuple(&locations, machine, system);

            if locationtuple.is_err() {
                return Err(locationtuple.err().unwrap());
            }

            let locationtuple = locationtuple.unwrap();

            if let Some(clock_constraints) = clock {
                let inital_federation = OwnedFederation::universe(system.get_dim());

                let mut clocks = HashMap::new();
                for decl in system.get_decls() {
                    clocks.extend(decl.clocks.clone());
                }

                let declarations = Declarations {
                    ints: HashMap::new(),
                    clocks,
                };

                let zone = apply_constraints_to_state(clock_constraints, &declarations, inital_federation)?;
                Ok(State::create(locationtuple, zone))
            } else {
                let zone = OwnedFederation::universe(system.get_dim());
                Ok(State::create(locationtuple, zone))
            }
        }
        _ => panic!("Expected QueryExpression::State, but got {:?}", state_query),
    }
}

fn build_location_tuple(
    locations: &Vec<&str>,
    machine: &SystemRecipe,
    system: &TransitionSystemPtr,
) -> Result<LocationTuple, String> {
    let location_id = get_location_id(&mut locations.iter(), machine);
    let locations_system = system.get_all_locations();
    let locationtuple = locations_system.iter().find(|loc| loc.id == location_id);

    if locationtuple.is_none() {
        return Err(format!(
            "The location {} is not found in the system",
            location_id
        ));
    }

    Ok(locationtuple.unwrap().clone())
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
        SystemRecipe::Component(_comp) => {
            LocationID::Simple(locations.next().unwrap().trim().to_string())
        }
    }
}
