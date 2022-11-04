use std::collections::HashMap;
use std::panic::AssertUnwindSafe;

use crate::ProtobufServer::ecdar_requests::helpers;
use crate::ProtobufServer::services::{
    DecisionPoint as ProtoDecisionPoint, 
    SimulationStartRequest, SimulationStepResponse, 
    State as ProtoState, 
    Edge as ProtoEdge, 
    LocationTuple as ProtoLocationTuple, 
    Federation as ProtoFederation,
    Location as ProtoLocation,
    Disjunction as ProtoDisjunction,
    Conjunction as ProtoConjunction,
    Constraint as ProtoConstraint, ComponentClock,
};
use crate::Simulation::decision_point::DecisionPoint;
use crate::Simulation::transition_decision_point::TransitionDecisionPoint;
use crate::TransitionSystems::{LocationTuple, TransitionSystemPtr};
use crate::component::{State, Edge};
use crate::TransitionSystems::location_id::LocationID;
use edbm::util::constraints::{Disjunction, Conjunction, Constraint, ClockIndex};
use edbm::zones::OwnedFederation;
use log::trace;

use tonic::{Request, Response, Status};

use crate::ProtobufServer::ConcreteEcdarBackend;

impl ConcreteEcdarBackend {
    pub async fn handle_start_simulation(
        request: AssertUnwindSafe<Request<SimulationStartRequest>>,
    ) -> Result<Response<SimulationStepResponse>, Status> {
        trace!("Received query: {:?}", request);

        let request_message = request.0.into_inner();
        let simulation_info = request_message.simulation_info.unwrap();
        let transition_system = helpers::simulation_info_to_transition_system(simulation_info);

        // Find Initial TransitionDecisionPoint in transition system
        let initial = TransitionDecisionPoint::initial(transition_system.clone()).unwrap(); // TODO remove clone

        let initial = 
            ProtoDecisionPoint::from_transition_decision_point(&initial, &transition_system);

        // Respond with initial
        let simulation_step_response = SimulationStepResponse {
            new_decision_point: Some(initial),
        };
        Ok(Response::new(simulation_step_response))
    }
}

impl ProtoDecisionPoint {
    pub fn from_transition_decision_point(decision_point: &TransitionDecisionPoint, system: &TransitionSystemPtr) -> ProtoDecisionPoint {
        let decision_point: DecisionPoint = DecisionPoint::from(decision_point);
        let decision_point: ProtoDecisionPoint = ProtoDecisionPoint::from(&decision_point, system);
        decision_point
    }
}

impl ProtoDecisionPoint {
    fn from(decision_point: &DecisionPoint, system: &TransitionSystemPtr) -> Self {
        let source = ProtoState::from(&decision_point.source, system);

        let edges =  decision_point
            .possible_decisions
            .iter()
            .map(|x| ProtoEdge::from(x))
            .collect();

        ProtoDecisionPoint {
            source: Some(source),
            edges: edges,
        }
    }
}

impl ProtoState {
    fn from(s: &State, system: &TransitionSystemPtr) -> Self {
        let location_tuple = ProtoLocationTuple::from(s.get_location());
        let federation = ProtoFederation::from(s.zone_ref(), system);

        ProtoState {
            location_tuple: Some(location_tuple),
            federation: Some(federation),
        }
    }
}

fn location_id_to_proto_location_vec(is: &LocationID) -> Vec<ProtoLocation> {
    match is {
        LocationID::Simple(s) => vec![ProtoLocation { id: s.clone(), specific_component: None }], // TODO figure out how to find specific_component
        LocationID::Conjunction(l, r) 
        | LocationID::Composition(l, r) 
        | LocationID::Quotient(l, r) 
            => location_id_to_proto_location_vec(l)
                .into_iter()
                .chain(location_id_to_proto_location_vec(r).into_iter())
                .collect(),
        LocationID::AnyLocation() => vec![],
    }
}

impl From<&LocationTuple> for ProtoLocationTuple {
    fn from(l: &LocationTuple) -> Self {
        ProtoLocationTuple { 
          locations:  location_id_to_proto_location_vec(&l.id),
        }
    }
}


impl ProtoFederation {
    fn from(f: &OwnedFederation, system: &TransitionSystemPtr) -> Self {
        ProtoFederation {
            disjunction: Some(ProtoDisjunction::from(&f.minimal_constraints(), system)),
        }
    }
}

impl ProtoDisjunction {
    fn from(d: &Disjunction, system: &TransitionSystemPtr) -> Self {
        ProtoDisjunction {
            conjunctions: d
                .conjunctions
                .iter()
                .map(|x| ProtoConjunction::from(x, system))
                .collect(),
        }
    }
}

impl ProtoConjunction {
    fn from(c: &Conjunction, system: &TransitionSystemPtr) -> Self {
        ProtoConjunction {
            constraints: c.constraints
            .iter()
            .map(|x| ProtoConstraint::from(x, system))
            .collect(),
        }
    }
}

// TODO finish this
#[allow(unused_must_use)]
impl ProtoConstraint {
    fn from(constraint: &Constraint, system: &TransitionSystemPtr) -> Self {
        let mut naming: HashMap<ClockIndex, &str> = HashMap::new();

        system
            .get_decls()
            .first()
            .unwrap()
            .clocks
            .iter()
            .map(|x| naming.insert(*x.1, x.0));

        ProtoConstraint {
            x: Some(ComponentClock{
                specific_component: None, // TODO how?
                clock_name: naming.get(&constraint.i).unwrap().to_string(),
            }),
            y: Some(ComponentClock{
                specific_component: None, // TODO how?
                clock_name: naming.get(&constraint.j).unwrap().to_string(),
            }),
            strict: constraint.ineq().is_strict(),
            c: constraint.ineq().bound(),
        }
    }
}

impl From<&Edge> for ProtoEdge {
    fn from(e: &Edge) -> Self {
        ProtoEdge {
            id: e.id.clone(),
            specific_component: None, // TODO fix this
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tests::{grpc::grpc_helper::create_initial_decision_point, Simulation::helper::{initial_transition_decision_point_EcdarUniversity_Machine, create_EcdarUniversity_Machine_system}},
        Simulation::decision_point::DecisionPoint,
        ProtobufServer::services::DecisionPoint as ProtoDecisionPoint,
    };

    #[test]
    fn from__good_DecisionPoint__returns_good_ProtoDecisionPoint(
    ) {
        // Arrange
        let transitionDecisionPoint = initial_transition_decision_point_EcdarUniversity_Machine();
        let decisionPoint = DecisionPoint::from(&transitionDecisionPoint);
        let system = create_EcdarUniversity_Machine_system();

        // Act
        let actual = ProtoDecisionPoint::from(&decisionPoint, &system);

        // Assert
        let expected = create_initial_decision_point();

        assert_eq!(actual.edges.len(), 2);
        assert!(actual.edges.contains(&expected.edges[0]));
        assert!(actual.edges.contains(&expected.edges[1]));
        assert_eq!(actual.source, expected.source);
    }
}
