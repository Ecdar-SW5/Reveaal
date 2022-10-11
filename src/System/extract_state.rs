use std::collections::HashMap;
use std::{error::Error, fmt};

use edbm::zones::OwnedFederation;

use crate::component::Declarations;
use crate::extract_system_rep::SystemRecipe;
use crate::EdgeEval::constraint_applyer::apply_constraints_to_state;
use crate::ModelObjects::component::State;
use crate::ModelObjects::representations::QueryExpression;
use crate::TransitionSystems::{LocationID, LocationTuple, TransitionSystemPtr};

/// This function takes a QueryExpresssion, the system recipe, and the transistionsystem -
/// to define a state form the QueryExpression which has clocks and locations.
/// The QueryExpression looks like this: State(Vec<LocName>, Option<BoolExpression>)
pub fn get_state(
    expr: &QueryExpression,
    machine: &SystemRecipe,
    system: &TransitionSystemPtr,
) -> Result<State, String> {
    match expr {
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

            if clock.is_some() {
                let initalFederation = OwnedFederation::universe(system.get_dim());
                let clock = &*clock.clone().unwrap();

                let decls = system.get_decls();
                let mut clocks = HashMap::new();
                for decl in decls {
                    clocks.extend(decl.clocks.clone());
                }

                let declarations = Declarations {
                    ints: HashMap::new(),
                    clocks,
                };

                let zone = apply_constraints_to_state(clock, &declarations, initalFederation);
                Ok(State::create(locationtuple, zone))
            } else {
                let zone = OwnedFederation::universe(system.get_dim());
                Ok(State::create(locationtuple, zone))
            }
        }
        _ => panic!("Wrong type"),
    }
}

fn build_location_tuple(
    locations: &Vec<&str>,
    machine: &SystemRecipe,
    system: &TransitionSystemPtr,
) -> Result<LocationTuple, String> {
    let mut index = 0;
    let locationID = get_locationID(locations, &mut index, machine);
    let locations_system = system.get_all_locations();
    let locationtuple = locations_system.iter().find(|loc| loc.id == locationID);

    if locationtuple.is_none() {
        return Err(format!(
            "The location {} is not found in the system",
            locationID
        ));
    }

    Ok(locationtuple.unwrap().clone())
}

fn get_locationID(locations: &Vec<&str>, index: &mut usize, machine: &SystemRecipe) -> LocationID {
    match machine {
        SystemRecipe::Composition(left, right) => LocationID::Composition(
            Box::new(get_locationID(locations, index, left)),
            Box::new(get_locationID(locations, index, right)),
        ),
        SystemRecipe::Conjunction(left, right) => LocationID::Conjunction(
            Box::new(get_locationID(locations, index, left)),
            Box::new(get_locationID(locations, index, right)),
        ),
        SystemRecipe::Quotient(left, right, _clock_index) => LocationID::Quotient(
            Box::new(get_locationID(locations, index, left)),
            Box::new(get_locationID(locations, index, right)),
        ),
        SystemRecipe::Component(_comp) => {
            let loc = locations[*index];
            *index += 1;
            LocationID::Simple(loc.trim().to_string())
        }
    }
}

#[derive(Debug)]
pub enum LocationError {
    InvalidLoaction(String),
}

impl Error for LocationError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match self {
            LocationError::InvalidLoaction(location) => location,
        }
    }
}

impl fmt::Display for LocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationError::InvalidLoaction(location) => write!(f, "invaild location: {}", location),
        }
    }
}
