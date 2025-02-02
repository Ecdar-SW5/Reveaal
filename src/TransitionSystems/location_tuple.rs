use edbm::{util::constraints::ClockIndex, zones::OwnedFederation};

use crate::{
    EdgeEval::constraint_applyer::apply_constraints_to_state,
    ModelObjects::component::{Declarations, Location, LocationType},
};

use super::LocationID;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum CompositionType {
    Conjunction,
    Composition,
    Quotient,
    Simple,
}

#[derive(Clone, Debug)]
pub struct LocationTuple {
    pub id: LocationID,
    /// The invariant for the `Location`
    pub invariant: Option<OwnedFederation>,
    pub loc_type: LocationType,
    left: Option<Box<LocationTuple>>,
    right: Option<Box<LocationTuple>>,
}

impl PartialEq for LocationTuple {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.loc_type == other.loc_type
    }
}

impl LocationTuple {
    pub fn simple(
        location: &Location,
        component_id: Option<String>,
        decls: &Declarations,
        dim: ClockIndex,
    ) -> Self {
        let invariant = if let Some(inv) = location.get_invariant() {
            let mut fed = OwnedFederation::universe(dim);
            fed = apply_constraints_to_state(inv, decls, fed).unwrap();
            Some(fed)
        } else {
            None
        };
        LocationTuple {
            id: LocationID::Simple {
                location_id: location.get_id().clone(),
                component_id,
            },
            invariant,
            loc_type: location.get_location_type().clone(),
            left: None,
            right: None,
        }
    }
    /// This method is used to a build partial [`LocationTuple`].
    /// A partial [`LocationTuple`] means it has a [`LocationID`] that is [`LocationID::AnyLocation`].
    /// A partial [`LocationTuple`] has `None` in the field `invariant` since a partial [`LocationTuple`]
    /// covers more than one location, and therefore there is no specific `invariant`
    pub fn build_any_location_tuple() -> Self {
        LocationTuple {
            id: LocationID::AnyLocation(),
            invariant: None,
            loc_type: LocationType::Any,
            left: None,
            right: None,
        }
    }

    //Merge two locations keeping the invariants seperate
    pub fn merge_as_quotient(left: &Self, right: &Self) -> Self {
        let id = LocationID::Quotient(Box::new(left.id.clone()), Box::new(right.id.clone()));

        if left.loc_type == right.loc_type
            && (left.loc_type == LocationType::Universal
                || left.loc_type == LocationType::Inconsistent)
        {
            return left.clone();
        }

        let loc_type =
            if left.loc_type == LocationType::Initial && right.loc_type == LocationType::Initial {
                LocationType::Initial
            } else {
                LocationType::Normal
            };

        LocationTuple {
            id,
            invariant: None,
            loc_type,
            left: Some(Box::new(left.clone())),
            right: Some(Box::new(right.clone())),
        }
    }

    //Compose two locations intersecting the invariants
    pub fn compose(left: &Self, right: &Self, comp: CompositionType) -> Self {
        let id = match comp {
            CompositionType::Conjunction => {
                LocationID::Conjunction(Box::new(left.id.clone()), Box::new(right.id.clone()))
            }
            CompositionType::Composition => {
                LocationID::Composition(Box::new(left.id.clone()), Box::new(right.id.clone()))
            }
            _ => panic!("Invalid composition type {:?}", comp),
        };

        if left.loc_type == right.loc_type && (left.is_universal() || left.is_inconsistent()) {
            return left.clone();
        }

        let invariant = if let Some(inv1) = &left.invariant {
            if let Some(inv2) = &right.invariant {
                Some(inv1.clone().intersection(inv2))
            } else {
                Some(inv1.clone())
            }
        } else {
            right.invariant.clone()
        };

        let loc_type =
            if left.loc_type == LocationType::Initial && right.loc_type == LocationType::Initial {
                LocationType::Initial
            } else {
                LocationType::Normal
            };

        LocationTuple {
            id,
            invariant,
            loc_type,
            left: Some(Box::new(left.clone())),
            right: Some(Box::new(right.clone())),
        }
    }

    pub fn get_invariants(&self) -> Option<&OwnedFederation> {
        self.invariant.as_ref()
    }

    pub fn apply_invariants(&self, mut fed: OwnedFederation) -> OwnedFederation {
        if let Some(inv) = &self.invariant {
            fed = fed.intersection(inv);
        }
        fed
    }

    pub fn get_left(&self) -> &LocationTuple {
        if self.is_universal() || self.is_inconsistent() {
            return self;
        }
        self.left.as_ref().unwrap()
    }

    pub fn get_right(&self) -> &LocationTuple {
        if self.is_universal() || self.is_inconsistent() {
            return self;
        }
        self.right.as_ref().unwrap()
    }

    pub fn is_initial(&self) -> bool {
        self.loc_type == LocationType::Initial
    }

    pub fn is_universal(&self) -> bool {
        self.loc_type == LocationType::Universal
    }

    pub fn is_inconsistent(&self) -> bool {
        self.loc_type == LocationType::Inconsistent
    }

    /// This function is used when you want to compare [`LocationTuple`]s that can contain partial locations.
    pub fn compare_partial_locations(&self, other: &LocationTuple) -> bool {
        match (&self.id, &other.id) {
            (LocationID::Composition(..), LocationID::Composition(..))
            | (LocationID::Conjunction(..), LocationID::Conjunction(..))
            | (LocationID::Quotient(..), LocationID::Quotient(..)) => {
                self.get_left().compare_partial_locations(other.get_left())
                    && self
                        .get_right()
                        .compare_partial_locations(other.get_right())
            }
            (LocationID::AnyLocation(), LocationID::Simple { .. })
            | (LocationID::Simple { .. }, LocationID::AnyLocation())
            | (LocationID::AnyLocation(), LocationID::AnyLocation()) => true,
            (
                LocationID::Simple {
                    location_id: loc_id_1,
                    component_id: comp_id_1,
                },
                LocationID::Simple {
                    location_id: loc_id_2,
                    component_id: comp_id_2,
                },
            ) => loc_id_1 == loc_id_2 && comp_id_1 == comp_id_2,
            // These six arms below are for comparing universal or inconsistent location with partial location.
            (LocationID::Simple { .. }, LocationID::Composition(..))
            | (LocationID::Simple { .. }, LocationID::Conjunction(..))
            | (LocationID::Simple { .. }, LocationID::Quotient(..)) => {
                self.handle_universal_inconsistent_compare(other)
            }
            (LocationID::Composition(..), LocationID::Simple { .. })
            | (LocationID::Conjunction(..), LocationID::Simple { .. })
            | (LocationID::Quotient(..), LocationID::Simple { .. }) => {
                other.handle_universal_inconsistent_compare(self)
            }
            (_, _) => false,
        }
    }

    fn handle_universal_inconsistent_compare(&self, other: &LocationTuple) -> bool {
        (self.is_universal() || self.is_inconsistent())
            && other.is_universal_or_inconsistent(&self.loc_type)
    }

    fn is_universal_or_inconsistent(&self, loc_type: &LocationType) -> bool {
        match self.id {
            LocationID::Conjunction(..)
            | LocationID::Composition(..)
            | LocationID::Quotient(..) => {
                self.get_left().is_universal_or_inconsistent(loc_type)
                    && self.get_right().is_universal_or_inconsistent(loc_type)
            }
            LocationID::Simple { .. } => self.loc_type == *loc_type,
            LocationID::AnyLocation() => true,
        }
    }
}
