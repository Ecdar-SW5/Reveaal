use edbm::util::constraints::{
    ClockIndex, Conjunction, Constraint, Disjunction, Inequality, RawInequality,
};
use edbm::zones::OwnedFederation;

use crate::component::{Declarations, Edge, State};
use crate::ProtobufServer::services::{
    Conjunction as ProtoConjunction, Constraint as ProtoConstraint, Decision as ProtoDecision,
    Disjunction as ProtoDisjunction, Edge as ProtoEdge, Federation as ProtoFederation,
    LocationTuple as ProtoLocationTuple, State as ProtoState,
};
use crate::System::save_component::combine_components;
use crate::System::save_component::PruningStrategy::NoPruning;
use crate::TransitionSystems::{LocationTuple, TransitionSystemPtr};

#[derive(Debug)]
pub struct Decision {
    source: State,
    decided: Edge,
}

impl Decision {
    pub fn new(source: State, decided: Edge) -> Self {
        Self { source, decided }
    }

    pub fn source(&self) -> &State {
        &self.source
    }

    pub fn decided(&self) -> &Edge {
        &self.decided
    }

    pub fn convert_protoedge_to_edge(protoedge: ProtoEdge, system: &TransitionSystemPtr) -> Edge {
        let component = combine_components(&system, NoPruning);

        let edges = component.get_edges();

        edges
            .iter()
            .filter(|e| e.id == protoedge.id)
            .nth(0)
            .unwrap()
            .to_owned()
    }

    // TODO: This needs to be rewritten, as it most
    pub fn from(proto_decision: ProtoDecision, system: &TransitionSystemPtr) -> Self {
        // Convert ProtoState to State
        let proto_state: ProtoState = match proto_decision.source {
            None => panic!("Not found"),
            Some(source) => source,
        };

        let proto_edge: ProtoEdge = match proto_decision.edge {
            None => panic!("Edge not found!"),
            Some(edge) => edge,
        };

        let proto_location_tuple: ProtoLocationTuple = match proto_state.location_tuple {
            None => panic!("No loc tuple"),
            Some(loc_tuple) => loc_tuple,
        };
    

        let proto_federation: ProtoFederation = match proto_state.federation {
            None => panic!("No federation found"),
            Some(federation) => federation,
        };

        let zone: OwnedFederation = proto_federation_to_owned_federation(proto_federation, system);

        let location_tuple =
            match LocationTuple::from_proto_location_tuple(&proto_location_tuple, system) {
                None => panic!("No location tuple found"),
                Some(loc_tuple) => loc_tuple,
            };

        let state = State::create(location_tuple, zone);

        let decided = Self::convert_protoedge_to_edge(proto_edge, &system);

        Decision {
            source: state,
            decided,
        }
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
    Constraint::new(i, j, ineq)
}

fn get_clock_index_from_name(name: &str, decls: &Vec<&Declarations>) -> ClockIndex {
    if name == "0" {
        return 0;
    } else {
        for dec in decls {
            match dec.get_clock_index_by_name(name) {
                None => continue,
                Some(clock) => return *clock,
            };
        }
    }
    panic!("Clock not found");
}
// BLOCKED!! :tf:
// TODO: This needs to be rewritten, as it most likely is not working corectly.
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
    // This does not work for some reason lol.
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
            create_EcdarUniversity_Machine_Decision, create_EcdarUniversity_Machine_system,
        },
        DataReader::json_reader::read_json_component,
        Simulation::decision::Decision,
    };

    // TODO this test is badly formatted
    #[test]
    fn from__ProtoDecision_with_universal_ProtoFederation__returns_correct_Decision() {
        // Arrange
        let project_path = "samples/json/EcdarUniversity";
        let proto_decision = create_EcdarUniversity_Machine_Decision();
        let system = create_EcdarUniversity_Machine_system();
        let component = read_json_component(project_path, "Machine");

        let expected_edge = component
            .get_edges()
            .iter()
            // .filter_map(|e| );
            .filter(|e| e.id.contains("E29"))
            .nth(0)
            .unwrap();

        let expected_source = match system.get_initial_state() {
            None => panic!("No inital state found"),
            Some(expected_source) => expected_source,
        };

        let expected_decision = Decision {
            source: expected_source,
            decided: expected_edge.to_owned(),
        };

        // Act
        let actual_decision = Decision::from(proto_decision, &system);

        let actual_decision = format!("{:?}", actual_decision);
        let expected_decision = format!("{:?}", expected_decision);

        // Assert
        assert_eq!(actual_decision, expected_decision);
    }

    fn from__ProtoDecision_with_nonuniversal_ProtoFederation__returns_correct_Decision() {
        // Arrange


        // Act

        // Assert
    }
}
