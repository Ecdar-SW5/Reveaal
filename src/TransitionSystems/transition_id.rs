use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum TransitionID {
    Conjunction(Box<TransitionID>, Box<TransitionID>),
    Composition(Box<TransitionID>, Box<TransitionID>),
    Quotient(i32,Vec<TransitionID>, Vec<TransitionID>),
    Simple(String),
    None,
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
