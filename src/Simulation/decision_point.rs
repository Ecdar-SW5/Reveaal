use crate::{component::{State, Edge, Transition}, TransitionSystems::LocationTuple};

use src::Simulation::transition_decision;

use super::transition_decision::TransitionDecision;



pub struct DecisionPoint {
    source: State,
    edges: Vec<Edge>,
}

impl DecisionPoint {
    // Creates the new decision point
    pub fn new(transitionDecision: TransitionDecision) -> &Self {
        let source = transitionDecision.source;
        //TODO: make transitions = edge

        
    }

    // Allows us to access location tuple to find locations
    pub fn get_location_tuple(transitionDecision: TransitionDecision) -> LocationTuple {
        
       let locationTuple = transitionDecision.source.locationTuple;
       locationTuple
        
    }

    // Allows us to access transitions to add to edge ids
    pub fn get_transitions(transitionDecision: TransitionDecision) -> Vec<Transition> {
        
        let transitions = transitionDecision.transitions;
        transitions
    }

    // Get all edges from components
    pub fn get_all_edges_from_components(simulationsInfo: SimulationsInfo) -> Vec<Edge> {

        let components = simulationsInfo.components_info.components;
        let all_edges: Vec<Edge>;
        for component in components {
            let edges = get_edges(component);
            all_edges.push(edges)
        }
        all_edges
    }

    // Add transitions to corrospondent edge ID
    pub fn add_transition_to_edge(transitionDecision: TransitionDecision, simulationsInfo: SimulationsInfo) -> Vec<Transition>{

        let transitions = get_transitions(transitionDecision);
        let locationTuple = get_location_tuple(transitionDecision);
        let new_transitions: Vec<Transition>;
        let edges = get_all_edges_from_components(simulationsInfo);
        let mut dim: ClockIndex = 0;

        // 1. Loop over transitions
        for transition in transitions {
            // 2. Loop over edges
            for edge in edges {
                // 3. Check if transitions is connected with edges
                if (locationTuple.id == edge.target_location) {
                    // 4. Add transition to the corropsondent edge ID
                    let transition = transition::from(locationTuple.id, edge, dim);
                    new_transitions.push(transition);
                }
            }     
        }
        // 5. Return the new transitions with corropsodent edge ids
        new_transitions
    }

    // Get all edges corrospodent to the chosen ID
    pub fn get_transitions_for_chosen_component(transitionDecision: TransitionDecision, simulationsInfo: SimulationsInfo, chosen_state: String) -> Vec<Edge> {

    }
}



