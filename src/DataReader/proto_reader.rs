use edbm::util::constraints::{Conjunction, Constraint, Disjunction, Inequality, RawInequality};
use edbm::zones::OwnedFederation;

use crate::component::{Component, Edge, State};
use crate::ProtobufServer::services::{
    ComponentClock as ProtoComponentClock, Conjunction as ProtoConjunction,
    Constraint as ProtoConstraint, Decision as ProtoDecision, Disjunction as ProtoDisjunction,
    Edge as ProtoEdge, Federation as ProtoFederation, LocationTuple as ProtoLocationTuple,
    State as ProtoState,
};
use crate::Simulation::decision::Decision;
use crate::TransitionSystems::{LocationTuple, TransitionSystemPtr};

pub fn proto_decision_to_decision(
    proto_decision: ProtoDecision,
    system: &TransitionSystemPtr,
    components: Vec<Component>,
) -> Decision {
    let proto_state: ProtoState = match proto_decision.source {
        None => panic!("Not found"),
        Some(source) => source,
    };

    let proto_edge: ProtoEdge = match proto_decision.edge {
        None => panic!("Edge not found!"),
        Some(edge) => edge,
    };
    let state = proto_state_to_state(proto_state, system);

    let decided = proto_edge_to_edge(proto_edge, components);

    Decision::new(state, decided)
}

pub fn proto_state_to_state(state: ProtoState, system: &TransitionSystemPtr) -> State {
    // Convert ProtoState to State
    let proto_location_tuple: ProtoLocationTuple = match state.location_tuple {
        None => panic!("No loc tuple"),
        Some(loc_tuple) => loc_tuple,
    };
    let proto_federation: ProtoFederation = match state.federation {
        None => panic!("No federation found"),
        Some(federation) => federation,
    };
    let zone: OwnedFederation = proto_federation_to_owned_federation(proto_federation, system);
    let location_tuple =
        match LocationTuple::from_proto_location_tuple(&proto_location_tuple, system) {
            None => panic!("No location tuple found"),
            Some(loc_tuple) => loc_tuple,
        };
    State::create(location_tuple, zone)
}

fn proto_edge_to_edge(proto_edge: ProtoEdge, components: Vec<Component>) -> Edge {
    components
        .into_iter()
        .map(|c| c.get_edges().to_owned())
        .reduce(|acc, es| acc.into_iter().chain(es.into_iter()).collect())
        .unwrap()
        .into_iter()
        .find(|e| e.id == proto_edge.id)
        .unwrap()
}

fn proto_constraint_to_constraint(
    proto_constraint: ProtoConstraint,
    system: &TransitionSystemPtr,
) -> Constraint {
    fn determine_index(clock: ProtoComponentClock, system: &TransitionSystemPtr) -> usize {
        if clock.clock_name == "0" && clock.specific_component.is_none() {
            return 0;
        } else {
            system
                .clock_name_and_component_to_index(
                    &clock.clock_name,
                    &clock.specific_component.unwrap().component_name,
                )
                .unwrap()
        }
    }
    let x_clock = match proto_constraint.x {
        None => panic!("No clock name"),
        Some(clock) => clock,
    };
    let y_clock = match proto_constraint.y {
        None => panic!("No clock name"),
        Some(clock) => clock,
    };

    let i = determine_index(x_clock, system);
    let j = determine_index(y_clock, system);

    let inequality = match proto_constraint.strict {
        true => Inequality::LS(proto_constraint.c),
        false => Inequality::LE(proto_constraint.c),
    };

    let ineq: RawInequality = RawInequality::from_inequality(&inequality);
    Constraint::new(i, j, ineq)
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
    OwnedFederation::from_disjunction(&disjunction, system.get_dim())
}

#[cfg(test)]
mod tests {
    use crate::{
        tests::Simulation::helper::{
            create_EcdarUniversity_Machine3and1_with_nonempty_Federation_Decision,
            create_EcdarUniversity_Machine_Decision, create_EcdarUniversity_Machine_component,
            create_EcdarUniversity_Machine_system,
            create_EcdarUniversity_Machine_with_nonempty_Federation_Decision,
        },
        DataReader::{json_reader::read_json_component, proto_reader::proto_decision_to_decision},
        Simulation::decision::Decision,
        TransitionSystems::CompiledComponent,
    };

    // TODO this test is badly formatted
    #[test]
    fn from__ProtoDecision_with_universal_ProtoFederation__returns_correct_Decision() {
        // Arrange
        let component = create_EcdarUniversity_Machine_component();
        let proto_decision = create_EcdarUniversity_Machine_Decision();
        let system = create_EcdarUniversity_Machine_system();

        let expected_edge = component.find_edge_from_id("E29");

        let expected_source = match system.get_initial_state() {
            None => panic!("No inital state found"),
            Some(expected_source) => expected_source,
        };

        let expected_decision = Decision::new(expected_source, expected_edge.to_owned());

        // Act
        let actual_decision = proto_decision_to_decision(proto_decision, &system, vec![component]);

        let actual_decision = format!("{:?}", actual_decision);
        let expected_decision = format!("{:?}", expected_decision);

        // Assert
        assert_eq!(actual_decision, expected_decision);
    }

    #[test]
    fn from__ProtoDecision_with_nonuniversal_ProtoFederation__returns_correct_Decision() {
        // Arrange
        let component = create_EcdarUniversity_Machine_component();
        let proto_decision = create_EcdarUniversity_Machine_with_nonempty_Federation_Decision();
        let system = create_EcdarUniversity_Machine_system();

        let expected_edge = component.find_edge_from_id("E29");

        let action = "tea";
        let mut expected_source = system.get_initial_state().unwrap();
        let transition =
            system.next_transitions_if_available(expected_source.get_location(), action);
        transition
            .first()
            .unwrap()
            .use_transition(&mut expected_source);

        let expected_decision = Decision::new(expected_source, expected_edge.to_owned());

        // Act
        let actual_decision = proto_decision_to_decision(proto_decision, &system, vec![component]);

        let actual_decision = format!("{:?}", actual_decision);
        let expected_decision = format!("{:?}", expected_decision);

        // Assert
        assert_eq!(actual_decision, expected_decision);
    }

    #[test]
    fn from__ProtoDecision_with_composite_components__returns_correct_Decision() {
        // Arrange
        let machine3 = read_json_component("samples/json/EcdarUniversity", "Machine3");
        let machine = read_json_component("samples/json/EcdarUniversity", "Machine");
        let components = vec![machine3.clone(), machine.clone()];
        let system = CompiledComponent::from(components.clone(), "( Machine3 && Machine )");
        let proto_decision =
            create_EcdarUniversity_Machine3and1_with_nonempty_Federation_Decision();

        let expected_edge = machine.find_edge_from_id("E29");

        let expected_source = system.get_initial_state().unwrap();

        let expected_decision = Decision::new(expected_source.clone(), expected_edge.to_owned());

        // Act
        let actual_decision = proto_decision_to_decision(proto_decision, &system, components);

        let actual_decision = format!("{:?}", actual_decision);
        let expected_decision = format!("{:?}", expected_decision);

        // Assert
        assert_eq!(actual_decision, expected_decision);
    }
}
