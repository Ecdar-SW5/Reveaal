use edbm::util::constraints::ClockIndex;
use edbm::zones::OwnedFederation;
use log::{debug, warn};

use crate::EdgeEval::updater::CompiledUpdate;
use crate::ModelObjects::component::Declarations;
use crate::ModelObjects::component::{Location, LocationType, State, Transition};
use crate::System::local_consistency::{ConsistencyResult, DeterminismResult};
use crate::TransitionSystems::transition_system::PrecheckResult;
use edbm::util::bounds::Bounds;

use crate::ModelObjects::representations::{ArithExpression, BoolExpression};

use crate::TransitionSystems::{
    LocationTuple, TransitionID, TransitionSystem, TransitionSystemPtr,
};
use std::collections::hash_set::HashSet;
use std::collections::HashMap;

use super::CompositionType;

#[derive(Clone)]
pub struct Quotient {
    T: TransitionSystemPtr,
    S: TransitionSystemPtr,
    inputs: HashSet<String>,
    outputs: HashSet<String>,
    universal_location: Location,
    inconsistent_location: Location,
    decls: Declarations,
    quotient_clock_index: ClockIndex,
    new_input_name: String,

    dim: ClockIndex,
}

static INCONSISTENT_LOC_NAME: &str = "Inconsistent";
static UNIVERSAL_LOC_NAME: &str = "Universal";
impl Quotient {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        T: TransitionSystemPtr,
        S: TransitionSystemPtr,
        new_clock_index: ClockIndex,
        dim: ClockIndex,
    ) -> Result<TransitionSystemPtr, String> {
        if !S.get_output_actions().is_disjoint(&T.get_input_actions()) {
            return Err(format!(
                "s_out and t_in not disjoint in quotient! s_out: {:?} t_in {:?}",
                S.get_output_actions(),
                T.get_input_actions()
            ));
        }

        match T.precheck_sys_rep() {
            PrecheckResult::Success => {}
            _ => {
                return Err("T (left) must be least consistent for quotient".to_string());
            }
        }
        match S.precheck_sys_rep() {
            PrecheckResult::Success => {}
            _ => {
                return Err("T (left) must be least consistent for quotient".to_string());
            }
        }

        let universal_location = Location {
            id: UNIVERSAL_LOC_NAME.to_string(),
            invariant: None,
            location_type: LocationType::Universal,
            urgency: "".to_string(),
        };

        let inconsistent_location = Location {
            id: INCONSISTENT_LOC_NAME.to_string(),
            // xnew <= 0
            invariant: Some(BoolExpression::LessEQ(
                Box::new(ArithExpression::VarName("quotient_xnew".to_string())),
                Box::new(ArithExpression::Int(0)),
            )),
            location_type: LocationType::Inconsistent,
            urgency: "".to_string(),
        };

        let mut inputs: HashSet<String> = T
            .get_input_actions()
            .union(&S.get_output_actions())
            .cloned()
            .collect();
        let mut i = 0;
        let new_input_name = loop {
            let test = format!("quotient_new_input{}", i);
            if !inputs.contains(&test) {
                break test;
            }
            i += 1;
        };

        inputs.insert(new_input_name.clone());

        let output_dif: HashSet<String> = T
            .get_output_actions()
            .difference(&S.get_output_actions())
            .cloned()
            .collect();
        let input_dif: HashSet<String> = S
            .get_input_actions()
            .difference(&T.get_input_actions())
            .cloned()
            .collect();

        let outputs: HashSet<String> = output_dif.union(&input_dif).cloned().collect();

        let mut decls = Declarations::empty();
        decls
            .clocks
            .insert("quotient_xnew".to_string(), new_clock_index);

        debug!("S//T Inputs: {inputs:?}, Outputs: {outputs:?}");
        debug!(
            "S Inputs: {:?}, Outputs: {:?}",
            S.get_input_actions(),
            S.get_output_actions()
        );
        debug!(
            "T Inputs: {:?}, Outputs: {:?}",
            T.get_input_actions(),
            T.get_output_actions()
        );

        let ts = Box::new(Quotient {
            T,
            S,
            inputs,
            outputs,
            universal_location,
            inconsistent_location,
            decls,
            quotient_clock_index: new_clock_index,
            new_input_name,
            dim,
        });
        Ok(ts)
    }
}

impl TransitionSystem for Quotient {
    fn get_local_max_bounds(&self, loc: &LocationTuple) -> Bounds {
        if loc.is_universal() || loc.is_inconsistent() {
            let mut b = Bounds::new(self.get_dim());
            b.add_upper(self.quotient_clock_index, 0);
            b
        } else {
            let (left, right) = self.get_children();
            let loc_l = loc.get_left();
            let loc_r = loc.get_right();
            let mut bounds_l = left.get_local_max_bounds(loc_l);
            let bounds_r = right.get_local_max_bounds(loc_r);
            bounds_l.add_bounds(&bounds_r);
            bounds_l.add_upper(self.quotient_clock_index, 0);
            bounds_l
        }
    }

