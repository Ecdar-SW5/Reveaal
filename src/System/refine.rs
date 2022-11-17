use edbm::zones::OwnedFederation;
use log::{debug, info, log_enabled, trace, warn, Level};

use crate::DataTypes::{PassedStateList, PassedStateListExt, WaitingStateList};
use crate::ModelObjects::component::Transition;

use crate::ModelObjects::statepair::StatePair;
use crate::System::local_consistency::ConsistencyFailure;
use crate::TransitionSystems::transition_system::PrecheckResult;
use crate::TransitionSystems::{LocationID, LocationTuple, TransitionID, TransitionSystemPtr};
use std::collections::HashSet;
use std::fmt;

/// The result of a refinement check. [RefinementFailure] specifies the failure.
#[allow(clippy::large_enum_variant)] //TODO: consider boxing the large fields to reduce the total size of the enum
pub enum RefinementResult {
    Success,
    Failure(RefinementFailure),
}

/// The failure of a refinement check. Variants with [StatePair] include the
/// state that caused the failure. Variants with [LocationID] include the
/// locations that caused failure.
#[derive(Debug)]
pub enum RefinementFailure {
    NotDisjointAndNotSubset,
    NotDisjoint,
    NotSubset,
    CutsDelaySolutions(StatePair),
    InitialState(StatePair),
    EmptySpecification,
    EmptyImplementation,
    EmptyTransition2s(StatePair),
    NotEmptyResult(StatePair),
    ConsistencyFailure(Option<LocationID>, Option<String>),
    DeterminismFailure(Option<LocationID>, Option<String>),
}
enum StatePairResult {
    Valid,
    EmptyTransition2s,
    NotEmptyResult,
    CutsDelaySolutions,
}

impl fmt::Display for RefinementFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RefinementFailure::NotDisjointAndNotSubset => write!(f, "Not Disjoint and Not Subset"),
            RefinementFailure::NotDisjoint => write!(f, "Not Disjoint"),
            RefinementFailure::NotSubset => write!(f, "Not Subset"),
            RefinementFailure::CutsDelaySolutions(_) => write!(f, "Cuts Delay Solutions"),
            RefinementFailure::InitialState(_) => write!(f, "Error in Initial State"),
            RefinementFailure::EmptySpecification => write!(f, "Empty Specification"),
            RefinementFailure::EmptyImplementation => write!(f, "Empty Implementation"),
            RefinementFailure::EmptyTransition2s(_) => write!(f, "Empty Transition2s"),
            RefinementFailure::NotEmptyResult(_) => write!(f, "Not Empty Result on State Pair"),
            RefinementFailure::ConsistencyFailure(location, action) => {
                write!(
                    f,
                    "Not Consistent From {} failing action {}",
                    location.as_ref().unwrap(),
                    action.as_ref().unwrap()
                )
            }
            RefinementFailure::DeterminismFailure(location, action) => {
                write!(
                    f,
                    "Not Deterministic From {} failing action {}",
                    location.as_ref().unwrap(),
                    action.as_ref().unwrap()
                )
            }
        }
    }
}

fn common_actions(
    sys1: &TransitionSystemPtr,
    sys2: &TransitionSystemPtr,
    is_input: bool,
) -> HashSet<String> {
    if is_input {
        sys2.get_input_actions()
    } else {
        sys1.get_output_actions()
    }
}

fn extra_actions(
    sys1: &TransitionSystemPtr,
    sys2: &TransitionSystemPtr,
    is_input: bool,
) -> HashSet<String> {
    if is_input {
        sys2.get_input_actions()
            .difference(&sys1.get_input_actions())
            .cloned()
            .collect()
    } else {
        sys1.get_output_actions()
            .difference(&sys2.get_output_actions())
            .cloned()
            .collect()
    }
}

struct RefinementContext<'a> {
    pub passed_list: PassedStateList,
    pub waiting_list: WaitingStateList,
    pub sys1: &'a TransitionSystemPtr,
    pub sys2: &'a TransitionSystemPtr,
}

impl<'a> RefinementContext<'a> {
    fn new(sys1: &'a TransitionSystemPtr, sys2: &'a TransitionSystemPtr) -> RefinementContext<'a> {
        RefinementContext {
            passed_list: PassedStateList::new(),
            waiting_list: WaitingStateList::new(),
            sys1,
            sys2,
        }
    }
}

