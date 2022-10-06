
use std::vec;

//Remove locationTuple test and state test
use crate::ModelObjects::representations::QueryExpression;
use crate::ModelObjects::component::State;
use crate::TransitionSystems::{LocationTuple, LocationID, TransitionSystem};
use crate::component::LocationType;
use crate::extract_system_rep::SystemRecipe;

pub fn get_state(expr: &QueryExpression, machine: &SystemRecipe) -> State {
    let locationID: LocationID = match expr {
        QueryExpression::State(loc,clock) => {
            let locations = match &**loc {
                QueryExpression::LocName(name) => name,
                (_) => panic!(),
            };
            let locations: Vec<&str> = locations.split(",").collect();
            let mut index = 0;
            test(&locations, &mut index, &machine)
        }
        (_) => panic!("Wrong type"),
    };

    println!("LocationID: {:?}",locationID);

    let l: LocationTuple = LocationTuple::test();
    State::test(l)
}


fn test(locations: &Vec<&str>, index: &mut usize, machine: &SystemRecipe)-> LocationID{
    match machine {
        SystemRecipe::Composition(left, right) => {
           LocationID::Composition(Box::new(test(&locations, index, left)), Box::new(test(&locations, index, right)))
        }
        SystemRecipe::Conjunction(left, right) => {
            LocationID::Conjunction(Box::new(test(&locations, index, left)), Box::new(test(&locations, index, right)))
        }
        SystemRecipe::Quotient(left, right, clock_index) => {
            LocationID::Quotient(Box::new(test(&locations, index, left)), Box::new(test(&locations, index, right)))
        },
        SystemRecipe::Component(comp) => {
            let loc = locations[*index];
            *index = *index + 1;
            LocationID::Simple(loc.to_string())
        },
    }



}