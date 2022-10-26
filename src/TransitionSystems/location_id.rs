use std::fmt::{Display, Formatter};

use crate::ModelObjects::representations::QueryExpression;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum LocationID {
    Conjunction(Box<LocationID>, Box<LocationID>),
    Composition(Box<LocationID>, Box<LocationID>),
    Quotient(Box<LocationID>, Box<LocationID>),
    Simple(String),
    /// Used for representing a partial state and it is generated when a location's name is set as `_`
    AnyLocation(),
}

impl LocationID {
    /// A debug method to construct a location ID from a string.
    /// e.g. "A" -> Simple("A"), "A && B" -> LocationID::Conjunction(Simple("A"), Simple("B")), etc.
    pub fn from_string(string: &str) -> Self {
        // A bit of a hack but we use the parser get the a query expression from which we can
        // determine to composition types needed to construct the location ID
        // TODO: This is a bit of a hack, but it works for now.
        let query = crate::DataReader::parse_queries::parse_to_expression_tree(&format!(
            "consistency: {}",
            string
        ))
        .unwrap()
        .remove(0);

        match query {
            QueryExpression::Consistency(x) => (*x).into(),
            _ => unreachable!(),
        }
    }

    /// This function is used when you want to compare a [`LocationID`] containing a partial location [`LocationID::AnyLocation`] with another [`LocationID`].
    /// [`LocationID::AnyLocation`] should always be true when compared to [`LocationID::Simple`]
    /// ```
    /// use reveaal::TransitionSystems::LocationID;
    /// // Make two locations where `a` has LocationID::AnyLocation
    /// let a = LocationID::Quotient(Box::new(LocationID::Simple("L5".to_string())),
    ///                              Box::new(LocationID::AnyLocation()));
    ///
    /// let b = LocationID::Quotient(Box::new(LocationID::Simple("L5".to_string())),
    ///                              Box::new(LocationID::Simple("L1".to_string())));
    ///
    /// assert!(a.compare_partial_locations(&b));
    /// ```
    pub fn compare_partial_locations(&self, other: &Self) -> bool {
        match (self, other) {
            (
                LocationID::Composition(self_left, self_right),
                LocationID::Composition(other_left, other_right),
            )
            | (
                LocationID::Conjunction(self_left, self_right),
                LocationID::Conjunction(other_left, other_right),
            )
            | (
                LocationID::Quotient(self_left, self_right),
                LocationID::Quotient(other_left, other_right),
            ) => {
                self_left.compare_partial_locations(other_left)
                    && self_right.compare_partial_locations(other_right)
            }
            (LocationID::AnyLocation(), LocationID::Simple(_))
            | (LocationID::Simple(_), LocationID::AnyLocation())
            | (LocationID::AnyLocation(), LocationID::AnyLocation()) => true,
            (LocationID::Simple(loc1), LocationID::Simple(loc2)) => loc1 == loc2,
            (_, _) => false,
        }
    }

    pub fn is_partial_location(&self) -> bool {
        match self {
            LocationID::Composition(left, right)
            | LocationID::Conjunction(left, right)
            | LocationID::Quotient(left, right) => {
                left.is_partial_location() || right.is_partial_location()
            }
            LocationID::Simple(_) => false,
            LocationID::AnyLocation() => true,
        }
    }
}

impl From<QueryExpression> for LocationID {
    fn from(item: QueryExpression) -> Self {
        match item {
            QueryExpression::Conjunction(left, right) => {
                LocationID::Conjunction(Box::new((*left).into()), Box::new((*right).into()))
            }
            QueryExpression::Composition(left, right) => {
                LocationID::Composition(Box::new((*left).into()), Box::new((*right).into()))
            }
            QueryExpression::Quotient(left, right) => {
                LocationID::Quotient(Box::new((*left).into()), Box::new((*right).into()))
            }
            QueryExpression::Parentheses(inner) => (*inner).into(),
            QueryExpression::VarName(name) => LocationID::Simple(name),
            _ => panic!(
                "Cannot convert queryexpression with {:?} to LocationID",
                item
            ),
        }
    }
}

impl Display for LocationID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationID::Conjunction(left, right) => {
                match *(*left) {
                    LocationID::Conjunction(_, _) => write!(f, "{}", (*left))?,
                    LocationID::Simple(_) => write!(f, "{}", (*left))?,
                    _ => write!(f, "({})", (*left))?,
                };
                write!(f, "&&")?;
                match *(*right) {
                    LocationID::Conjunction(_, _) => write!(f, "{}", (*right))?,
                    LocationID::Simple(_) => write!(f, "{}", (*right))?,
                    _ => write!(f, "({})", (*right))?,
                };
            }
            LocationID::Composition(left, right) => {
                match *(*left) {
                    LocationID::Composition(_, _) => write!(f, "{}", (*left))?,
                    LocationID::Simple(_) => write!(f, "{}", (*left))?,
                    _ => write!(f, "({})", (*left))?,
                };
                write!(f, "||")?;
                match *(*right) {
                    LocationID::Composition(_, _) => write!(f, "{}", (*right))?,
                    LocationID::Simple(_) => write!(f, "{}", (*right))?,
                    _ => write!(f, "({})", (*right))?,
                };
            }
            LocationID::Quotient(left, right) => {
                match *(*left) {
                    LocationID::Simple(_) => write!(f, "{}", (*left))?,
                    _ => write!(f, "({})", (*left))?,
                };
                write!(f, "\\\\")?;
                match *(*right) {
                    LocationID::Simple(_) => write!(f, "{}", (*right))?,
                    _ => write!(f, "({})", (*right))?,
                };
            }
            LocationID::Simple(name) => write!(f, "{}", name)?,
            LocationID::AnyLocation() => write!(f, "_")?,
        }
        Ok(())
    }
}
