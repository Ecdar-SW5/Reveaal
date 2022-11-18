use crate::ComponentLoader;
use crate::ModelObjects::component::{
    Component, DeclarationProvider, Declarations, State, Transition,
};
use crate::System::local_consistency::{self, ConsistencyResult, DeterminismResult};
use crate::TransitionSystems::{LocationTuple, TransitionSystem, TransitionSystemPtr};
use edbm::util::bounds::Bounds;
use edbm::util::constraints::ClockIndex;
use log::warn;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;

use super::transition_system::PrecheckResult;
use super::{CompositionType, LocationID};

use crate::DataReader::parse_queries::Rule;
use crate::{
    extract_system_rep::get_system_recipe,
    parse_queries::{build_expression_from_pair, QueryParser},
    DataReader::component_loader::ComponentContainer,
};
use pest::Parser;

type Action = String;

#[derive(Clone)]
struct ComponentInfo {
    //name: String,
    declarations: Declarations,
    max_bounds: Bounds,
}

#[derive(Clone)]
pub struct CompiledComponent {
    inputs: HashSet<Action>,
    outputs: HashSet<Action>,
    locations: HashMap<LocationID, LocationTuple>,
    location_edges: HashMap<LocationID, Vec<(Action, Transition)>>,
    initial_location: Option<LocationTuple>,
    comp_info: ComponentInfo,
    dim: ClockIndex,
}

impl CompiledComponent {
    pub fn compile_with_actions(
        component: Component,
        inputs: HashSet<String>,
        outputs: HashSet<String>,
        dim: ClockIndex,
    ) -> Result<Box<Self>, String> {
        if !inputs.is_disjoint(&outputs) {
            return Err("Inputs and outputs must be disjoint in component".to_string());
        }

        let locations: HashMap<LocationID, LocationTuple> = component
            .get_locations()
            .iter()
            .map(|loc| {
                let tuple = LocationTuple::simple(
                    loc,
                    Some(component.get_name().to_owned()),
                    component.get_declarations(),
                    dim,
                );
                (tuple.id.clone(), tuple)
            })
            .collect();

        let mut location_edges: HashMap<LocationID, Vec<(Action, Transition)>> =
            locations.keys().map(|k| (k.clone(), vec![])).collect();

        for edge in component.get_edges() {
            let id = LocationID::Simple {
                location_id: edge.source_location.clone(),
                component_id: Some(component.get_name().to_owned()),
            };
            let transition = Transition::from(&component, edge, dim);
            location_edges
                .get_mut(&id)
                .unwrap()
                .push((edge.sync.clone(), transition));
        }

        let initial_location = locations.values().find(|loc| loc.is_initial()).cloned();

        let max_bounds = component.get_max_bounds(dim);
        Ok(Box::new(CompiledComponent {
            inputs,
            outputs,
            locations,
            location_edges,
            initial_location,
            dim,
            comp_info: ComponentInfo {
                //name: component.name,
                declarations: component.declarations,
                max_bounds,
            },
        }))
    }

    pub fn compile(component: Component, dim: ClockIndex) -> Result<Box<Self>, String> {
        let inputs: HashSet<_> = component
            .get_input_actions()
            .iter()
            .map(|c| c.name.clone())
            .collect();
        let outputs: HashSet<_> = component
            .get_output_actions()
            .iter()
            .map(|c| c.name.clone())
            .collect();

        Self::compile_with_actions(component, inputs, outputs, dim)
    }

    pub fn from(components: Vec<Component>, composition: &str) -> TransitionSystemPtr {
        let mut component_container = ComponentContainer::create_component_container(components);
        Self::from_component_loader(&mut component_container, composition)
    }

    pub fn from_component_loader(
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
}

impl TransitionSystem for CompiledComponent {
    fn get_local_max_bounds(&self, loc: &LocationTuple) -> Bounds {
        if loc.is_universal() || loc.is_inconsistent() {
            Bounds::new(self.get_dim())
        } else {
            self.comp_info.max_bounds.clone()
        }
    }

    fn get_composition_type(&self) -> CompositionType {
        panic!("Components do not have a composition type")
    }

    fn get_decls(&self) -> Vec<&Declarations> {
        vec![&self.comp_info.declarations]
    }

    fn get_input_actions(&self) -> HashSet<String> {
        self.inputs.clone()
    }

    fn get_output_actions(&self) -> HashSet<String> {
        self.outputs.clone()
    }

    fn get_actions(&self) -> HashSet<String> {
        self.inputs.union(&self.outputs).cloned().collect()
    }

    fn get_initial_location(&self) -> Option<LocationTuple> {
        self.initial_location.clone()
    }

    fn get_all_locations(&self) -> Vec<LocationTuple> {
        self.locations.values().cloned().collect()
    }

    fn next_transitions(&self, locations: &LocationTuple, action: &str) -> Vec<Transition> {
        assert!(self.actions_contain(action));
        let is_input = self.inputs_contain(action);

        if locations.is_universal() {
            return vec![Transition::new(locations, self.dim)];
        }

        if locations.is_inconsistent() && is_input {
            return vec![Transition::new(locations, self.dim)];
        }

        let mut transitions = vec![];
        let edges = self.location_edges.get(&locations.id).unwrap();

        for (channel, transition) in edges {
            if *channel == action {
                transitions.push(transition.clone());
            }
        }

        transitions
    }

    fn get_initial_state(&self) -> Option<State> {
        let init_loc = self.get_initial_location().unwrap();

        State::from_location(init_loc, self.dim)
    }

    fn get_children(&self) -> (&TransitionSystemPtr, &TransitionSystemPtr) {
        unimplemented!()
    }

    fn precheck_sys_rep(&self) -> PrecheckResult {
        if let DeterminismResult::Failure(location, action) = self.is_deterministic() {
            warn!("Not deterministic");
            return PrecheckResult::NotDeterministic(location, action);
        }

        if let ConsistencyResult::Failure(failure) = self.is_locally_consistent() {
            warn!("Not consistent");
            return PrecheckResult::NotConsistent(failure);
        }
        PrecheckResult::Success
    }

    fn is_deterministic(&self) -> DeterminismResult {
        local_consistency::is_deterministic(self)
    }

    fn is_locally_consistent(&self) -> ConsistencyResult {
        local_consistency::is_least_consistent(self)
    }

    fn get_dim(&self) -> ClockIndex {
        self.dim
    }
}
