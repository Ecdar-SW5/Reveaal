use crate::DataReader::parse_edge;

use crate::DataReader::serialization::{
    decode_declarations, decode_guard, decode_invariant, decode_location_type, decode_sync,
    decode_sync_type, decode_update, DummyComponent, DummyEdge, DummyLocation,
};

use crate::EdgeEval::constraint_applyer::apply_constraints_to_state;
use crate::EdgeEval::updater::CompiledUpdate;
use edbm::util::bounds::Bounds;
use edbm::util::constraints::ClockIndex;

use crate::ModelObjects::representations::BoolExpression;
use crate::TransitionSystems::{CompositionType, TransitionSystem};
use crate::TransitionSystems::{LocationTuple, TransitionID};
use edbm::zones::OwnedFederation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// The basic struct used to represent components read from either Json or xml
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(into = "DummyComponent")]
pub struct Component {
    pub name: String,

    #[serde(
        deserialize_with = "decode_declarations",
        serialize_with = "encode_declarations"
    )]
    pub declarations: Declarations,
    pub locations: Vec<Location>,
    pub edges: Vec<Edge>,
    #[serde(skip)]
    pub input_edges: Option<Vec<Edge>>,
    #[serde(skip)]
    pub output_edges: Option<Vec<Edge>>,
}

impl DeclarationProvider for Component {
    fn get_declarations(&self) -> &Declarations {
        &self.declarations
    }
}

#[allow(dead_code)]
impl Component {
    pub fn set_clock_indices(&mut self, indices: &mut ClockIndex) {
        self.declarations.set_clock_indices(*indices);
        *indices += self.declarations.get_clock_count();
    }

    ///Start of basic methods for manipulating fields
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_locations(&self) -> &Vec<Location> {
        &self.locations
    }
    pub fn get_mut_locations(&mut self) -> &mut Vec<Location> {
        &mut self.locations
    }

    pub fn get_location_by_name(&self, name: &str) -> &Location {
        let loc_vec = self
            .locations
            .iter()
            .filter(|l| l.id == name)
            .collect::<Vec<&Location>>();

        if loc_vec.len() == 1 {
            loc_vec[0]
        } else {
            panic!("Unable to retrieve location based on id: {}", name)
        }
    }
    pub fn get_edges(&self) -> &Vec<Edge> {
        &self.edges
    }
    pub fn get_mut_edges(&mut self) -> &mut Vec<Edge> {
        &mut self.edges
    }
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
    pub fn add_edges(&mut self, edges: &mut Vec<Edge>) {
        self.edges.append(edges);
    }
    pub fn add_input_edges(&mut self, edges: &mut Vec<Edge>) {
        self.add_edges(edges);
        if let Some(input_edges) = &mut self.input_edges {
            input_edges.append(edges);
        } else {
            self.input_edges = Some(edges.to_vec());
        }
    }
    pub fn get_mut_declaration(&mut self) -> &mut Declarations {
        &mut self.declarations
    }

    pub fn get_input_edges(&self) -> &Vec<Edge> {
        if let Some(input_edges) = &self.input_edges {
            input_edges
        } else {
            panic!("attempted to get input edges before they were created")
        }
    }
    pub fn get_output_edges(&self) -> &Vec<Edge> {
        if let Some(output_edges) = &self.output_edges {
            output_edges
        } else {
            panic!("attempted to get output edges before they were created")
        }
    }

    pub fn get_initial_location(&self) -> Option<&Location> {
        let vec: Vec<&Location> = self
            .get_locations()
            .iter()
            .filter(|location| location.get_location_type() == &LocationType::Initial)
            .collect();

        vec.first().copied()
    }

    pub fn get_actions(&self) -> Vec<Channel> {
        let mut actions: Vec<Channel> = vec![];
        for edge in self.get_edges() {
            actions.push(Channel {
                name: edge.get_sync().clone(),
            });
        }

        actions
    }

    pub fn get_input_actions(&self) -> Vec<Channel> {
        let mut actions = vec![];
        for edge in self.input_edges.as_ref().unwrap() {
            if *edge.get_sync_type() == SyncType::Input && !contain(&actions, edge.get_sync()) {
                if edge.get_sync() == "*" {
                    continue;
                };
                actions.push(Channel {
                    name: edge.get_sync().clone(),
                });
            }
        }
        actions
    }

    pub fn get_output_actions(&self) -> Vec<Channel> {
        let mut actions = vec![];
        for edge in self.output_edges.as_ref().unwrap() {
            if *edge.get_sync_type() == SyncType::Output && !contain(&actions, edge.get_sync()) {
                if edge.get_sync() == "*" {
                    continue;
                };
                actions.push(Channel {
                    name: edge.get_sync().clone(),
                });
            }
        }
        actions
    }

