pub mod test {
    use std::collections::HashSet;
    use std::ops::Deref;
    use crate::component::{ClockReductionReason, RedundantClock};
    use crate::ModelObjects::representations::{ArithExpression, BoolExpression};

    fn assert_duplicated_clock_detection(redundant_clocks: &Vec<RedundantClock>, expected_amount_to_reduce: u32, expected_duplicate_clocks: HashSet<&str>, unused_allowed: bool) {
        let mut global_clock: String = String::from("");

        let mut clocksReduced: HashSet<String> = HashSet::new();

        for redundancy in redundant_clocks {
            match &redundancy.reason {
                ClockReductionReason::Duplicate(replaced_by) => {
                    if global_clock == "" {
                        global_clock = replaced_by.clone();
                    }
                    assert!(expected_duplicate_clocks.contains(redundancy.clock.as_str()), "Clock ({}) was marked as duplicate unexpectedly", redundancy.clock);
                    assert!(!clocksReduced.contains(&redundancy.clock), "Clock ({}) was marked as duplicate multiple times", redundancy.clock);
                    assert_eq!(&global_clock, replaced_by, "Multiple clocks were chosen as global clock {} and {}", global_clock, replaced_by);
                    clocksReduced.insert(redundancy.clock.clone());
                }
                ClockReductionReason::Unused => {
                    assert!(unused_allowed, "Unexpected unused optimization");
                }
            }
        }
        assert_eq!(clocksReduced.len(), 2, "Too many clocks were reduced, expected only 2, got {}", clocksReduced.len());
    }

    fn get_dependent_clocks(expr: &BoolExpression, out: &mut HashSet<String>) {
        match expr.deref() {
            BoolExpression::Bool(_) => {},
            BoolExpression::Parentheses(op) => get_dependent_clocks(op, out),
            BoolExpression::Arithmetic(op) => get_dependent_clocks_arithmetic(op, out),

            BoolExpression::AndOp(lhs, rhs)
            | BoolExpression::OrOp(lhs, rhs) => {
                get_dependent_clocks(lhs, out);
                get_dependent_clocks(rhs, out);
            },

            BoolExpression::LessEQ(lhs, rhs)
            | BoolExpression::GreatEQ(lhs, rhs)
            | BoolExpression::LessT(lhs, rhs)
            | BoolExpression::GreatT(lhs, rhs)
            | BoolExpression::EQ(lhs, rhs) => {
                get_dependent_clocks_arithmetic(lhs, out);
                get_dependent_clocks_arithmetic(rhs, out);
            }
        }
    }

    fn get_dependent_clocks_arithmetic(expr: &ArithExpression, out: &mut HashSet<String>) {
        match expr.deref() {
            ArithExpression::Parentheses(op) => get_dependent_clocks_arithmetic(op, out),

            ArithExpression::Difference(lhs, rhs)
            | ArithExpression::Addition(lhs, rhs)
            | ArithExpression::Multiplication(lhs, rhs)
            | ArithExpression::Division(lhs, rhs)
            | ArithExpression::Modulo(lhs, rhs) => {
                get_dependent_clocks_arithmetic(lhs, out);
                get_dependent_clocks_arithmetic(rhs, out);
            },

            ArithExpression::Clock(index) => panic!("aaaaa"),
            ArithExpression::VarName(name) => {out.insert(name.clone());},
            ArithExpression::Int(i) => {}

        }
    }
}