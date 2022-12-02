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
use std::collections::{HashMap, HashSet};

use crate::TransitionSystems::{
    CompiledComponent, Composition, Conjunction, Quotient, TransitionSystemPtr,
};

use crate::component::State;
use crate::ProtobufServer::services::query_request::{settings::ReduceClocksLevel, Settings};
use crate::System::pruning;
use crate::TransitionSystems::transition_system::{ClockReductionInstruction, Heights};
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
        match query {
            QueryExpression::Refinement(left_side, right_side) => {
                let mut quotient_index = None;

                let mut left = get_system_recipe(left_side, component_loader, &mut dim, &mut quotient_index);
                let mut right = get_system_recipe(right_side, component_loader, &mut dim, &mut quotient_index);
                clock_reduction::clock_reduction_two_hands(&mut left, &mut right, component_loader.get_settings(), &mut dim, quotient_index.is_some())?;

                Ok(Box::new(RefinementExecutor {
                sys1: left.compile(dim)?,
                sys2: right.compile(dim)?,
            }))},
            QueryExpression::Reachability(automata, start, end) => {
                let machine = get_system_recipe(automata, component_loader, &mut dim, &mut None);
                let transition_system = machine.clone().compile(dim)?;

                validate_reachability_input(&machine, end)?;

                let start_state: State = if let Some(state) = start.as_ref() {
                    validate_reachability_input(&machine, state)?;
                    let state = get_state(state, &machine, &transition_system).map_err(|err| format!("Invalid Start state: {}",err))?;
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

                let end_state: State = get_state(end, &machine, &transition_system).map_err(|err| format!("Invalid End state: {}",err))?;

                Ok(Box::new(ReachabilityExecutor {
                    transition_system,
                    start_state,
                    end_state,
                }))
            },
            QueryExpression::Consistency(query_expression) => {
                let mut quotient_index = None;
                let mut recipe = get_system_recipe(
                    query_expression,
                    component_loader,
                    &mut dim,
                    &mut quotient_index
                );
                clock_reduction::clock_reduce(&mut recipe, component_loader.get_settings(), &mut dim, quotient_index.is_some())?;
                Ok(Box::new(ConsistencyExecutor {
                    recipe,
                    dim
                }))
            },
            QueryExpression::Determinism(query_expression) => {
                let mut quotient_index = None;
                let mut recipe = get_system_recipe(
                    query_expression,
                    component_loader,
                    &mut dim,
                    &mut quotient_index
                );
                clock_reduction::clock_reduce(&mut recipe, component_loader.get_settings(), &mut dim, quotient_index.is_some())?;
                Ok(Box::new(DeterminismExecutor {
                    system: recipe.compile(dim)?,
                }))
            },
            QueryExpression::GetComponent(save_as_expression) => {
                if let QueryExpression::SaveAs(query_expression, comp_name) = save_as_expression.as_ref() {
                    Ok(Box::new(
                        GetComponentExecutor {
                            system: get_system_recipe(query_expression, component_loader, &mut dim, &mut None).compile(dim)?,
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
                    let mut quotient_index = None;
                    let mut recipe = get_system_recipe(
                        query_expression,
                        component_loader,
                        &mut dim,
                        &mut quotient_index
                    );
                    clock_reduction::clock_reduce(&mut recipe, component_loader.get_settings(), &mut dim, quotient_index.is_some())?;
                    Ok(Box::new(
                        GetComponentExecutor {
                            system: pruning::prune_system(recipe.compile(dim)?, dim),
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
    pub fn compile(self, dim: ClockIndex) -> Result<TransitionSystemPtr, String> {
        match self {
            SystemRecipe::Composition(left, right) => {
                Composition::new(left.compile(dim)?, right.compile(dim)?, dim + 1)
            }
            SystemRecipe::Conjunction(left, right) => {
                Conjunction::new(left.compile(dim)?, right.compile(dim)?, dim + 1)
            }
            SystemRecipe::Quotient(left, right, clock_index) => Quotient::new(
                left.compile(dim)?,
                right.compile(dim)?,
                clock_index,
                dim + 1,
            ),
            SystemRecipe::Component(comp) => match CompiledComponent::compile(*comp, dim + 1) {
                Ok(comp) => Ok(comp),
                Err(err) => Err(err),
            },
        }
    }

    /// Gets the height of the `SystemRecipe` tree
    fn height(&self) -> usize {
        match self {
            SystemRecipe::Composition(l, r)
            | SystemRecipe::Conjunction(l, r)
            | SystemRecipe::Quotient(l, r, _) => 1 + max(l.height(), r.height()),
            SystemRecipe::Component(_) => 1,
        }
    }

    /// Gets the count `Components`s in the `SystemRecipe`
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

    ///Applies the clock-reduction
    pub(crate) fn reduce_clocks(&mut self, clock_instruction: Vec<ClockReductionInstruction>) {
        let mut comps = self.get_components();
        let mut omitting = HashSet::new();
        for redundant in clock_instruction {
            match redundant {
                ClockReductionInstruction::RemoveClock { clock_index } => {
                    match comps
                        .iter_mut()
                        .find(|c| c.declarations.clocks.values().any(|ci| *ci == clock_index))
                    {
                        Some(c) => c.remove_clock(clock_index),
                        None => continue,
                    }
                    omitting.insert((clock_index, 1));
                }
                ClockReductionInstruction::ReplaceClocks {
                    clock_indices,
                    clock_index,
                } => {
                    comps
                        .iter_mut()
                        .for_each(|c| c.replace_clock(clock_index, &clock_indices));
                    omitting.insert((clock_index, clock_indices.len()));
                }
            }
        }
    }

    /// Gets all components in `SystemRecipe`
    fn get_components(&mut self) -> Vec<&mut Component> {
        match self {
            SystemRecipe::Composition(left, right)
            | SystemRecipe::Conjunction(left, right)
            | SystemRecipe::Quotient(left, right, _) => {
                let mut o = left.get_components();
                o.extend(right.get_components());
                o
            }
            SystemRecipe::Component(c) => vec![c],
        }
    }

    fn change_quotient(&mut self, index: ClockIndex) {
        match self {
            SystemRecipe::Composition(l, r) |
            SystemRecipe::Conjunction(l, r) => {
                l.change_quotient(index);
                r.change_quotient(index);
            }
            SystemRecipe::Quotient(l, r, q) => {
                *q = index;
                l.change_quotient(index);
                r.change_quotient(index);
            }
            SystemRecipe::Component(_) => (),
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

mod clock_reduction {
    use super::*;

    pub fn clock_reduction_two_hands(
        lhs: &mut Box<SystemRecipe>,
        rhs: &mut Box<SystemRecipe>,
        settings: &Settings,
        dim: &mut usize,
        has_quotient: bool,
    ) -> Result<(), String> {
        let heights = match heights(
            &settings.reduce_clocks_level,
            max(lhs.height(), rhs.height()),
        )? {
            Some(h) => h,
            None => return Ok(()),
        };

        let clocks = intersect(
            lhs.clone().compile(*dim)?.find_redundant_clocks(heights),
            rhs.clone().compile(*dim)?.find_redundant_clocks(heights),
        );

        debug!("Clocks to be reduced: {clocks:?}");
        *dim -= clocks
            .iter()
            .fold(0, |acc, c| acc + c.clocks_removed_count());
        debug!("New dimension: {dim}");

        lhs.reduce_clocks(clocks.clone());
        rhs.reduce_clocks(clocks);
        compress_component_decls(lhs.get_components(), Some(rhs.get_components()));

        if has_quotient {
            lhs.change_quotient(*dim);
            rhs.change_quotient(*dim);
        }
        Ok(())
    }

    pub fn clock_reduce(
        sys: &mut Box<SystemRecipe>,
        settings: &Settings,
        dim: &mut usize,
        has_quotient: bool,
    ) -> Result<(), String> {
        let heights = match heights(&settings.reduce_clocks_level, sys.height())? {
            Some(h) => h,
            None => return Ok(()),
        };
        let clocks = sys.clone().compile(*dim)?.find_redundant_clocks(heights);

        debug!("Clocks to be reduced: {clocks:?}");
        *dim -= clocks
            .iter()
            .fold(0, |acc, c| acc + c.clocks_removed_count());
        debug!("New dimension: {dim}");

        sys.reduce_clocks(clocks);
        compress_component_decls(sys.get_components(), None);
        if has_quotient {
            sys.change_quotient(*dim);
        }
        Ok(())
    }

    fn heights(lvl: &Option<ReduceClocksLevel>, height: usize) -> Result<Option<Heights>, String> {
        match lvl.to_owned().ok_or_else(|| "No clock reduction level specified".to_string())? {
            ReduceClocksLevel::Level(y) if y >= 0 => Ok(Some(Heights::new(height, y as usize))),
            ReduceClocksLevel::All(true) => Ok(Some(Heights::new(height, height))),
            ReduceClocksLevel::All(false) => Ok(None),
            ReduceClocksLevel::Level(err) => Err(format!("Clock reduction error: Couldn't parse argument correctly. Got {err}, expected a value above")),
        }
    }

    fn intersect(
        lhs: Vec<ClockReductionInstruction>,
        rhs: Vec<ClockReductionInstruction>,
    ) -> Vec<ClockReductionInstruction> {
        lhs.iter()
            .filter(|r| r.is_replace())
            .chain(rhs.iter().filter(|r| r.is_replace()))
            .chain(
                lhs.iter()
                    .filter(|r| !r.is_replace())
                    .filter_map(|c| rhs.iter().filter(|r| !r.is_replace()).find(|rc| *rc == c)),
            )
            .cloned()
            .collect()
    }

    fn compress_component_decls(
        mut comps: Vec<&mut Component>,
        other: Option<Vec<&mut Component>>,
        //  omit: HashSet<(ClockIndex, usize)>,
    ) {
        let mut seen: HashMap<ClockIndex, ClockIndex> = HashMap::new(); // Key = current, value = Should be
        let mut l: Vec<&mut ClockIndex> = comps
            .iter_mut()
            .flat_map(|c| c.declarations.clocks.values_mut())
            .collect();
        let mut temp = other.unwrap_or_default();
        l.extend(
            temp.iter_mut()
                .flat_map(|c| c.declarations.clocks.values_mut()),
        );
        l.sort();
        let mut index = 1; //**l.first().unwrap_or_else(|| panic!("No clocks"));
        for clock in l {
            if let Some(val) = seen.get(clock) {
                *clock = *val;
            } else {
                seen.insert(*clock, index);
                *clock = index;
                index += 1;
            }
        }
    }
}
