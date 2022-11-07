use edbm::zones::OwnedFederation;
use log::{debug, info, log_enabled, trace, warn, Level};

use crate::DataTypes::{PassedStateList, PassedStateListExt, WaitingStateList};
use crate::ModelObjects::component::Transition;

use crate::ModelObjects::statepair::StatePair;
use crate::TransitionSystems::{LocationTuple, TransitionSystemPtr, TransitionID};
use std::collections::HashSet;

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

pub fn check_refinement(
    sys1: TransitionSystemPtr,
    sys2: TransitionSystemPtr,
) -> Result<bool, String> {
    let mut context = RefinementContext::new(&sys1, &sys2);
    let dimensions = sys1.get_dim();
    debug!("Dimensions: {}", dimensions);

    //Firstly we check the preconditions
    if !check_preconditions(&sys1, &sys2) {
        warn!("preconditions failed - refinement false");
        return Ok(false);
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

    if initial_locations_1 == None {
        return Ok(initial_locations_2 == None);
    }

    if initial_locations_2 == None {
        return Ok(false); //The empty automata cannot implement
    }

    let initial_locations_1 = initial_locations_1.unwrap();
    let initial_locations_2 = initial_locations_2.unwrap();

    let mut initial_pair = StatePair::create(
        dimensions,
        initial_locations_1.clone(),
        initial_locations_2.clone(),
    );

    if !prepare_init_state(&mut initial_pair, initial_locations_1, initial_locations_2) {
        return Ok(false);
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
                vec![Transition::new(TransitionID::None ,curr_pair.get_locations2(), dimensions)]
            } else {
                sys2.next_outputs(curr_pair.get_locations2(), output)
            };

            let cond = has_valid_state_pairs(
                &output_transition1,
                &output_transition2,
                &curr_pair,
                &mut context,
                true,
            );

            if cond {
                trace!("Created state pairs for output {}", output);
            } else {
                info!("Refinement check failed for Output {:?}", output);
                if log_enabled!(Level::Debug) {
                    debug!("Transitions1:");
                    for t in &output_transition1 {
                        debug!("{}", t);
                    }
                    debug!("Transitions2:");
                    for t in &output_transition2 {
                        debug!("{}", t);
                    }
                    debug!("Current pair: {}", curr_pair);
                    debug!("Relation:");
                    print_relation(&context.passed_list);
                }

                return Ok(false);
            };
        }

        for input in &inputs {
            let extra = extra_inputs.contains(input);

            let input_transitions1 = if extra {
                vec![Transition::new(TransitionID::None, curr_pair.get_locations1(), dimensions)]
            } else {
                sys1.next_inputs(curr_pair.get_locations1(), input)
            };

            let input_transitions2 = sys2.next_inputs(curr_pair.get_locations2(), input);

            let cond = has_valid_state_pairs(
                &input_transitions2,
                &input_transitions1,
                &curr_pair,
                &mut context,
                false,
            );

            if cond {
                trace!("Created state pairs for input {}", input);
            } else {
                info!("Refinement check failed for Input {:?}", input);
                if log_enabled!(Level::Debug) {
                    debug!("Transitions1:");
                    for t in &input_transitions1 {
                        debug!("{}", t);
                    }
                    debug!("Transitions2:");
                    for t in &input_transitions2 {
                        debug!("{}", t);
                    }
                    debug!("Current pair: {}", curr_pair);
                    debug!("Relation:");
                    print_relation(&context.passed_list);
                }

                return Ok(false);
            };
        }
    }
    info!("Refinement check passed");
    if log_enabled!(Level::Debug) {
        debug!("With relation:");
        print_relation(&context.passed_list);
    }

    Ok(true)
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
) -> bool {
    let (fed1, fed2) = get_guard_fed_for_sides(transitions1, transitions2, curr_pair, is_state1);

    // If there are no valid transition1s, continue
    if fed1.is_empty() {
        return true;
    }

    // If there are (valid) transition1s but no transition2s there are no valid pairs
    if fed2.is_empty() {
        trace!("Empty transition2s");
        return false;
    };

    let result_federation = fed1.subtraction(&fed2);

    // If the entire zone of transition1s cannot be matched by transition2s
    if !result_federation.is_empty() {
        return false;
    }

    // Finally try to create the pairs
    let res = try_create_new_state_pairs(transitions1, transitions2, curr_pair, context, is_state1);

    match res {
        BuildResult::Success => true,
        BuildResult::Failure => false,
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

fn check_preconditions(sys1: &TransitionSystemPtr, sys2: &TransitionSystemPtr) -> bool {
    if !(sys2.precheck_sys_rep() && sys1.precheck_sys_rep()) {
        info!("Preconditions failed");
        return false;
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

    disjoint && subset
}
