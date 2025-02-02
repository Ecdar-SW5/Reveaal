use crate::extract_system_rep::SystemRecipeFailure;
use crate::ModelObjects::component::{
    Component, DeclarationProvider, Declarations, State, Transition,
};
use crate::System::local_consistency::{self, ConsistencyResult, DeterminismResult};
use crate::TransitionSystems::{LocationTuple, TransitionSystem, TransitionSystemPtr};
use edbm::util::bounds::Bounds;
use edbm::util::constraints::ClockIndex;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;

use super::common::CollectionOperation;
use super::{CompositionType, LocationID};

type Action = String;

#[derive(Clone)]
struct ComponentInfo {
    name: String,
    declarations: Declarations,
    max_bounds: Bounds,
}

impl ComponentInfo {
    pub fn _name(&self) -> &str {
        self.name.as_ref()
    }
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
    ) -> Result<Box<Self>, SystemRecipeFailure> {
        if let Err(actions) = inputs.is_disjoint_action(&outputs) {
            return Err(SystemRecipeFailure::new_from_component(
                "Input is not disjoint from output".to_string(),
                component,
                actions,
            ));
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
                name: component.name,
                declarations: component.declarations,
                max_bounds,
            },
        }))
    }

    pub fn compile(
        component: Component,
        dim: ClockIndex,
    ) -> Result<Box<Self>, SystemRecipeFailure> {
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

    fn _comp_info(&self) -> &ComponentInfo {
        &self.comp_info
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

    fn get_dim(&self) -> ClockIndex {
        self.dim
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

    fn get_decls(&self) -> Vec<&Declarations> {
        vec![&self.comp_info.declarations]
    }

    fn is_deterministic(&self) -> DeterminismResult {
        local_consistency::is_deterministic(self)
    }

    fn is_locally_consistent(&self) -> ConsistencyResult {
        local_consistency::is_least_consistent(self)
    }

    fn get_initial_state(&self) -> Option<State> {
        let init_loc = self.get_initial_location().unwrap();

        State::from_location(init_loc, self.dim)
    }

    fn get_children(&self) -> (&TransitionSystemPtr, &TransitionSystemPtr) {
        unimplemented!()
    }

    fn get_composition_type(&self) -> CompositionType {
        CompositionType::Simple
    }

    fn get_combined_decls(&self) -> Declarations {
        self.comp_info.declarations.clone()
    }

    fn get_location(&self, id: &LocationID) -> Option<LocationTuple> {
        self.locations.get(id).cloned()
    }

    fn component_names(&self) -> Vec<&str> {
        vec![&self.comp_info.name]
    }
}
