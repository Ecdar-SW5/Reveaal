use edbm::zones::OwnedFederation;

use crate::TransitionSystems::{LocationTuple, TransitionSystemPtr};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct StatePair {
    pub locations1: LocationTuple,
    pub locations2: LocationTuple,
    /// The sentinel (Option) allows us to take ownership of the internal fed from a mutable reference
    zone_sentinel: Option<OwnedFederation>,
}

impl StatePair {
    pub fn create(
        dimensions: usize,
        locations1: LocationTuple,
        locations2: LocationTuple,
    ) -> StatePair {
        let zone = OwnedFederation::init(dimensions);

        StatePair {
            locations1,
            locations2,
            zone_sentinel: Some(zone),
        }
    }

    pub fn get_locations1(&self) -> &LocationTuple {
        &self.locations1
    }

    pub fn get_locations2(&self) -> &LocationTuple {
        &self.locations2
    }

    //Used to allow borrowing both states as mutable
    pub fn get_mut_states(&mut self, is_states1: bool) -> (&mut LocationTuple, &mut LocationTuple) {
        if is_states1 {
            (&mut self.locations1, &mut self.locations2)
        } else {
            (&mut self.locations2, &mut self.locations1)
        }
    }

    #[allow(dead_code)]
    pub fn get_locations(&self, is_states1: bool) -> (&LocationTuple, &LocationTuple) {
        if is_states1 {
            (&self.locations1, &self.locations2)
        } else {
            (&self.locations2, &self.locations1)
        }
    }

    #[allow(dead_code)]
    pub fn clone_zone(&self) -> OwnedFederation {
        self.ref_zone().clone()
    }

    pub fn ref_zone(&self) -> &OwnedFederation {
        self.zone_sentinel.as_ref().unwrap()
    }

    pub fn take_zone(&mut self) -> OwnedFederation {
        self.zone_sentinel.take().unwrap()
    }

    pub fn set_zone(&mut self, zone: OwnedFederation) {
        self.zone_sentinel = Some(zone);
    }

    pub fn extrapolate_max_bounds(
        &mut self,
        sys1: &TransitionSystemPtr,
        sys2: &TransitionSystemPtr,
    ) {
        let mut bounds = sys1.get_local_max_bounds(&self.locations1);
        bounds.add_bounds(&sys2.get_local_max_bounds(&self.locations2));
        let zone = self.take_zone().extrapolate_max_bounds(&bounds);
        self.set_zone(zone);
    }
}

impl Display for StatePair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Pair: 1:{} where {}, 2:{} where {}, zone: {}",
            self.locations1.id,
            self.locations1
                .get_invariants()
                .map(|f| f.to_string())
                .unwrap_or_else(|| "no invariant".to_string()),
            self.locations2.id,
            self.locations2
                .get_invariants()
                .map(|f| f.to_string())
                .unwrap_or_else(|| "no invariant".to_string()),
            self.ref_zone()
        ))?;

        Ok(())
    }
}
