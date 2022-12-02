use crate::{
    component::{Edge, State},
    TransitionSystems::TransitionSystemPtr,
};

use super::{decision_point::DecisionPoint, transition_decision::TransitionDecision};

/// Represent a decision in a any composition of components, that has been taken: In the current `source` state I have `decided` to use this [`Edge`].
#[derive(Debug)]
pub struct Decision {
    source: State,
    decided: Edge,
}

impl Decision {
    pub fn new(source: State, decided: Edge) -> Self {
        Self { source, decided }
    }

    pub fn source(&self) -> &State {
        &self.source
    }

    pub fn decided(&self) -> &Edge {
        &self.decided
    }

    /// Resolves a [`Decision`]: use the `decided` [`Edge`] and returns a [`Vec`] of the [`DecisionPoint`]s of the destination [`State`]s.
    ///
    /// Some `decided` [`Edge`]s lead to ambiguity ie. they correspond to multiple [`Transition`]s. Thus one [`Edge`] can lead to multiple [`State`]s.  
    pub fn resolve(&self, system: &TransitionSystemPtr) -> Vec<DecisionPoint> {
        TransitionDecision::from(self, system)
            .into_iter()
            .filter_map(|transition_decision| transition_decision.resolve(system))
            .map(|transition_decision_point| DecisionPoint::from(&transition_decision_point))
            .collect()
    }
}
