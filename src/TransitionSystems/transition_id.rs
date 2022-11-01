use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum TransitionID {
    Conjunction(Box<TransitionID>, Box<TransitionID>),
    Composition(Box<TransitionID>, Box<TransitionID>),
    Quotient(Box<TransitionID>, Box<TransitionID>),
    Simple(String),
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
            TransitionID::Quotient(left, right) => {
                match *(*left) {
                    TransitionID::Simple(_) => write!(f, "{}", (*left))?,
                    _ => write!(f, "({})", (*left))?,
                };
                write!(f, "\\\\")?;
                match *(*right) {
                    TransitionID::Simple(_) => write!(f, "{}", (*right))?,
                    _ => write!(f, "({})", (*right))?,
                };
            }
            TransitionID::Simple(name) => write!(f, "{}", name)?,
        }
        Ok(())
    }
}
