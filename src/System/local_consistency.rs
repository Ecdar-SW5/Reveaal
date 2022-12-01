use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender, TryRecvError};
use std::fmt;
use std::ops::Deref;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use edbm::zones::OwnedFederation;
use log::warn;
use tokio::runtime;

use crate::ModelObjects::component::State;
use crate::ProtobufServer::threadpool::{ThreadPool, ThreadPoolFuture};
use crate::TransitionSystems::{LocationID, TransitionSystem};

/// The result of a consistency check.
/// If there was a failure, [ConsistencyFailure] will specify the failure.
pub enum ConsistencyResult {
    Success,
    Failure(ConsistencyFailure),
}

impl fmt::Display for ConsistencyResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsistencyResult::Success => write!(f, "Succes"),
            ConsistencyResult::Failure(_) => write!(f, "failure"),
        }
    }
}
/// The failure of a consistency check.
/// Variants with [LocationID] are specific locations that cause the failure.
#[derive(Debug)]
pub enum ConsistencyFailure {
    NoInitialLocation,
    EmptyInitialState,
    NotConsistentFrom(LocationID, String),
    NotDeterministicFrom(LocationID, String),
}

impl fmt::Display for ConsistencyFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsistencyFailure::NoInitialLocation => write!(f, "No Initial State"),
            ConsistencyFailure::EmptyInitialState => write!(f, "Empty Initial State"),
            ConsistencyFailure::NotConsistentFrom(location, action) => {
                write!(
                    f,
                    "Not Consistent From {} Failing action {}",
                    location, action
                )
            }
            ConsistencyFailure::NotDeterministicFrom(location, action) => {
                write!(
                    f,
                    "Not Deterministic From {} Failing action {}",
                    location, action
                )
            }
        }
    }
}

/// The result of a determinism check.
/// Failure includes the [LocationID].
pub enum DeterminismResult {
    Success,
    Failure(LocationID, String),
}

impl fmt::Display for DeterminismResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeterminismResult::Success => write!(f, "Success"),
            DeterminismResult::Failure(location, action) => {
                write!(
                    f,
                    "Not Deterministic From {} failing action {}",
                    location, action
                )
            }
        }
    }
}

///Local consistency check WITH pruning.
pub fn is_least_consistent(system: &dyn TransitionSystem) -> ConsistencyResult {
    if system.get_initial_location().is_none() {
        return ConsistencyResult::Failure(ConsistencyFailure::NoInitialLocation);
        //TODO: figure out whether we want empty TS to be consistent
    }

    let mut passed = vec![];
    let state = system.get_initial_state();
    if state.is_none() {
        warn!("Empty initial state");
        return ConsistencyResult::Failure(ConsistencyFailure::EmptyInitialState);
    }
    let mut state = state.unwrap();
    state.extrapolate_max_bounds(system);
    consistency_least_helper(state, &mut passed, system)
}

///Checks if a [TransitionSystem] is deterministic.
pub fn is_deterministic(
    system: Arc<Box<dyn TransitionSystem + Sync + Send>>,
    threadpool: &Arc<ThreadPool>,
) -> DeterminismResult {
    if system.get_initial_location().is_none() {
        return DeterminismResult::Success;
    }
    let state = system.get_initial_state();
    if state.is_none() {
        return DeterminismResult::Success;
    }
    let mut state = state.unwrap();
    state.set_zone(OwnedFederation::universe(system.get_dim()));

    // This is broken on single core setups and cannot be fixed!
    // We need some way to pause execution on a thread in the threadpool and let it run another task
    // and come back to the other one later.
    let passed = Arc::new(Mutex::new(vec![]));
    let failure_thing: Arc<RwLock<Option<DeterminismResult>>> = Arc::new(RwLock::new(None));
    let (sender, receiver) = unbounded();
    let (state_sender, state_receiver) = unbounded();
    let threads = num_cpus::get() - 1;
    let pending_work = Arc::new(AtomicUsize::new(1));
    let mut futures: Vec<ThreadPoolFuture<()>> = (0..threads)
        .map(|_| {
            let thread_sender = sender.clone();
            let thread_receiver = receiver.clone();
            let thread_state_sender = state_sender.clone();
            let thread_state_receiver = state_receiver.clone();
            let thread_failure_thing = failure_thing.clone();
            let thread_system = system.clone();
            let thread_passed = passed.clone();
            let thread_pending_work = pending_work.clone();
            threadpool.enqueue(move |_| {
                thingamagic(
                    threads,
                    thread_sender,
                    thread_receiver,
                    thread_state_sender,
                    thread_state_receiver,
                    thread_failure_thing,
                    thread_system,
                    thread_passed,
                    thread_pending_work,
                );
            })
        })
        .collect();

    sender.send(state).unwrap();

    thingamagic(
        threads,
        sender,
        receiver,
        state_sender,
        state_receiver,
        failure_thing.clone(),
        system,
        passed,
        pending_work,
    );

    let runner = runtime::Builder::new_current_thread().build().unwrap();

    loop {
        match futures.pop() {
            None => break,
            Some(f) => runner.block_on(f),
        }
    }

    let result = failure_thing.write().unwrap().take();
    let result = result.unwrap_or_else(|| DeterminismResult::Success);
    result
}