    /// End of basic methods

    /// Method used to get the next edges based on a current location and a specific sync type (i.e input or output)
    pub fn get_next_edges(
        &self,
        location: &Location,
        channel_name: &str,
        sync_type: SyncType,
    ) -> Vec<&Edge> {
        return match sync_type {
            SyncType::Input => {
                let result: Vec<&Edge> = self
                    .get_input_edges()
                    .iter()
                    .filter(|e| {
                        (e.get_source_location() == location.get_id())
                            && (e.get_sync() == (channel_name.to_string()).as_str()
                                || e.get_sync() == "*")
                    })
                    .collect();
                result
            }
            SyncType::Output => {
                let result: Vec<&Edge> = self
                    .get_output_edges()
                    .iter()
                    .filter(|e| {
                        (e.get_source_location() == location.get_id())
                            && (e.get_sync() == (channel_name.to_string()).as_str()
                                || e.get_sync() == "*")
                    })
                    .collect();
                result
            }
        };
    }

    pub fn get_all_edges_from(&self, location: &Location) -> Vec<&Edge> {
        let result: Vec<&Edge> = self
            .get_output_edges()
            .iter()
            .filter(|e| e.get_source_location() == location.get_id())
            .collect();
        result
    }

    pub fn get_max_bounds(&self, dimensions: ClockIndex) -> Bounds {
        let mut max_bounds = Bounds::new(dimensions);
        for (clock_name, clock_id) in &self.declarations.clocks {
            let mut max_bound = 0;
            for edge in &self.edges {
                if let Some(guard) = edge.get_guard() {
                    let new_bound = guard.get_max_constant(*clock_id, clock_name);
                    if max_bound < new_bound {
                        max_bound = new_bound;
                    }
                }
            }

            for location in &self.locations {
                if let Some(inv) = location.get_invariant() {
                    let new_bound = inv.get_max_constant(*clock_id, clock_name);
                    if max_bound < new_bound {
                        max_bound = new_bound;
                    }
                }
            }

            // TODO: find more precise upper and lower bounds for clocks
            max_bounds.add_lower(*clock_id, max_bound);
            max_bounds.add_upper(*clock_id, max_bound);
        }

        max_bounds
    }

    /// Used in initial setup to split edges based on their sync type
    pub fn create_edge_io_split(&mut self) {
        let mut o_edges = vec![];
        let mut i_edges = vec![];

        for edge in &self.edges {
            match edge.sync_type {
                SyncType::Input => i_edges.push(edge.clone()),
                SyncType::Output => o_edges.push(edge.clone()),
            }
        }

        self.output_edges = Some(o_edges);
        self.input_edges = Some(i_edges);
    }

    /// Redoes the components Edge IDs by giving them new unique IDs based on their index.
    pub fn remake_edge_ids(&mut self) {
        // Give all edges a name
        for (index, edge) in self.get_mut_edges().iter_mut().enumerate() {
            edge.id = format!("E{}", index);
        }

        // Remake the input and output edges
        self.create_edge_io_split();
    }
}
pub fn contain(channels: &[Channel], channel: &str) -> bool {
    for c in channels {
        if c.name == channel {
            return true;
        }
    }

    false
}

/// FullState is a struct used for initial verification of consistency, and determinism as a state that also hols a dbm
/// This is done as the type used in refinement state pair assumes to sides of an operation
/// this should probably be refactored as it causes unnecessary confusion
#[derive(Clone, Debug)]
pub struct State {
    pub decorated_locations: LocationTuple,
    zone_sentinel: Option<OwnedFederation>,
}

impl State {
    pub fn create(decorated_locations: LocationTuple, zone: OwnedFederation) -> Self {
        State {
            decorated_locations,
            zone_sentinel: Some(zone),
        }
    }

    pub fn is_contained_in_list(&self, list: &[State]) -> bool {
        list.iter().any(|s| self.is_subset_of(s))
    }

    pub fn from_location(
        decorated_locations: LocationTuple,
        dimensions: ClockIndex,
    ) -> Option<Self> {
        let mut fed = OwnedFederation::init(dimensions);

        fed = decorated_locations.apply_invariants(fed);
        if fed.is_empty() {
            return None;
        }

        Some(State {
            decorated_locations,
            zone_sentinel: Some(fed),
        })
    }

    pub fn zone_ref(&self) -> &OwnedFederation {
        self.zone_sentinel.as_ref().unwrap()
    }

