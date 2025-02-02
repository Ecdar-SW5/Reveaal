use crate::DataReader::component_loader::ComponentLoader;
use crate::ModelObjects::component::Component;
use crate::ModelObjects::queries::Query;
use crate::ModelObjects::representations::QueryExpression;
use crate::System::executable_query::{
    ConsistencyExecutor, DeterminismExecutor, ExecutableQuery, GetComponentExecutor,
    ReachabilityExecutor, RefinementExecutor,
};
use crate::System::extract_state::get_state;
use std::collections::HashMap;

use crate::TransitionSystems::{
    CompiledComponent, Composition, Conjunction, Quotient, TransitionSystemPtr,
};

use crate::component::State;
use crate::System::pruning;
use crate::TransitionSystems::transition_system::ClockReductionInstruction;
use edbm::util::constraints::ClockIndex;
use log::debug;
use simple_error::bail;
use std::error::Error;

pub struct SystemRecipeFailure {
    pub reason: String,
    pub left_name: Option<String>,
    pub right_name: Option<String>,
    pub actions: Vec<String>,
}

impl From<SystemRecipeFailure> for String {
    fn from(failure: SystemRecipeFailure) -> Self {
        failure.reason
    }
}

impl std::fmt::Display for SystemRecipeFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SystemRecipeFailure: {}", self.reason)
    }
}

impl std::fmt::Debug for SystemRecipeFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SystemRecipeFailure: {}", self.reason)
    }
}
impl Error for SystemRecipeFailure {
    fn description(&self) -> &str {
        &self.reason
    }
}

impl SystemRecipeFailure {
    pub fn new_from_component(reason: String, component: Component, actions: Vec<String>) -> Self {
        SystemRecipeFailure {
            reason,
            left_name: Some(component.get_name().to_string()),
            right_name: None,
            actions,
        }
    }
    pub fn new(
        reason: String,
        left: TransitionSystemPtr,
        right: TransitionSystemPtr,
        actions: Vec<String>,
    ) -> Self {
        let mut left_name = None;
        let mut right_name = None;

        if let Some(location) = left.get_initial_location() {
            left_name = location.id.get_component_id()
        }
        if let Some(location) = right.get_initial_location() {
            right_name = location.id.get_component_id()
        }

        SystemRecipeFailure {
            reason,
            left_name,
            right_name,
            actions,
        }
    }
}

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

                if !component_loader.get_settings().disable_clock_reduction {
                    clock_reduction::clock_reduce(&mut left, Some(&mut right), &mut dim, quotient_index.is_some())?;
                }

                Ok(Box::new(RefinementExecutor {
                sys1: left.compile(dim)?,
                sys2: right.compile(dim)?,
            }))},
            QueryExpression::Reachability(automata, start, end) => {
                let machine = get_system_recipe(automata, component_loader, &mut dim, &mut None);
                let transition_system = machine.clone().compile(dim)?;

                validate_reachability_input(&machine, end)?;
                // Assign the start state to the initial state of the transition system if no start state is given by the query
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

                if !component_loader.get_settings().disable_clock_reduction {
                    clock_reduction::clock_reduce(&mut recipe, None, &mut dim, quotient_index.is_some())?;
                }

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

                if !component_loader.get_settings().disable_clock_reduction {
                    clock_reduction::clock_reduce(&mut recipe, None, &mut dim, quotient_index.is_some())?;
                }

                Ok(Box::new(DeterminismExecutor {
                    system: recipe.compile(dim)?,
                }))
            },
            QueryExpression::GetComponent(save_as_expression) => {
                if let QueryExpression::SaveAs(query_expression, comp_name) = save_as_expression.as_ref() {
                    let mut quotient_index = None;
                    let mut recipe = get_system_recipe(
                        query_expression,
                        component_loader,
                        &mut dim,
                        &mut quotient_index
                    );

                    if !component_loader.get_settings().disable_clock_reduction {
                        clock_reduction::clock_reduce(&mut recipe, None, &mut dim, quotient_index.is_some())?;
                    }

                    Ok(Box::new(
                        GetComponentExecutor {
                            system: recipe.compile(dim)?,
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

                    if !component_loader.get_settings().disable_clock_reduction {
                        clock_reduction::clock_reduce(&mut recipe, None, &mut dim, quotient_index.is_some())?;
                    }

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
    pub fn compile(self, dim: ClockIndex) -> Result<TransitionSystemPtr, SystemRecipeFailure> {
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
    fn reduce_clocks(&mut self, clock_instruction: Vec<ClockReductionInstruction>) {
        let mut comps = self.get_components();
        for redundant in clock_instruction {
            match redundant {
                ClockReductionInstruction::RemoveClock { clock_index } => comps
                    .iter_mut()
                    .find(|c| c.declarations.clocks.values().any(|ci| *ci == clock_index))
                    .unwrap_or_else(|| {
                        panic!(
                            "A component could not be found for clock index {}",
                            clock_index
                        )
                    })
                    .remove_clock(clock_index),
                ClockReductionInstruction::ReplaceClocks {
                    clock_indices,
                    clock_index,
                } => comps
                    .iter_mut()
                    .for_each(|c| c.replace_clock(clock_index, &clock_indices)),
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
            SystemRecipe::Composition(l, r) | SystemRecipe::Conjunction(l, r) => {
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

/// Module containing a "safer" function for clock reduction, along with some helper functions
pub(crate) mod clock_reduction {
    use super::*;

    /// Function for a "safer" clock reduction that handles both the dimension of the DBM and the quotient index if needed be
    /// # Arguments
    /// `lhs`: The (main) `SystemRecipe` to clock reduce\n
    /// `rhs`: An optional `SystemRecipe` used for multiple operands (Refinement)\n
    /// `dim`: A mutable reference to the DBMs dimension for updating\n
    /// `has_quotient`: A boolean to indicate if there is a quotient clock to update
    /// # Returns
    /// A `Result` used if the `SystemRecipe`(s) fail during compilation
    pub fn clock_reduce(
        lhs: &mut Box<SystemRecipe>,
        mut rhs: Option<&mut Box<SystemRecipe>>,
        dim: &mut usize,
        has_quotient: bool,
    ) -> Result<(), String> {
        let clocks = if let Some(ref mut r) = rhs {
            intersect(
                lhs.clone().compile(*dim)?.find_redundant_clocks(),
                r.clone().compile(*dim)?.find_redundant_clocks(),
            )
        } else {
            lhs.clone().compile(*dim)?.find_redundant_clocks()
        };

        debug!("Clocks to be reduced: {clocks:?}");
        *dim -= clocks
            .iter()
            .fold(0, |acc, c| acc + c.clocks_removed_count());
        debug!("New dimension: {dim}");
        if let Some(r) = rhs {
            r.reduce_clocks(clocks.clone());
            lhs.reduce_clocks(clocks);
            compress_component_decls(lhs.get_components(), Some(r.get_components()));
            if has_quotient {
                lhs.change_quotient(*dim);
                r.change_quotient(*dim);
            }
        } else {
            lhs.reduce_clocks(clocks);
            compress_component_decls(lhs.get_components(), None);
            if has_quotient {
                lhs.change_quotient(*dim);
            }
        }
        Ok(())
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
    ) {
        let mut seen: HashMap<ClockIndex, ClockIndex> = HashMap::new();
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
        let mut index = 1;
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
