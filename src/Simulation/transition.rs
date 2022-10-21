use services::ProtobufServer::{DecisionPoint, Edge};
use src::TransitionSystem::component;
// TODO: use transDecision

pub struct DecisionPoint {
    source: State,
    edges: Edge
}

impl DecisionPoint {
    pub fn new(transitionDecision: TransitionDecision) -> &Self {
        
    }
    pub fn get_location_tuple(tranistionDecision: TransitionDecision) -> LocationTuple {
        
       let locationTuple = tranistionDecision.source.locationTuple;
        
    }

    pub fn get_transitions(transitionDecision: TransitionDecision) -> Vec<Transition> {
        
        let transitions = tranistionDecision.transitions;
    }

    pub fn create_id(transitionDecision: TransitionDecision) -> String {
        // SpecComp_id: 3, Edge_id: 2 -> 3_2
        let id_index = 0;
        
        let id = "{id_index++}" + transitionDecision.source.specific_component.component_index; // Scuffed
    }

    pub fn create_edge(transitionDecision: TransitionDecision) -> Edge {

        let transitions = get_transitions(transitionDecision);
        let i = 0;

        for Transition in transitions {

            let edges = Edge {
                id: i,
                specific_component: transitionDecision.source.specific_component,
                };
            }
            edges.push();
        }
    


    }



