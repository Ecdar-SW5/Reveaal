use super::{CompositionType, LocationID, LocationTuple};
use crate::DataReader::parse_queries::Rule;
use crate::EdgeEval::updater::CompiledUpdate;
use crate::System::local_consistency::DeterminismFailure;
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
use log::warn;
use pest::Parser;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::{
    collections::{hash_set::HashSet, HashMap},
    iter::zip,
};

pub type TransitionSystemPtr = Box<dyn TransitionSystem>;
pub type Action = String;
pub type EdgeTuple = (Action, Transition);
pub type EdgeIndex = (LocationID, usize);

/// Precheck can fail because of either consistency or determinism.
pub enum PrecheckResult {
    Success,
    NotDeterministic(LocationID, String),
    NotConsistent(ConsistencyFailure),
}

#[derive(Clone, Copy)]
/// Struct for determining the level for clock reduction
pub struct Heights {
    /// The level in the tree
    pub(crate) tree: usize,
    /// The level to reduce
    pub(crate) target: usize,
}

impl Heights {
    pub fn new(tree: usize, target: usize) -> Heights {
        Heights { tree, target }
    }

    /// Function to "go down" a level in the tree
    pub fn level_down(&self) -> Heights {
        Heights {
            tree: self.tree - 1,
            ..*self
        }
    }

    /// Creates an empty `Heights` (ALl values are `0`)
    pub fn empty() -> Heights {
        Heights::new(0, 0)
    }
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

    fn get_location(&self, id: &LocationID) -> Option<LocationTuple> {
        self.get_all_locations()
            .iter()
            .find(|loc| loc.id == *id)
            .cloned()
    }

    fn get_decls(&self) -> Vec<&Declarations>;

    fn precheck_sys_rep(&self) -> PrecheckResult {
        if let DeterminismResult::Failure(DeterminismFailure::NotDeterministicFrom(
            location,
            action,
        )) = self.is_deterministic()
        {
            warn!("Not deterministic");
            return PrecheckResult::NotDeterministic(location, action);
        }

        if let ConsistencyResult::Failure(failure) = self.is_locally_consistent() {
            warn!("Not consistent");
            return PrecheckResult::NotConsistent(failure);
        }
        PrecheckResult::Success
    }
    fn get_combined_decls(&self) -> Declarations {
        let mut clocks = HashMap::new();
        let mut ints = HashMap::new();

        for decl in self.get_decls() {
            clocks.extend(decl.clocks.clone());
            ints.extend(decl.ints.clone())
        }

        Declarations { ints, clocks }
    }

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

    ///Constructs a [CLockAnalysisGraph],
    ///where nodes represents locations and Edges represent transitions
    fn get_analysis_graph(&self) -> ClockAnalysisGraph {
        let mut graph: ClockAnalysisGraph = ClockAnalysisGraph::empty();
        graph.dim = self.get_dim();
        let location = self.get_initial_location().unwrap();
        let actions = self.get_actions();

        self.find_edges_and_nodes(&location, &actions, &mut graph);

        graph
    }

    ///Helper function to recursively traverse all transitions in a transitions system
    ///in order to find all transitions and location in the transition system, and
    ///saves these as [ClockAnalysisEdge]s and [ClockAnalysisNode]s in the [ClockAnalysisGraph]
    fn find_edges_and_nodes(
        &self,
        location: &LocationTuple,
        actions: &HashSet<String>,
        graph: &mut ClockAnalysisGraph,
    ) {
        //Constructs a node to represent this location and add it to the graph.
        let mut node: ClockAnalysisNode = ClockAnalysisNode {
            invariant_dependencies: HashSet::new(),
            id: location.id.to_owned(),
        };

        //Finds clocks used in invariants in this location.
        if let Some(invariant) = &location.invariant {
            let conjunctions = invariant.minimal_constraints().conjunctions;
            for conjunction in conjunctions {
                for constraint in conjunction.iter() {
                    node.invariant_dependencies.insert(constraint.i);
                    node.invariant_dependencies.insert(constraint.j);
                }
            }
        }
        graph.nodes.insert(node.id.clone(), node);

        //Constructs an edge to represent each transition from this graph and add it to the graph.
        for action in actions.iter() {
            let transitions = self.next_transitions_if_available(location, action);
            for transition in transitions {
                let mut edge = ClockAnalysisEdge {
                    from: location.id.to_owned(),
                    to: transition.target_locations.id.clone(),
                    guard_dependencies: HashSet::new(),
                    updates: transition.updates,
                    edge_type: action.to_string(),
                };

                //Finds clocks used in guards in this transition.
                let conjunctions = transition.guard_zone.minimal_constraints().conjunctions;
                for conjunction in &conjunctions {
                    for constraint in conjunction.iter() {
                        edge.guard_dependencies.insert(constraint.i);
                        edge.guard_dependencies.insert(constraint.j);
                    }
                }

                graph.edges.push(edge);

                //Calls itself on the transitions target location if the location is not already in
                //represented as a node in the graph.
                if !graph.nodes.contains_key(&transition.target_locations.id) {
                    self.find_edges_and_nodes(&transition.target_locations, actions, graph);
                }
            }
        }
    }

