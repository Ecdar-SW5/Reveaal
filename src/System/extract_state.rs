
use std::collections::HashMap;

use edbm::zones::OwnedFederation;

use crate::EdgeEval::constraint_applyer::apply_constraints_to_state;
use crate::ModelObjects::representations::QueryExpression;
use crate::ModelObjects::component::State;
use crate::TransitionSystems::{LocationID, TransitionSystemPtr};
use crate::component::Declarations;
use crate::extract_system_rep::SystemRecipe;

pub fn get_state(expr: &QueryExpression, machine: &SystemRecipe, system: &TransitionSystemPtr) -> State {
    match expr {
        QueryExpression::State(loc,clock) => {
            
            let mut locations:Vec<&str> = Vec::new();

            for location in loc{
                match &**location {
                    QueryExpression::LocName(name) => locations.push(name),
                    _ => panic!(),
                };
            }
        
            let mut index = 0;

            let locationID = get_locationID(&locations, &mut index, &machine);

            let locationtuple = system.get_all_locations().iter().filter(|loc| loc.id == locationID).next().unwrap().clone();

            let decls = system.get_decls();
            let mut initial_decl = HashMap::new(); 
            for decl in decls {
                initial_decl.extend(decl.clocks.clone());
            }   
        
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
                State::create(locationtuple, zone)
            }
            else{
                let zone = OwnedFederation::universe(system.get_dim());
                State::create(locationtuple, zone)
            }
        }
        _ => panic!("Wrong type"),
    }
}

fn get_locationID(locations: &Vec<&str>, index: &mut usize, machine: &SystemRecipe)-> LocationID{
    match machine {
        SystemRecipe::Composition(left, right) => {
           LocationID::Composition(Box::new(get_locationID(&locations, index, left)), Box::new(get_locationID(&locations, index, right)))
        }
        SystemRecipe::Conjunction(left, right) => {
            LocationID::Conjunction(Box::new(get_locationID(&locations, index, left)), Box::new(get_locationID(&locations, index, right)))
        }
        SystemRecipe::Quotient(left, right, _clock_index) => {
            LocationID::Quotient(Box::new(get_locationID(&locations, index, left)), Box::new(get_locationID(&locations, index, right)))
        },
        SystemRecipe::Component(_comp) => {  
            let loc = locations[*index];
            *index = *index + 1;
            LocationID::Simple(loc.trim().to_string())
        },
    }
}