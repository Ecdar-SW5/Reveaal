
use crate::ModelObjects::representations::QueryExpression;
use crate::ModelObjects::component::State;
use crate::TransitionSystems::{LocationTuple, LocationID};
use crate::component::LocationType;

pub fn get_state(expr: &QueryExpression) -> State {
    /*match expr {
        QueryExpression::VarName(str) => {
        
        }
        QueryExpression::B
    }*/
    let l: LocationTuple = LocationTuple::test();
    State::test(l)
}