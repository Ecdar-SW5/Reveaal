
use std::collections::HashMap;
use std::vec;

use edbm::zones::OwnedFederation;

use crate::EdgeEval::constraint_applyer::apply_constraints_to_state;
//Remove locationTuple test and state test
use crate::ModelObjects::representations::QueryExpression;
use crate::ModelObjects::component::State;
use crate::TransitionSystems::{LocationTuple, LocationID, TransitionSystemPtr};
use crate::component::Declarations;
use crate::extract_system_rep::SystemRecipe;

pub fn get_state(expr: &QueryExpression, machine: &SystemRecipe, system: &TransitionSystemPtr) -> State {
    let State = match expr {
        QueryExpression::State(loc,clock) => {
            let locations = match &**loc {
                QueryExpression::LocName(name) => name,
                _ => panic!(),
            };
            let locations: Vec<&str> = locations.split(",").collect();
            let mut index = 0;

            let locationID = get_locationID_based_on_locationstr_and_systemrecipe(&locations, &mut index, &machine);

            let locationtuple = system.get_all_locations().iter().filter(|loc| loc.id == locationID).next().unwrap().clone();
            
            let initalFederation = OwnedFederation::universe(system.get_dim());

            let decls = system.get_decls();

            let mut initial_decl = HashMap::new(); 

            for decl in decls {
                initial_decl.extend(decl.clocks.clone());
            }

            println!("\n\n\ninitial_decl: {:?}\n\n\n",initial_decl);    

            let zone = apply_constraints_to_state(clock, &Declarations::test(initial_decl), initalFederation);

            State::create(locationtuple, zone)

        }
        _ => panic!("Wrong type"),
    };

    let l: LocationTuple = LocationTuple::test();
    State::test(l)
}


fn get_locationID_based_on_locationstr_and_systemrecipe(locations: &Vec<&str>, index: &mut usize, machine: &SystemRecipe)-> LocationID{
    match machine {
        SystemRecipe::Composition(left, right) => {
           LocationID::Composition(Box::new(get_locationID_based_on_locationstr_and_systemrecipe(&locations, index, left)), Box::new(get_locationID_based_on_locationstr_and_systemrecipe(&locations, index, right)))
        }
        SystemRecipe::Conjunction(left, right) => {
            LocationID::Conjunction(Box::new(get_locationID_based_on_locationstr_and_systemrecipe(&locations, index, left)), Box::new(get_locationID_based_on_locationstr_and_systemrecipe(&locations, index, right)))
        }
        SystemRecipe::Quotient(left, right, _clock_index) => {
            LocationID::Quotient(Box::new(get_locationID_based_on_locationstr_and_systemrecipe(&locations, index, left)), Box::new(get_locationID_based_on_locationstr_and_systemrecipe(&locations, index, right)))
        },
        SystemRecipe::Component(_comp) => {  
            let loc = locations[*index];
            *index = *index + 1;
            LocationID::Simple(loc.trim().to_string())
        },
    }



}