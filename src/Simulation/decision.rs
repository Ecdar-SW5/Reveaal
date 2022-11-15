use edbm::util::constraints::{
    ClockIndex, Conjunction, Constraint, Disjunction, Inequality, RawInequality,
};
use edbm::zones::OwnedFederation;

use crate::component::{Declarations, Edge, Location, State};
use crate::ProtobufServer::services::{
    Conjunction as ProtoConjunction, Constraint as ProtoConstraint, Decision as ProtoDecision,
    Disjunction as ProtoDisjunction, Edge as ProtoEdge, Federation as ProtoFederation,
    Location as ProtoLocation, LocationTuple as ProtoLocationTuple, State as ProtoState,
};
use crate::TransitionSystems::{LocationID, TransitionSystemPtr};

#[derive(Debug)]
pub struct Decision {
    pub source: State,
    pub decided: Edge,
}

impl Decision {
    pub fn from(proto_decision: ProtoDecision, system: &TransitionSystemPtr) -> Self {
        // Convert ProtoState to State
        let proto_state: ProtoState = match proto_decision.source {
            None => panic!("Not found"),
            Some(source) => source,
        };

        let proto_location_tuple: ProtoLocationTuple = match proto_state.location_tuple {
            None => panic!("No loc tuple"),
            Some(loc_tuple) => loc_tuple,
        };

        let _proto_location_ids: Vec<LocationID> = proto_location_tuple
            .locations
            .iter()
            .map(|loc| LocationID::from_string(loc.id.as_str()))
            .collect();

        let proto_federation: ProtoFederation = match proto_state.federation {
            None => panic!("No federation found"),
            Some(federation) => federation,
        };

        let _zone: OwnedFederation = proto_federation_to_owned_federation(proto_federation, system);

        // Generate the location tuple
        // 1. Generate simple location tuples from the proto locations
        // 2. Compose each simple location tuple to a single location tuple using the composition type
        // 3. Return the location tuple

        let proto_locations: Vec<ProtoLocation> = proto_location_tuple
            .locations
            .iter()
            .map(|loc| loc.clone())
            .collect();

        let locations: Vec<Location> = proto_locations
            .iter()
            .map(|loc| Location::from_proto_location(loc, system))
            .collect();
        // let state = State::create(location_tuple, zone);

        // Convert ProtoEdge to Edge
        let _proto_edge: ProtoEdge = match proto_decision.edge {
            None => panic!("No edge found"),
            Some(edge) => edge,
        };

        todo!();
        // return Decision {
        //     source: todo!(),
        //     decided: todo!(),
        // };
    }
}

fn proto_constraint_to_constraint(
    proto_constraint: ProtoConstraint,
    system: &TransitionSystemPtr,
) -> Constraint {
    let decls: Vec<&Declarations> = system.get_decls();

    let x_clock_name = match proto_constraint.x {
        None => panic!("No clock name"),
        Some(clock) => clock.clock_name,
    };
    let y_clock_name = match proto_constraint.y {
        None => panic!("No clock name"),
        Some(clock) => clock.clock_name,
    };

    let i = get_clock_index_from_name(&x_clock_name, &decls);
    let j = get_clock_index_from_name(&y_clock_name, &decls);

    let inequality = match proto_constraint.strict {
        true => Inequality::LS(proto_constraint.c),
        false => Inequality::LE(proto_constraint.c),
    };

    let ineq: RawInequality = RawInequality::from_inequality(&inequality);
    let constraint = Constraint::new(i, j, ineq);

    constraint
}

fn get_clock_index_from_name(name: &str, decls: &Vec<&Declarations>) -> ClockIndex {
    for dec in decls {
        match dec.get_clock_index_by_name(name) {
            None => continue,
            Some(clock) => return *clock,
        };
    }

    panic!("Clock not found");
}

fn proto_federation_to_owned_federation(
    proto_federation: ProtoFederation,
    system: &TransitionSystemPtr,
) -> OwnedFederation {
    let proto_disjunction: ProtoDisjunction = match proto_federation.disjunction {
        None => panic!("No Disjuntion found"),
        Some(disjunction) => disjunction,
    };

    let proto_conjunctions: Vec<ProtoConjunction> = proto_disjunction.conjunctions;
    let proto_constraints: Vec<Vec<ProtoConstraint>> = proto_conjunctions
        .iter()
        .map(|conjunction| conjunction.constraints.clone())
        .collect();

    let mut constraints: Vec<Vec<Constraint>> = Vec::new();

    for vec_proto_constraint in proto_constraints {
        let mut constraint_vec: Vec<Constraint> = Vec::new();
        for proto_constraint in vec_proto_constraint {
            let constraint = proto_constraint_to_constraint(proto_constraint, system);
            constraint_vec.push(constraint);
        }
        constraints.push(constraint_vec);
    }

    let mut conjunctions: Vec<Conjunction> = Vec::new();

    for constraint_vec in constraints {
        let conjunction = Conjunction::new(constraint_vec);
        conjunctions.push(conjunction);
    }

    let disjunction: Disjunction = Disjunction::new(conjunctions);
    let owned_federation = OwnedFederation::from_disjunction(&disjunction, system.get_dim());

    owned_federation
}

#[cfg(test)]
mod tests {
    use crate::{
        component::Edge,
        tests::Simulation::helper::{
            create_EcdarUniversity_Machine_Decision, create_EcdarUniversity_Machine_system,
            initial_transition_decision_point_EcdarUniversity_Machine,
        },
        Simulation::decision::Decision,
    };

    // TODO this test is badly formatted
    #[test]
    fn Decision_from__ProtoDecision__returns_correct_Decision() {
        // Arrange
        let proto_decision = create_EcdarUniversity_Machine_Decision();

        let transition_decisions = initial_transition_decision_point_EcdarUniversity_Machine();
        let possible_decisions: Vec<Edge> = transition_decisions
            .possible_decisions
            .iter()
            .flat_map(|t| Vec::<Edge>::from(t))
            .collect();

        let expected_decision = match possible_decisions.into_iter().next() {
            None => panic!("No edges found"),
            Some(edge) => edge,
        };

        let system = create_EcdarUniversity_Machine_system();
        let actual_decision = Decision::from(proto_decision, &system);

        let expected_source = match system.get_initial_state() {
            None => panic!("No inital state found"),
            Some(expected_source) => expected_source,
        };

        let expected_decision = Decision {
            source: expected_source,
            decided: expected_decision,
        };

        // Act
        let actual_decision = format!("{:?}", actual_decision);
        let expected_decision = format!("{:?}", expected_decision);

        // Assert
        assert_eq!(actual_decision, expected_decision);
    }
}