    pub fn take_zone(&mut self) -> OwnedFederation {
        self.zone_sentinel.take().unwrap()
    }

    pub fn set_zone(&mut self, zone: OwnedFederation) {
        self.zone_sentinel = Some(zone);
    }

    pub fn is_subset_of(&self, other: &Self) -> bool {
        if self.decorated_locations != other.decorated_locations {
            return false;
        }

        self.zone_ref().subset_eq(other.zone_ref())
    }

    pub fn get_location(&self) -> &LocationTuple {
        &self.decorated_locations
    }

    pub fn extrapolate_max_bounds(&mut self, system: &dyn TransitionSystem) {
        let bounds = system.get_local_max_bounds(&self.decorated_locations);
        let zone = self.take_zone().extrapolate_max_bounds(&bounds);
        self.set_zone(zone);
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum LocationType {
    Normal,
    Initial,
    Universal,
    Inconsistent,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(into = "DummyLocation")]
pub struct Location {
    pub id: String,
    #[serde(
        deserialize_with = "decode_invariant",
        serialize_with = "encode_opt_boolexpr"
    )]
    pub invariant: Option<BoolExpression>,
    #[serde(
        deserialize_with = "decode_location_type",
        serialize_with = "encode_location_type",
        rename = "type"
    )]
    pub location_type: LocationType,
    pub urgency: String,
}

