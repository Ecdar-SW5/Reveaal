extern crate pest;

use crate::EdgeEval::updater::CompiledUpdate;

use crate::ModelObjects::representations::{ArithExpression, BoolExpression};

use crate::{DataReader::serialization::encode_boolexpr, ModelObjects::component::Declarations};
use edbm::util::constraints::ClockIndex;
use pest::error::Error;
use pest::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///This file handles parsing the edges based on the abstract syntax described in the .pest files in the grammar folder
///For clarification see documentation on pest crate
#[derive(Parser)]
#[grammar = "DataReader/grammars/edge_grammar.pest"]
pub struct EdgeParser;

#[derive(Debug, Clone, Deserialize)]
pub enum EdgeAttribute {
    Updates(Vec<Update>),
    Guard(BoolExpression),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Update {
    pub variable: String,
    #[serde(serialize_with = "encode_boolexpr")]
    pub expression: BoolExpression,
}

#[allow(dead_code)]
impl Update {
    pub fn get_expression(&self) -> &BoolExpression {
        &self.expression
    }

    pub fn get_variable_name(&self) -> &str {
        self.variable.as_str()
    }

    pub fn swap_var_name(&mut self, from_name: &str, to_name: &str) {
        if self.variable == from_name {
            self.variable = to_name.to_string();
        }

        self.expression.swap_var_name(from_name, to_name);
    }

    pub fn swap_clock_names(
        &mut self,
        from_vars: &HashMap<String, ClockIndex>,
        to_vars: &HashMap<ClockIndex, String>,
    ) {
        if let Some(index) = from_vars.get(&self.variable) {
            self.variable = to_vars[index].clone();
            self.expression = self.expression.swap_clock_names(from_vars, to_vars);
        }
    }

    pub fn compiled(&self, decl: &Declarations) -> CompiledUpdate {
        CompiledUpdate::compile(self, decl)
    }
}

pub fn parse(edge_attribute_str: &str) -> Result<EdgeAttribute, Error<Rule>> {
    let mut pairs = EdgeParser::parse(Rule::edgeAttribute, edge_attribute_str)
        .unwrap_or_else(|e| panic!("Could not parse as rule with error: {}", e));
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::edgeAttribute => Ok(build_edgeAttribute_from_pair(pair)),
        err => {
            panic!("Unable to match edgeAttribute string as rule: {:?}", err)
        }
    }
}

pub fn build_edgeAttribute_from_pair(pair: pest::iterators::Pair<Rule>) -> EdgeAttribute {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::update => build_update_from_pair(pair),
        Rule::guard => build_guard_from_pair(pair),
        err => {
            panic!("Unable to match update string as rule: {:?}", err)
        }
    }
}

fn build_guard_from_pair(pair: pest::iterators::Pair<Rule>) -> EdgeAttribute {
    match pair.as_rule() {
        Rule::guard => {
            let pair_span = pair.as_span();

            //check if we have an empty pair
            if pair_span.start() == pair_span.end() {
                return EdgeAttribute::Guard(BoolExpression::Bool(true));
            }

            let mut inner_pairs = pair.into_inner();
            let inner_pair = inner_pairs.next().unwrap();

            EdgeAttribute::Guard(build_expression_from_pair(inner_pair))
        }
        _ => panic!("Unable to match: {:?} as rule, guard", pair),
    }
}

fn build_update_from_pair(pair: pest::iterators::Pair<Rule>) -> EdgeAttribute {
    match pair.as_rule() {
        Rule::update => {
            let mut updates: Vec<Update> = vec![];
            let pair_span = pair.as_span();

            //check if we have an empty pair
            if pair_span.start() == pair_span.end() {
                return EdgeAttribute::Updates(updates);
            }

            let mut inner_pairs = pair.into_inner();
            let inner_pair = inner_pairs.next().unwrap();

            updates = build_assignments_from_pair(inner_pair);

            EdgeAttribute::Updates(updates)
        }
        _ => panic!("Unable to match: {:?} as rule, update", pair),
    }
}

fn build_assignments_from_pair(pair: pest::iterators::Pair<Rule>) -> Vec<Update> {
    let mut updates: Vec<Update> = vec![];
    match pair.as_rule() {
        Rule::assignments => {
            let mut pairs = pair.into_inner();
            let assignment_pair = pairs.next().unwrap();
            match assignment_pair.as_rule() {
                Rule::assignment => {
                    updates = build_assignments_from_pair(pairs.next().unwrap());
                    updates.push(build_assignment_from_pair(assignment_pair));
                }
                Rule::finalAssignment => {
                    updates.push(build_assignment_from_pair(assignment_pair));
                }
                err => panic!(
                    "Unable to match: {:?} as rule assignment or finalAssignment",
                    err
                ),
            }
        }
        unknown_pair => panic!(
            "Tried to match pair as assignment, but it was {:?}",
            unknown_pair
        ),
    }

    updates
}

fn build_assignment_from_pair(pair: pest::iterators::Pair<Rule>) -> Update {
    let mut inner_pairs = pair.into_inner();
    let variable = inner_pairs.next().unwrap().as_str();
    let expression_pair = inner_pairs.next().unwrap();

    let expression = build_expression_from_pair(expression_pair);
    let update = Update {
        variable: variable.trim().to_string(),
        expression,
    };

    update
}