    fn next_transitions(&self, location: &LocationTuple, action: &str) -> Vec<Transition> {
        assert!(self.actions_contain(action));
        let is_input = self.inputs_contain(action);

        let mut transitions = vec![];

        //Rules [universal] and [inconsistent]

        if location.is_inconsistent() {
            //Rule 10
            if is_input {
                let mut transition = Transition::new(location, self.dim);
                transition.guard_zone = transition
                    .guard_zone
                    .constrain_eq(self.quotient_clock_index, 0);
                transitions.push(transition);
            }
            return transitions;
        } else if location.is_universal() {
            // Rule 9
            let transition = Transition::new(location, self.dim);
            transitions.push(transition);
            return transitions;
        }

        // As it is not universal or inconsistent it must be a quotient loc
        let loc_t = location.get_left();
        let loc_s = location.get_right();
        let t = self.T.next_transitions_if_available(loc_t, action);
        let s = self.S.next_transitions_if_available(loc_s, action);

        let inconsistent_location =
            LocationTuple::simple(&self.inconsistent_location, None, &self.decls, self.dim);
        let universal_location =
            LocationTuple::simple(&self.universal_location, None, &self.decls, self.dim);

        //Rule 1
        if self.S.actions_contain(action) && self.T.actions_contain(action) {
            for t_transition in &t {
                for s_transition in &s {
                    // In the following comments we use ϕ to symbolize the guard of the transition
                    // ϕ_T ∧ Inv(l2_t)[r |-> 0] ∧ Inv(l1_t) ∧ ϕ_S ∧ Inv(l2_s)[r |-> 0] ∧ Inv(l1_s)
                    let guard_zone = get_allowed_fed(loc_t, t_transition)
                        .intersection(&get_allowed_fed(loc_s, s_transition));

                    let target_locations = merge(
                        &t_transition.target_locations,
                        &s_transition.target_locations,
                    );

                    //Union of left and right updates
                    let mut updates = t_transition.updates.clone();
                    updates.append(&mut s_transition.updates.clone());

                    transitions.push(Transition {
                        id: TransitionID::Quotient(
                            vec![t_transition.id.clone()],
                            vec![s_transition.id.clone()],
                        ),
                        guard_zone,
                        target_locations,
                        updates,
                    });
                }
            }
        }

        //Rule 2
        if self.S.actions_contain(action) && !self.T.actions_contain(action) {
            //Independent S
            for s_transition in &s {
                let guard_zone = get_allowed_fed(loc_s, s_transition);

                let target_locations = merge(loc_t, &s_transition.target_locations);
                let updates = s_transition.updates.clone();
                transitions.push(Transition {
                    id: TransitionID::Quotient(Vec::new(), vec![s_transition.id.clone()]),
                    guard_zone,
                    target_locations,
                    updates,
                });
            }
        }

        if self.S.get_output_actions().contains(action) {
            // new Rule 3 (includes rule 4 by de-morgan)
            let mut g_s = OwnedFederation::empty(self.dim);

            for s_transition in &s {
                let allowed_fed = get_allowed_fed(loc_s, s_transition);
                g_s += allowed_fed;
            }

            // Rule 5 when Rule 3 applies
            let inv_l_s = loc_s.apply_invariants(OwnedFederation::universe(self.dim));

            transitions.push(Transition {
                id: TransitionID::Quotient(Vec::new(), s.iter().map(|t| t.id.clone()).collect()),
                guard_zone: (!inv_l_s) + (!g_s),
                target_locations: universal_location,
                updates: vec![],
            });
        } else {
            // Rule 5 when Rule 3 does not apply
            let inv_l_s = loc_s.apply_invariants(OwnedFederation::universe(self.dim));

            transitions.push(Transition {
                id: TransitionID::None,
                guard_zone: !inv_l_s,
                target_locations: universal_location,
                updates: vec![],
            });
        }

        //Rule 6
        if self.S.get_output_actions().contains(action)
            && self.T.get_output_actions().contains(action)
        {
            //Calculate inverse G_T
            let mut g_t = OwnedFederation::empty(self.dim);
            for t_transition in &t {
                g_t = g_t.union(&get_allowed_fed(loc_t, t_transition));
            }
            let inverse_g_t = g_t.inverse();

            for s_transition in &s {
                // In the following comments we use ϕ to symbolize the guard of the transition
                // ϕ_S ∧ Inv(l2_s)[r |-> 0] ∧ Inv(l1_s) ∧ ¬G_T
                let guard_zone = get_allowed_fed(loc_s, s_transition).intersection(&inverse_g_t);

                let updates = vec![CompiledUpdate {
                    clock_index: self.quotient_clock_index,
                    value: 0,
                }];

                transitions.push(Transition {
                    id: TransitionID::Quotient(
                        t.iter().map(|t| t.id.clone()).collect(),
                        vec![s_transition.id.clone()],
                    ),
                    guard_zone,
                    target_locations: inconsistent_location.clone(),
                    updates,
                })
            }
        }

        //Rule 7
        if action == self.new_input_name {
            let inverse_t_invariant = get_invariant(loc_t, self.dim).inverse();
            let s_invariant = get_invariant(loc_s, self.dim);
            let guard_zone = inverse_t_invariant.intersection(&s_invariant);

            let updates = vec![CompiledUpdate {
                clock_index: self.quotient_clock_index,
                value: 0,
            }];

            transitions.push(Transition {
                id: TransitionID::None,
                guard_zone,
                target_locations: inconsistent_location,
                updates,
            })
        }
        //Rule 8
        if self.T.actions_contain(action) && !self.S.actions_contain(action) {
            for t_transition in &t {
                let mut guard_zone = get_allowed_fed(loc_t, t_transition);

                guard_zone = loc_s.apply_invariants(guard_zone);

                let target_locations = merge(&t_transition.target_locations, loc_s);
                let updates = t_transition.updates.clone();

                transitions.push(Transition {
                    id: TransitionID::Quotient(vec![t_transition.id.clone()], Vec::new()),
                    guard_zone,
                    target_locations,
                    updates,
                });
            }
        }

        transitions
            .into_iter()
            .filter(|e| !e.guard_zone.is_empty())
            .collect()
    }

