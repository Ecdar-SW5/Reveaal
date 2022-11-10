use std::{
    fmt::{Display, Formatter},
    vec,
};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum TransitionID {
    Conjunction(Box<TransitionID>, Box<TransitionID>),
    Composition(Box<TransitionID>, Box<TransitionID>),
    Quotient(i32, Vec<TransitionID>, Vec<TransitionID>),
    Simple(String),
    None,
}

impl TransitionID {
    pub fn get_leaves(&self) -> Vec<TransitionID> {
        let mut result = Vec::new();
        self.get_leaves_helper(&mut result);
        result
    }

    fn get_leaves_helper(&self, current_leaves: &mut Vec<TransitionID>) {
        match self {
            TransitionID::Conjunction(l, r) => {
                l.get_leaves_helper(current_leaves);
                r.get_leaves_helper(current_leaves);
            }
            TransitionID::Composition(l, r) => {
                l.get_leaves_helper(current_leaves);
                r.get_leaves_helper(current_leaves);
            }
            TransitionID::Quotient(_, l, r) => {
                current_leaves.push(self.clone());
            }
            TransitionID::Simple(_) => {
                current_leaves.push(self.clone());
            }
            TransitionID::None => {
                current_leaves.push(self.clone());
            }
        };
    }

    pub fn split_into_component_lists(path: &Vec<TransitionID>) -> Vec<Vec<TransitionID>> {
        let count: usize = Self::count_leaves(path[0].clone());

        let mut paths: Vec<Vec<TransitionID>> = vec![Vec::new(); count];

        for id in path {
            for (i, subId) in id.get_leaves().iter().enumerate() {
                paths[i].push(subId.clone());
            }
        }
        paths
    }
    fn count_leaves(transition_id: TransitionID) -> usize {
        match transition_id {
            TransitionID::Conjunction(l, r) => Self::count_leaves(*l) + Self::count_leaves(*r),
            TransitionID::Composition(l, r) => Self::count_leaves(*l) + Self::count_leaves(*r),
            TransitionID::Quotient(_, l, r) => 1,
            TransitionID::Simple(_) => 1,
            TransitionID::None => 1,
        }
    }
}

impl Display for TransitionID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionID::Conjunction(left, right) => {
                match *(*left) {
                    TransitionID::Conjunction(_, _) => write!(f, "{}", (*left))?,
                    TransitionID::Simple(_) => write!(f, "{}", (*left))?,
                    _ => write!(f, "({})", (*left))?,
                };
                write!(f, "&&")?;
                match *(*right) {
                    TransitionID::Conjunction(_, _) => write!(f, "{}", (*right))?,
                    TransitionID::Simple(_) => write!(f, "{}", (*right))?,
                    _ => write!(f, "({})", (*right))?,
                };
            }
            TransitionID::Composition(left, right) => {
                match *(*left) {
                    TransitionID::Composition(_, _) => write!(f, "{}", (*left))?,
                    TransitionID::Simple(_) => write!(f, "{}", (*left))?,
                    _ => write!(f, "({})", (*left))?,
                };
                write!(f, "||")?;
                match *(*right) {
                    TransitionID::Composition(_, _) => write!(f, "{}", (*right))?,
                    TransitionID::Simple(_) => write!(f, "{}", (*right))?,
                    _ => write!(f, "({})", (*right))?,
                };
            }
            TransitionID::Quotient(ruleNr, left, right) => {
                write!(f, "{}|", ruleNr)?;
                for l in left {
                    match *(l) {
                        TransitionID::Simple(_) => write!(f, "{}", (l))?,
                        _ => write!(f, "({})", (l))?,
                    };
                }
                write!(f, "\\\\")?;
                for r in right {
                    match *(r) {
                        TransitionID::Simple(_) => write!(f, "{}", (r))?,
                        _ => write!(f, "({})", (r))?,
                    };
                }
            }
            TransitionID::Simple(name) => write!(f, "{}", name)?,
            TransitionID::None => write!(f, "NoID")?,
        }
        Ok(())
    }
}