#[allow(dead_code)]
impl Location {
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_invariant(&self) -> &Option<BoolExpression> {
        &self.invariant
    }
    pub fn get_location_type(&self) -> &LocationType {
        &self.location_type
    }
    pub fn get_urgency(&self) -> &String {
        &self.urgency
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SyncType {
    Input,
    Output,
}

/// Represents a single transition from taking edges in multiple components
#[derive(Debug, Clone)]
pub struct Transition {
    /// The ID of the transition, based on the edges it is created from.
    pub id: TransitionID,
    pub guard_zone: OwnedFederation,
    pub target_locations: LocationTuple,
    pub updates: Vec<CompiledUpdate>,
}

impl Transition {
    /// Create a new transition not based on an edge with no identifier
    pub fn new(target_locations: &LocationTuple, dim: ClockIndex) -> Transition {
        Transition {
            id: TransitionID::None,
            guard_zone: OwnedFederation::universe(dim),
            target_locations: target_locations.clone(),
            updates: vec![],
        }
    }

    pub fn from(comp: &Component, edge: &Edge, dim: ClockIndex) -> Transition {
        //let (comp, edge) = edges;

        let target_loc_name = &edge.target_location;
        let target_loc = comp.get_location_by_name(target_loc_name);
        let target_locations = LocationTuple::simple(
            target_loc,
            Some(comp.get_name().to_owned()),
            comp.get_declarations(),
            dim,
        );

        let mut compiled_updates = vec![];
        if let Some(updates) = edge.get_update() {
            compiled_updates.extend(
                updates
                    .iter()
                    .map(|update| CompiledUpdate::compile(update, comp.get_declarations())),
            );
        }

        Transition {
            id: TransitionID::Simple(edge.id.clone()),
            guard_zone: Transition::combine_edge_guards(&vec![(comp, edge)], dim),
            target_locations,
            updates: compiled_updates,
        }
    }

    pub fn use_transition(&self, state: &mut State) -> bool {
        let mut zone = state.take_zone();
        zone = self.apply_guards(zone);
        if !zone.is_empty() {
            zone = self.apply_updates(zone).up();
            self.move_locations(&mut state.decorated_locations);
            zone = state.decorated_locations.apply_invariants(zone);
            if !zone.is_empty() {
                state.set_zone(zone);
                return true;
            }
        }
        state.set_zone(zone);
        false
    }

    pub fn combinations(
        left: &Vec<Transition>,
        right: &Vec<Transition>,
        comp: CompositionType,
    ) -> Vec<Transition> {
        let mut out: Vec<Transition> = vec![];
        for l in left {
            for r in right {
                let target_locations =
                    LocationTuple::compose(&l.target_locations, &r.target_locations, comp);

                let guard_zone = l.guard_zone.clone().intersection(&r.guard_zone);

                let mut updates = l.updates.clone();
                updates.append(&mut r.updates.clone());

                out.push(Transition {
                    id: match comp {
                        CompositionType::Conjunction => TransitionID::Conjunction(
                            Box::new(l.id.clone()),
                            Box::new(r.id.clone()),
                        ),
                        CompositionType::Composition => TransitionID::Composition(
                            Box::new(l.id.clone()),
                            Box::new(r.id.clone()),
                        ),
                        _ => unreachable!("Invalid composition type {:?}", comp),
                    },
                    guard_zone,
                    target_locations,
                    updates,
                });
            }
        }

        out
    }

    pub fn apply_updates(&self, mut fed: OwnedFederation) -> OwnedFederation {
        for update in &self.updates {
            fed = update.apply(fed);
        }

        fed
    }

    pub fn inverse_apply_updates(&self, mut fed: OwnedFederation) -> OwnedFederation {
        for update in &self.updates {
            fed = update.apply_as_guard(fed);
        }
        for update in &self.updates {
            fed = update.apply_as_free(fed);
        }

        fed
    }

    // TODO: will we ever need this method?
    #[allow(dead_code)]
    fn get_guard_from_allowed(
        from_loc: &LocationTuple,
        to_loc: &LocationTuple,
        updates: Vec<CompiledUpdate>,
        guard: Option<OwnedFederation>,
        dim: ClockIndex,
    ) -> OwnedFederation {
        let mut fed = match to_loc.get_invariants() {
            Some(fed) => fed.clone(),
            None => OwnedFederation::universe(dim),
        };
        for update in &updates {
            fed = update.apply_as_guard(fed);
        }
        for update in &updates {
            fed = update.apply_as_free(fed);
        }
        if let Some(g) = guard {
            fed = fed.intersection(&g);
        }
        from_loc.apply_invariants(fed)
    }

    pub fn get_allowed_federation(&self) -> OwnedFederation {
        let mut fed = match self.target_locations.get_invariants() {
            Some(fed) => fed.clone(),
            None => OwnedFederation::universe(self.guard_zone.dim()),
        };
        fed = self.inverse_apply_updates(fed);
        self.apply_guards(fed)
    }

    pub fn apply_guards(&self, zone: OwnedFederation) -> OwnedFederation {
        zone.intersection(&self.guard_zone)
    }

    pub fn move_locations(&self, locations: &mut LocationTuple) {
        *locations = self.target_locations.clone();
    }

    pub fn combine_edge_guards(
        edges: &Vec<(&Component, &Edge)>,
        dim: ClockIndex,
    ) -> OwnedFederation {
        let mut fed = OwnedFederation::universe(dim);
        for (comp, edge) in edges {
            fed = edge.apply_guard(comp.get_declarations(), fed);
        }
        fed
    }

    pub fn get_renamed_guard_expression(
        &self,
        naming: &HashMap<String, ClockIndex>,
    ) -> Option<BoolExpression> {
        BoolExpression::from_disjunction(&self.guard_zone.minimal_constraints(), naming)
    }

    pub fn get_renamed_updates(
        &self,
        naming: &HashMap<String, ClockIndex>,
    ) -> Option<Vec<parse_edge::Update>> {
        let updates: Vec<_> = self.updates.iter().map(|u| u.as_update(naming)).collect();

        if updates.is_empty() {
            None
        } else {
            Some(updates)
        }
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "Transition{{{} to {} where {} [{}]}}",
            self.guard_zone,
            self.target_locations.id,
            self.target_locations
                .get_invariants()
                .map(|f| format!("invariant is {}", f))
                .unwrap_or_else(|| "no invariant".to_string()),
            self.updates
                .iter()
                .map(|u| u.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ))?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(into = "DummyEdge")]
pub struct Edge {
    /// Uniquely identifies the edge within its component
    pub id: String,
    #[serde(rename = "sourceLocation")]
    pub source_location: String,
    #[serde(rename = "targetLocation")]
    pub target_location: String,
    #[serde(
        deserialize_with = "decode_sync_type",
        serialize_with = "encode_sync_type",
        rename = "status"
    )]
    pub sync_type: SyncType,
    #[serde(
        deserialize_with = "decode_guard",
        serialize_with = "encode_opt_boolexpr"
    )]
    pub guard: Option<BoolExpression>,
    #[serde(
        deserialize_with = "decode_update",
        serialize_with = "encode_opt_updates"
    )]
    pub update: Option<Vec<parse_edge::Update>>,
    #[serde(deserialize_with = "decode_sync")]
    pub sync: String,
}

const TRUE: BoolExpression = BoolExpression::Bool(true);
impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "Edge {{{}-({}{})->{}, Guard: {}, Update: {:?}}}",
            self.source_location,
            self.sync,
            match self.sync_type {
                SyncType::Input => "?",
                SyncType::Output => "!",
            },
            self.target_location,
            self.guard.as_ref().unwrap_or(&TRUE),
            self.update
        ))?;
        Ok(())
    }
}

