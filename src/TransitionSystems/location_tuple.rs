use edbm::{util::constraints::ClockIndex, zones::OwnedFederation};

use crate::{
    EdgeEval::constraint_applyer::apply_constraints_to_state,
    ModelObjects::component::{Declarations, Location, LocationType},
};

use crate::ProtobufServer::services::LocationTuple as ProtoLocationTuple;
use crate::TransitionSystems::TransitionSystemPtr;

use super::LocationID;

#[derive(Debug, Clone, std::cmp::PartialEq, Eq, Hash, Copy)]
pub enum CompositionType {
    Conjunction,
    Composition,
    Quotient,
    Simple,
}

#[derive(Clone, Debug)]
pub struct LocationTuple {
    pub id: LocationID,
    invariant: Option<OwnedFederation>,
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

    pub fn from_proto_location_tuple(
        location_tuple: &ProtoLocationTuple,
        system: &TransitionSystemPtr,
    ) -> Option<Self> {
        let id_looking_for: Vec<LocationID> = location_tuple
            .locations
            .iter()
            .map(|l| LocationID::Simple {
                location_id: l.id.to_string(),
                component_id: l
                    .specific_component
                    .as_ref()
                    .map(|c| c.component_name.to_string()),
            })
            .collect();

        system
            .get_all_locations()
            .into_iter()
            .map(|tuple| (tuple.id.clone(), tuple))
            .map(|(id, tuple)| (id.inorder_vec_tranform(), tuple))
            .filter(|(id, _)| id.iter().eq(id_looking_for.iter()))
            .collect::<Vec<_>>()
            .first()
            .map(|(_, tuple)| tuple.to_owned())
    }

    /// This method is used to a create partial [`LocationTuple`].
    /// A partial [`LocationTuple`] means it has a [`LocationID`] that consists of atleast one [`LocationID::AnyLocation`].
    /// A partial [`LocationTuple`] has `None` in these fields: `invariant`, `left` and `right` since a partial [`LocationTuple`]
    /// covers more than one [`LocationTuple`], and therefore there is no specific `invariant`, `left` and `right`
    pub fn create_partial_location(id: LocationID) -> Self {
        LocationTuple {
            id,
            invariant: None,
            loc_type: crate::component::LocationType::Normal,
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
}