/// Checks if sys1 refines sys2
pub fn check_refinement(sys1: TransitionSystemPtr, sys2: TransitionSystemPtr) -> RefinementResult {
    let mut context = RefinementContext::new(&sys1, &sys2);
    let dimensions = sys1.get_dim();
    debug!("Dimensions: {}", dimensions);

    //Firstly we check the preconditions
    if let RefinementResult::Failure(failure) = check_preconditions(&sys1, &sys2) {
        warn!("Refinement failed with failure: {}", failure);
        return RefinementResult::Failure(failure);
    }

    // Common inputs and outputs
    let inputs = common_actions(&sys1, &sys2, true);
    let outputs = common_actions(&sys1, &sys2, false);

    info!(
        "Left inputs: {:?}, Left outputs: {:?}",
        sys1.get_input_actions(),
        sys1.get_output_actions()
    );

    info!(
        "Right inputs: {:?}, Right outputs; {:?}",
        sys2.get_input_actions(),
        sys2.get_output_actions()
    );

    // Extra inputs and outputs are ignored by default
    let extra_inputs = extra_actions(&sys1, &sys2, true);
    let extra_outputs = extra_actions(&sys1, &sys2, false);

    let initial_locations_1 = sys1.get_initial_location();
    let initial_locations_2 = sys2.get_initial_location();

    debug!("Extra inputs {:?}", extra_inputs);
    debug!("Extra outputs {:?}", extra_outputs);

    if initial_locations_1.is_none() {
        if initial_locations_2.is_none() {
            // Both are empty, so trivially true
            return RefinementResult::Success;
        }
        return RefinementResult::Failure(RefinementFailure::EmptyImplementation);
    }

    if initial_locations_2.is_none() {
        //The empty automata cannot implement
        return RefinementResult::Failure(RefinementFailure::EmptySpecification);
    }

    let initial_locations_1 = initial_locations_1.unwrap();
    let initial_locations_2 = initial_locations_2.unwrap();

    let mut initial_pair = StatePair::create(
        dimensions,
        initial_locations_1.clone(),
        initial_locations_2.clone(),
    );

    if !prepare_init_state(&mut initial_pair, initial_locations_1, initial_locations_2) {
        return RefinementResult::Failure(RefinementFailure::InitialState(initial_pair));
    }
    initial_pair.extrapolate_max_bounds(context.sys1, context.sys2);

    debug!("Initial {}", initial_pair);
    context.waiting_list.put(initial_pair);

    while !context.waiting_list.is_empty() {
        let curr_pair = context.waiting_list.pop().unwrap();
        trace!("Checking {}", curr_pair);

        context.passed_list.put(curr_pair.clone());
        for output in &outputs {
            let extra = extra_outputs.contains(output);

            let output_transition1 = sys1.next_outputs(curr_pair.get_locations1(), output);
            let output_transition2 = if extra {
                vec![Transition::new(
                    curr_pair.get_locations2(),
                    dimensions,
                )]
            } else {
                sys2.next_outputs(curr_pair.get_locations2(), output)
            };

            match has_valid_state_pairs(
                &output_transition1,
                &output_transition2,
                &curr_pair,
                &mut context,
                true,
            ) {
                StatePairResult::Valid => trace!("Created state pairs for input {}", output),
                StatePairResult::EmptyTransition2s => {
                    log_refinement_check_failure(
                        &output_transition1,
                        &output_transition2,
                        &curr_pair,
                        &context,
                        output,
                        false,
                    );
                    return RefinementResult::Failure(RefinementFailure::EmptyTransition2s(
                        curr_pair,
                    ));
                }
                StatePairResult::NotEmptyResult => {
                    log_refinement_check_failure(
                        &output_transition1,
                        &output_transition2,
                        &curr_pair,
                        &context,
                        output,
                        false,
                    );
                    return RefinementResult::Failure(RefinementFailure::NotEmptyResult(curr_pair));
                }
                StatePairResult::CutsDelaySolutions => {
                    log_refinement_check_failure(
                        &output_transition1,
                        &output_transition2,
                        &curr_pair,
                        &context,
                        output,
                        false,
                    );
                    return RefinementResult::Failure(RefinementFailure::CutsDelaySolutions(
                        curr_pair,
                    ));
                }
            }
        }

        for input in &inputs {
            let extra = extra_inputs.contains(input);

            let input_transitions1 = if extra {
                vec![Transition::new(
                    curr_pair.get_locations1(),
                    dimensions,
                )]
            } else {
                sys1.next_inputs(curr_pair.get_locations1(), input)
            };

            let input_transitions2 = sys2.next_inputs(curr_pair.get_locations2(), input);

            match has_valid_state_pairs(
                &input_transitions2,
                &input_transitions1,
                &curr_pair,
                &mut context,
                false,
            ) {
                StatePairResult::Valid => trace!("Created state pairs for input {}", input),
                StatePairResult::EmptyTransition2s => {
                    log_refinement_check_failure(
                        &input_transitions1,
                        &input_transitions2,
                        &curr_pair,
                        &context,
                        input,
                        true,
                    );
                    return RefinementResult::Failure(RefinementFailure::EmptyTransition2s(
                        curr_pair,
                    ));
                }
                StatePairResult::NotEmptyResult => {
                    log_refinement_check_failure(
                        &input_transitions1,
                        &input_transitions2,
                        &curr_pair,
                        &context,
                        input,
                        true,
                    );
                    return RefinementResult::Failure(RefinementFailure::NotEmptyResult(curr_pair));
                }
                StatePairResult::CutsDelaySolutions => {
                    log_refinement_check_failure(
                        &input_transitions1,
                        &input_transitions2,
                        &curr_pair,
                        &context,
                        input,
                        true,
                    );
                    return RefinementResult::Failure(RefinementFailure::CutsDelaySolutions(
                        curr_pair,
                    ));
                }
            }
        }
    }
    info!("Refinement check passed");
    if log_enabled!(Level::Debug) {
        debug!("With relation:");
        print_relation(&context.passed_list);
    }

    RefinementResult::Success
}