impl Edge {
    pub fn apply_update(
        &self,
        decl: &Declarations, //Will eventually be mutable
        mut fed: OwnedFederation,
    ) -> OwnedFederation {
        if let Some(updates) = self.get_update() {
            for update in updates {
                fed = update.compiled(decl).apply(fed);
            }
        }

        fed
    }

    pub fn apply_guard(&self, decl: &Declarations, mut fed: OwnedFederation) -> OwnedFederation {
        if let Some(guards) = self.get_guard() {
            fed = apply_constraints_to_state(guards, decl, fed).unwrap();
        };

        fed
    }

    pub fn get_source_location(&self) -> &String {
        &self.source_location
    }

    pub fn get_target_location(&self) -> &String {
        &self.target_location
    }

    pub fn get_sync_type(&self) -> &SyncType {
        &self.sync_type
    }

    pub fn get_guard(&self) -> &Option<BoolExpression> {
        &self.guard
    }

    pub fn get_update(&self) -> &Option<Vec<parse_edge::Update>> {
        &self.update
    }

    pub fn get_sync(&self) -> &String {
        &self.sync
    }

    pub fn get_update_clocks(&self) -> Vec<&str> {
        let mut clock_vec = vec![];
        if let Some(updates) = self.get_update() {
            for u in updates {
                clock_vec.push(u.get_variable_name())
            }
        }

        clock_vec
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Channel {
    pub name: String,
}

impl Channel {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Clone)]
pub struct DecoratedLocation<'a> {
    pub location: &'a Location,
    pub decls: &'a Declarations,
}

impl PartialEq for DecoratedLocation<'_> {
    fn eq(&self, other: &DecoratedLocation) -> bool {
        self.location == other.location
    }
}

#[allow(dead_code)]
impl<'a> DecoratedLocation<'a> {
    pub fn create(location: &'a Location, decls: &'a Declarations) -> DecoratedLocation<'a> {
        DecoratedLocation { location, decls }
    }

    pub fn apply_invariant(&self, mut fed: OwnedFederation) -> OwnedFederation {
        if let Some(inv) = self.get_location().get_invariant() {
            fed = apply_constraints_to_state(inv, self.decls, fed).unwrap();
        }

        fed
    }

    pub fn get_invariant(&self) -> &Option<BoolExpression> {
        self.get_location().get_invariant()
    }

    pub fn get_declarations(&self) -> &Declarations {
        self.decls
    }

    pub fn get_location(&self) -> &Location {
        self.location
    }

    pub fn set_location(&mut self, location: &'a Location) {
        self.location = location;
    }

    pub fn get_clock_count(&self) -> ClockIndex {
        self.get_declarations().get_clock_count()
    }
}

pub trait DeclarationProvider {
    fn get_declarations(&self) -> &Declarations;
}

/// The declaration struct is used to hold the indices for each clock, and is meant to be the owner of int variables once implemented
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Serialize)]
pub struct Declarations {
    pub ints: HashMap<String, i32>,
    pub clocks: HashMap<String, ClockIndex>,
}

#[allow(dead_code)]
impl Declarations {
    pub fn empty() -> Declarations {
        Declarations {
            ints: HashMap::new(),
            clocks: HashMap::new(),
        }
    }

    pub fn get_ints(&self) -> &HashMap<String, i32> {
        &self.ints
    }

    pub fn get_mut_ints(&mut self) -> &mut HashMap<String, i32> {
        &mut self.ints
    }

    pub fn get_clocks(&self) -> &HashMap<String, ClockIndex> {
        &self.clocks
    }

    pub fn get_clock_count(&self) -> usize {
        self.clocks.len()
    }

    pub fn get_max_clock_index(&self) -> ClockIndex {
        *self.clocks.values().max().unwrap_or(&0)
    }

    pub fn set_clock_indices(&mut self, start_index: ClockIndex) {
        for (_, v) in self.clocks.iter_mut() {
            *v += start_index
        }
    }

    pub fn update_clock_indices(&mut self, start_index: ClockIndex, old_offset: ClockIndex) {
        for (_, v) in self.clocks.iter_mut() {
            *v -= old_offset;
            *v += start_index;
        }
    }

    pub fn reset_clock_indices(&mut self) {
        let mut i = 1;
        for (_, v) in self.clocks.iter_mut() {
            *v = i;
            i += 1;
        }
    }

    pub fn get_clock_index_by_name(&self, name: &str) -> Option<&ClockIndex> {
        self.get_clocks().get(name)
    }
}
