use super::transition_decision_point::TransitionDecisionPoint;
use crate:: component::{Edge, State};
use crate::ProtobufServer::services::Decision as ProtoDecision;

#[allow(dead_code)]
#[derive(Clone)]
pub struct DecisionPoint {
    pub(crate) source: State,
    pub(crate) possible_decisions: Vec<Edge>,
}

impl From<TransitionDecisionPoint> for DecisionPoint {
    fn from(_: TransitionDecisionPoint) -> Self {
        todo!()
    }
}

pub struct Decision {
    source: State,
    decided: Edge,
}

impl From<ProtoDecision> for Decision {
    fn from(_: ProtoDecision) -> Self {
        todo!()
    }
}

// impl DecisionPoint {
//     pub fn from(
//         _transition_decision: &TransitionDecisionPoint,
//         _component_loader: &dyn ComponentLoader,
//     ) -> Self {
//         todo!()
//     }

//     pub fn serialize(&self) -> ProtoDecisionPoint {
//         todo!()
//     }

//     // Creates the new decision point
//     pub fn new(
//         transitionDecision: TransitionDecisionPoint,
//         components: Vec<Component>,
//         chosen_state: usize,
//     ) -> Self {
//         let source = (transitionDecision.source).clone();
//         let edges = Self::get_edges_with_transitions_for_chosen_component(
//             transitionDecision,
//             components,
//             chosen_state,
//         );
//         DecisionPoint {
//             source: source,
//             edges: edges,
//         }
//     }

//     // Allows us to access location tuple to find locations
//     pub fn get_location_tuple(transitionDecision: TransitionDecisionPoint) -> LocationTuple {
//         let locationTuple = transitionDecision.source.decorated_locations;
//         locationTuple
//     }

//     // Allows us to access transitions to add to edge ids
//     pub fn get_transitions(transitionDecision: TransitionDecisionPoint) -> Vec<Transition> {
//         let transitions = transitionDecision.possible_decisions;
//         transitions
//     }

//     // Get all edges from components
//     pub fn get_all_edges_from_components(components: Vec<Component>) -> Vec<Edge> {
//         let mut all_edges: Vec<Edge> = Vec::new();
//         let mut cloned_components: Vec<Component> = Vec::new();

//         for component in components {
//             cloned_components.push(component.clone())
//         }

//         for component in cloned_components {
//             let edges = component.get_edges();
//             for edge in edges {
//                 let e = edge.clone();
//                 all_edges.push(e);
//             }
//         }
//         return all_edges;
//     }

//     // Add transitions to correspondent edge ID
//     pub fn add_transition_to_edge(
//         transitionDecision: TransitionDecisionPoint,
//         components: Vec<Component>,
//     ) -> Vec<Transition> {
//         let transitions = Self::get_transitions(transitionDecision.clone());
//         let locationTuple = Self::get_location_tuple(transitionDecision.clone());
//         let mut new_transitions: Vec<Transition> = Vec::new();
//         let edges = Self::get_all_edges_from_components(components.clone());
//         let dim: ClockIndex = 0;

//         // 1. Loop over transitions
//         for _transition in transitions {
//             // 2. Loop over edges in all transitions
//             for edge in &edges {
//                 // 3. Check if transitions is connected with edges
//                 if locationTuple.id == LocationID::from_string(edge.get_target_location()) {
//                     // 4. Add transition to the correspondent edge ID
//                     for component in &components {
//                         let transition = Transition::from(&component, &edge, dim);
//                         new_transitions.push(transition);
//                     }
//                 }
//             }
//         }
//         // 5. Return the new transitions with correspondent edge ids
//         new_transitions
//     }

//     // Get all edges correspondent to the chosen ID
//     pub fn get_edges_with_transitions_for_chosen_component(
//         transitionDecision: TransitionDecisionPoint,
//         components: Vec<Component>,
//         chosen_state: usize,
//     ) -> Vec<Edge> {
//         let chosen_component = components[chosen_state].clone();
//         let mut chosen_edges: Vec<Edge> = Vec::new();
//         let edges = Component::get_edges(&chosen_component);

//         let transitions = Self::add_transition_to_edge(transitionDecision, components);

//         for transition in transitions {
//             for edge in edges {
//                 if transition.target_locations.id
//                     == LocationID::from_string(edge.get_target_location())
//                 {
//                     let e = edge.clone();
//                     chosen_edges.push(e);
//                 }
//             }
//         }
//         return chosen_edges;
//     }
// }

// #[cfg(test)]

// mod tests {

//     use super::{DecisionPoint, TransitionDecisionPoint};
//     use crate::{
//         component::Transition,
//         DataReader::json_reader::read_json_component,
//         TransitionSystems::{CompiledComponent, TransitionSystemPtr},
//     };

//     fn create_EcdarUniversity_Machine_system() -> TransitionSystemPtr {
//         let mut component = read_json_component("samples/json/EcdarUniversity", "Machine");
//         component.create_edge_io_split();
//         CompiledComponent::from(vec![component], "Machine")
//     }
//     // get_transitions test
//     #[test]
//     fn get_transitions_CorrectTransitionsReturned_ReturnsVectorOfTransitions() {
//         // arrange
//         let system = create_EcdarUniversity_Machine_system();
//         let source = system.get_initial_state().unwrap();

//         let dummyTransitionDecision = TransitionDecisionPoint::from(system, source);
//         // act

//         let _actual = DecisionPoint::get_transitions(dummyTransitionDecision);
//         let _dummyTransitions: Vec<Transition>;

//         // assert
//         //assert_type!(actual, dummyTransitions)
//     }
//     // get_location_tuple test
//     // new test
//     // get_all_edges_from_components test
//     // add_transition_to_edge test
//     // get_edges_with_transitions_for_chosen_component test
// }
