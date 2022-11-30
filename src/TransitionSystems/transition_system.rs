use super::{CompositionType, LocationID, LocationTuple};
use crate::DataReader::parse_queries::Rule;
use crate::{
    component::Component,
    extract_system_rep::get_system_recipe,
    parse_queries::{build_expression_from_pair, QueryParser},
    ComponentLoader,
    DataReader::component_loader::ComponentContainer,
    ModelObjects::component::{Declarations, State, Transition},
    System::local_consistency::DeterminismResult,
    System::local_consistency::{ConsistencyFailure, ConsistencyResult},
};
use dyn_clone::{clone_trait_object, DynClone};
use edbm::util::{bounds::Bounds, constraints::ClockIndex};
use pest::Parser;
use std::hash::Hash;
use std::{
    collections::{hash_set::HashSet, HashMap},
    iter::zip,
};

pub type TransitionSystemPtr = Box<dyn TransitionSystem>;

/// Precheck can fail because of either consistency or determinism.
pub enum PrecheckResult {
    Success,
    NotDeterministic(LocationID, String),
    NotConsistent(ConsistencyFailure),
}

pub trait TransitionSystem: DynClone {
    fn get_local_max_bounds(&self, loc: &LocationTuple) -> Bounds;

    fn get_dim(&self) -> ClockIndex;

    fn next_transitions_if_available(
        &self,
        location: &LocationTuple,
        action: &str,
    ) -> Vec<Transition> {
        if self.actions_contain(action) {
            self.next_transitions(location, action)
        } else {
            vec![]
        }
    }

    fn next_transitions(&self, location: &LocationTuple, action: &str) -> Vec<Transition>;

    fn next_outputs(&self, location: &LocationTuple, action: &str) -> Vec<Transition> {
        debug_assert!(self.get_output_actions().contains(action));
        self.next_transitions(location, action)
    }

    fn next_inputs(&self, location: &LocationTuple, action: &str) -> Vec<Transition> {
        debug_assert!(self.get_input_actions().contains(action));
        self.next_transitions(location, action)
    }

    fn get_input_actions(&self) -> HashSet<String>;

    fn inputs_contain(&self, action: &str) -> bool {
        self.get_input_actions().contains(action)
    }

    fn get_output_actions(&self) -> HashSet<String>;

    fn outputs_contain(&self, action: &str) -> bool {
        self.get_output_actions().contains(action)
    }

    fn get_actions(&self) -> HashSet<String>;

    fn actions_contain(&self, action: &str) -> bool {
        self.get_actions().contains(action)
    }

    fn get_initial_location(&self) -> Option<LocationTuple>;

    fn get_all_locations(&self) -> Vec<LocationTuple>;

    fn get_decls(&self) -> Vec<&Declarations>;

    fn precheck_sys_rep(&self) -> PrecheckResult;

    fn is_deterministic(&self) -> DeterminismResult;

    fn is_locally_consistent(&self) -> ConsistencyResult;

    fn get_initial_state(&self) -> Option<State>;

    fn get_children(&self) -> (&TransitionSystemPtr, &TransitionSystemPtr);

    fn get_composition_type(&self) -> CompositionType;

    /// Returns a [`Vec`] of all component names in a given [`TransitionSystem`].
    fn component_names(&self) -> Vec<&str> {
        let children = self.get_children();
        let left_child = children.0;
        let right_child = children.1;
        left_child
            .component_names()
            .into_iter()
            .chain(right_child.component_names().into_iter())
            .collect()
    }

    /// Maps a clock- and component name to a clock index for a given [`TransitionSystem`].
    fn clock_name_and_component_to_index(&self, name: &str, component: &str) -> Option<usize> {
        let index_to_clock_name_and_component = self.clock_name_and_component_to_index_map();
        index_to_clock_name_and_component
            .get(&(name.to_string(), component.to_string()))
            .copied()
    }

    /// Maps a clock index to a clock- and component name for a given [`TransitionSystem`].
    fn index_to_clock_name_and_component(&self, index: &usize) -> Option<(String, String)> {
        fn invert<T1, T2>(hash_map: HashMap<T1, T2>) -> HashMap<T2, T1>
        where
            T2: Hash + Eq,
        {
            hash_map.into_iter().map(|x| (x.1, x.0)).collect()
        }

        let index_to_clock_name_and_component = self.clock_name_and_component_to_index_map();
        let index_to_clock_name_and_component = invert(index_to_clock_name_and_component);
        index_to_clock_name_and_component
            .get(index)
            .map(|x| x.to_owned())
    }

    /// Returns a [`HashMap`] from clock- and component names to clock indices.
    fn clock_name_and_component_to_index_map(&self) -> HashMap<(String, String), usize> {
        let binding = self.component_names();
        let component_names = binding.into_iter();
        let binding = self.get_decls();
        let clock_to_index = binding.into_iter().map(|decl| decl.clocks.to_owned());

        zip(component_names, clock_to_index)
            .map(|x| {
                x.1.iter()
                    .map(|y| ((y.0.to_owned(), x.0.to_string()), y.1.to_owned()))
                    .collect::<HashMap<(String, String), usize>>()
            })
            .fold(HashMap::new(), |accumulator, head| {
                accumulator.into_iter().chain(head).collect()
            })
    }
}

pub fn components_to_transition_system(
    components: Vec<Component>,
    composition: &str,
) -> TransitionSystemPtr {
    let mut component_container = ComponentContainer::create_component_container(components);
    component_loader_to_transition_system(&mut component_container, composition)
}

pub fn component_loader_to_transition_system(
    loader: &mut dyn ComponentLoader,
    composition: &str,
) -> TransitionSystemPtr {
    let mut dimension = 0;
    let composition = QueryParser::parse(Rule::expr, composition)
        .unwrap()
        .next()
        .unwrap();
    let composition = build_expression_from_pair(composition);
    get_system_recipe(&composition, loader, &mut dimension, &mut None)
        .compile(dimension)
        .unwrap()
}

clone_trait_object!(TransitionSystem);
