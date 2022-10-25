use crate::{
    component::{State, Transition},
    TransitionSystems::TransitionSystemPtr,
};

#[allow(dead_code)]
pub struct TransitionDecision {
    source: State,
    transitions: Vec<Transition>,
}

impl TransitionDecision {
    /// Constructs the inital TransitionDecision for a given TransitionSystemPtr
    pub fn initial(system: TransitionSystemPtr) -> Option<Self> {
        match system.get_initial_state() {
            Some(source) => Some(Self::from(system, source)),
            None => None
        }
    }

    /// Constructs the TransitionDecision from a source State and a given TransitionSystemPtr
    pub fn from(system: TransitionSystemPtr, source: State) -> TransitionDecision {
        let mut transitions = vec![];
        // let actions = system.get_actions();

        // get all transitions
        // for action in actions {
        //     let transition = system.next_transitions_if_available(source.get_location(), &action);
        //     transitions.append(&mut transition.clone());
        // }

        // prune transitions that can not be taken
        // for (index, transition) in transitions.clone().iter().enumerate() {
        //     if !transition.use_transition(&mut source.clone()) {
        //         transitions.remove(index);
        //     }
        // }

        TransitionDecision {
            source: source,
            transitions: transitions,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashSet, HashMap};
    use edbm::util::{bounds::Bounds, constraints::ClockIndex};
    use crate::{TransitionSystems::{TransitionSystem, LocationTuple, TransitionSystemPtr, CompositionType}, component::{Transition, Declarations, State, Component}, DataReader::{json_reader::read_json_component, component_loader::{self, ComponentContainer}, json_writer::component_to_json}, extract_system_rep::{SystemRecipe, get_system_recipe}, parse_queries::{QueryParser, build_expression_from_pair}};
    use super::TransitionDecision;
    use crate::DataReader::parse_queries::Rule;
    use pest::Parser;

    fn initial__no_initial_state__returns_none() {
    }

    fn system_from(components: Vec<Component>, composition: &str) -> TransitionSystemPtr {
        let mut component_map = HashMap::new();

        for component in components  {
            component_map.insert(component.name.clone(), component);
        }

        let mut component_container = ComponentContainer {
            loaded_components: component_map
        };

        let mut dimension = 0;
        let composition = QueryParser::parse(Rule::expr, composition)
            .unwrap()
            .next()
            .unwrap();
        let composition = build_expression_from_pair(composition);
        get_system_recipe(
            &composition,
            &mut component_container,
            &mut dimension,
            &mut None,
        )
        .compile(dimension)
        .unwrap()
    }


    fn create_EcdarUniversity_Machine_system() -> TransitionSystemPtr {
        let mut component = read_json_component("samples/json/EcdarUniversity", "Machine");
        component.create_edge_io_split();
        system_from(vec![component], "Machine")
    }

    #[test]
    fn initial__EcdarUniversity_Machine__return_state_L5() {
        // Arrange
        let system = create_EcdarUniversity_Machine_system();

        // Act
        let actual = TransitionDecision::initial(system).unwrap();
        
        // Assert
        assert_eq!(actual.source.get_location().id.to_string(), "L5")
    }
}
