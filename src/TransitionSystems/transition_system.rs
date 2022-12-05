use super::{CompositionType, LocationID, LocationTuple};
use crate::EdgeEval::updater::CompiledUpdate;
use crate::{
    ModelObjects::component::{Declarations, State, Transition},
    System::local_consistency::DeterminismResult,
    System::local_consistency::{ConsistencyFailure, ConsistencyResult},
};
use dyn_clone::{clone_trait_object, DynClone};
use edbm::util::{bounds::Bounds, constraints::ClockIndex};
use log::warn;
use std::collections::hash_map::Entry;
use std::collections::{hash_set::HashSet, HashMap};
use std::hash::Hash;
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
            id: location.id.get_unique_string(),
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
                    from: location.id.get_unique_string(),
                    to: transition.target_locations.id.get_unique_string(),
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
                if !graph
                    .nodes
                    .contains_key(&transition.target_locations.id.get_unique_string())
                {
                    self.find_edges_and_nodes(&transition.target_locations, actions, graph);
                }
            }
        }
    }

    fn find_redundant_clocks(&self) -> Vec<ClockReductionInstruction> {
        self.get_analysis_graph().find_clock_redundancies()
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

    pub(crate) fn is_replace(&self) -> bool {
        match self {
            ClockReductionInstruction::RemoveClock { .. } => false,
            ClockReductionInstruction::ReplaceClocks { .. } => true,
        }
    }
}

#[derive(Debug)]
pub struct ClockAnalysisNode {
    pub invariant_dependencies: HashSet<ClockIndex>,
    pub id: String,
}

#[derive(Debug)]
pub struct ClockAnalysisEdge {
    pub from: String,
    pub to: String,
    pub guard_dependencies: HashSet<ClockIndex>,
    pub updates: Vec<CompiledUpdate>,
    pub edge_type: String,
}

#[derive(Debug)]
pub struct ClockAnalysisGraph {
    pub nodes: HashMap<String, ClockAnalysisNode>,
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

        //This function works by maintaining the loop invariant that equivalent_clock_groups contains
        //groups containing clocks where all clocks contained are equivalent in all edges we have iterated
        //through. We also have to make sure that each clock are only present in one group at a time.
        //This means that for the first iteration all clocks are equivalent. We do not include
        //unused clocks since they are all equivalent and will removed completely in another stage.
        let mut equivalent_clock_groups: Vec<HashSet<ClockIndex>> = vec![used_clocks.clone()];

        for edge in &self.edges {
            //First the clocks which are equivalent in this edge are found. This is defined by every
            //clock in their respective group are set to the same value. This is done in a HashMap
            //where each clock group has their own unique u32, the clock indices
            //with the same value are in the same group
            let mut locally_equivalent_clock_groups: HashMap<ClockIndex, u32> = HashMap::new();

            //Then we create the groups in the hashmap
            for update in edge.updates.iter() {
                locally_equivalent_clock_groups.insert(update.clock_index, update.value as u32);
            }

            //Then the locally equivalent clock groups will be combined with the globally equivalent
            //clock groups to identify the new globally equivalent clocks
            let mut new_groups: HashMap<usize, HashSet<ClockIndex>> = HashMap::new();
            let mut group_offset: usize = u32::MAX as usize;

            //For each of the existing clock groups we will remove the clocks from the groups
            //that are locally equivalent, this means that each global group will now be
            //updated to uphold the loop invariant.
            //This is done by giving each globally equivalent clock group a group offset
            //So all groups in the locally equivalent clock groups will be partitioned
            //by the group they are in, in their globally equivalent group
            for (old_group_index, equivalent_clock_group) in
                equivalent_clock_groups.iter_mut().enumerate()
            {
                for clock in equivalent_clock_group.iter() {
                    if let Some(groupId) = locally_equivalent_clock_groups.get(clock) {
                        ClockAnalysisGraph::get_or_insert(
                            &mut new_groups,
                            group_offset + ((*groupId) as usize),
                        )
                        .insert(*clock);
                    } else {
                        ClockAnalysisGraph::get_or_insert(&mut new_groups, old_group_index)
                            .insert(*clock);
                    }
                }
                group_offset += (u32::MAX as usize) * 2;
            }

            //Then we just have to take each of the values in the map and collect them into a vec
            equivalent_clock_groups = new_groups
                .into_iter()
                .map(|pair| pair.1)
                .filter(|group| group.len() > 1)
                .collect();
        }
        equivalent_clock_groups
    }

    fn get_or_insert<K: Eq + Hash, V: Default>(map: &'_ mut HashMap<K, V>, key: K) -> &'_ mut V {
        match map.entry(key) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(V::default()),
        }
    }
}

clone_trait_object!(TransitionSystem);
