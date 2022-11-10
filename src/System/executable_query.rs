use edbm::util::constraints::ClockIndex;

use crate::DataReader::component_loader::ComponentLoader;
use crate::ModelObjects::component::Component;
use crate::ModelObjects::component::State;
use crate::System::reachability;
use crate::System::reachability::Path;
use crate::System::refine;
use crate::System::save_component::combine_components;
use crate::TransitionSystems::transition_system::PrecheckResult;
use crate::TransitionSystems::TransitionSystemPtr;

use super::extract_system_rep::SystemRecipe;
use super::local_consistency::{ConsistencyFailure, ConsistencyResult, DeterminismResult};
use super::refine::RefinementResult;
use super::save_component::PruningStrategy;

pub enum QueryResult {
    Reachability(Path), // This represents a path from start state to end state
    Refinement(RefinementResult),
    GetComponent(Component),
    Consistency(ConsistencyResult),
    Determinism(DeterminismResult),
    Error(String),
}

impl QueryResult {
    pub fn print_result(&self, query_str: &str) {
        match self {
            QueryResult::Refinement(RefinementResult::Success) => satisfied(query_str),
            QueryResult::Refinement(RefinementResult::Failure(failure)) => {
                not_satisfied(query_str);
                println!("\nGot failure: {}", failure);
            }

            QueryResult::Reachability(path) => {
                if path.was_reachable {
                    satisfied(query_str)
                } else {
                    not_satisfied(query_str)
                }
            }

            QueryResult::Consistency(ConsistencyResult::Success) => satisfied(query_str),
            QueryResult::Consistency(ConsistencyResult::Failure(_)) => not_satisfied(query_str),

            QueryResult::Determinism(DeterminismResult::Success) => satisfied(query_str),
            QueryResult::Determinism(DeterminismResult::Failure(_, _)) => not_satisfied(query_str),

            QueryResult::GetComponent(_) => {
                println!("{} -- Component succesfully created", query_str)
            }

            QueryResult::Error(_) => println!("{} -- Failed", query_str),
        };
    }
}

fn satisfied(query_str: &str) {
    println!("{} -- Property is satisfied", query_str);
}

fn not_satisfied(query_str: &str) {
    println!("{} -- Property is NOT satisfied", query_str);
}

pub trait ExecutableQuery {
    fn execute(self: Box<Self>) -> QueryResult;
}

pub struct RefinementExecutor {
    pub sys1: TransitionSystemPtr,
    pub sys2: TransitionSystemPtr,
}

impl ExecutableQuery for RefinementExecutor {
    fn execute(self: Box<Self>) -> QueryResult {
        let (sys1, sys2) = (self.sys1, self.sys2);

        match refine::check_refinement(sys1, sys2) {
            RefinementResult::Success => QueryResult::Refinement(RefinementResult::Success),
            RefinementResult::Failure(the_failure) => {
                QueryResult::Refinement(RefinementResult::Failure(the_failure))
            }
        }
    }
}

/// Used to store input for the reachability checker
pub struct ReachabilityExecutor {
    // sys represents the transition system
    pub transition_system: TransitionSystemPtr,

    // s_state is the start state
    pub start_state: State,

    // e_state is the end state, where we want to see whether end state is reachable from start state
    pub end_state: State,
}
impl ExecutableQuery for ReachabilityExecutor {
    fn execute(self: Box<Self>) -> QueryResult {
        match reachability::find_path(
            self.start_state,
            self.end_state,
            self.transition_system.as_ref(),
        ) {
            Ok(res) => QueryResult::Reachability(res),
            Err(err_msg) => QueryResult::Error(err_msg),
        }
    }
}

pub struct GetComponentExecutor<'a> {
    pub system: TransitionSystemPtr,
    pub comp_name: String,
    pub component_loader: &'a mut dyn ComponentLoader,
}

impl<'a> ExecutableQuery for GetComponentExecutor<'a> {
    fn execute(self: Box<Self>) -> QueryResult {
        let mut comp = combine_components(&self.system, PruningStrategy::Reachable);
        comp.name = self.comp_name;

        comp.create_edge_io_split();

        self.component_loader.save_component(comp.clone());

        QueryResult::GetComponent(comp)
    }
}

pub struct ConsistencyExecutor {
    pub recipe: Box<SystemRecipe>,
    pub dim: ClockIndex,
}

impl ExecutableQuery for ConsistencyExecutor {
    fn execute(self: Box<Self>) -> QueryResult {
        let res = match self.recipe.compile(self.dim) {
            Ok(system) => match system.precheck_sys_rep() {
                PrecheckResult::Success => QueryResult::Consistency(ConsistencyResult::Success),
                PrecheckResult::NotDeterministic(location, action) => {
                    QueryResult::Consistency(ConsistencyResult::Failure(
                        ConsistencyFailure::NotDeterministicFrom(location, action),
                    ))
                }
                PrecheckResult::NotConsistent(failure) => {
                    QueryResult::Consistency(ConsistencyResult::Failure(failure))
                }
            },
            Err(error) => QueryResult::Error(error),
        };
        res
    }
}

pub struct DeterminismExecutor {
    pub system: TransitionSystemPtr,
}

impl ExecutableQuery for DeterminismExecutor {
    fn execute(self: Box<Self>) -> QueryResult {
        let is_deterministic = self.system.is_deterministic();

        QueryResult::Determinism(is_deterministic)
    }
}