fn thingamagic(
    num_threads: usize,
    sender: Sender<State>,
    receiver: Receiver<State>,
    state_sender: Sender<()>,
    state_receiver: Receiver<()>,
    failure_thing: Arc<RwLock<Option<DeterminismResult>>>,
    system: Arc<Box<dyn TransitionSystem + Send + Sync>>,
    passed: Arc<Mutex<Vec<State>>>,
    pending_work: Arc<AtomicUsize>,
) {
    loop {
        select! {
            recv(state_receiver) -> _ => return,
            recv(receiver) -> state => is_deterministic_helper(num_threads + 1, &failure_thing, state.unwrap(), &passed, &system, &sender, &pending_work, &state_sender),
        }

        // Exit when done
        if pending_work.load(Ordering::SeqCst) == 0 {
            for _ in 0..num_threads + 1 {
                state_sender.send(()).unwrap();
            }
        }
    }
}

fn is_deterministic_helper(
    num_threads: usize,
    failure_thing: &RwLock<Option<DeterminismResult>>,
    state: State,
    passed_list: &Arc<Mutex<Vec<State>>>,
    system: &Arc<Box<dyn TransitionSystem + Send + Sync>>,
    sender: &Sender<State>,
    pending_work: &Arc<AtomicUsize>,
    state_sender: &Sender<()>,
) {
    for action in system.get_actions() {
        let mut location_fed = OwnedFederation::empty(system.get_dim());
        for transition in &system.next_transitions(&state.decorated_locations, &action) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                let mut allowed_fed = transition.get_allowed_federation();
                let allowed_fed = state.decorated_locations.apply_invariants(allowed_fed);
                if allowed_fed.has_intersection(&location_fed) {
                    warn!(
                        "Not deterministic from location {} failing action {}",
                        state.get_location().id,
                        action
                    );
                    {
                        let mut failure_thing = failure_thing.write().unwrap();
                        *failure_thing = Some(DeterminismResult::Failure(
                            state.get_location().id.clone(),
                            action.clone(),
                        ));
                        for _ in 0..num_threads {
                            state_sender.send(()).unwrap();
                        }
                        pending_work.fetch_sub(1, Ordering::SeqCst);
                        return;
                    }
                }
                location_fed += allowed_fed;
                let system = system.deref().deref();
                new_state.extrapolate_max_bounds(system);

                {
                    let mut passed_list = passed_list.lock().unwrap();
                    if !new_state.is_contained_in_list(&passed_list) {
                        passed_list.push(new_state.clone());
                        pending_work.fetch_add(1, Ordering::SeqCst);
                        sender.send(new_state).unwrap();
                    }
                }
            }
        }
    }
    pending_work.fetch_sub(1, Ordering::SeqCst);
}