fn log_refinement_check_failure(
    transitions1: &Vec<Transition>,
    transitions2: &Vec<Transition>,
    curr_pair: &StatePair,
    context: &RefinementContext,
    action: &String,
    is_input: bool,
) {
    let action_type = if is_input {
        String::from("Input")
    } else {
        String::from("Output")
    };
    info!("Refinement check failed for {} {:?}", action_type, action);
    if log_enabled!(Level::Debug) {
        debug!("Transitions1:");
        for t in transitions1 {
            debug!("{}", t);
        }
        debug!("Transitions2:");
        for t in transitions2 {
            debug!("{}", t);
        }
        debug!("Current pair: {}", curr_pair);
        debug!("Relation:");
        print_relation(&context.passed_list);
    }
}

fn print_relation(passed_list: &PassedStateList) {
    let verbose = false;

    let mut sorted_keys: Vec<_> = passed_list.keys().collect();
    sorted_keys.sort_by_key(|(a, b)| format!("1:{}, 2:{}", a, b));
    for (id1, id2) in sorted_keys {
        let zones = passed_list.zones(&(id1.clone(), id2.clone()));

        debug!(
            "{}",
            if zones.len() != 1 {
                format!("1:{} 2:{} {} zones", id1, id2, zones.len())
            } else if verbose {
                format!("1:{} 2:{} {} zone", id1, id2, zones[0])
            } else {
                format!("1:{} 2:{}", id1, id2)
            }
        );
    }
}

fn has_valid_state_pairs(
    transitions1: &[Transition],
    transitions2: &[Transition],
    curr_pair: &StatePair,
    context: &mut RefinementContext,
    is_state1: bool,
) -> StatePairResult {
    let (fed1, fed2) = get_guard_fed_for_sides(transitions1, transitions2, curr_pair, is_state1);

    // If there are no valid transition1s, continue
    if fed1.is_empty() {
        return StatePairResult::Valid;
    }

    // If there are (valid) transition1s but no transition2s there are no valid pairs
    if fed2.is_empty() {
        trace!("Empty transition2s");
        return StatePairResult::EmptyTransition2s;
    };

    let result_federation = fed1.subtraction(&fed2);

    // If the entire zone of transition1s cannot be matched by transition2s
    if !result_federation.is_empty() {
        return StatePairResult::NotEmptyResult;
    }

    // Finally try to create the pairs
    let res = try_create_new_state_pairs(transitions1, transitions2, curr_pair, context, is_state1);

    match res {
        BuildResult::Success => StatePairResult::Valid,
        BuildResult::Failure => StatePairResult::CutsDelaySolutions,
    }
}

