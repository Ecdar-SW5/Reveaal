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

    /// Does an inorder walk of the [`LocationID`] tree mapping it to a list of [`LocationID::Simple`].
    pub fn inorder_vec_tranform(&self) -> Vec<Self> {
        match self {
            LocationID::Composition(left, right)
            | LocationID::Quotient(left, right)
            | LocationID::Conjunction(left, right) => {
                let mut left = left.inorder_vec_tranform();
                let mut right = right.inorder_vec_tranform();
                left.append(&mut right);
                left
            }
            LocationID::Simple {
                location_id,
                component_id,
            } => vec![LocationID::Simple {
                location_id: location_id.to_string(),
                component_id: component_id.as_ref().map(|x| x.to_string()),
            }],
            LocationID::AnyLocation() => vec![LocationID::AnyLocation()],
        }
    }

    /// This function is used when you want to compare a [`LocationID`] containing a partial location [`LocationID::AnyLocation`] with another [`LocationID`].
    /// [`LocationID::AnyLocation`] should always be true when compared to [`LocationID::Simple`]
    /// ```
    /// use reveaal::TransitionSystems::LocationID;
    /// // Make two locations where `a` has LocationID::AnyLocation
    /// let a = LocationID::Quotient(Box::new(LocationID::Simple { location_id: "L5".to_string(),  component_id: None } ),
    ///                              Box::new(LocationID::AnyLocation()));
    ///
    /// let b = LocationID::Quotient(Box::new(LocationID::Simple { location_id: "L5".to_string(),  component_id: None } ),
    ///                              Box::new(LocationID::Simple { location_id: "L1".to_string(),  component_id: None } ));
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
            (
                LocationID::AnyLocation(),
                LocationID::Simple {
                    location_id: _,
                    component_id: _,
                },
            )
            | (
                LocationID::Simple {
                    location_id: _,
                    component_id: _,
                },
                LocationID::AnyLocation(),
            ) => true,
            (LocationID::AnyLocation(), LocationID::AnyLocation()) => true,
            (
                LocationID::Simple {
                    location_id: location_id_1,
                    component_id: component_id_1,
                },
                LocationID::Simple {
                    location_id: location_id_2,
                    component_id: component_id_2,
                },
            ) => location_id_1 == location_id_2 && component_id_1 == component_id_2,
            (_, _) => false,
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
            LocationID::Simple {
                location_id: _,
                component_id: _,
            } => false,
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
