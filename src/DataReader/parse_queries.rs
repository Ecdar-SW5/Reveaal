extern crate pest;

use crate::ModelObjects::queries::Query;
use crate::ModelObjects::representations::{BoolExpression, QueryExpression};

use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

use super::parse_invariant::parse;

#[derive(Parser)]
#[grammar = "DataReader/grammars/query_grammar.pest"]
pub struct QueryParser;

///This file handles parsing the queries based on the abstract syntax described in the .pest files in the grammar folder
///For clarification see documentation on pest crate

pub fn parse_to_query(query: &str) -> Vec<Query> {
    let queries = parse_to_expression_tree(query);
    queries
        .into_iter()
        .map(|q| Query {
            query: Option::from(q),
            comment: "".to_string(),
        })
        .collect()
}

pub fn parse_to_expression_tree(edge_attribute_str: &str) -> Vec<QueryExpression> {
    let mut pairs = QueryParser::parse(Rule::queries, edge_attribute_str)
        .unwrap_or_else(|e| panic!("Could not parse as rule with error: {}", e));
    let pair = pairs.next().unwrap();
    let mut queries = vec![];
    match pair.as_rule() {
        Rule::queries => {
            build_queries(pair, &mut queries);
            queries
        }
        err => {
            panic!("Unable to match query string as rule: {:?}", err)
        }
    }
}

pub fn build_queries(pair: pest::iterators::Pair<Rule>, list: &mut Vec<QueryExpression>) {
    match pair.as_rule() {
        Rule::queryList => {
            for p in pair.into_inner() {
                build_queries(p, list)
            }
        }
        Rule::queries => {
            for p in pair.into_inner() {
                build_queries(p, list)
            }
        }
        Rule::query => {
            list.push(build_query_from_pair(pair));
        }
        _ => {}
    }
}

pub fn build_query_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    let pair = pair.into_inner().next().unwrap();
    let pair_span = pair.as_span();

    //check if we have an empty pair
    if pair_span.start() == pair_span.end() {
        return QueryExpression::Bool(true);
    }

    match pair.as_rule() {
        Rule::refinement => build_refinement_from_pair(pair),
        Rule::reachability => build_reachability_from_pair(pair),
        Rule::getComponent => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::GetComponent(Box::new(build_expression_from_pair(inner_pair)))
        }
        Rule::prune => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Prune(Box::new(build_expression_from_pair(inner_pair)))
        }
        Rule::bisim => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::BisimMinimize(Box::new(build_expression_from_pair(inner_pair)))
        }
        Rule::consistency => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Consistency(Box::new(build_expression_from_pair(inner_pair)))
        }
        Rule::implementation => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Implementation(Box::new(build_expression_from_pair(inner_pair)))
        }
        Rule::determinism => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Determinism(Box::new(build_expression_from_pair(inner_pair)))
        }
        Rule::specification => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Specification(Box::new(build_expression_from_pair(inner_pair)))
        }
        Rule::logicFormulas => {
            let inner_pair = pair.into_inner().next().unwrap();
            build_expression_from_pair(inner_pair)
        }
        unknown => panic!("Got unknown pair: {:?}", unknown),
    }
}

fn build_expression_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    match pair.as_rule() {
        Rule::term => build_term_from_pair(pair),
        Rule::parenthesizedExp => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Parentheses(Box::new(build_expression_from_pair(inner_pair)))
        }
        Rule::possibly => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Possibly(Box::new(build_boolExpr_from_pair(inner_pair)))
        }
        Rule::invariantly => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Invariantly(Box::new(build_boolExpr_from_pair(inner_pair)))
        }
        Rule::eventuallyAlways => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::EventuallyAlways(Box::new(build_boolExpr_from_pair(inner_pair)))
        }
        Rule::potentially => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Potentially(Box::new(build_boolExpr_from_pair(inner_pair)))
        }
        Rule::expr => {
            //Set precedence in ascending order
            let precedence_climber = PrecClimber::new(vec![
                Operator::new(Rule::qoutient_op, Assoc::Left),
                Operator::new(Rule::composition_op, Assoc::Left),
                Operator::new(Rule::conjunction_op, Assoc::Left),
            ]);
            let primary = build_expression_from_pair;

            precedence_climber.climb(pair.into_inner(), primary, |lhs, op, rhs| {
                match op.as_rule() {
                    Rule::composition_op => {
                        QueryExpression::Composition(Box::new(lhs), Box::new(rhs))
                    }
                    Rule::conjunction_op => {
                        QueryExpression::Conjunction(Box::new(lhs), Box::new(rhs))
                    }
                    Rule::qoutient_op => QueryExpression::Quotient(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                }
            })
        }
        Rule::saveExpr => {
            let mut inner: Vec<pest::iterators::Pair<Rule>> = pair.into_inner().collect();
            let name = inner.pop().unwrap();
            let inner_pair = inner.pop().unwrap();
            let name = build_var_from_pair(name);
            match name {
                QueryExpression::VarName(save_name) => QueryExpression::SaveAs(
                    Box::new(build_expression_from_pair(inner_pair)),
                    save_name,
                ),
                _ => panic!("Could not parse save-as name"),
            }
        }
        Rule::terms => {
            let inner_pair = pair.into_inner().next().unwrap();
            build_expression_from_pair(inner_pair)
        }
        unknown => panic!("Got unknown pair: {:?}", unknown),
    }
}