    fn find_redundant_clocks(&self, height: Heights) -> Vec<ClockReductionInstruction> {
        if height.tree > height.target {
            let (a, b) = self.get_children();
            let mut out = a.find_redundant_clocks(height.clone().level_down());
            out.extend(b.find_redundant_clocks(height.level_down()));
            out
        } else {
            self.get_analysis_graph().find_clock_redundancies()
        }
    }
}

/// Returns a [`TransitionSystemPtr`] equivalent to a `composition` of some `components`.
pub fn components_to_transition_system(
    components: Vec<Component>,
    composition: &str,
) -> TransitionSystemPtr {
    let mut component_container = ComponentContainer::from_components(components);
    component_loader_to_transition_system(&mut component_container, composition)
}

/// Returns a [`TransitionSystemPtr`] equivalent to a `composition` of some components in a [`ComponentLoader`].
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

pub(crate) trait Intersect<T> {
    fn intersect(&self, other: &[T]) -> Vec<T>;
}

impl Intersect<ClockReductionInstruction> for Vec<ClockReductionInstruction> {
    // Prioritizes replacements over removals
    fn intersect(&self, other: &[ClockReductionInstruction]) -> Vec<ClockReductionInstruction> {
        self.iter()
            .filter(|r| r.is_replace())
            .chain(other.iter().filter(|r| r.is_replace()))
            .chain(
                self.iter()
                    .filter(|r| !r.is_replace())
                    .filter_map(|c| other.iter().filter(|r| !r.is_replace()).find(|rc| *rc == c)),
            )
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClockReductionInstruction {
    RemoveClock {
        clock_index: ClockIndex,
    },
    ReplaceClocks {
        clock_index: ClockIndex,
        clock_indices: HashSet<ClockIndex>,
    },
}

impl ClockReductionInstruction {
    pub(crate) fn clocks_removed_count(&self) -> usize {
        match self {
            ClockReductionInstruction::RemoveClock { .. } => 1,
            ClockReductionInstruction::ReplaceClocks { clock_indices, .. } => clock_indices.len(),
        }
    }

    fn is_replace(&self) -> bool {
        match self {
            ClockReductionInstruction::RemoveClock { .. } => false,
            ClockReductionInstruction::ReplaceClocks { .. } => true,
        }
    }
}

#[derive(Debug)]
pub struct ClockAnalysisNode {
    pub invariant_dependencies: HashSet<ClockIndex>,
    pub id: LocationID,
}

#[derive(Debug)]
pub struct ClockAnalysisEdge {
    pub from: LocationID,
    pub to: LocationID,
    pub guard_dependencies: HashSet<ClockIndex>,
    pub updates: Vec<CompiledUpdate>,
    pub edge_type: String,
}

#[derive(Debug)]
pub struct ClockAnalysisGraph {
    pub nodes: HashMap<LocationID, ClockAnalysisNode>,
    pub edges: Vec<ClockAnalysisEdge>,
    pub dim: ClockIndex,
}

impl ClockAnalysisGraph {
    pub fn empty() -> ClockAnalysisGraph {
        ClockAnalysisGraph {
            nodes: HashMap::new(),
            edges: vec![],
            dim: 0,
        }
    }

    pub fn find_clock_redundancies(&self) -> Vec<ClockReductionInstruction> {
        //First we find the used clocks
        let used_clocks = self.find_used_clocks();

        //Then we instruct the caller to remove the unused clocks, we start at 1 since the 0 clock is not a real clock
        let mut unused_clocks = (1..self.dim).collect::<HashSet<ClockIndex>>();
        for used_clock in &used_clocks {
            unused_clocks.remove(used_clock);
        }

        let mut rv: Vec<ClockReductionInstruction> = Vec::new();
        for unused_clock in &unused_clocks {
            rv.push(ClockReductionInstruction::RemoveClock {
                clock_index: *unused_clock,
            });
        }

        let mut equivalent_clock_groups = self.find_equivalent_clock_groups(&used_clocks);

        for equivalent_clock_group in &mut equivalent_clock_groups {
            let lowest_clock = *equivalent_clock_group.iter().min().unwrap();
            equivalent_clock_group.remove(&lowest_clock);
            rv.push(ClockReductionInstruction::ReplaceClocks {
                clock_index: lowest_clock,
                clock_indices: equivalent_clock_group.clone(),
            });
        }

        rv
    }

    fn find_used_clocks(&self) -> HashSet<ClockIndex> {
        let mut used_clocks = HashSet::new();

        //First we find the used clocks
        for edge in &self.edges {
            for guard_dependency in &edge.guard_dependencies {
                used_clocks.insert(*guard_dependency);
            }
        }

        for node in &self.nodes {
            for invariant_dependency in &node.1.invariant_dependencies {
                used_clocks.insert(*invariant_dependency);
            }
        }

        //Clock index 0 is not a real clock therefore it is removed
        used_clocks.remove(&0);

        used_clocks
    }

    fn find_equivalent_clock_groups(
        &self,
        used_clocks: &HashSet<ClockIndex>,
    ) -> Vec<HashSet<ClockIndex>> {
        if used_clocks.len() < 2 || self.edges.is_empty() {
            return Vec::new();
        }
        let mut equivalent_clock_groups: Vec<HashSet<ClockIndex>> = Vec::new();

        //This function works by maintaining the loop invariant that equivalent_clock_groups contains
        //groups containing clocks where all clocks contained are equivalent in all edges we have iterated
        //through. We also have to make sure that each clock are only present in one group at a time.
        //This means that for the first iteration all clocks are equivalent. We do not include
        //unused clocks since they are all equivalent and will removed completely in another stage.
        equivalent_clock_groups.push(used_clocks.clone());

        for edge in &self.edges {
            //First the clocks which are equivalent in this edge are found. This is defined by every
            //clock in their respective group are set to the same value. This is done through a
            //HashMap with the value being key and the group of clocks being the value
            let mut locally_equivalent_clock_groups: HashMap<i32, HashSet<ClockIndex>> =
                HashMap::new();
            //Here the clocks are grouped by the value they are set to
            for update in edge.updates.iter() {
                //This gets the values' clock group or creates a new one and inserts the new one
                //in the hashset and returns a mutable reference
                let clock_group: &mut HashSet<ClockIndex> =
                    match locally_equivalent_clock_groups.entry(update.value) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(HashSet::new()),
                    };
                clock_group.insert(update.clock_index);
            }
            //Then we maintain the loop invariant by creating a new list of clock groups and
            //dividing the old groups when clocks are found to not be equivalent
            let mut new_groups: Vec<HashSet<ClockIndex>> = Vec::new();
            //We do this by iterating on each globally equivalent clock group and removing the clocks
            //that are not updated to the same value
            for equivalent_clock_group in &mut equivalent_clock_groups {
                //For each of the locally equivalent clock groups we can construct a new clock group
                //for the clocks that are in the globally equivalant clock group we are iterating
                //over now.
                //Then we remove the clocks from the globally equivalent clocks that we use in
                //the new group
                for locally_equivalent_clock_group in &locally_equivalent_clock_groups {
                    let mut new_clock_group = HashSet::new();
                    for locally_equivalent_clock in locally_equivalent_clock_group.1 {
                        if equivalent_clock_group.contains(locally_equivalent_clock) {
                            new_clock_group.insert(*locally_equivalent_clock);
                            equivalent_clock_group.remove(locally_equivalent_clock);
                        }
                    }
                    //If the new clock group only contains one clock then there is no reason to keep
                    //Track of it, since it will never be redundant
                    if new_clock_group.len() > 1 {
                        new_groups.push(new_clock_group);
                    }
                }
                //The same thing is done here
                if equivalent_clock_group.len() > 1 {
                    new_groups.push(equivalent_clock_group.clone());
                }
            }
            //Then we use the new groups which uphold the loop invariant
            equivalent_clock_groups = new_groups;
        }
        equivalent_clock_groups
    }
}

clone_trait_object!(TransitionSystem);