fn get_guard_fed_for_sides(
    transitions1: &[Transition],
    transitions2: &[Transition],
    curr_pair: &StatePair,
    is_state1: bool,
) -> (OwnedFederation, OwnedFederation) {
    let dim = curr_pair.ref_zone().dim();

    let pair_zone = curr_pair.ref_zone();
    trace!("Zone: {}", pair_zone);
    //create guard zones left
    let mut feds = OwnedFederation::empty(dim);
    trace!("{}", if is_state1 { "Left:" } else { "Right:" });
    for transition in transitions1 {
        trace!("{}", transition);
        feds += transition.get_allowed_federation();
    }
    let fed1 = feds.intersection(pair_zone);
    trace!("{}", fed1);

    trace!("{}", if is_state1 { "Right:" } else { "Left:" });
    //Create guard zones right
    let mut feds = OwnedFederation::empty(dim);
    for transition in transitions2 {
        trace!("{}", transition);
        feds += transition.get_allowed_federation();
    }
    let fed2 = feds.intersection(pair_zone);
    trace!("{}", fed2);

    (fed1, fed2)
}

enum BuildResult {
    Success,
    Failure,
}

/// Returns a failure if the new state pairs cut delay solutions, otherwise returns success
fn try_create_new_state_pairs(
    transitions1: &[Transition],
    transitions2: &[Transition],
    curr_pair: &StatePair,
    context: &mut RefinementContext,
    is_state1: bool,
) -> BuildResult {
    for transition1 in transitions1 {
        for transition2 in transitions2 {
            if let BuildResult::Failure =
                build_state_pair(transition1, transition2, curr_pair, context, is_state1)
            {
                return BuildResult::Failure;
            }
        }
    }

    BuildResult::Success
}

fn build_state_pair(
    transition1: &Transition,
    transition2: &Transition,
    curr_pair: &StatePair,
    context: &mut RefinementContext,
    is_state1: bool,
) -> BuildResult {
    //Creates new state pair
    let mut new_sp: StatePair = curr_pair.clone();
    //Creates DBM for that state pair
    let mut new_sp_zone = new_sp.take_zone();
    //Apply guards on both sides
    let (locations1, locations2) = new_sp.get_mut_states(is_state1);

    //Applies the left side guards and checks if zone is valid
    new_sp_zone = transition1.apply_guards(new_sp_zone);
    //Applies the right side guards and checks if zone is valid
    new_sp_zone = transition2.apply_guards(new_sp_zone);

    // Continue to the next transition pair if the zone is empty
    if new_sp_zone.is_empty() {
        return BuildResult::Success;
    }

    //Apply updates on both sides
    new_sp_zone = transition1.apply_updates(new_sp_zone);
    new_sp_zone = transition2.apply_updates(new_sp_zone);

    //Perform a delay on the zone after the updates were applied
    new_sp_zone = new_sp_zone.up();

    //Update locations in states

    transition1.move_locations(locations1);
    transition2.move_locations(locations2);

    // Apply invariants on the left side of relation
    let (left_loc, right_loc) = if is_state1 {
        //(locations2, locations1)
        (locations1, locations2)
    } else {
        //(locations1, locations2)
        (locations2, locations1)
    };

    new_sp_zone = left_loc.apply_invariants(new_sp_zone);

    // Clone the zone before applying right side invariants
    let s_invariant = new_sp_zone.clone();

    // Apply right side invariants on the zone
    new_sp_zone = right_loc.apply_invariants(new_sp_zone);

    // Continue to the next transition pair if the newly built zones are empty
    if new_sp_zone.is_empty() || s_invariant.is_empty() {
        return BuildResult::Success;
    }

    // inv_s = x<10, inv_t = x>2 -> t cuts solutions but not delays, so it is fine and we can call down:
    let t_invariant = new_sp_zone.clone().down();

    // Check if the invariant of T (right) cuts delay solutions from S (left) and if so, report failure
    if !(s_invariant.subset_eq(&t_invariant)) {
        return BuildResult::Failure;
    }

    new_sp.set_zone(new_sp_zone);

    new_sp.extrapolate_max_bounds(context.sys1, context.sys2);

    if !context.passed_list.has(&new_sp) && !context.waiting_list.has(&new_sp) {
        debug!("New state {}", new_sp);

        context.waiting_list.put(new_sp);
    }

    BuildResult::Success
}