fn build_state_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    let mut inner_pair = pair.clone().into_inner();
    let locPair = inner_pair.next().unwrap();

    let mut loc_names: Vec<Box<QueryExpression>> = Vec::new();
    for loc_name in locPair.as_str().split(',') {
        loc_names.push(Box::new(QueryExpression::LocName(
            loc_name.trim().to_string(),
        )));
    }

    let clock_pair = inner_pair.next().unwrap();

    // In the following line of code, we build a BoolExprssion based on the clock constraints defined for the given location.
    // To make BoolExprssion we use the InvariantParser parser instead.
    // Becuase clocks is defined as c1&&c2... in the InvariantParser we replace ',' to match the format e.g., e.g., "x>0,y<5" => "x>0&&y<5"
    let invariant_version: Option<Box<BoolExpression>> = if clock_pair.as_str().trim() != "" {
        let clock_string = clock_pair.as_str().trim().to_string().replace(',', "&&");
        let invariant_version = parse(&clock_string).expect("");
        Some(Box::new(invariant_version))
    } else {
        None
    };

    match pair.as_rule() {
        Rule::state => QueryExpression::State(loc_names, invariant_version),
        err => panic!("Unable to match: {:?} as rule loc or clocks", err),
    }
}

fn build_reachability_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    let mut inner_pair = pair.into_inner();
    let automata_pair = inner_pair.next().unwrap();
    let start_state_pair = inner_pair.next().unwrap();
    let end_state_pair = inner_pair.next().unwrap();

    let automata = build_expression_from_pair(automata_pair);
    let start_state = build_state_from_pair(start_state_pair);
    let end_state = build_state_from_pair(end_state_pair);

    QueryExpression::Reachability(
        Box::new(automata),
        Box::new(start_state),
        Box::new(end_state),
    )
}

fn build_refinement_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();
    let right_side_pair = inner_pair.next().unwrap();

    let lside = build_expression_from_pair(left_side_pair);
    let rside = build_expression_from_pair(right_side_pair);

    QueryExpression::Refinement(Box::new(lside), Box::new(rside))
}

fn build_term_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    let inner_pair = pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::atom => {
            if let Ok(n) = inner_pair.as_str().trim().parse::<bool>() {
                QueryExpression::Bool(n)
            } else if let Ok(n) = inner_pair.as_str().trim().parse::<i32>() {
                QueryExpression::Int(n)
            } else {
                build_term_from_pair(inner_pair)
            }
        }
        Rule::var => build_var_from_pair(inner_pair),
        err => panic!("Unable to match: {:?} as rule atom or variable", err),
    }
}

fn build_var_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();

    let lside = QueryExpression::VarName(left_side_pair.as_str().trim().to_string());

    match inner_pair.next() {
        None => lside,
        Some(right_side_pair) => {
            let inner_right_pair = right_side_pair.into_inner().next().unwrap();
            let rside = build_expression_from_pair(inner_right_pair);

            QueryExpression::ComponentExpression(Box::new(lside), Box::new(rside))
        }
    }
}

fn build_boolExpr_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    match pair.as_rule() {
        Rule::boolExpr => {
            let inner_pair = pair.into_inner().next().unwrap();
            build_boolExpr_from_pair(inner_pair)
        }
        Rule::andExpr => build_and_from_pair(pair),
        Rule::orExpr => build_or_from_pair(pair),
        Rule::compExpr => build_compareExpr_from_pair(pair),
        Rule::subExpr => build_subExpression_from_pair(pair),
        unknown => panic!("Expected and pair but got unknown pair: {:?}", unknown),
    }
}

fn build_subExpression_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    match pair.as_rule() {
        Rule::term => build_term_from_pair(pair),
        Rule::parenthesizedSubExp => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Parentheses(Box::new(build_subExpression_from_pair(inner_pair)))
        }
        Rule::notExpr => {
            let inner_pair = pair.into_inner().next().unwrap();
            QueryExpression::Not(Box::new(build_subExpression_from_pair(inner_pair)))
        }
        Rule::subExpr => {
            let inner_pair = pair.into_inner().next().unwrap();
            build_subExpression_from_pair(inner_pair)
        }
        Rule::boolExpr => build_boolExpr_from_pair(pair),
        unknown => panic!("Got unknown pair: {:?}", unknown),
    }
}

fn build_and_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();

    match inner_pair.next() {
        None => build_or_from_pair(left_side_pair),
        Some(right_side_pair) => {
            let lside = build_boolExpr_from_pair(left_side_pair);
            let rside = build_boolExpr_from_pair(right_side_pair);

            QueryExpression::AndOp(Box::new(lside), Box::new(rside))
        }
    }
}

fn build_or_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();

    match inner_pair.next() {
        None => build_compareExpr_from_pair(left_side_pair),
        Some(right_side_pair) => {
            let lside = build_boolExpr_from_pair(left_side_pair);
            let rside = build_boolExpr_from_pair(right_side_pair);

            QueryExpression::OrOp(Box::new(lside), Box::new(rside))
        }
    }
}

fn build_compareExpr_from_pair(pair: pest::iterators::Pair<Rule>) -> QueryExpression {
    let mut inner_pair = pair.into_inner();
    let left_side_pair = inner_pair.next().unwrap();

    match inner_pair.next() {
        None => build_subExpression_from_pair(left_side_pair),
        Some(operator_pair) => {
            let right_side_pair = inner_pair.next().unwrap();

            let lside = build_boolExpr_from_pair(left_side_pair);
            let rside = build_boolExpr_from_pair(right_side_pair);

            match operator_pair.as_str() {
                ">=" => QueryExpression::GreatEQ(Box::new(lside), Box::new(rside)),
                "<=" => QueryExpression::LessEQ(Box::new(lside), Box::new(rside)),
                "<" => QueryExpression::LessT(Box::new(lside), Box::new(rside)),
                ">" => QueryExpression::GreatT(Box::new(lside), Box::new(rside)),
                unknown_operator => panic!(
                    "Got unknown boolean operator: {}. Only able to match >=,<=,<,>",
                    unknown_operator
                ),
            }
        }
    }
}
