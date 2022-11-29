use edbm::util::constraints::{Conjunction, Constraint, Disjunction, Inequality, RawInequality};
use edbm::zones::OwnedFederation;

use crate::component::{Component, Edge, State};
use crate::ProtobufServer::services::{
    ComponentClock as ProtoComponentClock, ComponentsInfo, Conjunction as ProtoConjunction,
    Constraint as ProtoConstraint, Decision as ProtoDecision, Disjunction as ProtoDisjunction,
    Edge as ProtoEdge, Federation as ProtoFederation, LocationTuple as ProtoLocationTuple,
    SimulationInfo, State as ProtoState,
};
use crate::Simulation::decision::Decision;
use crate::TransitionSystems::transition_system::component_loader_to_transition_system;
use crate::TransitionSystems::{LocationID, LocationTuple, TransitionSystemPtr};

use super::component_loader::{parse_components_if_some, ComponentContainer};

/// Borrows a [`SimulationInfo`] and returns the corresponding [`TransitionsSystemPtr`].
pub fn simulation_info_to_transition_system(
    simulation_info: &SimulationInfo,
) -> TransitionSystemPtr {
    let composition = simulation_info.component_composition.to_owned();
    let component_info = simulation_info.components_info.as_ref().unwrap();

    let mut component_container = ComponentContainer::from(component_info).unwrap();

    component_loader_to_transition_system(&mut component_container, &composition)
}

/// Borrows a [`ComponentsInfo`] and returns the corresponding [`Vec`] of [`Component`]s.
pub fn components_info_to_components(components_info: &ComponentsInfo) -> Vec<Component> {
    components_info
        .components
        .iter()
        .flat_map(parse_components_if_some)
        .flatten()
        .collect()
}

/// Consumes a [`ProtoDecision`] and the borrows the [`TransitionsSystemPtr`] it belongs to and returns the corresponding [`Decision`].
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

/// Consumes a [`ProtoState`] and the borrows the [`TransitionsSystemPtr`] it belongs to and returns the corresponding [`State`].
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
    let location_tuple = match proto_location_tuple_to_location_tuple(&proto_location_tuple, system)
    {
        None => panic!("No location tuple found"),
        Some(loc_tuple) => loc_tuple,
    };
    State::create(location_tuple, zone)
}

fn proto_location_tuple_to_location_tuple(
    location_tuple: &ProtoLocationTuple,
    system: &TransitionSystemPtr,
) -> Option<LocationTuple> {
    let id_looking_for: Vec<LocationID> = location_tuple
        .locations
        .iter()
        .map(|l| LocationID::Simple {
            location_id: l.id.to_string(),
            component_id: l
                .specific_component
                .as_ref()
                .map(|c| c.component_name.to_string()),
        })
        .collect();

    system
        .get_all_locations()
        .into_iter()
        .map(|tuple| (tuple.id.clone(), tuple))
        .map(|(id, tuple)| (id.inorder_vec_tranform(), tuple))
        .filter(|(id, _)| id.iter().eq(id_looking_for.iter()))
        .collect::<Vec<_>>()
        .first()
        .map(|(_, tuple)| tuple.to_owned())
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
            0
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
        tests::Simulation::test_data::{
            create_EcdarUniversity_Machine3and1_with_nonempty_Federation_Decision,
            create_EcdarUniversity_Machine_Decision, create_EcdarUniversity_Machine_component,
            create_EcdarUniversity_Machine_system,
            create_EcdarUniversity_Machine_with_nonempty_Federation_Decision,
        },
        DataReader::{json_reader::read_json_component, proto_reader::proto_decision_to_decision},
        Simulation::decision::Decision,
        TransitionSystems::transition_system::components_to_transition_system,
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
        let components = vec![machine3, machine.clone()];
        let system = components_to_transition_system(components.clone(), "( Machine3 && Machine )");
        let proto_decision =
            create_EcdarUniversity_Machine3and1_with_nonempty_Federation_Decision();

        let expected_edge = machine.find_edge_from_id("E29");

        let expected_source = system.get_initial_state().unwrap();

        let expected_decision = Decision::new(expected_source, expected_edge.to_owned());

        // Act
        let actual_decision = proto_decision_to_decision(proto_decision, &system, components);

        let actual_decision = format!("{:?}", actual_decision);
        let expected_decision = format!("{:?}", expected_decision);

        // Assert
        assert_eq!(actual_decision, expected_decision);
    }
}
