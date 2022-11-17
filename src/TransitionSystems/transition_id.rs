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
    pub fn get_leaves(&self) -> Vec<Vec<TransitionID>> {
        let mut result = Vec::new();
        self.get_leaves_helper(&mut result, 0);
        result
    }

    fn get_leaves_helper(
        &self,
        current_leaves: &mut Vec<Vec<TransitionID>>,
        index: usize,
    ) -> (&TransitionID, usize) {
        match self {
            TransitionID::Conjunction(l, r) | TransitionID::Composition(l, r) => {
                let a = l.get_leaves_helper(current_leaves, index);
                let b = r.get_leaves_helper(current_leaves, a.1 + 1);
                (self, b.1)
            }
            TransitionID::Quotient(_, _l, _r) => {
                let mut curIndex = index;
                for t in _l {
                    (_, curIndex) = t.get_leaves_helper(current_leaves, index);
                }
                let mut lastIndex = curIndex;
                for s in _r {
                    (_, lastIndex) = s.get_leaves_helper(current_leaves, curIndex + 1);
                }
                (self, lastIndex)
            }
            TransitionID::Simple(_) | TransitionID::None => {
                if current_leaves.len() <= index {
                    current_leaves.push(Vec::new());
                }
                current_leaves[index].push(self.clone());
                (self, index)
            }
        }
    }

    pub fn split_into_component_lists(path: &Vec<TransitionID>) -> Vec<Vec<Vec<TransitionID>>> {
        if path.len() == 0 {
            return Vec::new();
        }
        let leaves = path[0].get_leaves();

        let mut paths: Vec<Vec<Vec<TransitionID>>> = vec![Vec::new(); leaves.len()];

        for transitionID in path {
            for (componentIndex, transition) in transitionID.get_leaves().iter().enumerate() {
                paths[componentIndex].push(transition.iter().cloned().filter(|id| !matches!(id, TransitionID::None)).collect());
            }
        }
        paths
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
