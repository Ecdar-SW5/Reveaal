use crate::DataReader::component_loader::ComponentLoader;
use crate::ModelObjects::component::Component;
use crate::ModelObjects::queries::Query;
use crate::ModelObjects::representations::QueryExpression;
use crate::System::executable_query::{
    ConsistencyExecutor, DeterminismExecutor, ExecutableQuery, GetComponentExecutor,
    ReachabilityExecutor, RefinementExecutor,
};
use crate::System::extract_state::get_state;
use std::cmp::max;
use std::collections::HashMap;

use crate::TransitionSystems::{
    CompiledComponent, Composition, Conjunction, Quotient, TransitionSystemPtr,
};

use crate::component::State;
use crate::ProtobufServer::services::query_request::settings::ReduceClocksLevel;
use crate::System::pruning;
use crate::TransitionSystems::transition_system::Heights;
use edbm::util::constraints::ClockIndex;
use log::debug;
use simple_error::bail;
use std::error::Error;

/// This function fetches the appropriate components based on the structure of the query and makes the enum structure match the query
/// this function also handles setting up the correct indices for clocks based on the amount of components in each system representation
pub fn create_executable_query<'a>(
    full_query: &Query,
    component_loader: &'a mut (dyn ComponentLoader + 'static),
) -> Result<Box<dyn ExecutableQuery + 'a>, Box<dyn Error>> {
    let mut dim: ClockIndex = 0;

    if let Some(query) = full_query.get_query() {
        let mut clock: HashMap<String, ClockIndex> = HashMap::new();
        //clock.insert("y".to_owned(), 1);
        let clock_replacement: Option<HashMap<String, ClockIndex>> = Some(clock);
        match query {
            QueryExpression::Refinement(left_side, right_side) => {
                let mut quotient_index = None;
                let left = get_system_recipe(left_side, component_loader, &mut dim, &mut quotient_index);
                let right = get_system_recipe(right_side, component_loader, &mut dim, &mut quotient_index);
                let height = max(left.height(), right.height()) + 1;
                let mut compiled_left = left.compile(dim, &clock_replacement)?;
                let mut compiled_right = right.compile(dim, &clock_replacement)?;
                if let Some(x) = &component_loader.get_settings().reduce_clocks_level {
                    match x {
                        ReduceClocksLevel::Level(y) if *y >= 0 => {
                            let heights = Heights::new(height, (*y) as usize);
                            compiled_left.reduce_clocks(vec![],heights);
                            compiled_right.reduce_clocks(vec![],heights);
                        },
                        ReduceClocksLevel::All(true) =>{
                            let heights = Heights::new(height, height);
                            compiled_left.reduce_clocks(vec![],heights);
                            compiled_right.reduce_clocks(vec![],heights);
                        },
                        _ => (),
                    };
                }
                Ok(Box::new(RefinementExecutor {
                sys1: compiled_left,
                sys2: compiled_right,
            }))},
            QueryExpression::Reachability(automata, start, end) => {
                let machine = get_system_recipe(automata, component_loader, &mut dim, &mut None);
                let transition_system = machine.clone().compile(dim, &clock_replacement)?;

                validate_reachability_input(&machine, end)?;

                let start_state: State = if let Some(state) = &**start {
                    validate_reachability_input(&machine, state)?;
                    let state = get_state(state, &machine, &transition_system)?;
                    if state.get_location().id.is_partial_location() {
                        return Err("Start state is a partial state, which it must not be".into())
                    }
                    state
                }
                else {
                    match transition_system.get_initial_state() {
                        Some(state)=> state,
                        None => return Err("No start state in the transition system".into())
                    }
                };

                let end_state: State = get_state(end, &machine, &transition_system)?;

                Ok(Box::new(ReachabilityExecutor {
                    transition_system,
                    start_state,
                    end_state,
                }))
            },
            QueryExpression::Consistency(query_expression) => Ok(Box::new(ConsistencyExecutor {
                recipe: get_system_recipe(
                    query_expression,
                    component_loader,
                    &mut dim,
                    &mut None
                ),
                dim
            })),
            QueryExpression::Determinism(query_expression) => {
                Ok(Box::new(DeterminismExecutor {
                    system: get_system_recipe(
                        query_expression,
                        component_loader,
                        &mut dim,
                        &mut None,
                    ).compile(dim, &clock_replacement)?,
                }))
            },
            QueryExpression::GetComponent(save_as_expression) => {
                if let QueryExpression::SaveAs(query_expression, comp_name) = save_as_expression.as_ref() {
                    Ok(Box::new(
                        GetComponentExecutor {
                            system: get_system_recipe(query_expression, component_loader, &mut dim, &mut None).compile(dim, &clock_replacement)?,
                            comp_name: comp_name.clone(),
                            component_loader,
                        }
                    ))
                }else{
                    bail!("Unexpected expression type")
                }
            }
            ,
            QueryExpression::Prune(save_as_expression) => {
                if let QueryExpression::SaveAs(query_expression, comp_name) = save_as_expression.as_ref() {
                    Ok(Box::new(
                        GetComponentExecutor {
                            system: pruning::prune_system(get_system_recipe(query_expression, component_loader, &mut dim, &mut None).compile(dim, &clock_replacement)?, dim),
                            comp_name: comp_name.clone(),
                            component_loader
                        }
                    ))
                }else{
                    bail!("Unexpected expression type")
                }
            }
            ,
            // Should handle consistency, Implementation, determinism and specification here, but we cant deal with it atm anyway
            _ => bail!("Not yet setup to handle {:?}", query),
        }
    } else {
        bail!("No query was supplied for extraction")
    }
}

