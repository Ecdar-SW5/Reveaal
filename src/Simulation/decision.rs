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

    pub fn resolve(&self, system: &TransitionSystemPtr) -> Vec<DecisionPoint> {
        TransitionDecision::from(self, &system)
            .into_iter()
            .filter_map(|decision| decision.resolve(&system))
            .map(|decision_point| DecisionPoint::from(&decision_point))
            .collect()
    }
}