fn prepare_init_state(
    initial_pair: &mut StatePair,
    initial_locations_1: LocationTuple,
    initial_locations_2: LocationTuple,
) -> bool {
    let mut sp_zone = initial_pair.take_zone();
    sp_zone = initial_locations_1.apply_invariants(sp_zone);
    sp_zone = initial_locations_2.apply_invariants(sp_zone);

    initial_pair.set_zone(sp_zone);

    !initial_pair.ref_zone().is_empty()
}

fn check_preconditions(sys1: &TransitionSystemPtr, sys2: &TransitionSystemPtr) -> RefinementResult {
    match sys1.precheck_sys_rep() {
        PrecheckResult::Success => {}
        PrecheckResult::NotDeterministic(location, action) => {
            warn!("Refinement failed - sys1 is not deterministic");
            return RefinementResult::Failure(RefinementFailure::DeterminismFailure(
                Some(location),
                Some(action),
            ));
        }
        PrecheckResult::NotConsistent(failure) => {
            warn!("Refinement failed - sys1 is inconsistent");
            match failure {
                ConsistencyFailure::NoInitialLocation | ConsistencyFailure::EmptyInitialState => {
                    return RefinementResult::Failure(RefinementFailure::ConsistencyFailure(
                        None, None,
                    ))
                }
                ConsistencyFailure::NotConsistentFrom(location, action)
                | ConsistencyFailure::NotDeterministicFrom(location, action) => {
                    return RefinementResult::Failure(RefinementFailure::ConsistencyFailure(
                        Some(location),
                        Some(action),
                    ))
                }
            }
        }
    }
    match sys2.precheck_sys_rep() {
        PrecheckResult::Success => {}
        PrecheckResult::NotDeterministic(location, action) => {
            warn!("Refinement failed - sys2 is not deterministic");
            return RefinementResult::Failure(RefinementFailure::DeterminismFailure(
                Some(location),
                Some(action),
            ));
        }
        PrecheckResult::NotConsistent(failure) => {
            warn!("Refinement failed - sys2 is inconsistent");
            match failure {
                ConsistencyFailure::NoInitialLocation | ConsistencyFailure::EmptyInitialState => {
                    return RefinementResult::Failure(RefinementFailure::ConsistencyFailure(
                        None, None,
                    ))
                }
                ConsistencyFailure::NotConsistentFrom(location, action)
                | ConsistencyFailure::NotDeterministicFrom(location, action) => {
                    return RefinementResult::Failure(RefinementFailure::ConsistencyFailure(
                        Some(location),
                        Some(action),
                    ))
                }
            }
        }
    }

    let s_outputs = sys1.get_output_actions();
    let t_outputs = sys2.get_output_actions();

    let s_inputs = sys1.get_input_actions();
    let t_inputs = sys2.get_input_actions();

    let disjoint = s_inputs.is_disjoint(&t_outputs) && t_inputs.is_disjoint(&s_outputs);

    let subset = s_inputs.is_subset(&t_inputs) && t_outputs.is_subset(&s_outputs);

    debug!("Disjoint {disjoint}, subset {subset}");
    debug!("S i:{s_inputs:?} o:{s_outputs:?}");
    debug!("T i:{t_inputs:?} o:{t_outputs:?}");

    if !disjoint && !subset {
        warn!("Refinement failed - Systems are not disjoint and not subset");
        RefinementResult::Failure(RefinementFailure::NotDisjointAndNotSubset)
    } else if !subset {
        warn!("Refinement failed - Systems are not subset");
        RefinementResult::Failure(RefinementFailure::NotSubset)
    } else if !disjoint {
        warn!("Refinement failed - Systems not disjoint");
        RefinementResult::Failure(RefinementFailure::NotDisjoint)
    } else {
        RefinementResult::Success
    }
}