/// Local consistency check WITHOUT pruning
#[allow(dead_code)]
pub fn is_fully_consistent(system: &dyn TransitionSystem) -> ConsistencyResult {
    if system.get_initial_location().is_none() {
        return ConsistencyResult::Failure(ConsistencyFailure::NoInitialLocation);
    }

    let mut passed = vec![];
    let state = system.get_initial_state();
    if state.is_none() {
        warn!("Empty initial state");
        return ConsistencyResult::Failure(ConsistencyFailure::EmptyInitialState);
    }
    consistency_fully_helper(state.unwrap(), &mut passed, system)
}

pub fn consistency_least_helper(
    state: State,
    passed_list: &mut Vec<State>,
    system: &dyn TransitionSystem,
) -> ConsistencyResult {
    let mut failing_action = String::new();

    if state.is_contained_in_list(passed_list) {
        return ConsistencyResult::Success;
    }
    if state.decorated_locations.is_universal() {
        return ConsistencyResult::Success;
    }
    if state.decorated_locations.is_inconsistent() {
        return ConsistencyResult::Failure(ConsistencyFailure::NotConsistentFrom(
            state.get_location().id.clone(),
            failing_action,
        ));
    }

    passed_list.push(state.clone());

    for input in system.get_input_actions() {
        for transition in &system.next_inputs(&state.decorated_locations, &input) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                new_state.extrapolate_max_bounds(system);
                if let ConsistencyResult::Failure(failure) =
                    consistency_least_helper(new_state, passed_list, system)
                {
                    warn!(
                        "Input \"{input}\" not consistent from {}",
                        state.get_location().id
                    );
                    return ConsistencyResult::Failure(failure);
                }
            } else {
                failing_action = input.clone();
            }
        }
    }

    if state.zone_ref().can_delay_indefinitely() {
        return ConsistencyResult::Success;
    }

    for output in system.get_output_actions() {
        for transition in system.next_outputs(&state.decorated_locations, &output) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                new_state.extrapolate_max_bounds(system);
                if let ConsistencyResult::Success =
                    consistency_least_helper(new_state, passed_list, system)
                {
                    return ConsistencyResult::Success;
                }
            } else {
                failing_action = output.clone();
            }
        }
    }
    warn!("No saving outputs from {}", state.get_location().id);
    ConsistencyResult::Failure(ConsistencyFailure::NotConsistentFrom(
        state.get_location().id.clone(),
        failing_action,
    ))
}

#[allow(dead_code)]
fn consistency_fully_helper(
    state: State,
    passed_list: &mut Vec<State>,
    system: &dyn TransitionSystem,
) -> ConsistencyResult {
    let mut failing_action = String::new();

    if state.is_contained_in_list(passed_list) {
        return ConsistencyResult::Success;
    }
    passed_list.push(state.clone());

    for input in system.get_input_actions() {
        for transition in system.next_inputs(&state.decorated_locations, &input) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                new_state.extrapolate_max_bounds(system);
                if new_state.is_subset_of(&state) {
                    continue;
                }
                if let ConsistencyResult::Failure(failure) =
                    consistency_fully_helper(new_state, passed_list, system)
                {
                    return ConsistencyResult::Failure(failure);
                }
            } else {
                failing_action = input.clone();
            }
        }
    }

    let mut output_existed = false;
    for output in system.get_output_actions() {
        for transition in system.next_outputs(&state.decorated_locations, &output) {
            let mut new_state = state.clone();
            if transition.use_transition(&mut new_state) {
                new_state.extrapolate_max_bounds(system);
                if new_state.is_subset_of(&state) {
                    continue;
                }

                output_existed = true;

                if let ConsistencyResult::Failure(failure) =
                    consistency_fully_helper(new_state, passed_list, system)
                {
                    return ConsistencyResult::Failure(failure);
                }
            } else {
                failing_action = output.clone();
            }
        }
    }
    if output_existed {
        ConsistencyResult::Success
    } else {
        let last_state = passed_list.last().unwrap();
        match last_state.zone_ref().can_delay_indefinitely() {
            false => ConsistencyResult::Failure(ConsistencyFailure::NotConsistentFrom(
                last_state.get_location().id.clone(),
                failing_action,
            )),
            true => ConsistencyResult::Success,
        }
    }
}