#[derive(Clone)]
pub enum SystemRecipe {
    Composition(Box<SystemRecipe>, Box<SystemRecipe>),
    Conjunction(Box<SystemRecipe>, Box<SystemRecipe>),
    Quotient(Box<SystemRecipe>, Box<SystemRecipe>, ClockIndex),
    Component(Box<Component>),
}

impl SystemRecipe {
    pub fn compile(
        self,
        dim: ClockIndex,
        clock_replacement: &Option<HashMap<String, ClockIndex>>,
    ) -> Result<TransitionSystemPtr, String> {
        match self {
            SystemRecipe::Composition(left, right) => Composition::new(
                left.compile(dim, clock_replacement)?,
                right.compile(dim, clock_replacement)?,
                dim + 1,
            ),
            SystemRecipe::Conjunction(left, right) => Conjunction::new(
                left.compile(dim, clock_replacement)?,
                right.compile(dim, clock_replacement)?,
                dim + 1,
            ),
            SystemRecipe::Quotient(left, right, clock_index) => Quotient::new(
                left.compile(dim, clock_replacement)?,
                right.compile(dim, clock_replacement)?,
                clock_index,
                dim + 1,
            ),
            SystemRecipe::Component(comp) => {
                match CompiledComponent::compile(*comp, dim + 1, clock_replacement) {
                    Ok(comp) => Ok(comp),
                    Err(err) => Err(err),
                }
            }
        }
    }
    pub fn height(&self) -> usize {
        match self {
            SystemRecipe::Composition(l, r)
            | SystemRecipe::Conjunction(l, r)
            | SystemRecipe::Quotient(l, r, _) => max(l.height(), r.height()),
            SystemRecipe::Component(_) => 1,
        }
    }
    pub fn count_component(&self) -> usize {
        match self {
            SystemRecipe::Composition(left, right)
            | SystemRecipe::Conjunction(left, right)
            | SystemRecipe::Quotient(left, right, _) => {
                left.count_component() + right.count_component()
            }
            SystemRecipe::Component(_) => 1,
        }
    }
}

pub fn get_system_recipe(
    side: &QueryExpression,
    component_loader: &mut dyn ComponentLoader,
    clock_index: &mut ClockIndex,
    quotient_index: &mut Option<ClockIndex>,
) -> Box<SystemRecipe> {
    match side {
        QueryExpression::Parentheses(expression) => {
            get_system_recipe(expression, component_loader, clock_index, quotient_index)
        }
        QueryExpression::Composition(left, right) => Box::new(SystemRecipe::Composition(
            get_system_recipe(left, component_loader, clock_index, quotient_index),
            get_system_recipe(right, component_loader, clock_index, quotient_index),
        )),
        QueryExpression::Conjunction(left, right) => Box::new(SystemRecipe::Conjunction(
            get_system_recipe(left, component_loader, clock_index, quotient_index),
            get_system_recipe(right, component_loader, clock_index, quotient_index),
        )),
        QueryExpression::Quotient(left, right) => {
            let left = get_system_recipe(left, component_loader, clock_index, quotient_index);
            let right = get_system_recipe(right, component_loader, clock_index, quotient_index);

            let q_index = match quotient_index {
                Some(q_i) => *q_i,
                None => {
                    *clock_index += 1;
                    debug!("Quotient clock index: {}", *clock_index);

                    quotient_index.replace(*clock_index);
                    quotient_index.unwrap()
                }
            };

            Box::new(SystemRecipe::Quotient(left, right, q_index))
        }
        QueryExpression::VarName(name) => {
            let mut component = component_loader.get_component(name).clone();
            component.set_clock_indices(clock_index);
            debug!("{} Clocks: {:?}", name, component.declarations.clocks);

            Box::new(SystemRecipe::Component(Box::new(component)))
        }
        QueryExpression::SaveAs(comp, _) => {
            get_system_recipe(comp, component_loader, clock_index, &mut None)
        }
        _ => panic!("Got unexpected query side: {:?}", side),
    }
}

fn validate_reachability_input(
    machine: &SystemRecipe,
    state: &QueryExpression,
) -> Result<(), String> {
    if let QueryExpression::State(loc_names, _) = state {
        if loc_names.len() != machine.count_component() {
            return Err(
                "The number of automata does not match the number of locations".to_string(),
            );
        }
    } else {
        return Err(format!(
            "Expected QueryExpression::State but got {:?}",
            state
        ));
    }

    Ok(())
}