    fn get_all_locations(&self) -> Vec<LocationTuple> {
        let mut location_tuples = vec![];

        let left = self.T.get_all_locations();
        let right = self.S.get_all_locations();
        for loc_t in &left {
            for loc_s in &right {
                let location = merge(loc_t, loc_s);
                location_tuples.push(location);
            }
        }

        let inconsistent =
            LocationTuple::simple(&self.inconsistent_location, None, &self.decls, self.dim);
        let universal =
            LocationTuple::simple(&self.universal_location, None, &self.decls, self.dim);

        location_tuples.push(inconsistent);
        location_tuples.push(universal);

        location_tuples
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
        let (t, s) = self.get_children();
        Some(merge(
            &t.get_initial_location()?,
            &s.get_initial_location()?,
        ))
    }

    fn get_decls(&self) -> Vec<&Declarations> {
        let mut comps = self.T.get_decls();
        comps.extend(self.S.get_decls());
        comps.push(&self.decls);
        comps
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
        if let DeterminismResult::Success = self.T.is_deterministic() {
            if let DeterminismResult::Success = self.S.is_deterministic() {
                DeterminismResult::Success
            } else {
                self.S.is_deterministic()
            }
        } else {
            self.T.is_deterministic()
        }
    }

    fn is_locally_consistent(&self) -> ConsistencyResult {
        if let ConsistencyResult::Success = self.T.is_locally_consistent() {
            if let ConsistencyResult::Success = self.S.is_locally_consistent() {
                ConsistencyResult::Success
            } else {
                self.S.is_locally_consistent()
            }
        } else {
            self.T.is_locally_consistent()
        }
    }

    fn get_initial_state(&self) -> Option<State> {
        let init_loc = self.get_initial_location()?;
        let zone = OwnedFederation::init(self.dim);
        Some(State::create(init_loc, zone))
    }

    fn get_children(&self) -> (&TransitionSystemPtr, &TransitionSystemPtr) {
        (&self.T, &self.S)
    }

    fn get_composition_type(&self) -> CompositionType {
        CompositionType::Quotient
    }

    fn get_dim(&self) -> ClockIndex {
        self.dim
    }

    fn get_combined_decls(&self) -> Declarations {
        let mut clocks = HashMap::new();
        let mut ints = HashMap::new();
        for decl in [
            self.S.get_combined_decls(),
            self.T.get_combined_decls(),
            self.decls.clone(),
        ] {
            clocks.extend(decl.clocks);
            ints.extend(decl.ints)
        }

        Declarations { ints, clocks }
    }
}

fn merge(t: &LocationTuple, s: &LocationTuple) -> LocationTuple {
    LocationTuple::merge_as_quotient(t, s)
}

fn get_allowed_fed(from: &LocationTuple, transition: &Transition) -> OwnedFederation {
    let fed = transition.get_allowed_federation();
    from.apply_invariants(fed)
}

fn get_invariant(loc: &LocationTuple, dim: ClockIndex) -> OwnedFederation {
    match loc.get_invariants() {
        Some(inv) => inv.clone(),
        None => OwnedFederation::universe(dim),
    }
}
