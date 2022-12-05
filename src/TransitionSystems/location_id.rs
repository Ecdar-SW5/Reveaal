use std::fmt::{Display, Formatter};

use crate::ModelObjects::representations::QueryExpression;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum LocationID {
    Conjunction(Box<LocationID>, Box<LocationID>),
    Composition(Box<LocationID>, Box<LocationID>),
    Quotient(Box<LocationID>, Box<LocationID>),
    /// Represents the potentially complete identifier of a location
    Simple {
        location_id: String,
        component_id: Option<String>,
    },
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

    /// It check whether the [`LocationID`] is a partial location by search through [`LocationID`] structure and see if there is any [`LocationID::AnyLocation`]
    pub fn is_partial_location(&self) -> bool {
        match self {
            LocationID::Composition(left, right)
            | LocationID::Conjunction(left, right)
            | LocationID::Quotient(left, right) => {
                left.is_partial_location() || right.is_partial_location()
            }
            LocationID::Simple { .. } => false,
            LocationID::AnyLocation() => true,
        }
    }

    ///Gets the component_id of from a [`LocationID::Simple`] returns a clone.
    pub fn get_component_id(&self) -> Option<String> {
        if let LocationID::Simple {
            location_id: _,
            component_id,
        } = self
        {
            component_id.clone()
        } else {
            None
        }
    }

    pub fn get_unique_string(&self) -> String {
        match self {
            LocationID::Composition(a, b) => {
                format!("({}||{})", a.get_unique_string(), b.get_unique_string())
            }
            LocationID::Conjunction(a, b) => {
                format!("({}&&{})", a.get_unique_string(), b.get_unique_string())
            }
            LocationID::Quotient(a, b) => {
                format!("({}\\{})", a.get_unique_string(), b.get_unique_string())
            }
            LocationID::AnyLocation() => "_".to_string(),
            LocationID::Simple {
                location_id,
                component_id,
            } => format!(
                "{}.{}",
                component_id.clone().unwrap_or_else(|| "(None)".to_string()),
                location_id
            ),
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
            QueryExpression::VarName(name) => LocationID::Simple {
                location_id: name,
                component_id: None,
            },
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
                match **left {
                    LocationID::Conjunction(_, _) => write!(f, "{}", (*left))?,
                    LocationID::Simple {
                        location_id: _,
                        component_id: _,
                    } => write!(f, "{}", (*left))?,
                    _ => write!(f, "({})", (*left))?,
                };
                write!(f, "&&")?;
                match **right {
                    LocationID::Conjunction(_, _) => write!(f, "{}", (*right))?,
                    LocationID::Simple {
                        location_id: _,
                        component_id: _,
                    } => write!(f, "{}", (*right))?,
                    _ => write!(f, "({})", (*right))?,
                };
            }
            LocationID::Composition(left, right) => {
                match **left {
                    LocationID::Composition(_, _) => write!(f, "{}", (*left))?,
                    LocationID::Simple {
                        location_id: _,
                        component_id: _,
                    } => write!(f, "{}", (*left))?,
                    _ => write!(f, "({})", (*left))?,
                };
                write!(f, "||")?;
                match **right {
                    LocationID::Composition(_, _) => write!(f, "{}", (*right))?,
                    LocationID::Simple {
                        location_id: _,
                        component_id: _,
                    } => write!(f, "{}", (*right))?,
                    _ => write!(f, "({})", (*right))?,
                };
            }
            LocationID::Quotient(left, right) => {
                match **left {
                    LocationID::Simple {
                        location_id: _,
                        component_id: _,
                    } => write!(f, "{}", (*left))?,
                    _ => write!(f, "({})", (*left))?,
                };
                write!(f, "\\\\")?;
                match **right {
                    LocationID::Simple {
                        location_id: _,
                        component_id: _,
                    } => write!(f, "{}", (*right))?,
                    _ => write!(f, "({})", (*right))?,
                };
            }
            LocationID::Simple {
                location_id,
                component_id: _,
            } => {
                write!(f, "{}", location_id)?;
            }
            LocationID::AnyLocation() => write!(f, "_")?,
        }
        Ok(())
    }
}