fn build_expression_from_pair(pair: pest::iterators::Pair<Rule>) -> BoolExpression {
    match pair.as_rule() {
        Rule::parenthesizedExp => {
            let inner_pair = pair.into_inner().next().unwrap();
            BoolExpression::Parentheses(Box::new(build_expression_from_pair(inner_pair)))
        }
        Rule::expression => build_expression_from_pair(pair.into_inner().next().unwrap()),
        Rule::and => build_and_from_pair(pair),
        Rule::or => build_or_from_pair(pair),
        Rule::compareExpr => build_compareExpr_from_pair(pair),
        Rule::terms => build_expression_from_pair(pair.into_inner().next().unwrap()),
        Rule::term => BoolExpression::Arithmetic(Box::new(build_term_from_pair(pair))),
        unknown => panic!("Got unknown pair: {:?}", unknown),
    }
}

fn build_arithmetic_expression_from_pair(pair: pest::iterators::Pair<Rule>) -> ArithExpression {
    match pair.as_rule() {
        Rule::expression => {
            build_arithmetic_expression_from_pair(pair.into_inner().next().unwrap())
        }
        Rule::term => build_term_from_pair(pair),
        Rule::sub_add => build_sub_add_from_pair(pair),
        Rule::mult_div_mod => build_mult_div_mod_from_pair(pair),
        Rule::terms => build_arithmetic_expression_from_pair(pair.into_inner().next().unwrap()),
        unknown => panic!(
            "Got unknown pair: {:?} with string {:?}",
            unknown,
            pair.as_str()
        ),
    }
}

fn build_term_from_pair(pair: pest::iterators::Pair<Rule>) -> ArithExpression {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::int => {
            if let Ok(n) = inner_pair.as_str().trim().parse::<i32>() {
                ArithExpression::Int(n)
            } else {
                build_term_from_pair(inner_pair)
            }
        }
        Rule::variable => ArithExpression::VarName(inner_pair.as_str().trim().to_string()),
        err => panic!("Unable to match: {:?} as rule atom or variable", err),
    }
}

fn build_and_from_pair(pair: pest::iterators::Pair<Rule>) -> BoolExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();

    match inner_pair.next() {
        None => build_or_from_pair(left_side_pair),
        Some(right_side_pair) => {
            let lside = build_or_from_pair(left_side_pair);
            let rside = build_and_from_pair(right_side_pair);

            BoolExpression::AndOp(Box::new(lside), Box::new(rside))
        }
    }
}

fn build_or_from_pair(pair: pest::iterators::Pair<Rule>) -> BoolExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();

    match inner_pair.next() {
        None => build_compareExpr_from_pair(left_side_pair),
        Some(right_side_pair) => {
            let lside = build_compareExpr_from_pair(left_side_pair);
            let rside = build_or_from_pair(right_side_pair);

            BoolExpression::OrOp(Box::new(lside), Box::new(rside))
        }
    }
}

fn build_compareExpr_from_pair(pair: pest::iterators::Pair<Rule>) -> BoolExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();

    match inner_pair.next() {
        None => match left_side_pair.as_rule() {
            Rule::bool => {
                BoolExpression::Bool(left_side_pair.as_str().trim().parse::<bool>().unwrap())
            }
            Rule::terms => build_expression_from_pair(left_side_pair),
            err => panic!("Unable to match: {:?} as rule atom or variable", err),
        },
        Some(operator) => {
            let lhs = build_sub_add_from_pair(left_side_pair);
            let rhs = build_sub_add_from_pair(inner_pair.next().unwrap());

            match operator.as_str() {
                ">=" => BoolExpression::GreatEQ(Box::new(lhs), Box::new(rhs)),
                "<=" => BoolExpression::LessEQ(Box::new(lhs), Box::new(rhs)),
                "==" => BoolExpression::EQ(Box::new(lhs), Box::new(rhs)),
                ">" => BoolExpression::GreatT(Box::new(lhs), Box::new(rhs)),
                "<" => BoolExpression::LessT(Box::new(lhs), Box::new(rhs)),
                unknown_operator => panic!(
                    "Got unknown boolean operator: {}. Only able to match >=,<=, ==,<,>",
                    unknown_operator
                ),
            }
        }
    }
}

fn build_sub_add_from_pair(pair: pest::iterators::Pair<Rule>) -> ArithExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();

    match inner_pair.next() {
        None => build_mult_div_mod_from_pair(left_side_pair),
        Some(operator) => {
            let right_side_pair = inner_pair.next().unwrap();

            let lside = build_mult_div_mod_from_pair(left_side_pair);
            let rside = build_sub_add_from_pair(right_side_pair);
            match operator.as_str() {
                "-" => ArithExpression::Difference(Box::new(lside), Box::new(rside)),
                "+" => ArithExpression::Addition(Box::new(lside), Box::new(rside)),
                unknown_operator => panic!(
                    "Got unknown boolean operator: {}. Only able to match -,+",
                    unknown_operator
                ),
            }
        }
    }
}

fn build_mult_div_mod_from_pair(pair: pest::iterators::Pair<Rule>) -> ArithExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();

    match inner_pair.next() {
        None => build_arithmetic_expression_from_pair(left_side_pair),
        Some(operator) => {
            let right_side_pair = inner_pair.next().unwrap();

            let lside = build_arithmetic_expression_from_pair(left_side_pair);
            let rside = build_mult_div_mod_from_pair(right_side_pair);
            match operator.as_str() {
                "*" => ArithExpression::Multiplication(Box::new(lside), Box::new(rside)),
                "/" => ArithExpression::Division(Box::new(lside), Box::new(rside)),
                "%" => ArithExpression::Modulo(Box::new(lside), Box::new(rside)),
                unknown_operator => panic!(
                    "Got unknown boolean operator: {}. Only able to match /,*,%",
                    unknown_operator
                ),
            }
        }
    }
}
