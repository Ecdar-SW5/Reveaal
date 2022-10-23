use services::ProtobufServer::{DecisionPoint, Edge};
use src::TransitionSystem::component;
// TODO: use crate for transDecision

pub struct DecisionPoint {
    source: State,
    edges: Edge
}

impl DecisionPoint {
    // Creates the new decision point
    pub fn new(transitionDecision: TransitionDecision) -> &Self {
        
    }

    // Allows us to access location tuple to find locations
    pub fn get_location_tuple(tranistionDecision: TransitionDecision) -> LocationTuple {
        
       let locationTuple = tranistionDecision.source.locationTuple;
        
    }

    // Allows us to access transitions to add to edge ids
    pub fn get_transitions(transitionDecision: TransitionDecision) -> Vec<Transition> {
        
        let transitions = tranistionDecision.transitions;
    }

    // Create clean edges with no transitions attached
    pub fn create_edges(transitionDecision: TransitionDecision) -> Vec<Edge> {

        let transitions = get_transitions(transitionDecision);
        let mut i = 0;

        for Transition in transitions {

            let edges = Vec<Edge> {
                id: i,
                specific_component: transitionDecision.source.specific_component
                };
            }
            edges.push();
            i++;
        }
    
        // Add transitions to corrospondent edge ID
        pub fn add_transition_to_edge(tranistionDecision: TransitionDecision) -> Vec<Edge>{

            let transitions = get_transitions(tranistionDecision);
            let edges = create_edges(transitionDecision);
            let mut i = 0;

            // This is utterly retarded, but if we can choose correct ID/name, this will work
            for Transition in transitions {
                if (transitions.source.locationtuple.target_location == edges.specific_component.component_name) {
                    // TODO: add transition
                }
                
                i++;
            }
            // Push edges
        }
    


    }



